use std::fs::File;
use std::io::{self, Read};

const FILENAME: &str = "day5";
#[allow(unused_mut)]
#[allow(unused_variables)]
fn main() -> io::Result<()> {
    let file = File::open(FILENAME);

    if let Ok(mut file) = File::open(FILENAME) {
        let mut body = String::new();
        file.read_to_string(&mut body)?;

        let mut stacks: [Vec<char>; 9] = Default::default();

        let mut split = body.split("\n\n");
        let string_stack = split.next().unwrap();

        let mut string_stack_split = string_stack.split("\n");
        for line in string_stack_split {
            if line == "" || line.contains('1') {
                break;
            }
            for i in 0..9 {
                let curr = line.chars().nth(4 * i + 1).unwrap();
                print!("[{}] ", curr);

                if curr != ' ' {
                    stacks[i].push(curr);
                }
            }

            for mut stack in stacks {
                stack.reverse();
            }
            println!("\t{line}");
        }
    } else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to open the file!",
        ));
    }

    Ok(())
}
