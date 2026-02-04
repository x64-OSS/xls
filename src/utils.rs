use terminal_size::{Width, terminal_size};
use users::get_user_by_uid;

use crate::defs::Entry;

/// Calculate terminal width
///
/// Using `terminal_size` get the width of terminal.
///
/// # return: usize
pub fn compute_terminal_width() -> usize {
    terminal_size()
        .and_then(|(Width(w), _)| Some(w as usize))
        .unwrap_or(80)
}

/// Calculate visible width of given string
///
/// Using ansi escap calculate visible string len
pub fn visible_width(s: &str) -> usize {
    strip_ansi_escapes::strip(s).iter_mut().len()
}


/// 
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
    let term_width = compute_terminal_width();

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
