fn main() {
    const MAX_NUMBER: u32 = 100;

    println!("{MAX_NUMBER} is a constant.");

    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}