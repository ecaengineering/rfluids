use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    if target != "wasm32-unknown-emscripten" {
        return;
    }

    // Allow dynamic memory growth for CoolProp's C++ heap allocations.
    println!("cargo:rustc-link-arg=-sALLOW_MEMORY_GROWTH=1");
    // Enable runtime assertions to surface C++ errors cleanly.
    println!("cargo:rustc-link-arg=-sASSERTIONS=1");

    // Rust's wasm32-unknown-emscripten target already links with
    // -fwasm-exceptions (the native Wasm exception-handling proposal), which
    // is mutually exclusive with the legacy JS-based
    // -sDISABLE_EXCEPTION_CATCHING=0 scheme - do not add that here.
    // CoolProp must instead be built with -fwasm-exceptions to match, whether
    // linked as a side module (below) or as a static archive (linked in
    // directly below).

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    if env::var("CARGO_FEATURE_STATIC_LINK").is_ok() {
        // libCoolProp.a is linked directly into this single wasm module - no
        // separate dlopen()'d module, so none of the MAIN_MODULE/embed-file
        // setup below applies.
        //
        // A plain `cargo:rustc-link-lib=static=CoolProp` (or the equivalent
        // `#[link(name = "CoolProp", kind = "static")]` on the extern block
        // in coolprop-sys's static bindings) places the archive on the link
        // line *before* the object/rlib files that reference its symbols.
        // wasm-ld resolves archives lazily in a single left-to-right pass, so
        // by the time those references show up the archive has already been
        // searched and the link fails with undefined symbols. `--whole-archive`
        // sidesteps this entirely: it forces every member of the archive to
        // be pulled in unconditionally, as if they were plain object files,
        // so position on the link line no longer matters.
        let lib_dir = PathBuf::from(&manifest_dir).join("lib");
        let archive = lib_dir.join("libCoolProp.a");
        if !archive.exists() {
            panic!("libCoolProp.a not found at {:?}", archive);
        }
        println!("cargo:rerun-if-changed={}", archive.display());
        println!("cargo:rustc-link-arg=-Wl,--whole-archive");
        println!("cargo:rustc-link-arg={}", archive.display());
        println!("cargo:rustc-link-arg=-Wl,--no-whole-archive");
        return;
    }

    let wasm_source = PathBuf::from(&manifest_dir)
        .join("lib")
        .join("CoolProp.wasm");

    if !wasm_source.exists() {
        panic!("CoolProp.wasm not found at {:?}", wasm_source);
    }

    println!("cargo:rerun-if-changed={}", wasm_source.display());

    // Core dynamic linking setup: load CoolProp.wasm as a dlopen'd side module.
    println!("cargo:rustc-link-arg=-sMAIN_MODULE=1");
    println!("cargo:rustc-link-arg=-sFORCE_FILESYSTEM=1");

    // Embed CoolProp.wasm inside MEMFS.
    println!(
        "cargo:rustc-link-arg=--embed-file={}@/CoolProp.wasm",
        wasm_source.display()
    );
}
