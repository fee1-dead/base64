pub fn encode_base64(data: &[u8]) -> String {
    data.chunks(3)
        .flat_map(|chunk| match chunk {
            &[a, b, c] => [
                encode_base64_digit(a >> 2),
                encode_base64_digit(((a & 0b11) << 4) + (b >> 4)),
                encode_base64_digit(((b & 0b1111) << 2) + (c >> 6)),
                encode_base64_digit(c & 0b111111),
            ],
            &[a, b] => [
                encode_base64_digit(a >> 2),
                encode_base64_digit(((a & 0b11) << 4) + (b >> 4)),
                encode_base64_digit((b & 0b1111) << 2),
                '=',
            ],
            &[a] => [
                encode_base64_digit(a >> 2),
                encode_base64_digit((a & 0b11) << 4),
                '=',
                '=',
            ],
            _ => unreachable!(),
        })
        .collect()
}

pub fn encode_base64_digit(n: u8) -> char {
    (match n {
        0..=25 => b'A' + n,
        26..=51 => b'a' + (n - 26),
        52..=61 => b'0' + (n - 52),
        62 => b'+',
        63 => b'/',
        0b1000000.. => panic!("too big!"),
    }) as char
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn man_example() {
        let result = encode_base64("Man".as_bytes());
        assert_eq!(result, "TWFu");
        let result = encode_base64("Ma".as_bytes());
        assert_eq!(result, "TWE=");
        let result = encode_base64("M".as_bytes());
        assert_eq!(result, "TQ==");
    }
}
