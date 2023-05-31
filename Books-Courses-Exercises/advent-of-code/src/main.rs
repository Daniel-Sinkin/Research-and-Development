use std::fs::File;
use std::io::{self, Read};
use std::vec;

const FILENAME: &str = "day8";
#[allow(unused_mut)]
#[allow(unused_variables)]
fn main() -> io::Result<()> {
    let file = File::open(FILENAME);

    if let Ok(mut file) = File::open(FILENAME) {
        let mut body = String::new();
        file.read_to_string(&mut body)?;

        let mut grid: Vec<Vec<u8>> = Default::default();
        let mut grid: Vec<Vec<bool> = Default::default();

        for line in body.lines() {
            let mut row: Vec<u8> = Default::default();

            for c in line.chars() {
                row.push(c.to_digit(10).unwrap() as u8);
            }
            grid.push(row);
        }

        for y in 1..=98 {
            for x in 1..=98 {
                for 
            }
            println!("{:?}", &grid[y]);
        }
    } else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to open the file!",
        ));
    }

    Ok(())
}
