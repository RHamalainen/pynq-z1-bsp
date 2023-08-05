//! Build script for the runtime.

fn main() {
    use std::env::var;
    use std::fs;
    use std::path::PathBuf;

    // Check build target.
    let target_expected = "armv7a-none-eabi";
    let target_actual = var("TARGET").unwrap();
    if target_actual != target_expected {
        panic!(
            "attempted to build for target {target_actual} but correct target is {target_expected}"
        );
    }

    // Move linker script so that linker can find it.
    let out_dir = PathBuf::from(var("OUT_DIR").unwrap());
    fs::write(
        out_dir.join("linker_script.ld"),
        include_bytes!("linker_script.ld"),
    )
    .unwrap();
    println!("cargo:rustc-link-search={}", out_dir.display());

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
