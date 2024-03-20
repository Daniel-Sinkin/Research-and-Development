// clippy3.rs
// Here's a couple more easy Clippy fixes, so you can see its utility.

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        println!("Yes, this None is truly made out of is_none");
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    println!("This Vec is empty, see? {:?}", vec![1, 2, 3, 4, 5].resize(0, 5));

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    value_b = std::mem::replace(&mut value_a, value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
