use crate::mobs::{Boss, Member, Mob, Role};
mod mobs;

impl Boss {
    pub fn new(&self, name: String, age: u8) -> Self {
        Self {
            name,
            age,
        }
    }
}

impl Mob {
    pub fn recruit(&mut self, name: String, age: u8) {
        self.members.push(Member {
            age,
            name,
            role: Role::Associate,
        });
    }
    pub fn steal(&mut self, mob2: &mut Mob, attack: u32) {
        self.wealth -= attack;
        mob2.wealth += attack;
    }

    pub fn attack(&mut self, mob2: &mut Mob) {
        let mut attack1 = 0;
        let mut attack2 = 0;
        for member in self.members.iter() {
            attack1 += member.role.clone() as u8
        }
        for member in mob2.members.iter() {
            attack2 += member.role.clone() as u8
        }

        println!("{attack1}");
        println!("{attack2}");
    }
}

impl Member {
    pub fn new(&self, name: String, role: Role, age: u8) -> Self {
        Self {
            name,
            role,
            age,
        }
    }
}
