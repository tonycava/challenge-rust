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
    let pos = lights.iter().position(|r| r.alias == alias).unwrap_or_else(|| 20);
    if pos != 20 {
        lights[pos].brightness = value;
    }
}