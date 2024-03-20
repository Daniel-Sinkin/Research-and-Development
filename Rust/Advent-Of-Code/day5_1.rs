use std::fs::File;
use std::io::{self, Read};

struct Stacks {
    crates: [Vec<char>; 9],
}

impl Stacks {
    fn move_stack(&mut self, n: usize, from: usize, to: usize) {
        for i in 0..n {
            let x = self.crates[from].pop().unwrap();
            self.crates[to].push(x);
        }
    }

    fn print_stack(&self, i: usize) {
        dbg!(&self.crates[i]);
    }

    fn height(&self) -> usize {
        let mut max = 0;
        for c in &self.crates {
            if c.len() > max {
                max = c.len();
            }
        }

        max
    }
}

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

            println!("\t{line}");
        }
        for mut stack in &mut stacks {
            stack.reverse();
        }

        let mut ms = Stacks { crates: stacks };
        let string_stack = split.next().unwrap();
        let str = string_stack.split("\n");

        for s in str.into_iter() {
            if s == "" {
                break;
            }

            let mut sp = s.split(" ").into_iter();
            sp.next();
            let n = sp.next().unwrap().parse::<usize>().unwrap();
            sp.next();
            let from = sp.next().unwrap().parse::<usize>().unwrap() - 1;
            sp.next();
            let to = sp.next().unwrap().parse::<usize>().unwrap() - 1;
            ms.move_stack(n, from, to);
        }

        for i in 0..9 {
            ms.print_stack(i);
        }
    } else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to open the file!",
        ));
    }

    Ok(())
}
