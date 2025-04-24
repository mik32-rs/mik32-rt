use std::{env, fs};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let memory_file = if env::var("CARGO_FEATURE_MEMORY_SPIFI").is_ok() {
        "memory_spifi.x"
    } else {
        "memory_default.x"
    };

    let memory_path = PathBuf::from(memory_file);
    let out_memory = out.join("memory.x");

    fs::copy(memory_path, &out_memory).expect("Failed to copy memory.x");
    File::create(out.join("link.x"))
        .unwrap()
        .write_all(include_bytes!("link.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=link.x");
}
