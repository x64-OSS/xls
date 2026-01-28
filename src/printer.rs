use colored::Colorize;

use crate::defs::{Args, Entry};

pub fn print_entries(entries: Vec<Entry>, args: &Args) {
    if args.list {
        print_list_entries(entries);
    } else {
        print_normal_entries(entries);
    }
}

fn print_list_entries(entries: Vec<Entry>) {
    todo!();
}
fn print_normal_entries(entries: Vec<Entry>) {
    let entry_cell_width: usize = 12;
    for entry in entries {
        let printable = if entry.file_type.is_dir() {
            entry.name.blue().bold()
        } else if entry.file_type.is_symlink() {
            entry.name.italic()
        } else {
            entry.name.normal()
        };
        print!("{:entry_cell_width$}", printable);
    }
    println!();
}
