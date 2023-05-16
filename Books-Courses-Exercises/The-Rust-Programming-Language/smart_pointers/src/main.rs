enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let l = List::Nil;
    let l2 = List::Cons(0, Box::new(l));
    let l3 = List::Cons(1, Box::new(l2));

    if let List::Cons(x, y) = l3 {
        println!("{x}");

        if let List::Cons(xp, _) = *y {
            println!("{xp}");
        }
    }
}
