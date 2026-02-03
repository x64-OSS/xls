use crate::{defs::ListMode, reader::EntryReturn};

const PERM_WIDTH: usize = 12;
const SIZE_WIDTH: usize = 6;
const BLOCK_WIDTH: usize = 6;
const AUTHOR_WIDTH: usize = 18;
const OWNER_WIDTH: usize = 18;
const INODE_WIDTH: usize = 8;
const DATE_WIDTH: usize = 12;

pub fn print_entries(entry_return: &EntryReturn, list_mode: &ListMode) {
    match list_mode {
        ListMode::Column => print_column_entries(&entry_return),
        ListMode::List => print_list_entries(&entry_return),
        ListMode::Default => print_default_entries(&entry_return),
    }
}

fn print_list_entries(entry_return: &EntryReturn) {
    println!("W'll print entries in list");
}

fn print_column_entries(entry_return: &EntryReturn) {
    println!("w'll print entries in columns")
}

fn print_default_entries(entry_return: &EntryReturn) {
    let mut print_size: usize = 0;

    for entry in &entry_return.entries {
        print_size += entry.name.len() + 2
    }

    // Here 20 is the width of terminal, if the names of entries + 2 with each
    // exceads the terminal width then it will print entries in column
    if print_size > 20 {
        print_column_entries(entry_return);
        return;
    }

    for entry in &entry_return.entries {
        print!("{}  ", entry.name);
    }
    println!();
}
