use std::str::Bytes;

pub struct StringParser;

impl StringParser {
    pub fn get_unescaped_string_as_u8(input: &str) -> anyhow::Result<Vec<u8>> {
        let mut buffer = if input.starts_with("\"") && input.ends_with("\"") {
            input[1..input.len() - 1].bytes()
        } else {
            anyhow::bail!("input should start and and with '\"' ({input})")
        };

        let mut parsed = Vec::new();

        while let Some(byte) = buffer.next() {
            if byte != b'\\' {
                parsed.push(byte);
                continue;
            }

            let byte = match buffer.next() {
                Some(b'\\') => Some(b'\\'),
                Some(b'"') => Some(b'"'),
                Some(b'x') => Self::unescape_hex(&mut buffer),
                _ => None,
            };

            let Some(byte) = byte else {
                anyhow::bail!("invalid escape sequence in {input}");
            };

            parsed.push(byte);
        }

        Ok(parsed)
    }

    fn unescape_hex(buffer: &mut Bytes) -> Option<u8> {
        let higher = buffer.next()?;
        let lower = buffer.next()?;

        let higher = Self::hex_char_to_u8(higher as char)?;
        let lower = Self::hex_char_to_u8(lower as char)?;

        Some(higher << 4 | lower)
    }

    fn hex_char_to_u8(c: char) -> Option<u8> {
        match c {
            '0'..='9' => Some(c as u8 - b'0'),
            'a'..='f' => Some(c as u8 - b'a' + 10),
            'A'..='F' => Some(c as u8 - b'A' + 10),
            _ => None,
        }
    }
}
