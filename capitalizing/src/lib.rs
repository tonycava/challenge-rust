pub fn capitalize_first(input: &str) -> String {
    if input == "" {
        return "".to_string();
    }
    let first_chars = input.chars().nth(0).unwrap();
    if first_chars.is_uppercase() {
        return input.to_string();
    }

    let mut all_string = String::from(input);
    all_string.remove(0);
    return first_chars.to_uppercase().to_string() + &all_string;
}

pub fn title_case(input: &str) -> String {
    if input == "" {
        return "".to_string();
    }
    let all_words: Vec<&str> = input.split(" ").collect();
    let mut out: Vec<String> = Vec::new();
    for word in all_words.iter() {
        let first_char = word.chars().nth(0).unwrap();
        if first_char.is_uppercase() {
            out.push(word.to_string());
            continue;
        }
        out.push(capitalize_first(word))
    }
    return out.join(" ");
}

pub fn change_case(input: &str) -> String {
    if input == "" {
        return "".to_string();
    }
    let mut out: Vec<String> = Vec::new();
    for word in input.chars() {
        println!("{}", word);
        if word.is_uppercase() {
            out.push(word.to_lowercase().to_string());
            continue;
        }
        out.push(word.to_uppercase().to_string())
    }
    return out.join("");
}
