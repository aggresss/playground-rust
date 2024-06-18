use std::env;

fn target_os() -> String {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target = env::var("TARGET").unwrap();
    let is_simulator = target.ends_with("-sim");

    match target_os.as_str() {
        "windows" => "win",
        "macos" => "mac",
        "ios" => {
            if is_simulator {
                "ios-simulator"
            } else {
                "ios-device"
            }
        }
        _ => &target_os,
    }
    .to_string()
}

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("{}", target_os());
}