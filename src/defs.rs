#[derive(Debug)]
pub enum EntryType {
    Dir,
    File,
    Link,
}

impl EntryType {
    pub fn order(&self) -> u8 {
        match self {
            EntryType::Dir => 0,
            EntryType::File => 1,
            EntryType::Link => 2,
        }
    }

    pub fn marker(&self) -> char {
        match self {
            EntryType::Dir => 'd',
            EntryType::File => '-',
            EntryType::Link => 'l',
        }
    }
}

#[derive(Debug)]
pub struct Entry {
    pub name: String,
    pub size: u64,
    pub permissions: String,
    pub entry_type: EntryType,
    pub last_modified: String
}
