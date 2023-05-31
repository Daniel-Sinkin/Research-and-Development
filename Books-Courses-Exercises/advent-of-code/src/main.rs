use std::fs::File;
use std::io::{self, Read};

const FILENAME: &str = "day8";
const ENABLE_OUTPUT: bool = false;
fn main() -> io::Result<()> {
    let _file = File::open(FILENAME);

    if let Ok(mut file) = File::open(FILENAME) {
        let mut body = String::new();
        file.read_to_string(&mut body)?;

        let mut grid: Vec<Vec<u8>> = Default::default();

        for line in body.lines() {
            let mut row: Vec<u8> = Default::default();

            for c in line.chars() {
                row.push(c.to_digit(10).unwrap() as u8);
            }
            grid.push(row);
        }

        let mut count = 0;
        for y in 1..=98 {
            for x in 1..=98 {
                let mut hidden = false;
                let curr = &grid[y][x];
                for h in 0..y {
                    if &grid[h][x] >= curr {
                        hidden = true;
                        break;
                    }
                }

                if !hidden {
                    if ENABLE_OUTPUT {
                        println!("\tCell at <y = {}, x = {}> is not hidden (1)", y, x);
                    }
                } else {
                    for h in (y + 1)..99 {
                        if &grid[h][x] >= curr {
                            hidden = true;
                            break;
                        }
                    }

                    if !hidden {
                        if ENABLE_OUTPUT {
                            println!("\tCell at <y = {}, x = {}> is not hidden (1)", y, x);
                        }
                    } else {
                        for v in 0..x {
                            if &grid[y][v] >= curr {
                                hidden = true;
                                break;
                            }
                        }

                        if !hidden {
                            if ENABLE_OUTPUT {
                                println!("\tCell at <y = {}, x = {}> is not hidden (1)", y, x);
                            }
                        } else {
                            for v in (x + 1)..99 {
                                if &grid[y][v] >= curr {
                                    hidden = true;
                                    break;
                                }
                            }

                            if !hidden {
                                if ENABLE_OUTPUT {
                                    println!("\tCell at <y = {}, x = {}> is not hidden (1)", y, x);
                                }
                            }
                        }
                    }
                }

                if hidden {
                    if ENABLE_OUTPUT {
                        println!("\tCell at <y = {}, x = {}> is hidden!", y, x);
                    }
                    count += 1;
                }
            }
        }
        dbg!(count);
    } else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to open the file!",
        ));
    }

    Ok(())
}
