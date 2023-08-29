/*!
# The [AsRef] trait

Check the [AsRef] trait in the rust docs.

- Used to do a cheap reference-to-reference conversion.
- By using trait bounds we can accept arguments of different types as long as they can be converted to the specified type `T`.

## Examples

### AsRef with String or str

Here `AsRef<str>` means: **accept any type that can be turn into a reference of a `str`**.

```
fn is_hello<T: AsRef<str>>(s: T) {
   assert_eq!("hello", s.as_ref());
}

let s: &str = "hello";
is_hello(s);

let s: String = "hello".to_string();
is_hello(s);
```

Since both [`String`] and [`&str`][str] implement `AsRef<str>` we can accept both as input argument.

### AsRef with the Path struct

The [std::path::Path] implements [AsRef].

Here `AsRef<Path>` means: **accept any type that can be turn into a reference of a `Path`**.

```
use std::path::Path;

fn create_path_ref<'a, T: AsRef<Path> + 'a>(path: &'a T) -> &'a Path {
    let path: &Path = path.as_ref();
    path
}

let str_ref: &str = "hello/world.rs";
// => "hello/world.rs"

let string_from_str_ref: String = String::from(str_ref);
// => "hello/world.rs"

let path_from_str_ref: &Path = create_path_ref(&str_ref);
// => "hello/world.rs"

let path_from_string: &Path = create_path_ref(&string_from_str_ref);
// => "hello/world.rs"

let path_from_path: &Path = create_path_ref(&path_from_string);
// => "hello/world.rs"
```

### Example usage in a struct

Here the struct Message is generic over anything that implements `AsRef<str>`.

```
struct Message<T: AsRef<str>> {
    id: T,
}

let message_str = Message { id: "some str" };

let message_string = Message {
    id: String::from("Some str"),
};

dbg!(message_str.id);
dbg!(message_string.id);
```
*/

#[cfg(test)]
mod tests {
    use std::path::Path;

    fn create_path_ref<'a, T: AsRef<Path> + 'a>(path: &'a T) -> &'a Path {
        let path: &Path = path.as_ref();
        path
    }

    #[test]
    fn string_to_path_ref() {
        let str_ref: &str = "hello/world.rs";
        dbg!(str_ref);

        let string_from_str_ref: String = String::from(str_ref);
        dbg!(string_from_str_ref.clone());

        let path_from_str_ref: &Path = create_path_ref(&str_ref);
        dbg!(path_from_str_ref);

        let path_from_string: &Path = create_path_ref(&string_from_str_ref);
        dbg!(path_from_string);

        let path_from_path: &Path = create_path_ref(&path_from_string);
        dbg!(path_from_path);
    }

    #[test]
    fn as_ref_in_struct() {
        struct Message<T: AsRef<str>> {
            id: T,
        }

        let message_str = Message { id: "&str" };

        let message_string = Message {
            id: String::from("String"),
        };

        dbg!(message_str.id);
        dbg!(message_string.id);
    }
}
