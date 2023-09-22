# Example of a Module Level Rust Documentation

This is an example that showcase the generation of module level documentation. It can be useful to
write (and generate) general rust documentation like guides, tutorials, notes, etc in which the code
snippets and examples used throughout the documentation can be tested by the rust compiler.

Other things in this repo:

-   It shows how it's possible to test the rust code embedded in documentation blocks.
-   How to properly embed rust code that can fail and show that in the documentation.
-   How to hide part of the embedded code for simplicity purposes.
-   It shows examples of the linking to the documentation of other types or types in the rust's
    standard library.

## Generate and open the documentation

```bash
# Generate the documentation and open it in a browser
cargo doc --lib --open
```

The resulting documentation will be located at `./target/doc`. Check
<https://doc.rust-lang.org/cargo/commands/cargo-doc.html>

## Test it all

Is possible to test even the correctness of the rust code that is embedded in the documentation
blocks.

```bash
# Test all (including library tests and the rust
# code embedded in documentation blocks).
cargo test

# Test ONLY rust code embedded in documentation blocks.
cargo test --doc

# Same as above but do not hide the error messages that are "expected" to happen
# in documentation code blocks that are marked with the `compile_fail` attribute.
cargo test --doc -- --color always --nocapture

# Test ONLY the library tests (tests bellow #[test] markers).
cargo test --lib
```
