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

## modules2.rs
By default everything in rust is going to be private, to make it public use the `pub` keyword.
```rust
pub use self::fruits::PEAR as fruit;
pub use self::veggies::CUCUMBER as veggie;
```

## modules3.rs
If you want to `use` multiple things from a package you can do the following:
```rust
use std::time::{SystemTime, UNIX_EPOCH};
```

## hashmaps3.rs
The following 
```rust
scores
	.entry(team_1_name.clone())
	.and_modify(|team| {
		team.goals_scored += team_1_score;
		team.goals_conceded += team_2_score;
	})
	.or_insert(Team {
		name: team_1_name,
		goals_scored: team_1_score,
		goals_conceded: team_2_score,
	});
```
is a more rustified version of
```rust
if let Some(team) = scores.get_mut(&team_1_name) {
    team.goals_scored += team_1_score;
    team.goals_conceded += team_2_score;
} else {
    scores.insert(team_1_name, Team { name: team_1_name.clone(), goals_scored: team_1_score, goals_conceded: team_2_score });
}
```

## options1.rs
The code
```rust
match time_of_day {
	0..=21 => Some(5),
	22..=23 => Some(0),
	_ => None,
};
```
and
```rust
if time_of_day < 22 {
	Some(5)
} else if time_of_day < 24 {
	Some(0)
} else {
	None
}
```
is equivalent for `time_of_day: u16`.

## Options3.rs
https://doc.rust-lang.org/std/keyword.ref.html