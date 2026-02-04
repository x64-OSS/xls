use std::fs::Metadata;
use terminal_size::{Height, Width, terminal_size};

use std::os::unix::fs::PermissionsExt;

use users::get_user_by_uid;

use crate::defs::Entry;

pub fn visible_width(s: &str) -> usize {
    strip_ansi_escapes::strip(s)
        .into_iter()
        .map(|v| String::from_utf8_lossy(&v).len())
        .unwrap_or(s.len())
}
pub fn max_entry_width(entries: &[Entry]) -> usize {
    entries
        .iter()
        .map(|e| visible_width(&e.name))
        .max()
        .unwrap_or(0)
}

pub fn column_width(entries: &[Entry]) -> usize {
    max_entry_width(entries) + 2
}

pub fn compute_layout(entries: &[Entry]) -> (usize, usize) {
    let term_width = terminal_size()
        .and_then(|(Width(w), _)| Some(w as usize))
        .unwrap_or(80);

    let col_width = column_width(entries).max(1);
    let cols = (term_width / col_width).max(1);
    let rows = (entries.len() + cols - 1) / cols;

    (rows, cols)
}

pub fn get_user_name_by_id(id: u32) -> String {
    get_user_by_uid(id)
        .unwrap()
        .name()
        .to_string_lossy()
        .to_string()
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
