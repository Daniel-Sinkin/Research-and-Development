use ndarray;

fn main() {
    let a = ndarray::arr2(&[
        [1.0, 2.0],
        [4.0, 5.0],
    ]);
    let b = ndarray::arr2(&[
        [1.0, 2.0],
        [4.0, 5.0],
        [6.0, 7.0],
    ]);

    println!("{}", a.dot(&b));
}