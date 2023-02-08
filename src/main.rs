use std::env;
use std::fs;
use std::process;
use xxd::Dumper;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("error: no file provided");
        process::exit(1);
    }

    let file_path = &args[1];
    let contents = fs::read(file_path).unwrap_or_else(|_err| {
        println!("error: file not found");
        process::exit(1);
    });
    Dumper::new(contents, false).dump();
}
