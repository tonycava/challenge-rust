use copy::*;

fn main() {
    let a: i32 = 0;
    let b = String::from("1 2 4 5 6");
    let c = vec![1, 2, 4, 5];

    println!("{:?}", nbr_function(a));
    println!("{:?}", nbr_function(-12));
    println!("{:?}", str_function(b));
    println!("{:?}", vec_function(c));
}