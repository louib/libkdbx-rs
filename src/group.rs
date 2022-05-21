use crate::entry::Entry;

pub struct Group {
    pub uuid: String,

    pub name: String,
    pub notes: String,
    pub tags: String,

    entries: Vec<Entry>,
}
