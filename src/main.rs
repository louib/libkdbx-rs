#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        // TODO change this when we have a git submodule for keepassxc.
        include!("../keepassxc/src/core/Database.h");

        type Database;

        fn new_database(filePath: &str) -> UniquePtr<Database>;
    }
}

fn main() {
    println!("Hello, world!");
}
