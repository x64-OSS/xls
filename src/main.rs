use std::env;
use std::path::Path;
use xls::utils::{list_directory, print_table};

const VERSION: &str = "0.0.1";

fn print_help() {
    println!(
        "xls - simple ls utility written in Rust

USAGE:
    xls [OPTIONS] [PATH]

OPTIONS:
    -h, --help       Print this help message
    -v, --version    Print version information

ARGS:
    PATH             Directory to list (defaults to current directory)
"
    );
}

fn print_version() {
    println!("xls version {}", VERSION);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "-h" | "--help" => {
                print_help();
                return;
            }
            "-v" | "--version" => {
                print_version();
                return;
            }
            _ => {}
        }
    }

    let path = if args.len() > 1 {
        Path::new(&args[1])
    } else {
        Path::new(".")
    };

    let entries = list_directory(path);
    print_table(&entries);
}
