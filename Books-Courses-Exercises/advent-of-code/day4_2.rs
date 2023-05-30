use std::fs::File;
use std::io::{self, Read};

fn contains((x1, x2): (u32, u32), (y1, y2): (u32, u32)) -> bool {
    (x1 <= y1) && (y2 <= x2)
}

const FILENAME: &str = "day4";
fn main() -> io::Result<()> {
    let file = File::open(FILENAME);

    if let Ok(mut file) = File::open(FILENAME) {
        let mut body = String::new();
        file.read_to_string(&mut body)?;
        let mut counter = 0;
        for line in body.split("\n") {
            if line == "" {
                break;
            }
            let mut split = line.split(",");
            let left = split.next().unwrap();
            let right = split.next().unwrap();

            let mut left_split = left.split("-");
            let left1 = left_split.next().unwrap().parse::<u32>().unwrap();
            let left2 = left_split.next().unwrap().parse::<u32>().unwrap();
            let mut right_split = right.split("-");
            let right1 = right_split.next().unwrap().parse::<u32>().unwrap();
            let right2 = right_split.next().unwrap().parse::<u32>().unwrap();

            if !((right1 > left2) || (right2 < left1)) {
                counter += 1;
            }
        }
        dbg!(counter);
    } else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to open the file!",
        ));
    }

    Ok(())
}
