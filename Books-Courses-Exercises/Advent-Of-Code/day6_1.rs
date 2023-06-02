use std::fs::File;
use std::io::{self, Read};

const FILENAME: &str = "day6";
#[allow(unused_mut)]
#[allow(unused_variables)]
fn main() -> io::Result<()> {
    let file = File::open(FILENAME);

    if let Ok(mut file) = File::open(FILENAME) {
        let mut body = String::new();
        file.read_to_string(&mut body)?;

        let n = body.len();
        body.truncate(n - 1);
        let n = n - 1;

        let char_array = body.chars().collect::<Vec<char>>();

        let mut curr = [
            char_array[0],
            char_array[0 + 1],
            char_array[0 + 2],
            char_array[0 + 3]
        ];

        for i in 4..char_array.len() {
            // Adds the current character into the array, shifting everything over, maybe using a vec
            // to do this autommatically would be better
            curr[0] = curr[1];
            curr[1] = curr[2];
            curr[2] = curr[3];
            curr[3] = char_array[i];

            if curr[0] != curr[1]   && curr[0] != curr[2]
                                    && curr[0] != curr[3]
                                    && curr[1] != curr[2]
                                    && curr[1] != curr[3]
                                    && curr[2] != curr[3] {
                println!("The answer is {}.", i + 1);
                panic!();
            }
        }
    } else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to open the file!",
        ));
    }

    Ok(())
}
