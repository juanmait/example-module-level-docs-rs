/*!
# Const Vs Static

`const` can be used in `match` statements but `static` cannot.

```
const CONST_1: &str = "This is CONST 1!!!";
const CONST_2: &str = "This is CONST 2!!!";

fn main() {
    let value = "This is CONST 2!!!".to_string();

    match value.as_ref() {
        CONST_1 | CONST_2 => {
            println!("{}", value);
        }
        _ => panic!("no such case"),
    }
}
```
*/

#[cfg(test)]
mod tests {

    const CONST_1: &str = "This is CONST 1!!!";
    const CONST_2: &str = "This is CONST 2!!!";

    #[test]
    fn string_to_path_ref() {
        let value = "This is CONST 2!!!".to_string();

        match value.as_ref() {
            CONST_1 | CONST_2 => {
                println!("{}", value);
            }
            _ => panic!("no such case"),
        }
    }
}
