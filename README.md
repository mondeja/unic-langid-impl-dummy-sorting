# Dummy sorting implementation for unic-langid-impl's `Variant` type

- Previous results (`.dedup()` + `.sort_unstable()`): *dominators-with-sort.txt* (13057 bytes)
- Results after commenting out `.sort_unstable()`: *dominators-without-sort.txt* (3595 bytes)
- Results after patch: *dominators-after-patch.txt* (3620 bytes)

Search for `unic_langid_impl::LanguageIdentifier::from_bytes` in the files.

## How to reproduce

1. Clone the unic-langid-impl and put in the same directory as this repository.
1. Install [trunk.rs](https://trunkrs.dev/) with `cargo install trunk`.
1. Install [twiggy](https://github.com/rustwasm/twiggy) with `cargo install twiggy`.
1. Run `trunk build --release`.
1. Get the name of the WASM file generated at *dist/*.
1. Run `twiggy dominators dist/<WASM_FILE>`

Note that Trunk is building the WASM file in release mode but without demangling the symbols. See `data-keep-debug data-wasm-opt="0" data-no-demangle` in *index.html*.
