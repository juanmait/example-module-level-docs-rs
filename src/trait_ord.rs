/*!
# The [Ord] trait.

Trait for types that implements [total order]. Total order refers to a rule that estipulate that
every such pairs in a set of elements must be comparable.


### How can I implement Ord?

<code>[Ord]</code> requires that the type also implements <code>[PartialOrd]</code> and
<code>[Eq]</code> (which requires <code>[PartialEq]</code>).

Then you have to implement <code>fn [cmp]</code>().

-   See: [how-can-i-implement-ord](https://doc.rust-lang.org/std/cmp/trait.Ord.html#how-can-i-implement-ord)

---

[total order]: https://en.wikipedia.org/wiki/Total_order
[Ord]: https://doc.rust-lang.org/std/cmp/trait.Ord.html
[PartialOrd]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
[Eq]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
[PartialEq]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
[cmp]: https://doc.rust-lang.org/std/cmp/trait.Ord.html#tymethod.cmp

*/