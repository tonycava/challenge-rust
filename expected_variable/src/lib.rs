use edit_distance::edit_distance;

pub fn expected_variable(compared: &str, excepted: &str) -> Option<String> {
    if compared.contains('-') || excepted.contains('-') || compared.contains(' ') || excepted.contains(' ') {
        return None;
    }
    if compared.to_lowercase() == excepted.to_lowercase() {
        return Some("100%".to_string());
    }
    if edit_distance(compared, excepted) > compared.len() / 2 {
        return None;
    }
    let dist =  edit_distance(compared, excepted);
    let res =  99 - (dist  + excepted.len());
    return Some(res.to_string() + "%");
}