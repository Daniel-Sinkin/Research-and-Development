use std::fs::File;
use std::io::{self, Read};

use std::collections::HashMap;

fn main() -> io::Result<()> {
    if let Ok(mut file) = File::open("day3") {
        let mut body = String::new();

        let mut hm: HashMap<char, (bool, bool)> = HashMap::new();

        file.read_to_string(&mut body)?;
        let mut iter = body.split("\n");

        let mut i = 0;
        let mut score = 0;
        while let Some(str) = iter.next() {
            if str == "" {
                break;
            }
            let str1 = iter.next().unwrap();
            let str2 = iter.next().unwrap();

            let mut badge: Option<char> = None;

            hm = HashMap::new();
            for c in str.chars() {
                // Here we just want to add all keys that appear in str
                if !hm.contains_key(&c) {
                    hm.insert(c, (true, false));
                }
            }

            for c in str1.chars() {
                // Here we want to modify all keys that appear in str
                if hm.contains_key(&c) {
                    hm.insert(c, (true, true));
                }
            }

            for c in str2.chars() {
                if hm.contains_key(&c) {
                    if let (true, true) = hm[&c] {
                        badge = Some(c);
                        break;
                    }
                }
            }

            let cc = match badge {
                None => panic!(),
                Some(b) => b,
            };

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
        dbg!(score);
    } else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to open the File!",
        ));
    }

    Ok(())
}
