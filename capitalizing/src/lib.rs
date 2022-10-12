pub fn capitalize_first(input: &str) -> String {
    let firstChars = input.chars().nth(0).unwrap();
    if firstChars.is_uppercase() {
        return input.to_string();
    }

    let mut allString = String::from(input);
    allString.remove(0);
    return firstChars.to_uppercase().to_string() + &allString;
}

pub fn title_case(input: &str) -> String {
    let allWords: Vec<&str> = input.split(" ").collect();
    let mut out: Vec<String> = Vec::new();
    for word in allWords.iter() {
        let firstChar = word.chars().nth(0).unwrap();
        if firstChar.is_uppercase() {
            out.push(word.to_string());
            continue;
        }
        out.push(capitalize_first(word))
    }
    return out.join(" ");
}

pub fn change_case(input: &str) -> String {
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
