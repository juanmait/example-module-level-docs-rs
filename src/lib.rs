/*!
# Documentation and Guides about Rust

For more info see:

- [The rustdoc documentation](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html)
- [_How to write documentation_](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html)
in the [rustdoc book](https://doc.rust-lang.org/stable/rustdoc/).

## How to create a new document

> Documents are just rust modules. 

Create a new snake_case file in `src/`. Add `pub mod my_module;` in `lib.rs`

And that is it.
*/

pub mod const_vs_static;
pub mod dynamically_sized_types;
pub mod static_dispatch;
pub mod trait_as_ref;
pub mod trait_bounds;
pub mod trait_objects;
pub mod trait_partial_ord;
pub mod trait_ord;
