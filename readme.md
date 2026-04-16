# rsm

**rsm** (resource manager) is a crate dedicated to handling data in multiple file formats.

## Testing

| Type              | Command                                                         |
| ----------------- | --------------------------------------------------------------- |
| Coverage          | `cargo +nightly llvm-cov --html --branch --show-instantiations` |
| Fuzzing           | `cargo +nightly fuzz run <fuzz_target>`                         |
| Unit, Integration | `cargo test`                                                    |
| Mutation          | `cargo mutants`                                                 |
