use std::env;
use std::fs;
use std::path::Path;

const VERSION: &str = "0.0.1";

fn print_help() {
    println!(
        "rls - simple ls utility written in Rust

USAGE:
    rls [OPTIONS] [PATH]

OPTIONS:
    -h, --help       Print this help message
    -v, --version    Print version information

ARGS:
    PATH             Directory to list (defaults to current directory)
"
    );
}

fn print_version() {
    println!("rls version {}", VERSION);
}

fn list_directory(path: &Path) {
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let name = entry.file_name();
                        print!("{}  ", name.to_string_lossy());
                    }
                    Err(e) => eprintln!("Error reading entry: {}", e),
                }
            }
            println!();
        }
        Err(e) => eprintln!("Cannot access {}: {}", path.display(), e),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Handle flags
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

    // Determine path
    let path = if args.len() > 1 {
        Path::new(&args[1])
    } else {
        Path::new(".")
    };

    list_directory(path);
}
