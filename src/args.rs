use crate::help::print_help;
use std::env;

/// 解析命令行参数
/// 返回 Some(()) 如果应该继续执行，None 如果应该退出
pub fn parse_args() -> Option<()> {
    let args: Vec<String> = env::args().collect();

    for arg in &args[1..] {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                return None;
            }
            _ => {
                eprintln!("Warning: Unknown option '{}'", arg);
                eprintln!("Use -h or --help for usage information");
                return None;
            }
        }
    }

    Some(())
}
