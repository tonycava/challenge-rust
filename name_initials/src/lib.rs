pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut initias: Vec<String> = vec![];

    for name in names.iter() {
        if is_already_initials(*name) {
            let s: String = String::from(*name);
            initias.push(s);
            continue;
        }

        let s = String::from(*name);
        let mut final_string = String::from("");

        let index_of_space = s.find(' ').unwrap();
        let first_letter = String::from(s.chars().nth(0).unwrap());
        let second_letter = String::from(s.chars().nth(index_of_space + 1).unwrap());

        final_string += &*first_letter;
        final_string += ". ";
        final_string += &*second_letter;
        final_string += ".";

        initias.push(final_string.to_string())
    }
    return initias;
}

fn is_already_initials(s: &str) -> bool {
    let point_one = String::from(s.chars().nth(1).unwrap());
    let point_two = String::from(s.chars().nth(1).unwrap());
    return point_one == "." && point_two == ".";
}