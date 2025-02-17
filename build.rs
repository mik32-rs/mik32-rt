use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("link.x"))
        .unwrap()
        .write_all(include_bytes!("link.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=link.x");

    // cc::Build::new()
    //     .asm_flag("-march=rv32imc_zicsr_zifencei")
    //     .asm_flag("-mabi=ilp32")
    //     .asm_flag("-g")
    //     .file("/home/zbykovd/projects/mik32/mik32-rt/crt0.S")
    //     .compile("csr0");
}
