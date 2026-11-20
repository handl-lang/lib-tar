# lib-tar

`tar` — Handl stdlib port.

## Layout

```text
src/index.hnl   the interface (op declarations + handle bindings)
src/ops.rs      the Rust atoms behind them
pkg.hnl         package manifest — modules, deps, description, license
Cargo.toml      the crate manifest
xtask/          cargo xtask gate (fmt → clippy → test → doc)
ci.sh           bupha entry: hnl gate + cargo test
```

**Transitional** — plan `stdlib-lib-migration` Phase 7 rewrites the Rust ops as
`ffi.call` and deletes the crate.

## License

MIT.
