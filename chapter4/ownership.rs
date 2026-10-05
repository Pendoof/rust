fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{s}");

    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!");

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
}