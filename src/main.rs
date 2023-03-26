use clap::Parser;
use rxd::Dumper;
use std::fs;
use std::process;

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
    let contents = fs::read(&file_path).unwrap_or_else(|err| {
        println!("error: could not read file {file_path}: {err}");
        process::exit(1);
    });
    Dumper::new(contents, false, args.line_count).dump();
}
