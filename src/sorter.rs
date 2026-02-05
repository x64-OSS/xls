use crate::defs::{Entry, SortType};

pub fn sort_entires(entries: &mut Vec<Entry>, sort_type: &SortType, revers: bool) {
    match sort_type {
        SortType::SortExtension => entries.sort_by(|a, b| {
            a.name
                .split('.')
                .last()
                .unwrap()
                .cmp(&b.name.split('.').last().unwrap())
        }),
        SortType::SortSize => entries.sort_by(|a, b| b.size.cmp(&a.size)),
        SortType::SortModified => entries.sort_by(|a, b| a.modified.cmp(&b.modified)),
        SortType::SortCreated => entries.sort_by(|a, b| a.created.cmp(&b.created)),
        SortType::SortName | _ => {
            entries.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        }
    }

    if revers {
        entries.reverse();
    }
}
