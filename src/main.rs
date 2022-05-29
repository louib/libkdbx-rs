#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("libkdbx-rs/keepassxc/src/core/Entry.h");
        // include!("libkdbx-rs/keepassxc/src/core/AutoTypeAssociations.h");
        // include!("libkdbx-rs/keepassxc/src/core/ModifiableObject.h");
        // include!("libkdbx-rs/keepassxc/src/core/Database.h");
        // include!("libkdbx-rs/keepassxc/src/core/Group.h");
        // include!("libkdbx-rs/keepassxc/src/core/CustomData.h");
        // include!("libkdbx-rs/keepassxc/src/core/EntryAttachments.h");
        // include!("libkdbx-rs/keepassxc/src/core/EntryAttributes.h");
        // include!("libkdbx-rs/keepassxc/src/core/Metadata.h");
        // include!("libkdbx-rs/keepassxc/src/core/TimeInfo.h");
        // include!("libkdbx-rs/keepassxc/src/core/Global.h");
        // include!("libkdbx-rs/keepassxc/src/core/Merger.h");
        // Resources.h ???
        // Tools.h ???

        // TODO what types from Autotype Associations?
        // type AutoTypeAssociations;
        // type ModifiableObject;
        // type Database;
        type Entry;
        // type EntryData;
        // type Group;
        // type CustomData;
        // CloneFlag??
        // PlaceholderType??
        // Totp??
        // type EntryAttachments;
        // type EntryAttributes;
        // type EntryReferenceType;
        // type Metadata;
        // type TimeInfo;
        //
        // fn new_entry() -> UniquePtr<Database>;
    }


}

fn main() {
    // let entry = ffi::Entry::default();

    // let entry_data = ffi::EntryData{};
}
