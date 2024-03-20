macro_rules! print_bounds {
    ($t:ty) => {
        println!("{} : [{}, {}]", stringify!($t), <$t>::MIN, <$t>::MAX);
    };
}

macro_rules! print_bounds_all {
    ($($t:ty),*) => {
        $(
            print_bounds!($t);
        )*
    };
}

use std::collections::HashMap;

struct User {
    id: i32
}

fn main() {
    //
    // Basic Types & Variables
    //

    // The following integer types are supported
    print_bounds_all!(u8, u16, u32, u64, u128);
    print_bounds_all!(i8, i16, i32, i64, i128);

    // Floats are f32, f64
    println!("On this system usize is {} bits", usize::BITS);
    println!("On this system isize is {} bits", isize::BITS);

    // char, &str, String
    // Strings in Rust are quite complicated and go
    // beyond the scope of this, just recall that
    // &str is a reference pointer and String is variable
    // length, also unicode letters are betwee 1 and 4 bytes
    // so the size of a String is not uniquely defined
    // by its length
    let my_string = String::from("From");
    let my_slice = &my_string;

    println!("{}", my_string);
    println!("{}", my_slice);

    // Tuple

    // Array & slice
    let my_arr = [1..5];
    let my_arr2 = [0; 3];
    dbg!(my_arr);
    dbg!(my_arr2);

    // Hash map
    // use std::collections::HashMap;
    let mut my_hash_map = HashMap::new();
    my_hash_map.insert(String::from("fifty"), 50);
    my_hash_map.insert(String::from("thirty"), 30);
    my_hash_map.insert(String::from("Seventeen"), 17);

    for k in vec!["fifty", "thirty", "Seventeen"]  {
        println!("{}", my_hash_map[k]);
    }

    // Struct
    // Enum

    // Constants are defined in upper case
    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);

    // Static variables
    static MAJOR_VERSION: u32 = 1;
    println!("{}", MAJOR_VERSION);

    static mut COUNTER: u32 = 0;
    unsafe {
        COUNTER = 5;
        println!("{}", COUNTER);
    }

    // Mutability
    // Shadowing

    // Type alias
    type NanoSecond = u64;
    let my_timer: NanoSecond = 25446341;
    println!("{} ns have passed.", my_timer); 

    //
    // Control Flow
    //

    // if and if let
    // loop
    // Nester loops & labels
    // Returning from loops

    // The following assigns the value of counter to
    // result
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };

    println!("{}", result);

    // while and while let
    // for loop
    // match

    //
    // References, Ownership, and Borrowing
    //

    //
    // Pattern Matching
    //
    let x = 5;
    match x {
        // matching literals
        1 => println!("one"),
        // matching multiple patterns
        2 | 3 => println!("two or three"),
        // matching ranges
        4..=9 => println!("within range"),
        // matching named variables
        x => println!("{}", x),
    }

    // Destructing
    //match shape {
    //    Shape::Rectangle { x, y } => //...
    //    Shape::Circle(radius) => //...
    //}

    // Ignoring values

    // Match guards
    // @ bindings


    let num = Some(4);
        match num {
        Some(x) if x < 5 => println!("less than
        five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let user = User { id: 5 };

    match user {
        User { id: id_variable @ 3..=7 } => {
            println!("id: {}", id_variable);
        },
        User { id: 10..=12 } => {
            println!("within range");
        },
        User { id } => {
            println!("id: {}", id);
        },
    }
}