use crate::{
    defs::{Entry, ListMode},
    formatter::formatted_entry_size,
    reader::EntryReturn,
    utils::{column_width, compute_layout, get_user_name_by_id, visible_width},
};

use terminal_size::{Width, terminal_size};

pub fn print_entries(entry_return: &EntryReturn, list_mode: &ListMode) {
    match list_mode {
        ListMode::Column => print_column_entries(&entry_return),
        ListMode::Long => print_long_entries(&entry_return),
        ListMode::Default => print_default_entries(&entry_return),
    }
}

fn print_long_entries(entry_return: &EntryReturn) {
    let rows = build_long_rows(&entry_return.entries);
    let widths = compute_column_widths(&rows);
    print_aligned_rows(&rows, &widths);
    println!("{}", "-".repeat(10));
    println!(
        "Dirs: {}  Files: {}  Links: {}",
        entry_return.counts.dirs, entry_return.counts.files, entry_return.counts.links
    )
}

fn print_default_entries(entry_return: &EntryReturn) {
    let mut print_size: usize = 0;

    for entry in &entry_return.entries {
        print_size += entry.name.len() + 2
    }

    let (Width(width), _) = terminal_size().expect("Failed to get terminal size");

    if print_size > width.into() {
        print_column_entries(entry_return);
        return;
    }

    for entry in &entry_return.entries {
        print!("{}  ", entry.name);
    }
    println!();
}

fn print_column_entries(entry_return: &EntryReturn) {
    let entries = &entry_return.entries;
    let long_name_len = &entry_return.longest_name_len;

    if entries.is_empty() {
        return;
    }

    let col_width = column_width(entries);
    let (rows, cols) = compute_layout(entries);

    for row in 0..rows {
        for col in 0..cols {
            let idx = col * rows + row;

            if idx >= entries.len() {
                continue;
            }

            let name = &entries[idx].name;
            let width = visible_width(name);

            if col == cols - 1 || idx + rows >= entries.len() {
                print!("{}  ", name);
            } else {
                print!(
                    "{:<pad$}",
                    name,
                    pad = col_width + width - visible_width(name)
                );
            }
        }
        println!();
    }
}

fn build_long_rows(entries: &[Entry]) -> Vec<Vec<String>> {
    entries
        .iter()
        .map(|entry| {
            let mut row = Vec::new();

            if let Some(inode) = &entry.inode {
                row.push(inode.clone());
            }

            if let Some(perms) = &entry.permissions {
                row.push(perms.clone());
            }

            if let Some(owner) = entry.owner {
                row.push(get_user_name_by_id(owner));
            }

            if let Some(author) = entry.author {
                row.push(get_user_name_by_id(author));
            }

            if let Some(size) = entry.size {
                row.push(formatted_entry_size(size));
            }

            if let Some(modified) = &entry.modified {
                row.push(modified);
            }

            row.push(entry.name.clone());
            row
        })
        .collect()
}

fn print_aligned_rows(rows: &[Vec<String>], widths: &[usize]) {
    for row in rows {
        for (i, col) in row.iter().enumerate() {
            let width = widths[i];

            if i == row.len() - 1 {
                // last column (name)
                print!("{}", col);
            } else {
                print!("{:<width$}  ", col, width = width);
            }
        }
        println!();
    }
}

fn compute_column_widths(rows: &[Vec<String>]) -> Vec<usize> {
    let mut widths = Vec::new();

    for row in rows {
        for (i, col) in row.iter().enumerate() {
            let len = col.len(); // later we’ll fix this for colors
            if i >= widths.len() {
                widths.push(len);
            } else {
                widths[i] = widths[i].max(len);
            }
        }
    }

    widths
}
