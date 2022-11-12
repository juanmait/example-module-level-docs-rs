/*!
# Dynamically Sized Types

- [https://doc.rust-lang.org/reference/dynamically-sized-types.html](https://doc.rust-lang.org/reference/dynamically-sized-types.html)
- [https://doc.rust-lang.org/book/ch19-04-advanced-types.html?highlight=unsized#dynamically-sized-types-and-the-sized-trait](https://doc.rust-lang.org/book/ch19-04-advanced-types.html?highlight=unsized#dynamically-sized-types-and-the-sized-trait)
- [https://poignardazur.github.io/2022/02/23/rust-unsized-vars-analysis/](https://poignardazur.github.io/2022/02/23/rust-unsized-vars-analysis/)

Most types have a fixed size that is known at compile time and implement the trait [Sized].
Types whose size can only be known at runtime are called _dynamically sized type_ (DST).

Examples of _DST_ types are:

- [Slices](https://doc.rust-lang.org/reference/types/slice.html)
(the slice data structure just stores the starting position and the length of the slice)
- [Trait Objects](https://doc.rust-lang.org/reference/types/trait-object.html) (Every trait is a dynamically sized type).

Rust needs to know how much memory to allocate for any value of a particular type,
and all values of a type must use the same amount of memory.

The [str] type is a DST (not `&str` but `str`) cause it's a _string slice_. [str] can't
be used "as is". His size is not known at compile time. That's why we use `&str` instead.
The `&str` has two values: the memory address of the [str] and its length.
As such, we can know the size of a `&str` at compile time (it’s twice the length of a `usize`)
no matter how long the string it refers to is.

In general, this is the way in which dynamically sized types are used in Rust: they have an
extra bit of metadata that stores the size of the dynamic information.

The golden rule of dynamically sized types is that we must always put values of dynamically sized
types behind a pointer of some kind ([Box], [Rc][std::rc::Rc]).
*/
