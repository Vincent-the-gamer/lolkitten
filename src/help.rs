use crate::renderer::colorize_text;

/// 显示帮助信息
pub fn print_help() {
    let help_text = vec![
        "Usage: lolkitten [OPTIONS]",
        "",
        "Rainbow-colored text output (Rust implementation of lolcat)",
        "",
        "OPTIONS:",
        "    -h, --help    Prints help information",
        "",
        "EXAMPLE:",
        "    cat README.md | lolkitten",
        r#"    echo "Hello, World!" | lolkitten"#,
    ];

    // 将所有帮助文本合并成一个字符串
    let mut all_bytes: Vec<u8> = Vec::new();
    for line in &help_text {
        all_bytes.extend_from_slice(line.as_bytes());
        all_bytes.push(b'\n');
    }

    // 使用与 run 函数相同的渲染逻辑
    let total_bytes = all_bytes.len() - help_text.len(); // 减去换行符的数量
    let mut output = Vec::new();
    let mut current_index = 0;

    for &byte in &all_bytes {
        if byte == b'\n' {
            output.push(byte);
        } else {
            let hue = (current_index as f64 / total_bytes as f64) * 360.0;
            let colored = colorize_text(&[byte], hue);
            output.extend_from_slice(&colored);
            current_index += 1;
        }
    }

    println!("{}", String::from_utf8_lossy(&output));
}
