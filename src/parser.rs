/// 解析字节流，分离普通文本和 ANSI 转义码
pub fn parse_bytes_with_escapes(input: &[u8]) -> Vec<(Vec<u8>, bool)> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < input.len() {
        if input[i] == 0x1b {
            // 开始转义码
            let escape_start = i;

            // 跳过 ESC 字符
            i += 1;

            if i < input.len() {
                // 检查转义序列类型
                if input[i] == b'[' {
                    // CSI 序列: ESC [ <parameters> <command>
                    i += 1; // 跳过 '['

                    // 读取参数和命令字符
                    while i < input.len() {
                        let byte = input[i];
                        // 命令字符在 0x40-0x7e 范围内
                        // 但要跳过在中间也能出现的字符如 ;/<=? 等
                        if byte >= 0x40 && byte <= 0x7e && !":;".contains(byte as char) {
                            // 这是命令字符，转义序列结束
                            i += 1;
                            break;
                        }
                        i += 1;
                    }

                    // 收集整个 CSI 序列
                    let escape_bytes = &input[escape_start..i];
                    result.push((escape_bytes.to_vec(), true));
                    continue;
                } else if input[i] == b']' {
                    // OSC 序列: ESC ] <text> ST 或 BEL
                    i += 1; // 跳过 ']'

                    while i < input.len() {
                        let byte = input[i];
                        // OSC 序列以 ST (ESC \ 或 \x1b) 或 BEL (\x07) 结束
                        if byte == 0x07 {
                            i += 1;
                            break;
                        } else if byte == 0x1b {
                            // 可能是 ST，检查下一个字符
                            if i + 1 < input.len() && input[i + 1] == b'\\' {
                                i += 2;
                                break;
                            }
                        } else if byte == b'\\' {
                            // ST 单独出现
                            i += 1;
                            break;
                        }
                        i += 1;
                    }

                    let escape_bytes = &input[escape_start..i];
                    result.push((escape_bytes.to_vec(), true));
                    continue;
                } else if input[i] >= 0x40 && input[i] <= 0x5f {
                    // 其他转义序列 (如 ESC M 换行)
                    i += 1;
                    let escape_bytes = &input[escape_start..i];
                    result.push((escape_bytes.to_vec(), true));
                    continue;
                }
            }

            // 如果到这里，可能是无效序列或单个 ESC
            let escape_bytes = &input[escape_start..i];
            if !escape_bytes.is_empty() && escape_bytes[0] == 0x1b {
                result.push((escape_bytes.to_vec(), true));
            }
        } else {
            // 普通字节
            result.push((vec![input[i]], false));
            i += 1;
        }
    }

    result
}

/// 检查CSI序列是否包含颜色相关的参数
/// 返回: (是否是颜色相关, 过滤后的CSI序列)
pub fn filter_color_csi(csi_bytes: &[u8]) -> (bool, Vec<u8>) {
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
