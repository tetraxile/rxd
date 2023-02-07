fn print_byte_hex(byte: u8) {
    print!("{:02x} ", byte);
}

fn print_byte_char(byte: u8) {
    // show C0 control codes as unicode Control Pictures characters:
    // print!("{}", char::from_u32(byte as u32 + 0x2400).unwrap());

    match byte {
        byte if byte < 0x20 => print!("."),
        byte if byte < 0x7f => print!("{}", byte as char),
        _ => print!("."),
    }
}

fn print_line((line_num, contents): (usize, &[u8])) {
    let byte_count = contents.len();

    print!("{:08x}0 | ", line_num);
    contents.iter().for_each(|&b| print_byte_hex(b));
    print!("{}", String::from("   ").repeat(16 - byte_count));
    print!("| ");
    contents.iter().for_each(|&b| print_byte_char(b));
    print!("\n");
}

pub fn hex_dump(contents: Vec<u8>) {
    println!("          | 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f |                 ");
    println!("----------+-------------------------------------------------+-----------------");

    contents.chunks(16).enumerate().for_each(print_line);
}
