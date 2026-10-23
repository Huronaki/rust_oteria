pub fn format(bytes: &[u8]) -> String {
    let mut output = String::new();

    for (line_index, chunk) in bytes.chunks(16).enumerate() {
        let offset = line_index * 16;

        output.push_str(&format!("{offset:08x} "));

        for i in 0..16 {
            if let Some(byte) = chunk.get(i) {
                output.push_str(&format!("{byte:02x} "));
            } else {
                output.push_str("   ");
            }

            if i == 7 {
                output.push(' ');
            }
        }

        output.push('|');

        for byte in chunk {
            if (0x20..=0x7e).contains(byte) {
                output.push(*byte as char);
            } else {
                output.push('.');
            }
        }

        output.push_str("|\n");
    }

    output.push_str(&format!("{:08x}\n", bytes.len()));

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_ascii_and_non_printable_bytes() {
        let bytes = [b'A', 0x00, b'B'];
        let output = format(&bytes);

        assert!(output.contains("|A.B|"));
        assert!(output.ends_with("00000003\n"));
    }

    #[test]
    fn empty_file_shows_zero_offset() {
        assert_eq!(format(&[]), "00000000\n");
    }
}
