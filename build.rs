use std::env;

use cxx_build::CFG;

fn main() {
    // FIXME we should not have to hardcode this.
    env::set_var(
        "CPLUS_INCLUDE_PATH",
        "/usr/include/x86_64-linux-gnu/qt5/:/usr/include/x86_64-linux-gnu/qt5/QtCore",
    );

    println!("cargo:CXX_FLAGS=\"-I /usr/include/x86_64-linux-gnu/qt5/\"");
    cxx_build::bridge("src/main.rs")
        .file("keepassxc/src/core/Database.cpp")
        // .flag_if_supported("-std=c++14")
        .compile("libkdbx-rs");
}
