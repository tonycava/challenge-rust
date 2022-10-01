use tic_tac_toe::*;

fn main() {
    let dig = vec![
        vec!["O", "X", "O"],
        vec!["O", "X", "O"],
        vec!["X", "#", "X"]
    ];

    println!("{:?}",tic_tac_toe(dig));
}