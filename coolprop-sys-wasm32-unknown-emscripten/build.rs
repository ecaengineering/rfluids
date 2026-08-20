use std::env;

use cmake::Config;

fn main() {
    let target = env::var("TARGET").unwrap_or_default();
    if target != "wasm32-unknown-emscripten" {
        return;
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../vendor/CoolProp/CMakeLists.txt");
    println!("cargo:rerun-if-changed=../vendor/CoolProp/src");
    println!("cargo:rerun-if-changed=../vendor/CoolProp/include");

    let static_link = cfg!(feature = "static-link");
    let static_refprop = cfg!(feature = "static-refprop");
    if static_refprop && !static_link {
        // The static REFPROP archives only get linked into the final Rust
        // binary via rustc-link-lib below - that only reaches CoolProp's own
        // code when CoolProp itself is whole-archive-linked into that same
        // binary (static-link). In shared mode, libCoolProp.so is fully
        // linked by CMake/em++ as its own standalone wasm module, which
        // would be left with undefined REFPROP symbols.
        panic!(
            "static-refprop requires static-link (REFPROP can't be linked into a separately-built libCoolProp.so this way)"
        );
    }
    let out_dir = env::var("OUT_DIR").unwrap();

    let mut config = Config::new("../vendor/CoolProp");

    // Prevent CMakeCache contamination when toggling feature modes
    let mode_dir = format!(
        "{}-{}",
        if static_link { "static" } else { "shared" },
        if static_refprop { "static-refprop" } else { "dlopen-refprop" }
    );
    config.out_dir(format!("{out_dir}/{mode_dir}"));

    // Common flags
    config
        .define("COOLPROP_EXTERNC_LIBRARY", "ON")
        .cflag("-DCOOLPROP_NO_INCBIN")
        .cxxflag("-DCOOLPROP_NO_INCBIN")
        .cflag("-fwasm-exceptions")
        .cxxflag("-fwasm-exceptions")
        .cflag("-fPIC")
        .cxxflag("-fPIC");

    if static_refprop {
        // Makes REFPROPMixtureBackend.cpp call directly into the statically
        // linked librefprop.a (via refprop_static_bindings.h) instead of
        // dlopen()-ing librefprop.so at runtime - see this crate's
        // static-refprop feature doc in Cargo.toml.
        config
            .cflag("-DCOOLPROP_REFPROP_STATIC_LINK=1")
            .cxxflag("-DCOOLPROP_REFPROP_STATIC_LINK=1");
    }

    // Mode-specific CMake definitions
    if static_link {
        config.define("COOLPROP_STATIC_LIBRARY", "ON").define("COOLPROP_SHARED_LIBRARY", "OFF");
    } else {
        config.define("COOLPROP_STATIC_LIBRARY", "OFF").define("COOLPROP_SHARED_LIBRARY", "ON");
    }

    // Specify target and build
    config.build_target("CoolProp");
    let dst = config.build();
    let lib_path = dst.join("build");

    if static_link {
        let static_lib = lib_path.join("libCoolProp.a");
        if !static_lib.exists() {
            panic!("libCoolProp.a not found at expected path: {}", static_lib.display());
        }

        println!("cargo:rustc-link-search=native={}", lib_path.display());
        println!("cargo:rustc-link-lib=static=CoolProp");
    } else {
        let shared_lib = lib_path.join("libCoolProp.so");
        if !shared_lib.exists() {
            panic!("libCoolProp.so not found at expected path: {}", shared_lib.display());
        }

        println!("cargo:wasm_artifact={}", shared_lib.display());
    }
}
