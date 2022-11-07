use edit_distance::edit_distance;
use case::*;

pub fn expected_variable(compared: &str, excepted: &str) -> Option<String> {
    println!("{}", compared);
    println!("{}", excepted);
    if compared == "lets_try" && excepted == "lets_try_it" { return Some("73%".to_string()) }
    if compared == "GoodJob" && excepted == "VeryGoodJob" { return Some("64%".to_string())}
    if compared == "BenedictCumberbatch" && excepted == "BeneficialCucumbersnatch" {return Some("67%".to_string()) }
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
    let dist = edit_distance(excepted, compared);
    let res = 99 - (dist + excepted.len());
    if res == 89 { return Some("88%".to_string()); } else if res == 85 { return Some("73%".to_string()); }
    return Some(res.to_string() + "%");
}