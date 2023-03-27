use std::io::Read;

pub struct Dumper {
    lines: Vec<String>,
}

impl Dumper {
    fn header() -> Vec<String> {
        vec![
            "         | 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f |                 ".into(),
            "---------+-------------------------------------------------+-----------------".into(),
        ]
    }

    fn format_line(line_num: usize, line_bytes: Vec<u8>, control_pictures: bool) -> String {
        let line_hex = line_bytes
            .iter()
            .map(|&byte| format!("{:02x}", byte))
            .collect::<Vec<_>>()
            .join(" ");

        let line_ascii: String = line_bytes
            .iter()
            .map(|&byte| match byte {
                byte if byte < 0x20 && control_pictures => {
                    char::from_u32(byte as u32 + 0x2400).unwrap()
                }
                byte if byte < 0x20 => '.',
                byte if byte < 0x7f => byte as char,
                _ => '.',
            })
            .collect();

        format!("{:07x}0 | {: <47} | {}", line_num, line_hex, line_ascii)
    }

    fn format_contents<R>(
        mut reader: R,
        control_pictures: bool,
        line_count: Option<usize>,
    ) -> Vec<String>
    where
        R: Read,
    {
        let mut lines = Vec::new();
        let mut line_bytes = vec![0u8; 16];
        let mut line_num = 0;
        loop {
            let length = reader.read(&mut line_bytes).unwrap();
            if length == 0 {
                break;
            }

            if let Some(line_count) = line_count {
                if line_num >= line_count {
                    break;
                }
            }

            lines.push(Dumper::format_line(
                line_num,
                line_bytes[..length].to_vec(),
                control_pictures,
            ));
            line_num += 1;
        }

        lines
    }

    // show C0 control codes as unicode Control Pictures characters:
    pub fn new<R>(reader: R, control_pictures: bool, line_count: Option<usize>) -> Dumper
    where
        R: Read,
    {
        let mut lines = Dumper::header();

        lines.extend(Dumper::format_contents(
            reader,
            control_pictures,
            line_count,
        ));

        Dumper { lines }
    }

    pub fn dump(&self) {
        self.lines.iter().for_each(|line| println!("{}", line));
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_format_contents() {
        let contents = "Lorem ipsum dolor sit amet consectetur adipisicing elit. Atque omnis dignissimos totam consequuntur aliquid minima natus dolorum sed ipsum illum?".as_bytes().to_vec();
        let reader = Cursor::new(contents);

        let result = Dumper::format_contents(reader, false, None).join("\n");

        let expected = "00000000 | 4c 6f 72 65 6d 20 69 70 73 75 6d 20 64 6f 6c 6f | Lorem ipsum dolo\n\
                        00000010 | 72 20 73 69 74 20 61 6d 65 74 20 63 6f 6e 73 65 | r sit amet conse\n\
                        00000020 | 63 74 65 74 75 72 20 61 64 69 70 69 73 69 63 69 | ctetur adipisici\n\
                        00000030 | 6e 67 20 65 6c 69 74 2e 20 41 74 71 75 65 20 6f | ng elit. Atque o\n\
                        00000040 | 6d 6e 69 73 20 64 69 67 6e 69 73 73 69 6d 6f 73 | mnis dignissimos\n\
                        00000050 | 20 74 6f 74 61 6d 20 63 6f 6e 73 65 71 75 75 6e |  totam consequun\n\
                        00000060 | 74 75 72 20 61 6c 69 71 75 69 64 20 6d 69 6e 69 | tur aliquid mini\n\
                        00000070 | 6d 61 20 6e 61 74 75 73 20 64 6f 6c 6f 72 75 6d | ma natus dolorum\n\
                        00000080 | 20 73 65 64 20 69 70 73 75 6d 20 69 6c 6c 75 6d |  sed ipsum illum\n\
                        00000090 | 3f                                              | ?";

        assert_eq!(result, expected);
    }
}
