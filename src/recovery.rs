//! Per-process recovery checkpoints; never replace the user's chosen document.
use crate::{document::Document, storage};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::mpsc::{self, Receiver},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
const INTERVAL: Duration = Duration::from_secs(15);
pub struct Candidate {
    pub path: PathBuf,
    pub modified: SystemTime,
    pub document: Document,
}
pub struct Recovery {
    dir: PathBuf,
    path: PathBuf,
    last: Option<Document>,
    last_attempt: Instant,
    pending: Option<Receiver<Result<Document, String>>>,
    inherited: Option<PathBuf>,
}
fn owner_alive(path: &Path) -> bool {
    let pid = path
        .file_stem()
        .and_then(|s| s.to_str())
        .and_then(|s| s.strip_prefix("recovery-"))
        .and_then(|s| s.split('-').next())
        .and_then(|s| s.parse::<u32>().ok());
    let Some(pid) = pid else {
        return true;
    };
    if pid == std::process::id() {
        return true;
    }
    #[cfg(windows)]
    unsafe {
        #[link(name = "kernel32")]
        extern "system" {
            fn OpenProcess(access: u32, inherit: i32, pid: u32) -> isize;
            fn CloseHandle(handle: isize) -> i32;
            fn GetExitCodeProcess(handle: isize, code: *mut u32) -> i32;
        }
        let handle = OpenProcess(0x1000, 0, pid);
        if handle == 0 {
            return std::io::Error::last_os_error().raw_os_error() != Some(87);
        }
        let mut code = 259;
        GetExitCodeProcess(handle, &mut code);
        CloseHandle(handle);
        code == 259
    }
    #[cfg(not(windows))]
    {
        false
    }
}
impl Recovery {
    pub fn new() -> Option<Self> {
        let dir = std::env::var_os("ARTDRAW_RECOVERY_DIR")
            .map(PathBuf::from)
            .or_else(|| {
                std::env::var_os("LOCALAPPDATA").map(|p| PathBuf::from(p).join("ArtDraw/Recovery"))
            })?;
        Some(Self::at(dir))
    }
    fn at(dir: PathBuf) -> Self {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let path = dir.join(format!("recovery-{}-{stamp}.artdraw", std::process::id()));
        Self {
            dir,
            path,
            last: None,
            last_attempt: Instant::now() - INTERVAL,
            pending: None,
            inherited: None,
        }
    }
    pub fn candidates(&self) -> Vec<Candidate> {
        let mut items = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path == self.path
                    || !path.extension().is_some_and(|e| e == "artdraw")
                    || owner_alive(&path)
                {
                    continue;
                }
                if let Ok((document, _)) = storage::read(&path) {
                    items.push(Candidate {
                        path,
                        document,
                        modified: entry
                            .metadata()
                            .and_then(|m| m.modified())
                            .unwrap_or(UNIX_EPOCH),
                    });
                }
            }
        }
        items.sort_by_key(|c| std::cmp::Reverse(c.modified));
        items.truncate(20);
        items
    }
    pub fn chose(&mut self, path: PathBuf) {
        self.inherited = Some(path);
    }
    pub fn tick(&mut self, doc: &Document, dirty: bool) -> Option<String> {
        if let Some(rx) = self.pending.as_ref() {
            match rx.try_recv() {
                Ok(result) => {
                    self.pending = None;
                    match result {
                        Ok(d) => self.last = Some(d),
                        Err(e) => return Some(e),
                    }
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.pending = None;
                    return Some("Falha no serviço de recuperação".into());
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }
        }
        if !dirty {
            if self.pending.is_none() && self.last.is_some() {
                self.clear_current();
            }
            return None;
        }
        if self.pending.is_some()
            || self.last.as_ref() == Some(doc)
            || self.last_attempt.elapsed() < INTERVAL
        {
            return None;
        }
        self.last_attempt = Instant::now();
        let path = self.path.clone();
        let snapshot = doc.clone();
        let (tx, rx) = mpsc::channel();
        match std::thread::Builder::new()
            .name("ArtDraw recovery".into())
            .spawn(move || {
                let result = (|| {
                    fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;
                    storage::atomic_write(
                        &path,
                        storage::validated_encode(&snapshot)?.as_bytes(),
                        false,
                    )?;
                    Ok(snapshot)
                })();
                let _ = tx.send(result);
            }) {
            Ok(_) => self.pending = Some(rx),
            Err(e) => return Some(e.to_string()),
        }
        None
    }
    fn finish_pending(&mut self) {
        if let Some(rx) = self.pending.take() {
            if let Ok(Ok(doc)) = rx.recv_timeout(Duration::from_secs(5)) {
                self.last = Some(doc);
            }
        }
    }
    /// Called only after explicit save or confirmed normal close/new/open.
    pub fn clear_current(&mut self) {
        self.finish_pending();
        let _ = fs::remove_file(&self.path);
        self.last = None;
        self.last_attempt = Instant::now() - INTERVAL;
    }
    pub fn saved(&mut self) {
        self.clear_current();
        if let Some(path) = self.inherited.take() {
            if path.parent() == Some(self.dir.as_path())
                && path
                    .file_name()
                    .is_some_and(|s| s.to_string_lossy().starts_with("recovery-"))
            {
                let _ = fs::remove_file(path);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    // Windows may finish the directory entry removal after remove_file succeeds.
    // Only retry removing the empty test directory; never delete unexpected files.
    fn remove_test_dir(dir: PathBuf) {
        for _ in 0..100 {
            match fs::remove_dir(&dir) {
                Ok(()) => return,
                Err(e) if e.kind() == std::io::ErrorKind::DirectoryNotEmpty => {
                    std::thread::sleep(Duration::from_millis(10));
                }
                Err(e) => panic!("Falha ao remover pasta de teste {}: {e}", dir.display()),
            }
        }
        panic!("Pasta de teste permaneceu não vazia: {}", dir.display());
    }
    #[test]
    fn checkpoint_roundtrip_then_explicit_save_removes_it() {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target/test-work")
            .join(format!(
                "artdraw-recovery-{}-{}",
                std::process::id(),
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
        let mut r = Recovery::at(dir.clone());
        let mut d = Document::demo();
        d.add_page(true).unwrap();
        d.name = "Recuperar segunda página".into();
        assert!(r.tick(&d, true).is_none());
        r.finish_pending();
        assert_eq!(storage::read(&r.path).unwrap().0, d);
        r.saved();
        assert!(!r.path.exists());
        remove_test_dir(dir);
    }
    #[test]
    fn live_instance_is_not_offered_as_crash_recovery() {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target/test-work")
            .join(format!(
                "artdraw-live-{}-{}",
                std::process::id(),
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
        let mut r = Recovery::at(dir.clone());
        r.tick(&Document::demo(), true);
        r.finish_pending();
        let other = Recovery::at(dir.clone());
        assert!(other.candidates().is_empty());
        r.saved();
        remove_test_dir(dir);
    }

    #[test]
    fn abandoned_checkpoint_is_listed_and_preserved_until_save() {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target/test-work")
            .join(format!(
                "artdraw-abandoned-{}-{}",
                std::process::id(),
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("recovery-0-fixture.artdraw");
        let doc = Document::demo();
        storage::save(&path, &doc).unwrap();
        let mut r = Recovery::at(dir.clone());
        let items = r.candidates();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].document, doc);
        r.chose(path.clone());
        assert!(path.exists());
        r.saved();
        assert!(!path.exists());
        remove_test_dir(dir);
    }
    #[test]
    fn undo_to_saved_state_retires_stale_checkpoint() {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target/test-work")
            .join(format!(
                "artdraw-undo-{}-{}",
                std::process::id(),
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
        let mut r = Recovery::at(dir.clone());
        r.tick(&Document::demo(), true);
        r.finish_pending();
        assert!(r.path.exists());
        r.tick(&Document::default(), false);
        assert!(!r.path.exists());
        remove_test_dir(dir);
    }
}
