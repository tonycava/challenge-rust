use hashing::*;

fn main() {
    println!("Hello, world!");
    let v = vec![2, 1, 5, 2, 7, 4];
    let b = vec![1, 7, 5, 5, 6, 4];
    println!("median {}", hashing::median(&v));
    println!("median {}", hashing::median(&b));
}