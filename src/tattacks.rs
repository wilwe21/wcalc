use std::fmt;

use crate::tentity::Entity;
use crate::fight;

#[derive(Clone, Debug)]
pub struct Attack {
    pub name: String,
    pub id: String,
    pub dmg: usize,
    pub desc: String,
    pub dial: String,
    pub anim: String,
    //pub effect: String
}

impl fmt::Display for Attack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "id: {}, dmg: {}", self.id, self.dmg)
        } else {
            write!(f, "{}, {}", self.id, self.dmg)
        }
    }
}

impl Attack {
    pub fn new(name: &str, id: &str, dmg: usize, desc: &str, dial: &str, anim: &str/*, effect: String*/) -> Self {
        Self {
            name: name.to_string(),
            id: id.to_string(),
            dmg,
            desc: desc.to_string(),
            dial: dial.to_string(),
            anim: anim.to_string(),
            //effect
        }
    }

    pub fn list() -> Vec<Self> {
        let bite = Self::new("Bite", "bite", 10, "Bite your enemy for 10hp", "{} bite {} and deal {} damage", "attack");
        let divide = Self::new("Divide", "divide", 20, "Divide enemy by 20hp", "{} divide {} and deal {} damage", "attack");
        let substract = Self::new("Substract", "sub", 15, "Substract from your enemy 15 hp", "{} substract from {} {}hp", "attack");
        let venom = Self::new("Venom", "venom", 5, "Deal 5 dmg and give emeny poison", "{} applay venom on {} and deal {} damage", "attack");
        let kick = Self::new("Kick", "kick", 25, "Deal 25 dmg to your oponent", "{} kick {} and deal {} damage", "attack");
        let stand = Self::new("Stand Still", "standStill", 0, "Sand and stare at your oponent", "{} stare at {}", "None");
        let quack = Self::new("Quack", "quack", 5, "Quack at enemy", "{} quack at {}", "attack");
        let i = Self::new("Imaginary", "i", 75, "Shoot at your oponent imaginary bullet", "{} shoot at {} imaginary bullet and deal {} damage", "attack");
        let none = Self::new("", "", 0, "You don't have this attack", "", "None");
        return vec!(bite, divide, substract, venom, kick, stand, quack, i, none)
    }
    pub fn get_by_id(id: &str) -> Option<Self> {
        let list = Self::list();
        for i in list {
            if i.id == id.to_string() {
                return Some(i)
            }
        }
        None
    }

    pub fn r#use(&self, who: Entity, mut tar: Entity) -> (String, Entity, String) {
        tar.get_dmg(self.dmg);
        let sc = self.clone().dial;
        let sc = sc.split("{}").filter(|r| *r != "").collect::<Vec<_>>();
        let mut formated = String::new();
        if sc.len() < 2 {
            formated = format!("{}{}{}", who.character, sc[0], tar.character);
        } else if sc.len() < 3  {
            formated = format!("{}{}{}{}", who.character, sc[0], tar.character, sc[1]);
        } else {
            formated = format!("{}{}{}{}{}{}", who.character, sc[0], tar.character, sc[1], self.dmg, sc[2]);
        }
        return (formated, tar, self.anim.clone())
    }
}
