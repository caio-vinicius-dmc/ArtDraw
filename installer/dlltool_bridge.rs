fn main() {
    let tool = std::env::var_os("ARTDRAW_GNU_DLLTOOL").expect("ARTDRAW_GNU_DLLTOOL");
    let assembler = std::env::var_os("ARTDRAW_AS_BRIDGE").expect("ARTDRAW_AS_BRIDGE");
    let result = std::process::Command::new(tool)
        .arg("--as")
        .arg(assembler)
        .args(std::env::args_os().skip(1))
        .status()
        .expect("dlltool");
    std::process::exit(result.code().unwrap_or(1));
}
