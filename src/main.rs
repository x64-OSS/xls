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
    let sort_type = Args::parse_sort_type(&args);
    let ignore_mode = Args::parse_ignore_mode(&args);
    let list_mode = Args::parse_list_mode(&args);
    let path = Path::new(&args.path);
    let mut entries = list_directory(path, &args, &ignore_mode);

    sort_entires(&mut entries, &sort_type, args.reverse);
    print_entries(entries, &args);
}
