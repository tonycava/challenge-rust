#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Light {
    pub alias: String,
    pub brightness: u8,
}

impl Light {
    pub fn new(alias: &str) -> Self {
        Light {
            alias: alias.to_string(),
            brightness: 0,
        }
    }
}

pub fn change_brightness(lights: &mut Vec<Light>, alias: &str, value: u8) {
    println!("{:?}", lights);
    println!("{}", alias);
    println!("{}", value);

    let pos = lights.iter().position(|r| r.alias == alias).unwrap_or_else(|| 0);
    lights[pos].brightness = value;
}