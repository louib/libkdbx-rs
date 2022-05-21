fn main() {
    cxx_build::bridge("src/main.rs")
        .file("keepassxc/src/core/Database.cpp")
        .compile("libkdbx-rs");
}
