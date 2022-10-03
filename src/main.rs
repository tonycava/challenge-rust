use arrays::*;

fn main() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let a1 = (1..11).collect::<Vec<i32>>();
    let b = [5; 10]; //missing info here

    println!("{:?}",  a1);
    println!("{:?}",  a);

    println!("The Sum of the elements in {:?} = {}", a, sum(a.iter()));//missing info here

    // println!("The Sum of the elements in {:?} = {:?}", a1, sum(a1));//missing info here
    //
    // println!("The Sum of the elements in {:?} = {}", b, sum(b));//missing info here

    println!(
        "Array size {} with only 10's in it {:?}",
        thirtytwo_tens().len(),
        thirtytwo_tens()
    );
}