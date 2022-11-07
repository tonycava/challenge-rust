#[derive(Debug, Clone, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub cities: Vec<(String, u8)>,
    pub members: Vec<Member>,
    pub wealth: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Boss {
    pub name: String,
    pub age: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    pub name: String,
    pub role: Role,
    pub age: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Underboss = 4,
    Caporegime = 3,
    Soldier = 2,
    Associate = 1,
}