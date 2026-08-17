use cmake::Config;
use std::env;

fn main() {
    let target = env::var("TARGET").unwrap_or_default();
    if target != "wasm32-unknown-emscripten" {
        return;
    }

    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_STATIC_LINK");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../vendor/CoolProp/CMakeLists.txt");
    println!("cargo:rerun-if-changed=../vendor/CoolProp/src");
    println!("cargo:rerun-if-changed=../vendor/CoolProp/include");

    let static_link = env::var_os("CARGO_FEATURE_STATIC_LINK").is_some();
    let out_dir = env::var("OUT_DIR").unwrap();

    let mut config = Config::new("../vendor/CoolProp");

    // Prevent CMakeCache contamination when toggling feature modes
    let mode_dir = if static_link { "static" } else { "shared" };
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

    // Mode-specific CMake definitions
    if static_link {
        config
            .define("COOLPROP_STATIC_LIBRARY", "ON")
            .define("COOLPROP_SHARED_LIBRARY", "OFF");
    } else {
        config
            .define("COOLPROP_STATIC_LIBRARY", "OFF")
            .define("COOLPROP_SHARED_LIBRARY", "ON");
    }

    // Specify target and build
    config.build_target("CoolProp");
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

        println!("cargo:rustc-link-search=native={}", lib_path.display());
        println!("cargo:rustc-link-lib=static:+whole-archive=CoolProp");

    } else {
        let shared_lib = lib_path.join("libCoolProp.so");
        if !shared_lib.exists() {
            panic!(
                "libCoolProp.so not found at expected path: {}",
                shared_lib.display()
            );
        }

        println!("cargo:wasm_artifact={}", shared_lib.display());
    }
}