/*!
# Documentation and Guides about Rust

For more info see:

- [The rustdoc documentation](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html)
- [_How to write documentation_](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html)
in the [rustdoc book](https://doc.rust-lang.org/stable/rustdoc/).

## How to create a new document

Documents are rust modules. Create a file with the preferred name and make it public in the `lib.rs` file
by introducing this line:

```ignore
pub mod my_module;
```

And that is it.
*/

pub mod const_vs_static;
pub mod dynamically_sized_types;
pub mod closures;
pub mod static_dispatch;
pub mod trait_as_ref;
pub mod trait_objects;
pub mod trait_bounds;
