# rsm

[![Status](https://img.shields.io/github/actions/workflow/status/cm-verbose/rsm/rust.yml?branch=main&style=for-the-badge)](https://github.com/cm-verbose/rsm/actions/workflows/rust.yml)

**rsm** (resource manager) is a crate dedicated to handling data in multiple file formats. This library allows you to work with these files with simple functions.

## Testing

| Type              | Command                                                         |
| ----------------- | --------------------------------------------------------------- |
| Coverage          | `cargo +nightly llvm-cov --html --branch --show-instantiations` |
| Fuzzing           | `cargo +nightly fuzz run <fuzz_target>`                         |
| Unit, Integration | `cargo test -- features <features> -- --no-capture`             |
| Mutation          | `cargo mutants`                                                 |
