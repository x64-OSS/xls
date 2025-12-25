use std::fs;
use std::fs::Metadata;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use crate::{Entry, EntryType};

pub fn format_permissions(meta: &Metadata) -> String {
    let mode = meta.permissions().mode();
    let flags = [
        (mode & 0o400, 'r'),
        (mode & 0o200, 'w'),
        (mode & 0o100, 'x'),
        (mode & 0o040, 'r'),
        (mode & 0o020, 'w'),
        (mode & 0o010, 'x'),
        (mode & 0o004, 'r'),
        (mode & 0o002, 'w'),
        (mode & 0o001, 'x'),
    ];

    flags
        .iter()
        .map(|(bit, ch)| if *bit != 0 { *ch } else { '-' })
        .collect()
}

pub fn sort_entries(entries: &mut Vec<Entry>) {
    entries.sort_by_key(|e| e.entry_type.order());
}

const PERM_WIDTH: usize = 12;
const SIZE_WIDTH: usize = 8;
const NAME_WIDTH: usize = 24;

const TABLE_WIDTH: usize = 1 + PERM_WIDTH + 1 + SIZE_WIDTH + 1 + NAME_WIDTH + 1;

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max - 1])
    }
}

pub fn print_table(entries: &[Entry]) {
    let mut dirs = 0;
    let mut files = 0;
    let mut links = 0;

    let border = format!("{}", "-".repeat(TABLE_WIDTH - 2));

    println!("{}", border);
    println!(
        "{:<PERM_WIDTH$} {:>SIZE_WIDTH$} {:<NAME_WIDTH$}",
        "PERMS", "SIZE", "NAME"
    );
    println!("{}", border);

    for e in entries {
        match e.entry_type {
            EntryType::Dir => dirs += 1,
            EntryType::File => files += 1,
            EntryType::Link => links += 1,
        }

        let perm = format!("{}{}", e.entry_type.marker(), e.permissions);

        println!(
            "{:<PERM_WIDTH$} {:>SIZE_WIDTH$} {:<NAME_WIDTH$}",
            perm,
            human_size(e.size),
            truncate(&e.name, NAME_WIDTH)
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

        let entry_type = if meta.is_dir() {
            EntryType::Dir
        } else if meta.is_file() {
            EntryType::File
        } else {
            EntryType::Link
        };

        entries_list.push(Entry {
            name,
            size: meta.len(),
            permissions: format_permissions(&meta),
            entry_type,
        });
    }

    entries_list
}

pub fn human_size(size: u64) -> String {
    const UNITS: [&str; 5] = ["B", "K", "M", "G", "T"];
    let mut size = size as f64;
    let mut unit = 0;

    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    if unit == 0 {
        format!("{}{}", size as u64, UNITS[unit])
    } else {
        format!("{:.1}{}", size, UNITS[unit])
    }
}
