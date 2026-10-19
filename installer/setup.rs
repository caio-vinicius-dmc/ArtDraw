#![windows_subsystem = "windows"]
use std::os::windows::process::CommandExt;
use std::{
    env, fs, io,
    path::{Path, PathBuf},
    process::Command,
};
const APP: &[u8] = include_bytes!("../target/release/coreldraw_clone.exe");
const UNINSTALLER: &[u8] = include_bytes!("../dist/Uninstall-ArtDraw.exe");
const README: &[u8] = include_bytes!("../README.md");
const LICENSE: &[u8] = include_bytes!("../LICENSE");
const NOTICES: &[u8] = include_bytes!("../dist/THIRD_PARTY_NOTICES.txt");
const DOCS: [(&str, &[u8]); 4] = [
    (
        "CORELDRAW_PARITY.md",
        include_bytes!("../docs/CORELDRAW_PARITY.md"),
    ),
    (
        "FEATURE_MATRIX.md",
        include_bytes!("../docs/FEATURE_MATRIX.md"),
    ),
    (
        "FEATURE_MATRIX.csv",
        include_bytes!("../docs/FEATURE_MATRIX.csv"),
    ),
    ("ROADMAP.md", include_bytes!("../docs/ROADMAP.md")),
];
#[link(name = "user32")]
extern "system" {
    fn MessageBoxW(
        window: *mut std::ffi::c_void,
        text: *const u16,
        title: *const u16,
        kind: u32,
    ) -> i32;
}
fn message(text: &str, error: bool) -> i32 {
    let wide: Vec<u16> = text.encode_utf16().chain(Some(0)).collect();
    let title: Vec<u16> = "Instalador ArtDraw".encode_utf16().chain(Some(0)).collect();
    unsafe {
        MessageBoxW(
            std::ptr::null_mut(),
            wide.as_ptr(),
            title.as_ptr(),
            if error { 0x10 } else { 0x40 },
        )
    }
}
fn powershell(script: &str, dir: &Path) -> io::Result<()> {
    let shell = PathBuf::from(env::var_os("SystemRoot").unwrap_or_else(|| "C:\\Windows".into()))
        .join("System32/WindowsPowerShell/v1.0/powershell.exe");
    let output = Command::new(shell)
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            script,
        ])
        .env("ARTDRAW_INSTALL_DIR", dir)
        .creation_flags(0x08000000)
        .output()?;
    if output.status.success() {
        Ok(())
    } else {
        Err(io::Error::other(
            String::from_utf8_lossy(&output.stderr).into_owned(),
        ))
    }
}
fn install(dir: &Path, register: bool) -> io::Result<()> {
    fs::create_dir_all(dir)?;
    for (name, data) in [
        ("coreldraw_clone.exe", APP),
        ("Uninstall-ArtDraw.exe", UNINSTALLER),
        ("README.md", README),
        ("LICENSE", LICENSE),
        ("THIRD_PARTY_NOTICES.txt", NOTICES),
        (".artdraw-install", b"ArtDraw 1.0" as &[u8]),
    ] {
        fs::write(dir.join(name), data)?;
    }
    fs::create_dir_all(dir.join("docs"))?;
    for (name, data) in DOCS {
        fs::write(dir.join("docs").join(name), data)?;
    }
    if register {
        powershell(
            r#"
$ErrorActionPreference='Stop'
$dir=$env:ARTDRAW_INSTALL_DIR
$shell=New-Object -ComObject WScript.Shell
$link=$shell.CreateShortcut((Join-Path ([Environment]::GetFolderPath('Programs')) 'ArtDraw.lnk'))
$link.TargetPath=Join-Path $dir 'coreldraw_clone.exe'
$link.WorkingDirectory=$dir
$link.Description='ArtDraw — editor vetorial offline'
$link.Save()
$key='HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\ArtDraw'
New-Item -Path $key -Force | Out-Null
$values=@{DisplayName='ArtDraw';DisplayVersion='1.1.0';Publisher='ArtDraw';InstallLocation=$dir;DisplayIcon=(Join-Path $dir 'coreldraw_clone.exe');UninstallString=('"'+(Join-Path $dir 'Uninstall-ArtDraw.exe')+'"')}
foreach($name in $values.Keys){New-ItemProperty -LiteralPath $key -Name $name -Value $values[$name] -PropertyType String -Force | Out-Null}
New-ItemProperty -LiteralPath $key -Name NoModify -Value 1 -PropertyType DWord -Force | Out-Null
New-ItemProperty -LiteralPath $key -Name NoRepair -Value 1 -PropertyType DWord -Force | Out-Null
"#,
            dir,
        )?;
    }
    Ok(())
}
fn main() {
    let args: Vec<_> = env::args_os().collect();
    let silent = args.iter().any(|s| s == "--silent");
    let extract = args.iter().position(|s| s == "--extract-to");
    let target = if let Some(i) = extract {
        match args.get(i + 1) {
            Some(p) => PathBuf::from(p),
            None => {
                std::process::exit(2);
            }
        }
    } else {
        match env::var_os("LOCALAPPDATA") {
            Some(p) => PathBuf::from(p).join("Programs/ArtDraw"),
            None => {
                message("Não foi possível localizar a pasta do usuário.", true);
                std::process::exit(1);
            }
        }
    };
    match install(&target, extract.is_none()) {
        Ok(()) => {
            if !silent {
                message(&format!("ArtDraw instalado com sucesso.\n\n{}\n\nAbra pelo menu Iniciar ou pelo executável nesta pasta.",target.display()),false);
            }
        }
        Err(e) => {
            if !silent {
                message(&format!("Falha na instalação: {e}\n\nFeche o ArtDraw, se estiver aberto, e tente novamente."),true);
            }
            std::process::exit(1);
        }
    }
}
