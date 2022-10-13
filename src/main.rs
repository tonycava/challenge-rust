use hashing::*;

fn main() {
    println!("Hello, world!");
    let v = vec![2, 1, 5, 2, 7, 4];
    println!("median {}", hashing::median(&v));
}