use std::collections::HashMap;
use std::num::ParseFloatError;

#[derive(Debug)]
pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(l_h: &str, desc: &str) -> Flag {
        Flag {
            long_hand: format!("--{}", l_h),
            short_hand: format!("-{}", l_h.chars().nth(0).unwrap()),
            desc: desc.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<(String, String), Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: (String, String), func: Callback) {
        self.flags.insert((flag.0, flag.1), func);
    }
    pub fn exec_func(&mut self, flag: (String, String), argv: &[&str]) -> String {
        match self.flags[&flag](argv[0], argv[1]) {
            Ok(num) => return num,
            Err(error) => return error.to_string(),
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let mut res;

    match a.parse::<f32>() {
        Ok(num) => res = num,
        Err(error) => return Err(error),
    };

    match b.parse::<f32>() {
        Ok(num) => res = res / num,
        Err(error) => return Err(error),
    };

    Ok(res.to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let mut res;

    match a.parse::<f32>() {
        Ok(num) => res = num,
        Err(error) => return Err(error),
    };

    match b.parse::<f32>() {
        Ok(num) => res %= num,
        Err(error) => return Err(error),
    };

    Ok(res.to_string())
}