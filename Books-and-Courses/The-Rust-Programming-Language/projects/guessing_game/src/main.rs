use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    let mut counter = 1;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
    
        io::stdin()Yo
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        if guess.trim() == "quit" {
            println!("Quitting the game. Thank you for playing\nYou played {counter} round(s)");
            if counter == 1 {
                println!("You played 1 round!")
            } else {
                println!("You played {counter} round(s)");
            }
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess was: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                if counter == 1 {
                    println!("You played 1 round!")
                } else {
                    println!("You played {counter} round(s)");
                }
                break;
            }
        }
        counter += 1;
    }
}