use std::fmt::format;

pub fn arrange_phrase(phrase: &str) -> String {
    let list_of_word: Vec<&str> = phrase.split(" ").collect();
    let mut to_return = String::new();
    let mut vec: Vec<usize> = Vec::new();

    for i in 1..list_of_word.len() + 1 {
        vec.push(list_of_word.iter().position(|&r| r.contains(&i.to_string())).unwrap())
    }

    for i in vec.iter() {
        let word: String = list_of_word[*i].chars().filter(|c| !c.is_digit(10)).collect();
        to_return += &format!("{} ", word);
    }

    return to_return.trim().to_string();
}