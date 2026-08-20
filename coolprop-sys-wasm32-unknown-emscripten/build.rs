use std::{env, path::Path};

use cmake::Config;

fn main() {
    let target = env::var("TARGET").unwrap_or_default();
    if target != "wasm32-unknown-emscripten" {
        return;
    }

    println!("cargo:rerun-if-changed=build.rs");

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

    // Same mode_dir either way: it's what tells the two build paths below
    // (prebuilt vs. compiled-from-source) apart from each other, and lets
    // both keep more than one mode's output around at once without
    // clobbering it (cmake's own CMakeCache.txt, or a checked-in prebuilt
    // binary) when a workspace build switches between them.
    let mode_dir = format!(
        "{}-{}",
        if static_link { "static" } else { "shared" },
        if static_refprop { "static-refprop" } else { "dlopen-refprop" }
    );

    if cfg!(feature = "prebuilt") {
        if !static_link {
            // Only a static-link/static-refprop libCoolProp.a is checked in
            // (see lib/static-static-refprop/) - the shared-library mode is
            // less commonly used (coolprop-test's own dynamic-mode check is
            // the only consumer, per its Cargo.toml) and would mean shipping
            // a second, differently-produced prebuilt binary to keep in
            // sync. Build without `prebuilt` for that mode instead.
            panic!(
                "the `prebuilt` feature only ships a static-link/static-refprop libCoolProp.a - \
                 build without `prebuilt` (or with static-link enabled) to get a shared-library \
                 build compiled from source instead"
            );
        }
        use_prebuilt(&mode_dir);
        return;
    }

    println!("cargo:rerun-if-changed=../vendor/CoolProp/CMakeLists.txt");
    println!("cargo:rerun-if-changed=../vendor/CoolProp/src");
    println!("cargo:rerun-if-changed=../vendor/CoolProp/include");

    let out_dir = env::var("OUT_DIR").unwrap();

    let mut config = Config::new("../vendor/CoolProp");

    // Prevent CMakeCache contamination when toggling feature modes
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

/// Links a checked-in prebuilt `libCoolProp.a` from `lib/{mode_dir}/`
/// instead of compiling CoolProp from source - see the `prebuilt` feature's
/// doc comment in Cargo.toml. Publishes the same `rustc-link-lib`/
/// `rustc-link-search` build-script output as the cmake static-link path
/// above, so every dependent (coolprop-sys, heatpump-wasm, coolprop-test)
/// sees no difference between the two.
fn use_prebuilt(mode_dir: &str) {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_dir = Path::new(&manifest_dir).join("lib").join(mode_dir);

    println!("cargo:rerun-if-changed={}", lib_dir.display());

    let static_lib = lib_dir.join("libCoolProp.a");
    if !static_lib.is_file() {
        panic!(
            "prebuilt CoolProp library not found: {} \
             (the `prebuilt` feature is on, but no libCoolProp.a is checked in for the \
             `{mode_dir}` static-link/static-refprop combination - either add it there, or \
             build without `prebuilt` to compile it from source via cmake+em++ instead)",
            static_lib.display()
        );
    }

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=CoolProp");
}
