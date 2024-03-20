use core::panic;
use std::fs::File;
use std::io::{self, Read};

use std::collections::HashSet;

const FILENAME: &str = "day6";
const _ENABLE_OUTPUT: bool = false;
const k: usize = 14;

fn shift_push(frame: &mut [char; k], c: char) {
    for i in 0..=(k - 2) {
        frame[i] = frame[i + 1];
    }

    frame[k - 1] = c;
}

fn check_for_duplicates(frame: &[char; k]) -> bool {
    let mut set = HashSet::new();

    for &c in frame {
        if !set.insert(c) {
            return true;
        }
    }
    false
}

fn main() -> io::Result<()> {
    // Reading in the file, it might be faster to read the file in as we go through
    // the problem instead of all at once
    let mut file = File::open(FILENAME).expect("Couldn't open file!");
    let mut body = String::new();
    file.read_to_string(&mut body)?;
    body = body.lines().next().unwrap().to_string();

    let mut frame: [char; k] = body.chars()
                                .take(k)
                                .collect::<Vec<char>>()
                                .try_into()
                                .unwrap();

    frame = Default::default();

    dbg!(frame);
    
    for (i, c) in body.chars().enumerate() {

        println!("Checking the {}th character", i + 1);
        shift_push(&mut frame, c);
        for i in 0..k {
            print!("{}", frame[i]);
        }
        println!();
        if !check_for_duplicates(&frame) {
            print!("{} - ", i + 1);
            for i in 0..k {
                print!("{}", frame[i]);
            }
            println!();

            println!("The last char is = {}", body.chars().nth(i + 1).unwrap());
            panic!();
        }
    }

    Ok(())
}
