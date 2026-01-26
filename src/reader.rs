use crate::defs::{Args, Entry};
use crate::formatter::formatted_entry_permissions;
use std::fs;

use std::os::unix::fs::MetadataExt;
use std::path::Path;

pub fn list_directory(path: &Path, args: &Args) -> Vec<Entry> {
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

        if !args.all && name.starts_with('.') {
            continue;
        }

        let meta = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        let last_modified: String = if let Ok(time) = meta.modified() {
            let syst = time_format::from_system_time(time).unwrap();
            time_format::strftime_utc("%Y-%m-%d %H:%M:%S", syst).unwrap()
        } else {
            String::from("--")
        };

        let size = meta.size();
        let permissions = formatted_entry_permissions(&meta);

        entries_list.push(Entry {
            name,
            size,
            permissions,
            last_modified,
        });
    }

    entries_list
}
