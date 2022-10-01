pub fn delete_and_backspace(s: &mut String) {
    let mut idx = 0;

    while s.contains('-') || s.contains('+') {
        let char = s.chars().nth(idx).unwrap();
        if char == '-' && idx < s.len() {
            s.remove(idx);
            s.remove(idx - 1);
            idx = 0;
        } else if char == '+' && idx < s.len() {
            let mut pass_here = false;
            s.remove(idx);

            while s.chars().nth(idx).unwrap() == '+' {
                pass_here = true;
                idx += 1;
            }

            if pass_here {
                s.remove(idx + 1);
            } else {
                s.remove(idx);
            }
            idx = 0;
        }
        idx += 1;
    }
}

pub fn is_correct(v: &mut Vec<&str>) -> usize {
    let mut number_of_good_response = 0;
    for (idx, calc) in v.to_owned().iter().enumerate() {
        let index_of_equal = calc.find('=').unwrap();
        let attemped_result = &calc[index_of_equal + 1..];
        let index_of_operator = calc.find('+').unwrap_or_else(|| calc.find('-').unwrap());
        let operator = calc.chars().nth(index_of_operator).unwrap();

        let first_number_string = &calc[0..index_of_operator];
        let second_number_string = &calc[index_of_operator + 1..index_of_equal];

        let first_number = first_number_string.parse::<i32>().unwrap();
        let second_number = second_number_string.parse::<i32>().unwrap();

        if operator == '+' {
            let result = (first_number + second_number).to_string();
            if result == attemped_result {
                v[idx] = "✔";
                number_of_good_response += 1;
            } else {
                v[idx] = "✘";
            }
        }

        if operator == '-' {
            let result = (first_number - second_number).to_string();
            if result == attemped_result {
                v[idx] = "✔";
                number_of_good_response += 1;
            } else {
                v[idx] = "✘";
            }
        }
    }

    let response = number_of_good_response as f64 / v.len() as f64 * 100.0;
    return response as usize;
}