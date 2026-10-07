#[derive(Debug)]

fn main() {
    let v: Vec<i32> = Vec::new();
    print_vector(&v);

    let v = vec![1, 2, 3];
    print_vector(&v);

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    print_vector(&v);

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let mut v = vec![100, 32, 57];
    print_vector(&v);
    for i in &mut v {
        *i += 50;
    }
    print_vector(&v);

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn print_vector(v: &Vec<i32>) {
    for i in v {
        println!("{i}");
    }
}