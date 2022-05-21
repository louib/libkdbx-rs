pub enum EntryReference {
    Unknown,
    Title,
    UserName,
    Password,
    Url,
    Notes,
    QUuid,
    CustomAttributes,
}

pub struct Entry {
    pub uuid: String,

    pub title: String,
    pub username: String,
    pub password: String,
    pub url: String,
    pub notes: String,
}
