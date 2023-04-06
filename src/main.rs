use clap::Parser;
use rxd::Dumper;
use std::fs::File;
use std::io::BufReader;
use std::process;

// TODO: add args for byte count, start offset

#[derive(Parser)]
#[command(version)]
struct Args {
    /// input file
    file_path: String,

    /// number of lines to print
    #[arg(short)]
    line_count: Option<usize>,

    /// number of bytes per line
    #[arg(short = 'w', default_value_t = 16)]
    line_width: usize,

    /// number of bytes grouped together per line
    #[arg(short = 'g', default_value_t = 1)]
    byte_group_length: usize,

    /// display C0 control codes as characters
    #[arg(short)]
    control_pictures: bool,
}

fn main() {
    let args = Args::parse();

    let file_path = args.file_path;
    let file = File::open(&file_path).unwrap_or_else(|err| {
        println!("error: could not read file {file_path}: {err}");
        process::exit(1);
    });
    let reader = BufReader::new(file);
    Dumper::new(reader)
        .line_count(args.line_count)
        .line_width(args.line_width)
        .byte_group_length(args.byte_group_length)
        .control_pictures(args.control_pictures)
        .dump();
}
