#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("libkdbx-rs/keepassxc/src/core/ModifiableObject.h");
        include!("libkdbx-rs/keepassxc/src/core/Database.h");

        type Database;

        fn new_database(filePath: &str) -> UniquePtr<Database>;
    }
}

fn main() {
    let database = ffi::new_database("path/to/database.rs");
}
