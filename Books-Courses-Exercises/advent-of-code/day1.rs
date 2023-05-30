use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

fn main() -> io::Result<()> {
    if let Ok(mut file) = File::open("day_1") {
        let mut body = String::new();
        file.read_to_string(&mut body)?;

        let mut max: u32 = 0;
        let mut max2: u32 = 0;
        let mut max3: u32 = 0;

        for batch in body.split("\n\n").into_iter() {
            let mut count: u32 = 0;
            for line in batch.split("\n").into_iter() {
                if let Ok(number) = line.parse::<u32>() {
                    count += number;
                }
            }

            if count > max {
                max3 = max2;
                max2 = max;
                max = count;
            } else if count > max2 {
                max3 = max2;
                max2 = count;
            } else if count > max3 {
                max3 = count;
            }
        }

        println!("{}", max + max2 + max3);
    } else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to open the File!",
        ));
    }

    Ok(())
}
