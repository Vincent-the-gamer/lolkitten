/// 检查CSI序列是否包含颜色相关的参数
/// 返回: (是否是颜色相关, 过滤后的CSI序列)
pub fn filter_color_csi(csi_bytes: &[u8]) -> (bool, Vec<u8>) {
    filter_color_csi_impl(csi_bytes)
}

fn filter_color_csi_impl(csi_bytes: &[u8]) -> (bool, Vec<u8>) {
    // CSI序列格式: ESC [ <params> <command>
    // 我们只关心命令为'm'(SGR)且包含颜色参数的情况

    // 检查命令字符
    if csi_bytes.is_empty() {
        return (false, csi_bytes.to_vec());
    }

    let command = *csi_bytes.last().unwrap();

    // 只有SGR命令(Select Graphic Rendition)可能与颜色相关
    if command != b'm' {
        return (false, csi_bytes.to_vec());
    }

    // 提取参数部分（跳过ESC [)
    let param_start = 2; // ESC = 1byte, '[' = 1byte
    if csi_bytes.len() <= param_start {
        return (false, csi_bytes.to_vec());
    }

    let param_bytes = &csi_bytes[param_start..csi_bytes.len() - 1]; // 不包含命令字符
    let params_str = String::from_utf8_lossy(param_bytes);

    // 解析参数（用;分隔）
    let has_color = params_str.split(';').any(|p| {
        match p {
            "30" | "31" | "32" | "33" | "34" | "35" | "36" | "37" | // 前景色
            "38" | "39" | // 前景色扩展和重置
            "40" | "41" | "42" | "43" | "44" | "45" | "46" | "47" | // 背景色
            "48" | "49" => true, // 背景色扩展和重置
            _ => false,
        }
    });

    if has_color {
        // 有颜色参数，过滤掉整个序列（因为我们不知道如何单独移除颜色参数）
        return (true, Vec::new());
    }

    // 没有颜色参数，保留
    (false, csi_bytes.to_vec())
}

/// 表示文本中的一个段（普通文本或转义码）
#[derive(Debug, Clone)]
pub enum TextSegment {
    /// 普通文本（UTF-8编码）
    Text(String),
    /// 转义码（原始字节）
    Escape(Vec<u8>),
}

/// 解析UTF-8文本，分离出普通文本和 ANSI 转义码
/// 这个函数确保普通文本的UTF-8完整性，按字符而不是字节处理
pub fn parse_text_with_escapes(input: &str) -> Vec<TextSegment> {
    let bytes = input.as_bytes();
    let mut result = Vec::new();
    let mut text_start = 0;
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == 0x1b {
            // 在转义码之前可能有普通文本
            if text_start < i {
                // 尝试将之前的字节解析为UTF-8
                if let Ok(text) = std::str::from_utf8(&bytes[text_start..i]) {
                    if !text.is_empty() {
                        result.push(TextSegment::Text(text.to_string()));
                    }
                }
            }

            // 解析转义码
            let escape_start = i;
            i += 1; // 跳过 ESC

            if i < bytes.len() {
                if bytes[i] == b'[' {
                    // CSI 序列: ESC [ <parameters> <command>
                    i += 1; // 跳过 '['

                    while i < bytes.len() {
                        let byte = bytes[i];
                        // 命令字符在 0x40-0x7e 范围内
                        if byte >= 0x40 && byte <= 0x7e && !":;".contains(byte as char) {
                            i += 1;
                            break;
                        }
                        i += 1;
                    }

                    let escape_bytes = bytes[escape_start..i].to_vec();
                    result.push(TextSegment::Escape(escape_bytes));
                    text_start = i;
                    continue;
                } else if bytes[i] == b']' {
                    // OSC 序列: ESC ] <text> ST 或 BEL
                    i += 1; // 跳过 ']'

                    while i < bytes.len() {
                        let byte = bytes[i];
                        if byte == 0x07 {
                            i += 1;
                            break;
                        } else if byte == 0x1b {
                            if i + 1 < bytes.len() && bytes[i + 1] == b'\\' {
                                i += 2;
                                break;
                            }
                        } else if byte == b'\\' {
                            i += 1;
                            break;
                        }
                        i += 1;
                    }

                    let escape_bytes = bytes[escape_start..i].to_vec();
                    result.push(TextSegment::Escape(escape_bytes));
                    text_start = i;
                    continue;
                } else if bytes[i] >= 0x40 && bytes[i] <= 0x5f {
                    // 其他转义序列
                    i += 1;
                    let escape_bytes = bytes[escape_start..i].to_vec();
                    result.push(TextSegment::Escape(escape_bytes));
                    text_start = i;
                    continue;
                }
            }

            // 处理单个 ESC 或无效序列
            text_start = i;
        } else {
            i += 1;
        }
    }

    // 处理剩余的普通文本
    if text_start < bytes.len() {
        if let Ok(text) = std::str::from_utf8(&bytes[text_start..]) {
            if !text.is_empty() {
                result.push(TextSegment::Text(text.to_string()));
            }
        }
    }

    result
}
