use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    if let Ok(mut file) = File::open("day2") {
        let mut body = String::new();
        file.read_to_string(&mut body)?;

        let mut score: u32 = 0;
        for line in body.split("\n").into_iter() {
            if let Some(them) = line.chars().nth(0) {
                let you = line.chars().nth(2).unwrap();

                let you_n = (you as u8) - ('X' as u8);
                let them_n = (them as u8) - ('A' as u8);

                score += match you_n {
                    0 => 1,
                    1 => 2,
                    2 => 3,
                    _ => panic!(),
                };

                score += match (you_n + 2) - them_n {
                    0 | 3 => 6,
                    1 | 4 => 0,
                    2 => 3,
                    _ => panic!(),
                };
            } else {
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
