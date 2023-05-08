The three rules of ownership are:
-   Each value in Rust has an _owner_.
-   There can only be one owner at a time.
-   When the owner goes out of scope, the value will be dropped.

Because `String` does not have the `Clone` trait the following voids `s1` after giving `s2` the reference of `s1`. Note that we only push the reference into `s2` instead of making a copy of the underlying values (which is slow)
```rust
let s1 = String::from("hello")
let s2 = s1
```

If we actually do want a copy of ther underlying values we use the `clone()` method.
```rust
let s1 = String::from("hello");
let s2 = s1.clone();
```

Note that if a DataType has the `Clone` trait then `clone()` is the standard behaviour and adding it doesn't do anything. The assignments for `y` and`z` are identical.
```rust
let x = 5
let y = x
let z = x.clone()
```

List of some types which have the `Clone` trait:
-   All the integer types, such as `u32`.
-   The Boolean type, `bool`, with values `true` and `false`.
-   All the floating-point types, such as `f64`.
-   The character type, `char`.
-   Tuples, if they only contain types that also implement `Copy`. For example, `(i32, i32)` implements `Copy`, but `(i32, String)` does not.
