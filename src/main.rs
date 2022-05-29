#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("libkdbx-rs/keepassxc/src/core/AutoTypeAssociations.h");
        include!("libkdbx-rs/keepassxc/src/core/ModifiableObject.h");
        include!("libkdbx-rs/keepassxc/src/core/Database.h");
        include!("libkdbx-rs/keepassxc/src/core/Entry.h");
        include!("libkdbx-rs/keepassxc/src/core/Group.h");
        include!("libkdbx-rs/keepassxc/src/core/CustomData.h");
        include!("libkdbx-rs/keepassxc/src/core/EntryAttachments.h");
        include!("libkdbx-rs/keepassxc/src/core/EntryAttributes.h");
        include!("libkdbx-rs/keepassxc/src/core/Metadata.h");
        include!("libkdbx-rs/keepassxc/src/core/TimeInfo.h");
        include!("libkdbx-rs/keepassxc/src/core/Global.h");
        // include!("libkdbx-rs/keepassxc/src/core/Merger.h");
        // Resources.h ???
        // Tools.h ???

        // TODO what types from Autotype Associations?
        type ModifiableObject;
        type Database;
        type Entry;
        type Group;
        type CustomData;
        type EntryAttachments;
        type EntryAttributes;
        type Metadata;
        type TimeInfo;

        //fn new_entry(filePath: &str) -> UniquePtr<Database>;
    }
}

fn main() {
    //let database = ffi::new_database("path/to/database.rs");
}
