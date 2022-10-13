use std::collections::HashMap;

pub fn bigger(map: HashMap<&str, i32>) -> i32 {
    *map.iter().max_by_key(|entry| entry.1).unwrap().1
}
