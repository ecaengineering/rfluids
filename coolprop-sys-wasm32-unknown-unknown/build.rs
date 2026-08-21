use std::{env, path::Path};

fn main() {
    let target = env::var("TARGET").unwrap_or_default();
    if target != "wasm32-unknown-unknown" {
        return;
    }

    println!("cargo:rerun-if-changed=build.rs");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_dir = Path::new(&manifest_dir).join("lib");
    println!("cargo:rerun-if-changed={}", lib_dir.display());

    let static_lib = lib_dir.join("libCoolProp.a");
    if !static_lib.is_file() {
        panic!("libCoolProp.a not found at expected path: {}", static_lib.display());
    }

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=CoolProp");
}
