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
            .map(|&byte| format!("{byte:02x}"))
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

        format!("{line_num:07x}0 | {line_hex: <47} | {line_ascii}")
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
        self.lines.iter().for_each(|line| println!("{line}"));
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn lorem() {
        let expected =
            "00000000 | 4c 6f 72 65 6d 20 69 70 73 75 6d 20 64 6f 6c 6f | Lorem ipsum dolo\n\
             00000010 | 72 20 73 69 74 20 61 6d 65 74 20 63 6f 6e 73 65 | r sit amet conse\n\
             00000020 | 63 74 65 74 75 72 20 61 64 69 70 69 73 69 63 69 | ctetur adipisici\n\
             00000030 | 6e 67 20 65 6c 69 74 2e 20 41 74 71 75 65 20 6f | ng elit. Atque o\n\
             00000040 | 6d 6e 69 73 20 64 69 67 6e 69 73 73 69 6d 6f 73 | mnis dignissimos\n\
             00000050 | 20 74 6f 74 61 6d 20 63 6f 6e 73 65 71 75 75 6e |  totam consequun\n\
             00000060 | 74 75 72 20 61 6c 69 71 75 69 64 20 6d 69 6e 69 | tur aliquid mini\n\
             00000070 | 6d 61 20 6e 61 74 75 73 20 64 6f 6c 6f 72 75 6d | ma natus dolorum\n\
             00000080 | 20 73 65 64 20 69 70 73 75 6d 20 69 6c 6c 75 6d |  sed ipsum illum\n\
             00000090 | 3f                                              | ?";

        let lorem = "Lorem ipsum dolor sit amet consectetur adipisicing elit. Atque omnis dignissimos totam consequuntur aliquid minima natus dolorum sed ipsum illum?";
        let reader = Cursor::new(lorem);
        let result = Dumper::format_contents(reader, false, None).join("\n");

        assert_eq!(expected, result)
    }

    #[test]
    fn with_control_codes() {
        let expected =
            "00000000 | 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f | ␀␁␂␃␄␅␆␇␈␉␊␋␌␍␎␏\n\
             00000010 | 10 11 12 13 14 15 16 17 18 19 1a 1b 1c 1d 1e 1f | ␐␑␒␓␔␕␖␗␘␙␚␛␜␝␞␟\n\
             00000020 | 20 21 22 23 24 25 26 27 28 29 2a 2b 2c 2d 2e 2f |  !\"#$%&'()*+,-./\n\
             00000030 | 30 31 32 33 34 35 36 37 38 39 3a 3b 3c 3d 3e 3f | 0123456789:;<=>?\n\
             00000040 | 40 41 42 43 44 45 46 47 48 49 4a 4b 4c 4d 4e 4f | @ABCDEFGHIJKLMNO\n\
             00000050 | 50 51 52 53 54 55 56 57 58 59 5a 5b 5c 5d 5e 5f | PQRSTUVWXYZ[\\]^_\n\
             00000060 | 60 61 62 63 64 65 66 67 68 69 6a 6b 6c 6d 6e 6f | `abcdefghijklmno\n\
             00000070 | 70 71 72 73 74 75 76 77 78 79 7a 7b 7c 7d 7e 7f | pqrstuvwxyz{|}~.\n\
             00000080 | 80 81 82 83 84 85 86 87 88 89 8a 8b 8c 8d 8e 8f | ................\n\
             00000090 | 90 91 92 93 94 95 96 97 98 99 9a 9b 9c 9d 9e 9f | ................\n\
             000000a0 | a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 aa ab ac ad ae af | ................\n\
             000000b0 | b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 ba bb bc bd be bf | ................\n\
             000000c0 | c0 c1 c2 c3 c4 c5 c6 c7 c8 c9 ca cb cc cd ce cf | ................\n\
             000000d0 | d0 d1 d2 d3 d4 d5 d6 d7 d8 d9 da db dc dd de df | ................\n\
             000000e0 | e0 e1 e2 e3 e4 e5 e6 e7 e8 e9 ea eb ec ed ee ef | ................\n\
             000000f0 | f0 f1 f2 f3 f4 f5 f6 f7 f8 f9 fa fb fc fd fe ff | ................";

        let all_bytes = (0..=255).collect::<Vec<_>>();
        let reader = Cursor::new(all_bytes);
        let result = Dumper::format_contents(reader, true, None).join("\n");

        assert_eq!(expected, result)
    }

    #[test]
    fn with_line_count() {
        let expected =
            "00000000 | ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff | ................\n\
             00000010 | ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff | ................\n\
             00000020 | ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff | ................\n\
             00000030 | ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff | ................\n\
             00000040 | ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff | ................\n\
             00000050 | ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff | ................\n\
             00000060 | ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff | ................\n\
             00000070 | ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff | ................\n\
             00000080 | ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff | ................\n\
             00000090 | ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff | ................";

        let bytes = vec![0xff; 100 * 0x10];
        let reader = Cursor::new(bytes);
        let result = Dumper::format_contents(reader, false, Some(10)).join("\n");

        assert_eq!(expected, result);
    }

    // TODO: add png file and test for line count
    // TODO: add docs
}
