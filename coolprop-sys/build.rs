fn main() {
    // Forward coolprop-sys-wasm32-unknown-emscripten's `links = "coolprop"`
    // metadata under our own `links = "coolprop_sys"` name, so dependents
    // building for wasm32-unknown-emscripten in dynamic-link mode can read
    // DEP_COOLPROP_SYS_WASM_ARTIFACT without depending on that crate
    // directly. Absent on every other target/mode, since that crate's
    // build.rs only publishes it there.
    if let Ok(artifact) = std::env::var("DEP_COOLPROP_WASM_ARTIFACT") {
        println!("cargo:wasm_artifact={artifact}");
    }

    #[cfg(feature = "regen-bindings")]
    {
        use std::{env, path::PathBuf};

        let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

        println!("cargo:rerun-if-changed=build.rs");
        println!("cargo:rerun-if-changed=CoolPropLib.h");

        fn base_builder() -> bindgen::Builder {
            bindgen::Builder::default()
                .header("CoolPropLib.h")
                .derive_debug(true)
                .derive_default(true)
                .use_core()
                .generate_cstr(true)
                .generate_comments(false)
                .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        }

        let static_bindings = base_builder()
            .generate()
            .expect("bindgen should generate static bindings from `CoolPropLib.h`");

        static_bindings
            .write_to_file(out_dir.join("bindings_static.rs"))
            .expect("static bindings should be written to `OUT_DIR`");

        // Only generate dynamic library dynamic pointers for non-WASM targets
        let dynamic_bindings = base_builder()
            .dynamic_library_name("CoolProp")
            .dynamic_link_require_all(true)
            .generate()
            .expect("bindgen should generate dynamic bindings from `CoolPropLib.h`");

        dynamic_bindings
            .write_to_file(out_dir.join("bindings_dynamic.rs"))
            .expect("dynamic bindings should be written to `OUT_DIR`");
    }
}
