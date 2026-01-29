use std::path::Path;
pub mod defs;
pub mod formatter;
pub mod printer;
pub mod reader;
pub mod sorter;

use clap::Parser;

use crate::defs::Args;
use crate::printer::print_entries;
use crate::reader::list_directory;
use crate::sorter::sort_entires;

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.path);
    let mut entries = list_directory(path, &args);
    sort_entires(&mut entries, &args);
    print_entries(entries, &args);
}
