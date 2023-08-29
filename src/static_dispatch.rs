/*!
# Static Dispatch (monomorphization)

__Static dispatch__ is part of the rust
[_monomorphization_](https://rustc-dev-guide.rust-lang.org/backend/monomorph.html) process.
in which the rust complier generates code.

Is the process of statically (at compile time) dispatch (generate) the necessary variations of code to
represent for example the different versions of a method that is shared across types that implement the
same trait.

In that context, one could said that is the conversion of _generic_ code into _concrete_ code.

The more important outcome of that code generation is that later at runtime our binary will know
exactly where those methods live in memory and will have no trouble finding them and calling them
with their expected parameter types.

__Static dispatch__ can only happen when the compiler knows all the possible variations of code that has to
generate at compile time.

## Example
Let's say that we have a trait that can say _Hi_ to any type that implements it.

```
pub trait Hi {
    /// types that implements `Hi` can say "hi" to themselves
    fn hi(&self);
}
```

We implement the trait `Hi` for [`&str`][str] and [`String`][String]:

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

Now we can use the `hi` method in both [`&str`][str] and [`String`][String]:

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

And now we can have a _generic_ function that accepts anything that implements the trait `Hi` and call the `hi` method:

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
fn say_hi(name: impl Hi) {
    name.hi();
}

say_hi("Str");
say_hi("String".to_string());
```

In reality, since Rust needs to know the size of the param `name`, and `name` is a trait object,
and trait objects have no size, Rust will instead find all the types that implements `Hi`
which do have size (in this case `String` and `&str`) and generate one version of the function for
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
// hi for `&str`
fn say_hi_str_ref(name: &str) {
    name.hi();
}

// hi for `String`
fn say_hi_string(name: String) {
    name.hi();
}
```

So now once at runtime the final binary doesn't have to do any guess work to know what to call with which type.
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
