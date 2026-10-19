fn main() {
    let tool = std::env::var_os("ARTDRAW_ASSEMBLER").expect("ARTDRAW_ASSEMBLER");
    let args: Vec<_> = std::env::args_os()
        .skip(1)
        .filter(|a| a != "--64")
        .collect();
    let result = std::process::Command::new(tool)
        .arg("-c")
        .args(args)
        .status()
        .expect("assembler");
    std::process::exit(result.code().unwrap_or(1));
}
