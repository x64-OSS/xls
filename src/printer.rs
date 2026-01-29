use colored::Colorize;

use crate::defs::{Args, Entry};
use crate::formatter::formatted_entry_size;

pub fn print_entries(entries: Vec<Entry>, args: &Args) {
    if args.list {
        print_list_entries(entries);
    } else {
        print_normal_entries(entries);
    }
}

fn print_list_entries(entries: Vec<Entry>) {
    let print_width: usize = 12;
    for entry in entries {
        let name = if entry.file_type.is_dir() {
            entry.name.blue().bold()
        } else if entry.file_type.is_symlink() {
            entry.name.italic()
        } else {
            entry.name.normal()
        };

        let size = formatted_entry_size(entry.size);

        println!("{:<print_width$} {:<6} {:<print_width$} {:<print_width$} {:<print_width$} test", 
            name, size, entry.last_modified, entry.permissions, entry.inode.unwrap_or_default(), print_width = print_width);
    }
}
fn print_normal_entries(entries: Vec<Entry>) {
    let entry_cell_width: usize = 12;
    for entry in entries {
        let name = if entry.file_type.is_dir() {
            entry.name.blue().bold()
        } else if entry.file_type.is_symlink() {
            entry.name.italic()
        } else {
            entry.name.normal()
        };
        print!("{:<width$}", name, width = entry_cell_width);
    }
    println!();
}
