use crate::defs::{Args, Entry};

fn sort_entires_by_name(entries: &mut Vec<Entry>) {
    entries.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
}

fn sort_entires_by_size(entries: &mut Vec<Entry>) {
    entries.sort_by(|a, b| a.size.cmp(&b.size))
}

fn sort_entires_by_extension(entries: &mut Vec<Entry>) {
    entries.sort_by(|a, b| a.name.split('.').last().unwrap().cmp(&b.name.split('.').last().unwrap()))
}

fn sort_entires_by_time(entries: &mut Vec<Entry>) {
    entries.sort_by(|a, b| a.last_modified.cmp(&b.last_modified))
}

pub fn sort_entires(entries: &mut Vec<Entry>, args: &Args) {
    if args.sort_by_size {
        sort_entires_by_size(entries)
    } else if args.sort_by_extension {
        sort_entires_by_extension(entries)
    } else if args.sort_by_time {
        sort_entires_by_time(entries)
    } else {
        sort_entires_by_name(entries)
    }

    if args.reverse {
        entries.reverse();
    }
}
