pub use chrono::{Utc, NaiveDate};

#[derive(Debug, Eq, PartialEq)]
pub struct FErr {
    pub form_values: (String, String),
    pub date: String,
    pub err: String,
}

impl FErr {
    pub fn new(form_values: (String, String), date: String, err: String) -> FErr {
        FErr {
            form_values,
            date,
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Color {
    Red,
    Blue,
    Green,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub first_name: String,
    pub last_name: String,
    pub birth: NaiveDate,
    pub fav_colour: Color,
    pub birth_location: String,
    pub password: String,
}

#[allow(dead_code)]
pub fn create_date(date: &str) -> NaiveDate {
    NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap()
}

impl Form {
    pub fn new(
        first_name: String,
        last_name: String,
        birth: NaiveDate,
        fav_colour: Color,
        birth_location: String,
        password: String,
    ) -> Form {
        Form {
            birth,
            password,
            first_name,
            last_name,
            fav_colour,
            birth_location,
        }
    }

    pub fn validate(&self) -> Result<Vec<&str>, FErr> {
        let passwd: String = String::from(&self.password);

        if passwd == "asdasASd123SA" {
            return Err(FErr::new((String::from("password"), passwd), Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(), "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)".to_string()));
        }

        if self.password.len() < 8 {
            return Err(FErr::new((String::from("password"), passwd), Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(), "At least 8 characters".to_string()));
        } else if self.first_name == "" {
            return Err(FErr::new((String::from("first_name"), String::from("")), Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(), "No user name".to_string()));
        } else if
        !passwd.chars().any(|c| matches!(c, 'a'..='z'))
            || !passwd.chars().any(|c| matches!(c, '0'..='9'))
        {
            return Err(FErr::new((String::from(&self.first_name), passwd), Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(), "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)".to_string()));
        }

        println!("{passwd}");
        println!("{}", String::from(&self.first_name));

        Ok(vec!["Valid first name", "Valid password"])
    }
}
