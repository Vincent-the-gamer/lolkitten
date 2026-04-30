mod args;
mod color;
mod help;
mod parser;
mod renderer;

use args::parse_args;
use parser::{filter_color_csi, parse_bytes_with_escapes};
use renderer::colorize_text;
use std::io::{self, BufRead, Write};

fn main() {
    // 解析命令行参数
    if parse_args().is_none() {
        return;
    }

    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    // 读取所有输入行
    let mut lines_bytes: Vec<Vec<u8>> = Vec::new();
    for line in stdin.lock().lines() {
        if let Ok(text) = line {
            lines_bytes.push(text.into_bytes());
        }
    }

    // 如果没有输入，直接退出
    if lines_bytes.is_empty() {
        return Ok(());
    }

    // 计算所有普通文本的总字节数（用于彩虹渐变分布）
    let mut total_text_bytes = 0;
    for line_bytes in &lines_bytes {
        let segments = parse_bytes_with_escapes(line_bytes);
        for (bytes, is_escape) in &segments {
            if !*is_escape {
                total_text_bytes += bytes.len();
            }
        }
    }

    // 如果没有可染色的文本，直接输出原文本
    if total_text_bytes == 0 {
        for line_bytes in &lines_bytes {
            stdout.write_all(line_bytes)?;
            stdout.write_all(b"\n")?;
        }
        return Ok(());
    }

    // 处理每一行
    let mut current_text_byte_index = 0;
    for line_bytes in &lines_bytes {
        let segments = parse_bytes_with_escapes(line_bytes);
        let mut line_output = Vec::new();

        for (bytes, is_escape) in &segments {
            if *is_escape {
                // 检查是否是CSI颜色序列
                if bytes.len() >= 2 && bytes[0] == 0x1b && bytes[1] == b'[' {
                    let (is_color, filtered) = filter_color_csi(bytes);
                    if !is_color {
                        // 不是颜色序列，保留输出
                        line_output.extend_from_slice(&filtered);
                    }
                    // 如果是颜色序列，就过滤掉（不输出）
                } else {
                    // 其他转义码，原样输出
                    line_output.extend_from_slice(bytes);
                }
            } else {
                // 普通文本，染上彩虹色
                // 为每个字节计算彩虹颜色
                for &byte in bytes {
                    let hue = (current_text_byte_index as f64 / total_text_bytes as f64) * 360.0;
                    let colored = colorize_text(&[byte], hue);
                    line_output.extend_from_slice(&colored);
                    current_text_byte_index += 1;
                }
            }
        }

        // 输出这一行
        stdout.write_all(&line_output)?;
        stdout.write_all(b"\n")?;
    }

    stdout.flush()?;
    Ok(())
}
