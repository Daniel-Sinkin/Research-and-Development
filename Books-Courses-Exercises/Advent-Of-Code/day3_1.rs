use std::fs::File;
use std::io::{self, Read};

use std::collections::HashMap;

fn main() -> io::Result<()> {
    if let Ok(mut file) = File::open("day3") {
        let mut body = String::new();
        file.read_to_string(&mut body)?;

        let mut score = 0;
        for line in body.split("\n").into_iter() {
            if line.len() == 0 {
                break;
            }
            let n = line.len() / 2;
            let first_half = &line[..n];
            let second_half = &line[n..];

            let mut hm: HashMap<char, (bool, bool)> = HashMap::new();

            let mut common_char: Option<char> = None;

            for i in 0..n {
                // There is no reason to go through both at the same time
                // should just Hashmap through the first half and then
                // check the second half, this has two large advantages
                // to the ergonomics, I can use a list iterator
                // and I can use HashMap<char, bool> which feels better
                // Although now that I read through the second part the
                // Current approach might be the right one for that
                let lc = first_half.chars().nth(i).unwrap();
                if hm.contains_key(&lc) {
                    if let (_, true) = hm[&lc] {
                        common_char = Some(lc);
                        break;
                    }
                } else {
                    hm.insert(lc, (true, false));
                }
                let lc = second_half.chars().nth(i).unwrap();
                if hm.contains_key(&lc) {
                    if let (true, _) = hm[&lc] {
                        common_char = Some(lc);
                        break;
                    }
                } else {
                    hm.insert(lc, (false, true));
                }
            }

            if let None = common_char {
                panic!();
            } else {
                let cc = common_char.unwrap();
                if cc.is_ascii_lowercase() {
                    let val = (cc as u8) - 97 + 1;
                    println!(
                        "Adding {} ({}) to the score of {} to get {}",
                        cc,
                        val,
                        score,
                        score + val as u32
                    );
                    score += val as u32;
                } else if cc.is_ascii_uppercase() {
                    let val = (cc as u8) - 65 + 27;
                    println!(
                        "Adding {} ({}) to the score of {} to get {}",
                        cc,
                        val,
                        score,
                        score + val as u32
                    );
                    score += val as u32;
                } else {
                    panic!();
                }
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
