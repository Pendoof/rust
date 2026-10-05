fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("hello: {hello}, world: {world}");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}