pub fn arrange_phrase(phrase: &str) -> String {
    const LIST_OF_WORD: Vec<&str> = phrase.split(" ").collect();
    let mut to_return = String::new();
    let mut vec: Vec<usize> = Vec::new();

    for i in 1..LIST_OF_WORD.len() + 1 {
        vec.push(LIST_OF_WORD.iter().position(|&r| r.contains(&i.to_string())).unwrap())
    }

    for i in vec.iter() {
        to_return += LIST_OF_WORD[*i];
        to_return += " ";
    }

    return to_return.trim().to_string();
}