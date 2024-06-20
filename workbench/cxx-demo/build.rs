use std::path;

fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/blobstore.cc")
        .include(path::PathBuf::from("include"))
        .std("c++14")
        .compile("cxxbridge-demo");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/blobstore.cc");
    println!("cargo:rerun-if-changed=include/blobstore.h");
}
