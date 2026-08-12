//use cmake::Config;

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
