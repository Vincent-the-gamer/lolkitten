use crate::color::hsl_to_rgb;

/// 给普通文本字节染色（支持 UTF-8）
pub fn colorize_text(bytes: &[u8], hue: f64) -> Vec<u8> {
    let (r, g, b) = hsl_to_rgb(hue, 1.0, 0.5);

    let mut output = Vec::new();
    // 添加颜色转义码
    output.extend_from_slice(b"\x1b[38;2;");

    // 添加 RGB 值
    let r_str = format!("{}", r).into_bytes();
    output.extend_from_slice(&r_str);
    output.push(b';');
    let g_str = format!("{}", g).into_bytes();
    output.extend_from_slice(&g_str);
    output.push(b';');
    let b_str = format!("{}", b).into_bytes();
    output.extend_from_slice(&b_str);
    output.extend_from_slice(b"m");

    // 添加文本（保持 UTF-8 编码完整）
    output.extend_from_slice(bytes);

    // 重置颜色
    output.extend_from_slice(b"\x1b[0m");

    output
}
