use crate::defs::{Args, Entry, SortType};

fn sort_entires_by_name(entries: &mut Vec<Entry>) {
    entries.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
}

fn sort_entires_by_size(entries: &mut Vec<Entry>) {
    entries.sort_by(|a, b| b.size.cmp(&a.size))
}

fn sort_entires_by_extension(entries: &mut Vec<Entry>) {
    entries.sort_by(|a, b| a.name.split('.').last().unwrap().cmp(&b.name.split('.').last().unwrap()))
}

fn sort_entires_by_time(entries: &mut Vec<Entry>) {
    entries.sort_by(|a, b| a.last_modified.cmp(&b.last_modified))
}

pub fn sort_entires(entries: &mut Vec<Entry>, sort_type: &SortType, revers: bool) {
    match sort_type {
        SortType::SortExtension => sort_entires_by_extension(entries),
        SortType::SortSize => sort_entires_by_size(entries),
        SortType::SortTime => sort_entires_by_time(entries),
        SortType::SortName | _ => sort_entires_by_name(entries),
    }

    if revers {
        entries.reverse();
    }
}
