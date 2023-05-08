A big difference between general variables and `const` variables is that you can't make the latter mutable. Furthermore the type of the data must be declared, we can't use type inference for `const` variables.

"Rust’s naming convention for constants is to use all uppercase with underscores between words." Note that the compile pre-compiles the numbers on the right so the following is equivalent to typing `= 10800`
```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

`const` variables are valid (within their scope) for as long as the program runs. If we declare them in the global scope then all parts of the program can acces it.

> Shadowing is different from marking a variable as `mut` because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword. By using `let`, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

This is okay:
```rust
let spaces = "   ";
let spaces = spaces.len();
```

This gives a compile time error, because we are not allowed to change the data types after assigning them once.
```rust
let mut spaces = "   ";
spaces = spaces.len();
```