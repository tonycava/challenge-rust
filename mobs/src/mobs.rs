use crate::mobs::boss::Boss;
use crate::mobs::member::Member;

pub mod member;
pub mod boss;

#[derive(Debug, Clone, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub cities: Vec<(String, u8)>,
    pub members: Vec<Member>,
    pub wealth: u32,
}