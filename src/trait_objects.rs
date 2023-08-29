/*!
# Trait Objects

First read about [Static Dispatch].

There are cases in which [static dispatch][Static Dispatch] is not enough.
Going back to the `Hi` trait the we've implemented in the [static dispatch guide][Static Dispatch]
for [`&str`][str] and [`String`][String]:

[Static Dispatch]: ../static_dispatch/index.html

```
pub trait Hi {
    fn hi(&self);
}

impl Hi for &str {
    fn hi(&self) {
        println!("Hi {}!", self);
    }
}

impl Hi for String {
    fn hi(&self) {
        println!("Hi {}!", self);
    }
}
```
Let's say we have a vector of `&str` and we want to call the `hi` method on each of them:

```
# pub trait Hi {
#     fn hi(&self);
# }
#
# impl Hi for &str {
#     fn hi(&self) {
#         println!("Hi {}!", self);
#     }
# }
#
# impl Hi for String {
#     fn hi(&self) {
#         println!("Hi {}!", self);
#     }
# }
#
pub fn call_hi_on_str_refs() {
    for h in vec!["J", "Juan"] {
        h.hi();
    }
}

call_hi_on_str_refs();
// => "Hi J!"
// => "Hi Juan!"
```
That's cool but what if we want the vector to contain anything that implements the trait `Hi` and not only `&str`?;

```compile_fail
# pub trait Hi {
#     fn hi(&self);
# }
#
# impl Hi for &str {
#     fn hi(&self) {
#         println!("Hi {}!", self);
#     }
# }
#
# impl Hi for String {
#     fn hi(&self) {
#         println!("Hi {}!", self);
#     }
# }
#
pub fn call_hi_on_str_refs() {
    for name in vec!["J", "Juan".to_string()] {
//                        ^^^^^^^^^^^^^^^^^^ expected `&str`, found struct `String`
        name.hi();
    }
}

call_hi_on_str_refs();
```
This is because the things inside the vector has to be of the same type.

Let's explore this problem a little bit more. We can implement a more generic function and
we can see that we can indeed pass in a vec of either `&str` or `String`..

```
# pub trait Hi {
#     fn hi(&self);
# }
#
# impl Hi for &str {
#     fn hi(&self) {
#         println!("Hi {}!", self);
#     }
# }
#
# impl Hi for String {
#     fn hi(&self) {
#         println!("Hi {}!", self);
#     }
# }
#
pub fn say_hi_to_all(names: &[impl Hi]) {
    for name in names {
        name.hi();
    }
}

say_hi_to_all(&["J", "Juan"]);
// => "Hi J!"
// => "Hi Juan!"
say_hi_to_all(&["J".to_string(), "Juan".to_string()]);
// => "Hi J!"
// => "Hi Juan!"
```

But we still have the limitation of that the vector has to contain things of the same type.
Either they're all `&str` or they're all `String`:

```compile_fail
# pub trait Hi {
#     fn hi(&self);
# }
#
# impl Hi for &str {
#     fn hi(&self) {
#         println!("Hi {}!", self);
#     }
# }
#
# impl Hi for String {
#     fn hi(&self) {
#         println!("Hi {}!", self);
#     }
# }
#
# pub fn say_hi_to_all(names: &[impl Hi]) {
#     for name in names {
#         name.hi();
#     }
# }
#
say_hi_to_all(&["J", "Juan".to_string()]);
//                   ^^^^^^^^^^^^^^^^^^ expected `&str`, found struct `String`
```

## Entering Dynamic Dispatch

You can read more about it in the rust book's chapter about
[Using Trait Objects That Allow for Values of Different Types](https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types)

The scenario mentions a GUI library that has a bunch of UI components and all of them implements the `Drawable` trait.

The Library could have a `draw` function that takes an iterator of things that implements the `Drawable` trait like
Buttons, Images, Text Boxes, etc..

In our example we would like to do something like the following but that result
in a compilation error:

```compile_fail
# pub trait Hi {
#     fn hi(&self);
# }
#
# impl Hi for &str {
#     fn hi(&self) {
#         println!("Hi {}!", self);
#     }
# }
#
# impl Hi for String {
#     fn hi(&self) {
#         println!("Hi {}!", self);
#     }
# }
#
pub fn say_hi_to_all(names: &[dyn Hi]) {
// |                         ^^^^^^^^ doesn't have a size known at compile-time
// |
// = help: the trait `Sized` is not implemented for `dyn Hi`
// = note: slice and array elements must have `Sized` type
    for name in names {
        name.hi();
    }
}
```
Check the guide [Dynamically Sized Types][dts].

The error tells us that `dyn Hi` doesn't implement the [Sized] trait, which means that
it's size cannot be known at compile time.

The `dyn Hi` is like saying "anything that implements `Hi`" but the problems is that "anything"
is like say "any size", and that's not cool for rust.

How ca we solve this.

## Transforming "anything" into [Pointer Types][pointers].

All the [Pointer Types][pointers] are [Sized]. References are pointers as well as [Smart Pointers][smart].

[dts]: ../dynamically_sized_types/index.html
[pointers]: https://doc.rust-lang.org/reference/types/pointer.html
[smart]: https://doc.rust-lang.org/book/ch15-00-smart-pointers.html

### Examples

```
# pub trait Hi {
#     fn hi(&self);
# }
#
# impl Hi for &str {
#     fn hi(&self) {
#         println!("Hi {}!", self);
#     }
# }
#
# impl Hi for String {
#     fn hi(&self) {
#         println!("Hi {}!", self);
#     }
# }
#
pub fn say_hi_to_all_sized(names: &[&dyn Hi]) {
    for name in names {
        name.hi();
    }
}

pub fn say_hi_to_all_boxed(names: &[Box<dyn Hi>]) {
    for name in names {
        name.hi();
    }
}

say_hi_to_all_sized(&[&"J", &"Juan".to_string()]);
// => "Hi J!"
// => "Hi Juan!"
say_hi_to_all_boxed(&[Box::new("J"), Box::new("Juan".to_string())]);
// => "Hi J!"
// => "Hi Juan!"
```
Continue with dynamic dispatch or v-tables.. <https://youtu.be/xcygqF5LVmM?t=2906>
*/

#[cfg(test)]
mod tests {

    pub trait Hi {
        fn hi(&self);
    }

    impl Hi for &str {
        fn hi(&self) {
            println!("Hi {}!", self);
        }
    }

    impl Hi for String {
        fn hi(&self) {
            println!("Hi {}!", self);
        }
    }

    pub fn call_hi_on_str_refs() {
        for name in vec!["J", "Juan"] {
            name.hi();
        }
    }

    pub fn say_hi_to_all(names: &[impl Hi]) {
        for name in names {
            name.hi();
        }
    }

    pub fn say_hi_to_all_sized(names: &[&dyn Hi]) {
        for name in names {
            name.hi();
        }
    }

    pub fn say_hi_to_all_boxed(names: &[Box<dyn Hi>]) {
        for name in names {
            name.hi();
        }
    }

    #[test]
    fn test_call_hi_on_str_refs() {
        call_hi_on_str_refs();
    }

    #[test]
    fn test_say_hi_to_all() {
        say_hi_to_all(&["J", "Juan"]);
        say_hi_to_all(&["J".to_string(), "Juan".to_string()]);
    }

    #[test]
    fn test_say_hi_to_all_sized() {
        say_hi_to_all_sized(&[&"J", &"Juan".to_string()]);
    }

    #[test]
    fn test_say_hi_to_all_boxed() {
        say_hi_to_all_boxed(&[Box::new("J"), Box::new("Juan".to_string())]);
    }
}
