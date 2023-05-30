# 3. Variables
Done: {1, 2, 3, **4**, 5, 6, 7, 8}

```rust
// 3_4
// Fix the error with the use of define_x
fn main() {
    let x = define_x();
    println!("{}, world", x); 
}

fn define_x() -> String {
    let x = String::from("hello");
    x
}
```

```rust
// 3_7
#[allow(unused_variables)]
fn main() {
    let x = 1;
    //println!("{}", x);
}

// Warning: unused variable: `x`
```

```rust
// 3_9
fn main() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [3, 2]);

    println!("Success!");
}
```

# 4. Basic Types
## 4.1. Numbers
Done: {1, 2, 3, 4, 5, 6, 7, 8, 9, 10}
Missing: {}
```rust
// Modify `assert_eq!` to make it work
// Here we first cas x to u32 and then get the type
fn main() {
    let x = 5;
    assert_eq!("u32".to_string(), type_of(&(x as u32)));

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
```

```rust
// 4_1_8
fn main() {
    // assert!(0.1+0.2==0.3);
    assert!((((0.1 + 0.2) - 0.3) as f64).abs() < std::f64::EPSILON);
    assert!(10.0 * 0.1 + 10.0 * 0.2 == 10.0 * 0.3);

    println!("Success!");
}
```

```rust
// 4_1_11
// Fill the blanks and fix the errors
fn main() {
    // Integer addition
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1); 
    
    assert!(3 * 50 == 150);

    assert!(((9.6 / 3.2 - 3.0) as f64).abs() < 3.0 * std::f64::EPSILON); // error ! make it work

    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
```

## 4.2. Char, Bool and Unit
Done: {1, 2, 3, 4, 5, 6}
```rust
fn main() {
    let c1 = "中";
    print_char(c1.chars().nth(0).unwrap());
}

fn print_char(c: char) {
    println!("{}", c);
}
```

## 4.3. Statements and Expressions
Done = {1}