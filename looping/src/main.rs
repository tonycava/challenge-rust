use std::io::stdin;

fn main() {
    let response = "The letter e\n";
    let mut attemps = 0;
    let mut user_input = String::new();

    while response != user_input {
        user_input = "".to_string();
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?I don't know");
        stdin().read_line(&mut user_input);
        attemps += 1;
    }
    println!("Number of trials: {}", attemps)
}

