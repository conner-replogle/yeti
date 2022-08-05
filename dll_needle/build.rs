
use std::env;
use std::path::PathBuf;
// build.rs, in the project root folder
fn main() {
    cc::Build::new()
        .file("src/library_loader.c")
        .target("i686-pc-windows-msvc")
        .compile("lib_loader");
    // println!("cargo:rustc-link-search=all=src");      // works like "rustc -L src ..." 
    // println!("cargo:rustc-link-lib=dylib=lib_loader.lib"); // works like "rustc -l doubler.o"
}