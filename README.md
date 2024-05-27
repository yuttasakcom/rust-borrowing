# Rust Borrowing

## The Rules of References

```
- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.
```

## Code

```rust
struct Person {
    name: String,
}

fn main() {
    let mut s = String::from("Hello");
    change(&mut s);
    println!("{}", s);

    let mut p = Person {
        name: String::from("Yo"),
    };

    println!("{}", p.name);

    mutate(&mut p);

    println!("{}", p.name);
}

fn change(str: &mut String) {
    str.push_str(", world");
}

fn mutate(p: &mut Person) {
    p.name.push_str(" Yuttasak");
}

```
