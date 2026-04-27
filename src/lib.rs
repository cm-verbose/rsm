//! # rsm
//!
//! ![Status](https://img.shields.io/github/actions/workflow/status/cm-verbose/rsm/rust.yml?branch=main&style=for-the-badge)
//!
//! **rsm** (resource manager) is a crate dedicated to handling data in
//! multiple file formats. This library allows you to work with these files
//! with simple functions.
//!
//! ## Supported formats
//!
//! Format                              | Details
//! :-----------------------------------|:------------------------------------------
//! [PNG](https://w3c.github.io/png)    | Supports parts of the fourth edition draft
//!
#![forbid(
  ambiguous_glob_imports,
  inline_always_mismatching_target_features,
  meta_variable_misuse,
  missing_docs,
  unknown_lints,
  unreachable_pub,
  unsafe_attr_outside_unsafe,
  unused,
  unused_attributes,
  unused_crate_dependencies,
  unused_doc_comments,
  unused_extern_crates,
  unused_imports,
  unused_import_braces,
  unused_lifetimes,
  clippy::correctness,
  clippy::perf,
  clippy::suspicious,
  rustdoc::all
)]

/// Library
pub mod lib {
  /// Image formats
  pub mod img;

  /// Utility modules
  pub mod util;
}
