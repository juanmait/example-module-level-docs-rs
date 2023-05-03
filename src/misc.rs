/*!
# Closures

## Writing the type of a closure

```
fn make_it() -> impl Fn(u8) -> u8 {
   |a| a + 1
}

fn call_it(f: impl Fn(u8) -> u8) {
    println!("The result is {}", f(42))
}

fn main() {
    call_it(make_it());
}
```
*/

#[cfg(test)]
mod tests {

    #[test]
    fn works() {}
}
