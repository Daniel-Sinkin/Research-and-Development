fn main() {
    let c1 = "中";
    print_char(c1.chars().nth(0).unwrap());
}

fn print_char(c: char) {
    println!("{}", c);
}
