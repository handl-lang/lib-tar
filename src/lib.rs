//! `tar` — Handl stdlib port.
//!
//! `src/index.hnl` is the interface; `ops.rs` holds the Rust atoms behind it.
//! Transitional: plan `stdlib-lib-migration` Phase 7 rewrites them as
//! `handle … with ffi.call(…)` and deletes this crate.

/// The handl source declaring the `tar` interface.
pub const TAR_HNL: &str = include_str!("index.hnl");

pub mod ops;

pub use ops::{TAR_OPS, tar_op_registry};

/// `.hnl` package source set — index + sibling files (wired into kits via
/// `handl_module_from_sources`).
pub const TAR_FUNCS: &[&str] = &["pack","unpack"];
pub const TAR_SOURCES: &[(&str, &str, &[&str])] = &[
    ("list", include_str!("internal/list.hnl"), &["list_len","ix","take","drop","catx","zeros","pad_to","nz","range"]),
    ("b64", include_str!("internal/b64.hnl"), &["b64_enc","b64_dec_ints"]),
    ("oct", include_str!("internal/oct.hnl"), &["oct_int","int_oct","octdx"]),
    ("header", include_str!("internal/header.hnl"), &["header","segx","name_field","oct_field"]),
    ("pack", include_str!("pack.hnl"), &["pack"]),
    ("unpack", include_str!("unpack.hnl"), &["unpack"]),
];

/// Flattened single-source `.hnl` (for `stdlib::MODULES` table rows).
pub const TAR_FLAT_HNL: &str = include_str!("index_flat.hnl");
