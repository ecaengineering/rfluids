use cmake::Config;
use std::env;

fn main() {
    let target = env::var("TARGET").unwrap_or_default();
    if target != "wasm32-unknown-emscripten" {
        return;
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../vendor/CoolProp/CMakeLists.txt");
    println!("cargo:rerun-if-changed=../vendor/CoolProp/src");
    println!("cargo:rerun-if-changed=../vendor/CoolProp/include");

    let static_link = env::var_os("CARGO_FEATURE_STATIC_LINK").is_some();

    let mut config = Config::new("../vendor/CoolProp");
    config
        .define("COOLPROP_EXTERNC_LIBRARY", "ON")
        .cflag("-DCOOLPROP_NO_INCBIN")
        .cxxflag("-DCOOLPROP_NO_INCBIN")
        .cflag("-fwasm-exceptions")
        .cxxflag("-fwasm-exceptions")
        .cflag("-fPIC")
        .cxxflag("-fPIC")
        .build_target("CoolProp");

    if static_link {
        config
            .define("COOLPROP_STATIC_LIBRARY", "ON")
            .define("COOLPROP_SHARED_LIBRARY", "OFF");
    } else {
        config
            .define("COOLPROP_STATIC_LIBRARY", "OFF")
            .define("COOLPROP_SHARED_LIBRARY", "ON");
    }

    let dst = config.build();
    let lib_path = dst.join("build");

    if static_link {
        let static_lib = lib_path.join("libCoolProp.a");
        if !static_lib.exists() {
            panic!(
                "libCoolProp.a not found at expected path: {}",
                static_lib.display()
            );
        }

        // rustc-link-lib with +whole-archive is required (rather than a
        // plain `static=CoolProp` or a raw -Wl,--whole-archive link-arg)
        // for the reasons noted above: cargo:rustc-link-arg doesn't
        // propagate to a dependent crate's link step, and wasm-ld's
        // single-pass lazy archive resolution can otherwise drop symbols
        // depending on link order.
        println!("cargo:rustc-link-search=native={}", lib_path.display());
        println!("cargo:rustc-link-lib=static:+whole-archive=CoolProp");
    } else {
        // Emscripten shared-library output, dlopen'd at runtime by
        // coolprop-sys's dynamic bindings (bindings_generated.rs: a plain
        // struct of function pointers populated via `libloading::Library`
        // - no link-time symbol references at all, so this is NOT a
        // rustc-link-lib/rustc-link-search situation like the static-link
        // branch above). The only thing a dependent needs is this file's
        // path, so it can be embedded into the Emscripten module's virtual
        // filesystem at /CoolProp.wasm (the path coolprop-sys's
        // `load_coolprop()` passes to `dlopen`) - published via the
        // `links = "coolprop"` metadata mechanism as
        // DEP_COOLPROP_WASM_ARTIFACT, since heatpump-wasm (the thing that
        // actually needs to embed it) doesn't go through cmake itself.
        let shared_lib = lib_path.join("libCoolProp.so");
        if !shared_lib.exists() {
            panic!(
                "libCoolProp.so not found at expected path: {}",
                shared_lib.display()
            );
        }

        println!("cargo:wasm-artifact={}", shared_lib.display());
    }
}