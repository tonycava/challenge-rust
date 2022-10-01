pub fn arrange_phrase(phrase: &str) -> String {
    let list_of_word: Vec<&str> = phrase.split(" ").collect();
    let mut to_return = String::from("");
    let mut vec: Vec<usize> = Vec::new();


    for i in 1..list_of_word.len() + 1 {
        let condition = i.to_string();
        vec.push(list_of_word.iter().position(|&r| r.contains(&condition)).unwrap())
    }

    for i in vec.iter() {
        let word: String = list_of_word[*i].replace(&['1', '2', '3', '4', '5', '6', '7', '8', '9', ], "");

        to_return += &format!("{} ", word);
    }

    return to_return.trim().to_string();
}