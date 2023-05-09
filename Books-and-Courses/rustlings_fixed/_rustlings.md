## variables5.rs
```rust
fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", &number);
    let number = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);
}
```
This is an example of shadowing, while number is not mutable we can simply give the label `name` to something else and then drop the string before. This is fundamentally different then overwriting the value of number with something of a different datatype (which causes rust to panic).

## vecs2.rs
The following takes the iterator of the vector v, which is basically just a list of its elements and maps the lambda operator `\x.x->x*2` on that list. `.collect()` then creates a new vector out of the result of the `map` command.
```rust
v.iter().map(|num| { num * 2 }).collect()
```

## structs2.rs
The following is an example of the update syntax, a great way to speed up making slight struct variations off a base template.
```rust
let your_order = Order { name: String::from("Hacker in Rust"), count: 1, ..order_template };
```

## enums2.rs
```rust
enum Message {
    // TODO: define the different variants used below
    Move{x: i32, y: i32},
    Echo(String),
    ChangeColor(i32, i32, i32),
    Quit,
}
```

