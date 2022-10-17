#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    pub validation: bool,
    pub expected: String,
}

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
const REVERSE_ALPHABET: &str = "zyxwvutsrqponmlkjihgfedcba";

impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError {
            validation,
            expected,
        }
    }
}

fn encode(encode: &str) -> String {
    let mut to_return = String::new();
    let mut is_upper = false;
    for word in encode.chars() {
        if word.is_ascii_alphabetic() {
            if word.is_uppercase() { is_upper = true }
            let to_search = word.to_lowercase().nth(0).unwrap();
            let i = ALPHABET.chars().position(|w| w == to_search).unwrap();
            if is_upper {
                to_return += &REVERSE_ALPHABET.chars().nth(i).unwrap().to_uppercase().to_string();
                is_upper = false
            } else { to_return += &REVERSE_ALPHABET.chars().nth(i).unwrap().to_string(); }
            continue;
        }
        to_return += &word.to_string();
    }
    to_return.to_string()
}

pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    if ciphered == encode(original) {
        return Some(Ok(true));
    } else if original != "" && ciphered != "" {
        return Some(Err(CipherError::new(false, encode(original))));
    }
    return None;
}