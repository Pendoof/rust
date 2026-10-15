fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();
    let v2: Vec<i32> = v1_iter.map(|x| x + 1).collect();
    let total: i32 = v1_iter.sum();

    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
    println!("total: {:?}", total);
}