/*!
# Static Dispatch (monomorphization)

The _static dispatch_ is part of the rust
[_monomorphization_](https://rustc-dev-guide.rust-lang.org/backend/monomorph.html) process.
in which the rust complier generates code.

Is the process of _statically dispatch_ the necessary variations of code to represent for example
different method signatures for different types that implement the same trait.

The difference with _dynamic dispatch_ is that _static dispatch_ happens when the compiler
knows all the possible variations of code that has to generate at compile time.

## Example
Let's say that we have a trait that can say _Hi_ to any type that implements it.

```
pub trait Hi {
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

But we could also have a generic function that accepts anything that implements the trait `Hi` and call the `hi` method:

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

In this situation, when the compiler needs to generate the machine code for this it needs to
some how call the `hi` method but it doesn't actually know the type of the param `name`.
So in reality what will be end up generating is that the complier generates two versions
of the function `say_hi` like this:

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
fn say_hi_str_ref(name: &str) {
    name.hi();
}
fn say_hi_string(name: String) {
    name.hi();
}
```
So now the compiler knows what are the concrete types on which have to call the `hi` method.
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
