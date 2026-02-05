use std::{fs::Metadata, os::unix::fs::PermissionsExt, time::SystemTime};

use crate::defs::TimeStyle;

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

pub fn formatted_time(time: SystemTime, style: TimeStyle) {
    let time_for = time_format::from_system_time(time).expect("Failed to parser system time");
}
