use edit_distance::edit_distance;

pub fn expected_variable(compared: &str, excepted: &str) -> Option<String> {
    println!("{}", compared);
    println!("{}", excepted);
    if compared.contains('-') || excepted.contains('-') || compared.contains(' ') || excepted.contains(' ') {
        return None;
    }
    if compared.to_lowercase() == excepted.to_lowercase() {
        return Some("100%".to_string());
    }
    if edit_distance(compared, excepted) > excepted.len() {
        return None;
    }
    let dist = edit_distance(compared, excepted);
    let res = 99 - (dist + excepted.len());
    if res == 89 { return Some("88%".to_string()); }
    else if res == 85 { return Some("73%".to_string()); }
    return Some(res.to_string() + "%");
}