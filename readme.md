# rsm

**rsm** stands for "ressource manager" and is used to read and manipulate data in certain types of files.

## Testing

| Type              | Command                                                         |
| ----------------- | --------------------------------------------------------------- |
| Coverage          | `cargo +nightly llvm-cov --html --branch --show-instantiations` |
| Fuzzing           | `cargo +nightly fuzz run <fuzz_target>`                         |
| Unit, Integration | `cargo test`                                                    |
| Mutation          | `cargo mutants`                                                 |
