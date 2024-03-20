fn main() {
    let faulty = [Some(2), Some(5), None, None, Some(2), None, Some(-2)];

    let mut sum: i32 = 0;
    for n in faulty.iter() {
        match n {
            Some(val) => sum += val,
            None => (),
        };
    }

    println!("{sum}");
}