use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=monome");
    println!("cargo:rerun-if-changed=wrapper.h");

    let include_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("vendor")
        .join("libmonome")
        .join("public");

    let include_dir_str = include_dir
        .to_str()
        .expect("include path is a valid string");

    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}", include_dir_str))
        .header("wrapper.h")
        .allowlist_item("monome.*")
        .generate()
        .expect("generated bindings");

    let mut output_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    output_dir.push("gen");

    // let output_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(output_dir.join("bindings.rs"))
        .expect("writing bindings");
}
