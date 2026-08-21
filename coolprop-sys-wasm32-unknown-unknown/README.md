# coolprop-sys-wasm32-unknown-unknown

[<img alt="GitHub" src="https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="22">](https://github.com/portyanikhin/rfluids)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="22">](https://docs.rs/coolprop-sys)
[<img alt="crates.io" src="https://img.shields.io/crates/v/coolprop-sys?style=for-the-badge&logo=rust&labelColor=555555&color=fc8d62" height="22">](https://crates.io/crates/coolprop-sys)
[<img alt="CI" src="https://img.shields.io/github/actions/workflow/status/portyanikhin/rfluids/ci.yml?style=for-the-badge&logo=githubactions&logoColor=ffffff&label=ci&labelColor=555555" height="22">](https://github.com/portyanikhin/rfluids/actions/workflows/ci.yml)

[`CoolProp`](https://coolprop.org) native binary for bare `wasm32-unknown-unknown`
(browser `WebAssembly`, no Emscripten runtime - e.g. built via `wasm-pack`).

Unlike `coolprop-sys-wasm32-unknown-emscripten`, this crate has no from-source
build path: it always links the prebuilt `libCoolProp.a` checked in at `lib/`.
`coolprop-sys`'s own `static-link` feature must be enabled by anything
depending on it for this target, since this crate ships only a static
archive - there is no dynamic library for `libloading` to `dlopen` at
runtime, so the non-`static-link` bindings path does not build here.

#### License

<sup>
This project is licensed under
<a href="https://github.com/portyanikhin/rfluids/blob/main/LICENSE">MIT License</a>
</sup>
