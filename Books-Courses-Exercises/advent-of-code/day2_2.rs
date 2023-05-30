use std::fs::File;
use std::io::{self, Read};

enum Action {
    Rock,
    Paper,
    Scissors,
}

impl Action {
    fn points(&self) -> u32 {
        match self {
            Action::Rock => 1,
            Action::Paper => 2,
            Action::Scissors => 3,
        }
    }

    fn loses_to(&self) -> Action {
        match self {
            Action::Rock => Action::Paper,
            Action::Paper => Action::Scissors,
            Action::Scissors => Action::Rock,
        }
    }

    fn wins_against(&self) -> Action {
        match self {
            Action::Rock => Action::Scissors,
            Action::Paper => Action::Rock,
            Action::Scissors => Action::Paper,
        }
    }

    fn from_char(c: char) -> Action {
        match c {
            'A' => Action::Rock,
            'B' => Action::Paper,
            'C' => Action::Scissors,
            _ => panic!(),
        }
    }
}

const WINS: u32 = 6;
const DRAW: u32 = 3;
const LOSS: u32 = 0;

fn main() -> io::Result<()> {
    if let Ok(mut file) = File::open("day2") {
        let mut body = String::new();
        file.read_to_string(&mut body)?;

        let mut score: u32 = 0;
        let turns = body.split("\n").into_iter();
        for turn in turns {
            if let Some(them) = turn.chars().nth(0) {
                let you = turn.chars().nth(2).unwrap();
                let them = Action::from_char(them);

                score += match you {
                    'X' => LOSS + them.wins_against().points(),
                    'Y' => DRAW + them.points(),
                    'Z' => WINS + them.loses_to().points(),
                    _ => panic!(),
                }
            } else {
                // The only way this happens if we are at last line
                break;
            }
        }
        dbg!(score);
    } else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to open the File!",
        ));
    }

    Ok(())
}
