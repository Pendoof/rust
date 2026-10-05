fn main() {
    let x: i32 = 5;
    let y: f32 = 3.0;
    let z: bool = true;
    let heart_eyed_cat: char = '😻';
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
    println!("The value of heart_eyed_cat is: {heart_eyed_cat}");

    let tup: (i32, f32, bool) = (500, 6.4, true);
    let (a, b, c) = tup;
    println!("The value of a is: {a}");
    println!("The value of b is: {b}");
    println!("The value of c is: {c}");
    
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("The value of five_hundred is: {five_hundred}");
    println!("The value of six_point_four is: {six_point_four}");
    println!("The value of one is: {one}");

    let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let a = [3; 5];
    println!("The value of months is: {:?}", months);
    println!("The value of a is: {:#?}", a);
}