use crate::entry::Entry;
use crate::group::Group;

pub struct Database {
    entries: Vec<Entry>,
    groups: Vec<Group>,
}
