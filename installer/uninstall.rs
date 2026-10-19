#![windows_subsystem = "windows"]
use std::os::windows::process::CommandExt;
use std::{env, fs, path::PathBuf, process::Command};
#[link(name = "user32")]
extern "system" {
    fn MessageBoxW(
        window: *mut std::ffi::c_void,
        text: *const u16,
        title: *const u16,
        kind: u32,
    ) -> i32;
}
fn message(text: &str, kind: u32) -> i32 {
    let t: Vec<u16> = text.encode_utf16().chain(Some(0)).collect();
    let h: Vec<u16> = "ArtDraw".encode_utf16().chain(Some(0)).collect();
    unsafe { MessageBoxW(std::ptr::null_mut(), t.as_ptr(), h.as_ptr(), kind) }
}
fn main() {
    let silent = env::args().any(|s| s == "--silent");
    let Ok(exe) = env::current_exe() else {
        return;
    };
    let Some(dir) = exe.parent() else {
        return;
    };
    if fs::read_to_string(dir.join(".artdraw-install"))
        .ok()
        .as_deref()
        != Some("ArtDraw 1.0")
    {
        if !silent {
            message(
                "Esta pasta não contém uma instalação válida do ArtDraw.",
                0x10,
            );
        }
        std::process::exit(1);
    }
    if !silent
        && message(
            "Desinstalar o ArtDraw? Seus documentos SVG serão preservados.",
            0x24,
        ) != 6
    {
        return;
    }
    let app = dir.join("coreldraw_clone.exe");
    if app.exists() {
        if let Err(e) = fs::remove_file(&app) {
            if !silent {
                message(&format!("Feche o ArtDraw antes de desinstalar.\n{e}"), 0x10);
            }
            std::process::exit(1);
        }
    }
    let shell = PathBuf::from(env::var_os("SystemRoot").unwrap_or_else(|| "C:\\Windows".into()))
        .join("System32/WindowsPowerShell/v1.0/powershell.exe");
    let script = r#"
$ErrorActionPreference='Stop'
$dir=[IO.Path]::GetFullPath($env:ARTDRAW_UNINSTALL_DIR)
$self=[IO.Path]::GetFullPath($env:ARTDRAW_UNINSTALL_EXE)
if((Split-Path -Parent $self) -ne $dir -or (Split-Path -Leaf $self) -ne 'Uninstall-ArtDraw.exe'){exit 1}
$key='HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\ArtDraw'
$installed=(Get-ItemProperty -LiteralPath $key -ErrorAction SilentlyContinue).InstallLocation
if($installed -eq $dir){
    Remove-Item -LiteralPath (Join-Path ([Environment]::GetFolderPath('Programs')) 'ArtDraw.lnk') -Force -ErrorAction SilentlyContinue
    Remove-Item -LiteralPath $key -Force -ErrorAction SilentlyContinue
}
foreach($name in @('README.md','LICENSE','THIRD_PARTY_NOTICES.txt','.artdraw-install')){Remove-Item -LiteralPath (Join-Path $dir $name) -Force -ErrorAction SilentlyContinue}
foreach($name in @('CORELDRAW_PARITY.md','FEATURE_MATRIX.md','FEATURE_MATRIX.csv','ROADMAP.md')){Remove-Item -LiteralPath (Join-Path $dir ('docs/'+$name)) -Force -ErrorAction SilentlyContinue}
$docs=Join-Path $dir 'docs'
if((Test-Path -LiteralPath $docs) -and (Get-ChildItem -LiteralPath $docs -Force | Measure-Object).Count -eq 0){Remove-Item -LiteralPath $docs -Force}
Start-Sleep -Milliseconds 900
Remove-Item -LiteralPath $self -Force -ErrorAction SilentlyContinue
if((Get-ChildItem -LiteralPath $dir -Force | Measure-Object).Count -eq 0){Remove-Item -LiteralPath $dir -Force}
"#;
    match Command::new(shell)
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            script,
        ])
        .env("ARTDRAW_UNINSTALL_DIR", dir)
        .env("ARTDRAW_UNINSTALL_EXE", &exe)
        .creation_flags(0x08000000)
        .spawn()
    {
        Ok(_) => {}
        Err(e) => {
            if !silent {
                message(
                    &format!("Não foi possível finalizar a desinstalação: {e}"),
                    0x10,
                );
            }
            std::process::exit(1);
        }
    }
}
