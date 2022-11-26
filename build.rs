//! Build script for the runtime.

fn main() {
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=examples/");
    println!("cargo:rerun-if-changed=linker_script.ld");
    cc::Build::new()
        .file("src/runtime.S")
        .compiler("arm-none-eabi-gcc")
        .archiver("arm-none-eabi-ar")
        .compile("runtime");
}
