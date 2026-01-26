use crate::defs::{Entry, PrintStyle};

pub  fn print_entries(entries: Vec<Entry>, style: PrintStyle) {
    match style {
        PrintStyle::List => {}
        PrintStyle::Tree => {}
        _ => {
            print_normal_entries(entries);
        }
    }
}

fn print_normal_entries(entries: Vec<Entry>) {
    for entry in entries {
        print!("{}\t", entry.name);
    }
    println!();
}
