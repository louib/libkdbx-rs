use std::env;

use cxx_build::CFG;

fn main() {
    let include_paths = vec![
        "/usr/include/x86_64-linux-gnu/qt5/",
        "/usr/include/x86_64-linux-gnu/qt5/QtTest",
        "/usr/include/x86_64-linux-gnu/qt5/QtCore",
        "/usr/include/x86_64-linux-gnu/qt5/QtConcurrent",
        "/usr/include/botan-2",
        "./keepassxc/src",
    ];
    // FIXME we should not have to hardcode this.
    env::set_var("CPLUS_INCLUDE_PATH", include_paths.join(":"));
    env::set_var("LD_LIBRARY_PATH", "/usr/lib/x86_64-linux-gnu/");

    println!("cargo:rustc-link-lib=Qt5Test");
    println!("cargo:rustc-link-lib=Qt5Core");
    println!("cargo:rustc-link-lib=Qt5Concurrent");
    println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu/");
    println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu/qt5");

    cxx_build::bridge("src/main.rs")
        .file("keepassxc/src/core/ModifiableObject.cpp")
        .file("keepassxc/src/core/Database.cpp")
        // .flag_if_supported("-std=c++14")
        .flag_if_supported("-std=gnu++17")
        .flag_if_supported("-DQT_NO_DEBUG")
        .flag_if_supported("-DQT_NO_DEBUG_OUTPUT")
        .flag_if_supported("-DQT_NO_DEPRECATED_WARNINGS")
        .flag_if_supported("-DQT_NO_EXCEPTIONS")
        .compile("libkdbx-rs");
}
