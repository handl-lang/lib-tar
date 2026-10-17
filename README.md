# lib-tar

`tar` — Handl stdlib port.

Part of the stdlib → `lib-*` split (ADR-0128). History is a `git subtree split`
of `stdlib/handl-packages/archive_tar/`.

## Layout

```text
src/index.hnl   the interface (op declarations + handle bindings)
src/ops.rs      the Rust atoms behind them
pkg.hnl         { modules: ["tar"] }
ci.sh           bupha entry: hnl gate + cargo test
```

**Transitional** — plan `stdlib-lib-migration` Phase 7 rewrites the Rust ops as
`ffi.call` and deletes the crate.
