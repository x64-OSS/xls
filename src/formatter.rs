use crate::defs::EntryType;
use std::fs::Metadata;
use std::os::unix::fs::PermissionsExt;

const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";
const ITALIC: &str = "\x1b[3m";

pub fn bold_formatter(val: &str) -> String {
    return format!("{}{}{} ", BOLD, val, RESET);
}

pub fn italic_formatter(val: &str) -> String {
    return format!("{}{}{} ", ITALIC, val, RESET);
}

pub fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max - 1])
    }
}

pub fn formatted_entry_size(size: u64) -> String {
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

pub fn formatted_entry_name(val: &str, ent_type: &EntryType) -> String {
    return match ent_type {
        EntryType::Dir => bold_formatter(val),
        EntryType::Link => italic_formatter(val),
        EntryType::File => val.to_string(),
    };
}

pub fn formatted_entry_permissions(meta: &Metadata) -> String {
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
