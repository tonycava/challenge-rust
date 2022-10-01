pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    return (c, (c as f64).exp(), (c as f64).abs().log10());
}

pub fn str_function(a: String) -> (String, String) {
    let parsed = a.split(" ");
    let mut s = String::new();

    for nbr_string in parsed {
        let nbr = nbr_string.parse::<f64>().unwrap();
        s += &format!("{} ", nbr.exp().to_string())
    }

    return (a, s.trim().to_string());
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut log: Vec<f64> = vec![];
    for nbr in b.iter() {
        log.push((*nbr as f64).abs().log10())
    }
    return (b, log)
}