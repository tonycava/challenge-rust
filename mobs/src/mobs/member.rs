#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    pub name: String,
    pub role: Role,
    pub age: u8,
}

pub fn new(name: &str, role: Role, age: u8) -> Member {
    Member {
        name: name.to_string(),
        role,
        age,
    }
}

impl Member {
    pub fn get_promotion(&mut self) {
        match self.role {
            Role::Associate => self.role = Role::Soldier,
            Role::Soldier  => self.role = Role::Caporegime,
            Role::Caporegime   => self.role = Role::Underboss,
            _ => self.role = Role::Caporegime,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Underboss = 4,
    Caporegime = 3,
    Soldier = 2,
    Associate = 1,
}
