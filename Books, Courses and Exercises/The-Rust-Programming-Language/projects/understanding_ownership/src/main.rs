use std::str;
fn main() {
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();

        for(i, &item) in bytes.iter().enumerate() {
            if i % 2 == 0 {
                println!("s[2 * {}] : {:?}", i / 2, str::from_utf8(&[item]));
            }
        }

        for byte_ref in bytes.iter() {
            println!("{byte_ref}");
        }

        s.len()
    }
    
    let s = String::from("Hello World!");
    let fw = first_word(&s);
    println!("{fw}");
}