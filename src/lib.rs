//! # rsm
//! 
//! ![Custom Badge](https://img.shields.io/github/actions/workflow/status/cm-verbose/rsm/rust.yml?branch=main&style=for-the-badge)
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
  missing_docs,
  unknown_lints,
  unused,
  unused_attributes,
  unused_crate_dependencies,
  unused_doc_comments,
  unused_extern_crates,
  unused_lifetimes,
  clippy::all,
  rustdoc::all,
)]

/// Library
pub mod lib {}
