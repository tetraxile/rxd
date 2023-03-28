use clap::Parser;
use rxd::Dumper;
use std::fs::File;
use std::io::BufReader;
use std::process;

// TODO: add args for byte count, line width, control codes

#[derive(Parser)]
#[command(version)]
struct Args {
    /// input file
    file_path: String,

    /// number of lines to print
    #[arg(short)]
    line_count: Option<usize>,
}

fn main() {
    let args = Args::parse();

    let file_path = args.file_path;
    let file = File::open(file_path.clone()).unwrap_or_else(|err| {
        println!("error: could not read file {file_path}: {err}");
        process::exit(1);
    });
    let reader = BufReader::new(file);
    Dumper::new(reader, false, args.line_count).dump();
}
