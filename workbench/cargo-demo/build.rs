use std::env;
use std::fs;
use std::path::Path;

fn hello() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("hello.rs");
    fs::write(
        &dest_path,
        "pub fn message() -> &'static str {
            \"Hello, World!\"
        }
        "
    ).unwrap();

}

fn link_z() {
    pkg_config::Config::new().probe("zlib").unwrap();
}

fn main() {
    hello();
    link_z();
    println!("cargo:rerun-if-changed=build.rs");
}