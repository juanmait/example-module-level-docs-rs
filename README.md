# Rust docs

```sh
rustdoc src/lib.rs --crate-name RustDocs && open doc/RustDocs/index.html && cargo watch -s 'rustdoc src/lib.rs --crate-name RustDocs && rustdoc --test src/lib.rs'
# Or just
./start
```

Check https://doc.rust-lang.org/cargo/commands/cargo-doc.html
