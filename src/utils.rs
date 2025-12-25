use crate::defs::{Entry, EntryType};
use crate::formatter::{formatted_entry_permissions, formatted_entry_size};
use std::fs;

use std::os::unix::fs::MetadataExt;
use std::path::Path;

const PERM_WIDTH: usize = 12;
const SIZE_WIDTH: usize = 8;
const NAME_WIDTH: usize = 36;
const MODIFIED_WIDTH: usize = 12;

const TABLE_WIDTH: usize =
    1 + PERM_WIDTH + 1 + SIZE_WIDTH + 1 + NAME_WIDTH + 1 + MODIFIED_WIDTH + 1;

pub fn print_table(entries: &[Entry]) {
    let mut dirs = 0;
    let mut files = 0;
    let mut links = 0;

    let border = format!("{}", "-".repeat(TABLE_WIDTH - 2));

    println!("{}", border);
    println!(
        "{:<PERM_WIDTH$} {:>SIZE_WIDTH$} {:<NAME_WIDTH$} {:>MODIFIED_WIDTH$}",
        "PERMS", "SIZE", "NAME", "MODIFIED"
    );
    println!("{}", border);

    for e in entries {
        match e.entry_type {
            EntryType::Dir => dirs += 1,
            EntryType::File => files += 1,
            EntryType::Link => links += 1,
        }

        let perm = format!("{}{}", e.entry_type.marker(), e.permissions);
        let size = formatted_entry_size(e.size);
        let name = match e.entry_type {
            EntryType::Dir => format!("{}/", e.name),
            _ => e.name.clone(),
        };

        println!(
            "{:<PERM_WIDTH$} {:>SIZE_WIDTH$} {:<NAME_WIDTH$} {:>MODIFIED_WIDTH$}",
            perm, size, name, e.last_modified
        );
    }

    println!("{}", border);
    println!(
        "DIRECTORIES: {}   FILES: {}    LINKS:{:<11}",
        dirs, files, links
    );
}

pub fn list_directory(path: &Path) -> Vec<Entry> {
    let mut entries_list = Vec::new();

    let entries = match fs::read_dir(path) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Cannot access {}: {}", path.display(), e);
            return entries_list;
        }
    };

    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();

        let meta = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        let last_modified: String = if let Ok(time) = meta.modified() {
            String::from("--")
        } else {
            String::from("-")
        };

        let size = meta.size();
        let permissions = formatted_entry_permissions(&meta);

        let entry_type = if meta.is_dir() {
            EntryType::Dir
        } else if meta.is_file() {
            EntryType::File
        } else {
            EntryType::Link
        };

        entries_list.push(Entry {
            name,
            size,
            permissions,
            entry_type,
            last_modified,
        });
    }

    entries_list
}

pub fn sort_entries(entries: &mut Vec<Entry>) {
    entries.sort_by_key(|e| e.entry_type.order());
}
