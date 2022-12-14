pub fn first_subword(s: String) -> String {
    for (idx, letter) in s.chars().enumerate() {
        if letter.is_uppercase() && idx != 0 {
            return s[0..idx].to_string()
        }
        if letter == '_' && idx != 0 {
            return s[0..idx].to_string()
        }
    }
    s.to_string()
}