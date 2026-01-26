use clap::{Parser, ValueEnum};

#[derive(Debug)]
pub struct Entry {
    pub name: String,
    pub size: u64,
    pub permissions: String,
    pub last_modified: String,
}

#[derive(Parser, Debug, Clone, ValueEnum)]
pub enum PrintStyle {
    #[clap()]
    Normal,
    List,
    Tree,
}

#[derive(Parser, Debug)]
#[command(
    author = "x64-tech",
    version = "0.0.2",
    about = "List all files",
    long_about = "Small utility tool for list files"
)]
pub struct Args {
    #[arg(short, long, default_value = ".")]
    pub path: String,

    #[arg(short, long, help = "do not ignore entries starting with .")]
    pub all: bool,

    #[arg(short = 'A', long, help="do not list implied . and ..")]
    pub all_most: bool,

    #[arg(long, help = "Print authors of all files")]
    pub author: bool,

    #[arg(short, long, help = "Print only directory not their contant")]
    pub directry: bool,

    #[arg(short, long, help = "print the index number of each file")]
    pub inode: bool,

    #[arg(short, long, value_enum, default_value_t=PrintStyle::Normal)]
    pub style: PrintStyle,
}
