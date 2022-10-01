pub fn arrange_phrase(phrase: &str) -> String {
    println!("{}", phrase);
    let list_of_word: Vec<&str> = phrase.split(" ").collect();
    let mut to_return = String::new();
    let mut vec: Vec<usize> = Vec::new();

    for i in 1..list_of_word.len() + 1 {
        vec.push(list_of_word.iter().position(|&r| r.contains(&i.to_string())).unwrap())
    }

    for i in vec.iter() {
        to_return += list_of_word[*i];
        to_return += " ";
    }

    return to_return.trim().to_string();
}