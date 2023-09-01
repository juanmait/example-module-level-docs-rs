/*!
# Static Dispatch (monomorphization)

__Static dispatch__ is part of the rust
[_monomorphization_](https://rustc-dev-guide.rust-lang.org/backend/monomorph.html) process.
in which the rust complier generates code.

Is the process of statically (at compile time) dispatch (generate) the necessary
variations of code to represent for example the different versions of a method that
is shared across types that implement the same trait.

In that context, one could said that is the conversion of _generic_ code into
_concrete_ code.

__Static dispatch__ can only happen when the compiler knows all the possible
variations of code that it has to generate at compile time.

## Example
Let's say that we have a trait `Hi`. Any type that implements Hi receives the
ability to speak is name:

```
pub trait Hi {
    /// types that implements `Hi` can say "hi" to themselves
    fn hi(&self);
}
```

If we implement `Hi` for the types [`&str`][str] and [`String`][String]:

```
# pub trait Hi {
#    fn hi(&self);
# }
#
impl Hi for str {
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

then we can use `hi` on both [`&str`][str] & [`String`][String]:

```
# pub trait Hi {
#    fn hi(&self);
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
"Str".hi(); // "Hi Str!"
"String".to_string().hi(); // "Hi String!"
```

We could have some `fn` that accepts `Hi` through `&str` and `String`:

```
# pub trait Hi {
#    fn hi(&self);
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
/// I'm fine with anything that implements `Hi`
fn say_hi(name: impl Hi) {
    name.hi();
}

say_hi("Str");
say_hi("String".to_string());
```

In reality, since Rust needs to know the size of the param `name`, and `name` is a trait object (`name: impl Hi`),
and trait objects have no size, Rust will instead find all the types that implements `Hi`
which do have size (in this case `String` and `&str`) and generate one version of the `say_hi` function for
every one of them like this:
```
# pub trait Hi {
#    fn hi(&self);
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
// say hi to <&str>
fn say_hi_str_ref(name: &str) {
    name.hi();
}

// say hi to <String>
fn say_hi_string(name: String) {
    name.hi();
}
```

So then, once at runtime, the final binary doesn't have to do any guess work to
figure out what do you mean when you pass `Hi` to a function.
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

    fn say_hi(h: impl Hi) {
        h.hi();
    }

    #[test]
    fn using_the_hi_trait() {
        "Str".hi();
        "String".to_string().hi();
    }

    #[test]
    fn test_say_hi() {
        say_hi("Juan Str");
        say_hi("Juan String".to_string());
    }
}
