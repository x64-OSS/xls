use crate::defs::{Args, Entry, IgnoreMode, ListMode};
use crate::formatter::formatted_entry_permissions;
use std::fs::{self, FileType, metadata};
use users::get_user_by_uid;

use std::os::unix::fs::MetadataExt;
use std::path::Path;

pub fn list_directory(path: &Path, args: &Args, ignore_mode: &IgnoreMode) -> Vec<Entry> {
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
        let file_type = entry.file_type().expect("Failed to get file type");

        // FIXME: "~" suffix for backups files is unknown
        if args.ignore_backups && name.ends_with("~") {
            continue;
        }

        match ignore_mode {
            IgnoreMode::IgnoreNone => {}
            IgnoreMode::IgnoreDefault => {
                if name.starts_with(".") {
                    continue;
                }
            }
            IgnoreMode::IgnoreDotAndDotDot => {
                if name == "." || name == ".." {
                    continue;
                }
            }
        }

        let meta = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        let owner = meta.uid();
        let author = meta.gid();

        let last_modified: String = if let Ok(time) = meta.modified() {
            let syst = time_format::from_system_time(time).unwrap();
            time_format::strftime_utc("%Y-%m-%d %H:%M:%S", syst).unwrap()
        } else {
            String::from("--")
        };

        let inode: Option<String> = if args.inode {
            Some(meta.ino().to_string())
        } else {
            None
        };

        let size = meta.size();
        let permissions = formatted_entry_permissions(&meta);

        entries_list.push(Entry {
            name,
            size,
            permissions,
            last_modified,
            inode,
            file_type,
            author,
            owner,
        });
    }

    entries_list
}
