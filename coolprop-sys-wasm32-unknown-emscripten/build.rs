use cmake::Config;
use std::env;

fn main() {
    // This crate only makes sense for wasm32-unknown-emscripten - skip
    // cleanly on any other target so a workspace-wide `cargo build`/`check`
    // (host target, no --target) doesn't try to cross-compile CoolProp.
    let target = env::var("TARGET").unwrap_or_default();
    if target != "wasm32-unknown-emscripten" {
        return;
    }

    println!("cargo:rerun-if-changed=build.rs");
    // Watch only the actual source inputs, not the whole vendor/CoolProp tree:
    // CMake's generate_headers custom target unconditionally rewrites
    // ../vendor/CoolProp/.version and ../vendor/CoolProp/dev/hashes.json on
    // every build, and a directory-wide rerun-if-changed picks those up
    // recursively, marking this crate dirty on every subsequent build even
    // when nothing meaningful changed.
    println!("cargo:rerun-if-changed=../vendor/CoolProp/CMakeLists.txt");
    println!("cargo:rerun-if-changed=../vendor/CoolProp/src");
    println!("cargo:rerun-if-changed=../vendor/CoolProp/include");

    // cmake-rs recognizes the wasm32-unknown-emscripten TARGET and drives
    // the configure through `emcmake` (which sets CMAKE_TOOLCHAIN_FILE to
    // Emscripten's own toolchain file) automatically - no manual
    // CC/CXX/CMAKE_TOOLCHAIN_FILE wiring needed here, same as running
    // `emcmake cmake` by hand would do.
    let dst = Config::new("../vendor/CoolProp")
        .define("COOLPROP_STATIC_LIBRARY", "ON")
        .define("COOLPROP_EXTERNC_LIBRARY", "ON")
        .cflag("-DCOOLPROP_NO_INCBIN")
        .cxxflag("-DCOOLPROP_NO_INCBIN")
        .cflag("-fwasm-exceptions")
        .cxxflag("-fwasm-exceptions")
        .cflag("-fPIC")
        .cxxflag("-fPIC")
        .build_target("CoolProp")
        .build();

    let lib_path = dst.join("build");
    if !lib_path.join("libCoolProp.a").exists() {
        panic!(
            "libCoolProp.a not found at expected path: {}",
            lib_path.display()
        );
    }

    // cargo:rustc-link-arg (tried first) is scoped to *this* crate's own
    // targets and silently does not propagate to a dependent's link step -
    // only rustc-link-lib/rustc-link-search do that. So: rustc-link-lib
    // with the +whole-archive modifier (stable since Rust 1.61), not a raw
    // -Wl,--whole-archive link-arg. +whole-archive also sidesteps the
    // ordering problem a plain `static=CoolProp` would have: wasm-ld
    // resolves archives lazily in a single left-to-right pass, and could
    // place this one before the Rust object files that reference its
    // symbols (see coolprop-test's build.rs in rfluids for the same issue
    // with the prebuilt archive) - +whole-archive pulls in every member
    // unconditionally, so position no longer matters.
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=static:+whole-archive=CoolProp");
}


/*//use cmake::Config;

fn main() {
    /*println!("cargo:rerun-if-changed=build.rs");
    // Watch only the actual source inputs, not the whole vendor/CoolProp tree:
    // CMake's generate_headers custom target unconditionally rewrites
    // ../vendor/CoolProp/.version and ../vendor/CoolProp/dev/hashes.json on
    // every build, and a directory-wide rerun-if-changed picks those up
    // recursively, marking this crate dirty on every subsequent build even
    // when nothing meaningful changed.
    println!("cargo:rerun-if-changed=../vendor/CoolProp/CMakeLists.txt");
    println!("cargo:rerun-if-changed=../vendor/CoolProp/src");
    println!("cargo:rerun-if-changed=../vendor/CoolProp/include");

    let dst = Config::new("../vendor/CoolProp")
        .define("COOLPROP_STATIC_LIBRARY", "ON")
        .define("COOLPROP_EXTERNC_LIBRARY", "ON")
        .cflag("-DCOOLPROP_NO_INCBIN")
        .cxxflag("-DCOOLPROP_NO_INCBIN")
        .cflag("-fwasm-exceptions")
        .cxxflag("-fwasm-exceptions")
        .cflag("-fPIC")
        .cxxflag("-fPIC")
        .build();
    
    let lib_path = dst.join("build");
    if !lib_path.join("libCoolProp.a").exists() {
        panic!("libCoolProp.a not found at expected path: {}", lib_path.display());
    }

    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=static=CoolProp");*/
}
*/