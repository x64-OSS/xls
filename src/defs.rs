use std::fs::FileType;

use clap::Parser;

#[derive(Debug)]
pub struct Entry {
    pub name: String,
    pub size: Option<u64>,
    pub file_type: FileType,
    pub permissions: Option<String>,
    pub last_modified: Option<String>,
    pub inode: Option<String>,
    pub author: Option<u32>,
    pub owner: Option<u32>,
}

#[derive(Debug)]
pub struct FileTypeCount {
    pub files: usize,
    pub dirs: usize,
    pub links: usize,
}

pub enum SortType {
    SortName,
    SortTime,
    SortExtension,
    SortSize,
    SortVersion,
    SortNone,
    SortNumTypes,
}

pub enum IgnoreMode {
    IgnoreDefault, // ignore hidden files with . prefix
    IgnoreNone,
    IgnoreDotAndDotDot, // ignore . and ..
}

pub enum ListMode {
    Default,
    Long,
    Column,
}

#[derive(Debug, Clone)]
pub enum TimeStyle {
    FullISO,
    LongISO,
    ISO,
    Locale,
}

#[derive(Parser, Debug)]
#[command(
    author = "x64-tech",
    version = "0.0.2",
    about = "List files info",
    long_about = "List information about the FILEs (the current directory by default)."
)]
pub struct Args {
    #[arg(default_value = ".")]
    pub path: String,

    #[arg(short, long, help = "do not ignore entries starting with .")]
    pub all: bool,

    #[arg(short = 'A', long, help = "do not list implied . and ..")]
    pub all_most: bool,

    #[arg(long, help = "Print authors of all files")]
    pub author: bool,

    #[arg(short = 'B', long, help = "do not list implied entries ending with ~")]
    pub ignore_backups: bool,

    #[arg(short, long, help = "Print only directory not their contant")]
    pub directry: bool,

    #[arg(short, long, help = "print the index number of each file")]
    pub inode: bool,

    #[arg(short, long, help = "Print entries in long format")]
    pub long: bool,

    #[arg(short = 'C', long, help = "list entries by columns")]
    pub column: bool,

    #[arg(short = 'x', help = "list entries by lines instead of by columns")]
    pub line: bool,

    #[arg(short, long, help = "reverse order while sorting")]
    pub reverse: bool,

    #[arg(short = 'R', long, help = "list subdirectories recursively")]
    pub recursive: bool,

    #[arg(short = 'S', long, help = "sort by file size, largest first")]
    pub sort_by_size: bool,

    #[arg(short = 'X', long, help = "sort alphabetically by entry extension")]
    pub sort_by_extension: bool,

    #[arg(short = 't', long, help = "sort by time, newest first")]
    pub sort_by_time: bool,

    #[arg(short = 'Q', help = "enclose entry names in double quotes")]
    pub quoted_name: bool,

    #[arg(
        long,
        help = "use quoting style WORD for entry names:
literal, locale, shell, shell-always,
shell-escape, shell-escape-always, c, escape"
    )]
    pub quoting_style: Option<String>,
}

impl Args {
    pub fn parse_sort_type(&self) -> SortType {
        if self.sort_by_extension {
            SortType::SortExtension
        } else if self.sort_by_size {
            SortType::SortSize
        } else if self.sort_by_time {
            SortType::SortTime
        } else {
            SortType::SortName
        }
    }

    pub fn parse_ignore_mode(&self) -> IgnoreMode {
        if self.all {
            IgnoreMode::IgnoreNone
        } else if self.all_most {
            IgnoreMode::IgnoreDotAndDotDot
        } else {
            IgnoreMode::IgnoreDefault
        }
    }

    pub fn parse_list_mode(&self) -> ListMode {
        if self.long {
            ListMode::Long
        } else if self.column {
            ListMode::Column
        } else {
            ListMode::Default
        }
    }

    pub fn parse_time_style(&self) -> TimeStyle {
        TimeStyle::FullISO
    }
}
