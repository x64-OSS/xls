use crate::defs::{Args, Entry, FileTypeCount, IgnoreMode};
use crate::formatter::formatted_entry_permissions;

use colored::Colorize;

use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

pub struct EntryReturn {
    pub entries: Vec<Entry>,
    pub counts: FileTypeCount,
}

pub fn list_directory(path: &Path, args: &Args, ignore_mode: &IgnoreMode) -> EntryReturn {
    let mut entries_list = Vec::new();
    let mut file_type_count = FileTypeCount {
        files: 0,
        dirs: 0,
        links: 0,
    };

    let entries = fs::read_dir(path).expect("Failed to read directory");

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

        let name = if file_type.is_dir() {
            file_type_count.dirs += 1;
            name.blue().bold().to_string()
        } else if file_type.is_symlink() {
            file_type_count.links += 1;
            name.italic().to_string()
        } else {
            file_type_count.files += 1;
            name.normal().to_string()
        };

        let meta = entry.metadata().expect("Failed to read file metadata");

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

    return EntryReturn {
        entries: entries_list,
        counts: file_type_count,
    };
}
