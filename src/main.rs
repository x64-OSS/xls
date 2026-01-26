use std::path::Path;
pub mod defs;
pub mod formatter;
pub mod printer;
pub mod reader;

use clap::Parser;

use crate::defs::Args;
use crate::printer::print_entries;

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.path);
    let entries = reader::list_directory(path, &args);
    print_entries(entries, args.style);
}
