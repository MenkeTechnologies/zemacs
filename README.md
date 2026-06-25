# zemacs

A modal text editor in Rust, forked from [Helix](https://github.com/helix-editor/helix).

zemacs starts from the Helix/vim-style modal core — selection-first editing,
tree-sitter syntax, LSP, multiple selections — and is being built out toward
full Spacemacs-style functionality (layered keymaps, an extension layer, and
editor-as-environment workflows) on top of that base.

## Status

Early. This is the vendored Helix base (v25.7.1) with the binary renamed to
`zemacs`. Build-out toward the Spacemacs feature set is in progress.

## Build

```sh
cargo build --bin zemacs
./target/debug/zemacs
```

The toolchain floats to `stable` (see `rust-toolchain.toml`).

## License

Helix-derived source is licensed under the Mozilla Public License 2.0; see
`LICENSE`. Provenance and licensing details are in `ATTRIBUTION.md`.
