use std::{env, path::PathBuf, process::Command};

fn msvc_resource_compiler() -> PathBuf {
    if let Some(path) = env::var_os("ARTDRAW_RC") {
        return path.into();
    }
    // SDK binaries are not necessarily on PATH outside a Developer PowerShell.
    if let Some(base) = env::var_os("ProgramFiles(x86)") {
        let bin = PathBuf::from(base).join("Windows Kits/10/bin");
        if let Ok(entries) = std::fs::read_dir(bin) {
            let mut versions: Vec<_> = entries.filter_map(Result::ok).map(|e| e.path()).collect();
            versions.sort();
            for version in versions.into_iter().rev() {
                let rc = version.join("x64/rc.exe");
                if rc.is_file() {
                    return rc;
                }
            }
        }
    }
    "rc.exe".into()
}

fn main() {
    for name in [
        "ARTDRAW_WINDOWS_IMPORT_LIBS",
        "ARTDRAW_WINDRES",
        "ARTDRAW_RC",
        "ProgramFiles(x86)",
    ] {
        println!("cargo:rerun-if-env-changed={name}");
    }
    for path in [
        "assets/app.rc",
        "assets/windows.manifest",
        "assets/branding/app.ico",
    ] {
        println!("cargo:rerun-if-changed={path}");
    }
    if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("Cargo OUT_DIR"));
        let msvc = env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc");
        let output = out_dir.join(if msvc { "branding.res" } else { "branding.o" });
        let mut command = if msvc {
            let mut rc = Command::new(msvc_resource_compiler());
            rc.args(["/nologo", "/I", ".", "/fo"])
                .arg(&output)
                .arg("assets/app.rc");
            rc
        } else {
            let compiler = env::var_os("ARTDRAW_WINDRES").unwrap_or_else(|| "windres".into());
            let mut windres = Command::new(compiler);
            windres
                .args([
                    "--input",
                    "assets/app.rc",
                    "--output-format",
                    "coff",
                    "--target",
                    "pe-x86-64",
                    "--output",
                ])
                .arg(&output);
            windres
        };
        let status = command.status().expect(
            "Install Windows SDK (MSVC rc.exe) or MinGW windres (GNU); see docs/BUILDING.md",
        );
        assert!(
            status.success(),
            "Could not compile Windows branding resource"
        );
        println!("cargo:rustc-link-arg={}", output.display());
    }
    if let Ok(path) = env::var("ARTDRAW_WINDOWS_IMPORT_LIBS") {
        println!("cargo:rustc-link-search=native={path}");
    }
}
