use std::fmt::format;

use colored::Colorize;

use crate::defs::{Args, Entry};
use crate::formatter::formatted_entry_size;

const PERM_WIDTH: usize = 12;
const SIZE_WIDTH: usize = 6;
const BLOCK_WIDTH: usize = 6;
const AUTHOR_WIDTH: usize = 18;
const OWNER_WIDTH: usize = 18;
const INODE_WIDTH: usize = 8;
const DATE_WIDTH: usize = 12;

pub fn print_entries(entries: Vec<Entry>, args: &Args) {
    if args.list {
        print_list_entries(entries, args);
    } else if args.column {
        print_column_entries(entries);
    } else {
        print_normal_entries(entries);
    }
}

fn print_list_entries(entries: Vec<Entry>, args: &Args) {
    let mut placeholder = String::new();
    let mut values: Vec<String> = Vec::new();

    if args.inode {
        placeholder.push_str("{:<INODE_WIDTH$}");
    }

    for entry in entries {
        let size = formatted_entry_size(entry.size);

        let name = if entry.file_type.is_dir() {
            entry.name.blue().bold()
        } else if entry.file_type.is_symlink() {
            entry.name.italic()
        } else {
            entry.name.normal()
        };

        

        let mut placeholder = String::new();
        let mut values: Vec<String> = Vec::new();


    }
}

fn print_normal_entries(entries: Vec<Entry>) {
    for entry in entries {
        let name = if entry.file_type.is_dir() {
            entry.name.blue().bold()
        } else if entry.file_type.is_symlink() {
            entry.name.italic()
        } else {
            entry.name.normal()
        };
        print!("{}  ", name);
    }
    println!();
}

fn print_column_entries(entries: Vec<Entry>) {
    todo!()
}