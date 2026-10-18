//! `tar` — Handl stdlib port: pure `.hnl` POSIX ustar pack/unpack over
//! `bytes`/`str`/`math`/`char`. This crate is a data carrier — the impls
//! live in `src/*.hnl`.

/// The handl source declaring the `tar` interface.
pub const TAR_HNL: &str = include_str!("index.hnl");

/// Exported `.hnl` arrows.
pub const TAR_FUNCS: &[&str] = &["pack", "unpack"];

/// `.hnl` package source set — index + sibling files (wired into kits via
/// `handl_module_from_sources`).
pub const TAR_SOURCES: &[(&str, &str, &[&str])] = &[
    ("oct", include_str!("internal/oct.hnl"), &["oct_int", "int_oct", "octdx"]),
    ("b64b", include_str!("internal/b64b.hnl"), &["b64_enc_bytes", "b64_dec_bytes"]),
    ("headerb", include_str!("internal/headerb.hnl"), &["header", "block"]),
    ("pack", include_str!("pack.hnl"), &["pack"]),
    ("unpack", include_str!("unpack.hnl"), &["unpack"]),
];

/// Flattened single-source `.hnl` (for `stdlib::MODULES` table rows).
pub const TAR_FLAT_HNL: &str = include_str!("index_flat.hnl");
