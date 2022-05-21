use std::env;

use cxx_build::CFG;

fn main() {
    let include_paths = vec![
        "/usr/include/x86_64-linux-gnu/qt5/",
        "/usr/include/x86_64-linux-gnu/qt5/QtCore",
        "/usr/include/botan-2",
        "./keepassxc/src",
    ];
    // FIXME we should not have to hardcode this.
    env::set_var("CPLUS_INCLUDE_PATH", include_paths.join(":"));

    cxx_build::bridge("src/main.rs")
        .file("keepassxc/src/core/ModifiableObject.cpp")
        .file("keepassxc/src/core/Database.cpp")
        // .flag_if_supported("-std=c++14")
        .compile("libkdbx-rs");
}
