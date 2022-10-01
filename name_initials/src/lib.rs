use std::fmt::format;

pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut initias: Vec<String> = vec![];

    for name in names.iter() {
        if is_already_initials(*name) {
            initias.push((*name.to_string()).parse().unwrap());
            continue;
        }

        let s = String::from(*name);
        let index_of_space = s.find(' ').unwrap();
        let first_letter = s.chars().nth(0).unwrap();
        let second_letter = s.chars().nth(index_of_space + 1).unwrap();

        initias.push(format!("{}. {}.", first_letter, second_letter))
    }
    return initias;
}

fn is_already_initials(s: &str) -> bool {
    let point_one = s.chars().nth(1).unwrap();
    let point_two = s.chars().nth(1).unwrap();
    return point_one == '.' && point_two == '.';
}