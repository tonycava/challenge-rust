use std::collections::HashMap;

pub fn mean(list: &Vec<i32>) -> f64 {
    let mut total = 0;
    for nbr in list.iter() {
        total += nbr
    }
    return total as f64 / list.len() as f64;
}

pub fn median(list: &Vec<i32>) -> i32 {
    let mut srtd = list.clone();
    srtd.sort();

    if list.len() % 2 == 0 {
        return (srtd[list.len() / 2 - 1] + list[list.len() / 2]) / 2;
    }
    return srtd[list.len() / 2];
}

pub fn mode(list: &Vec<i32>) -> i32 {
    let mut map: HashMap<String, u32> = HashMap::new();

    for nbr in list.iter() {
        if map.contains_key(&nbr.to_string()) {
            *map.entry(nbr.to_string()).or_insert(0) += 1;
            continue;
        }
        map.insert(nbr.to_string(), 1);
    }
    let str: &String = map.iter().max_by_key(|entry| entry.1).unwrap().0;
    str.parse().unwrap()
}
