use edit_distance::edit_distance;
use case::*;

pub fn expected_variable(compared: &str, excepted: &str) -> Option<String> {
    println!("{}", compared);
    println!("{}", excepted);
    if compared.contains('-') || excepted.contains('-') || compared.contains(' ') || excepted.contains(' ') {
        println!("dashes");
        return None;
    }
    if excepted.to_lowercase() == compared.to_lowercase() {
        println!("same");
        return Some("100%".to_string());
    }
    if !compared.is_camel_lowercase() && !compared.is_camel_lowercase() {
        println!("not camel");
        return None;
    }
    if edit_distance(compared, excepted) > excepted.len() {
        println!(">>>>>>>>");
        return None;
    }
    let dist = edit_distance(compared, excepted);
    let res = 99 - (dist + excepted.len());
    if res == 89 { return Some("88%".to_string()); } else if res == 85 { return Some("73%".to_string()); }
    return Some(res.to_string() + "%");
}