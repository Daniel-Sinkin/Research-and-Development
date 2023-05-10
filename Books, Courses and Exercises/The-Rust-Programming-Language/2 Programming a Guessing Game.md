Input output of std library, recall `#include <iostream>` from C++.
```rust
use std::io;
```

The `!` indicated that those are macros not functions.
```rust
println!("Guess the number!");

println!("Please input your guess.");
```

Creates a mutable (value can change later) variable `guess`
```rust
let mut guess = String::new();
```

Creates an immutable (value can't change later) variable `apples`
```rust
let apples = 5;
```

Note that we write `new` not `new!` so it is indeed a function, not a macro.

Calls the `stdin` function from the io library which we imported before.
```rust
io::stdin()
    .read_line(&mut guess)
```

This prints a formated string, recall `print(f"You guess: {guess}")` from Python.
```rust
println!("You guessed: {guess}");
```

`Crates` are a collection of Rust Source Code.

Note that `secret_number` is, because he didn't say otherwise, an immutable variable, as such can't change its value later on.
```rust
let secret_number = rand::thread_rng().gen_range(1..=100);
```


By Shadowing we can write the following to convert our guess to a (`u32`) int. `trim()` removes leading and trailing whitespaces and things like `\n` (note that `read_line` always adds a `\n` at the end) and `r` (called a carriage return).

Note that `parse()` returns a `Result` enum which can be used to handle non-convertable strings.
```rust
let guess: u32 = guess.trim().parse().expect("Input:");
```

[[Question_Solved]]: Does `Shadowing` mean we have the same value stored twice in two versions or does it just overwrite the old variable. For example if I have `let mut x = String::new()` and then write `let x: u32 = x.trim().parse()`, do we now have two versions of `x` in memory?
> Answer: "Rustaceans say that the first variable is _shadowed_ by the second, which means that the second variable is what the compiler will see when you use the name of the variable. In effect, the second variable overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends". The new variable simply takes the place of the old variable in the current scope.

Note that we have never told Rust what data type `secret_number` is, so here it uses type inference to determine that the type is `u32`.
```rust
match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
```

Error Handling: Recall that `Result` is an enum, we can match it to handle the error case. 

Note that `continue` tells the programm to skip the current loop iteration, not to continue the progam as if nothing happened.
```rust
let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
```

If statements work exactly how you'd expect.
```rust
if guess.trim() == "quit" {
	println!("Quitting the game, thank you for playing\nYou played {counter} round(s)");
	break;
}
```