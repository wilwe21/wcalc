use crate::tentity::Entity;
use crate::fight;

#[derive(Clone, Debug)]
pub struct Attack {
    pub name: String,
    pub dmg: usize,
    pub desc: String,
    //pub effect: String
}
impl Attack {
    pub fn new(name: &str, dmg: usize, desc: &str/*, effect: String*/) -> Self {
        Self {
            name: name.to_string(),
            dmg,
            desc: desc.to_string(),
            //effect
        }
    }

    pub fn list() -> Vec<Self> {
        let bite = Self::new("Bite", 10, "Bite your enemy for 10hp");
        let divide = Self::new("Divide", 20, "Divide enemy by 20hp");
        let substract = Self::new("Substract", 15, "Substract from your enemy 15 hp");
        let venom = Self::new("Venom", 5, "Deal 5 dmg and give emeny poison");
        let none = Self::new("", 0, "You don't have this attack");
        return vec!(bite, divide, substract, venom, none)
    }
    pub fn get_by_name(name: &str) -> Option<Self> {
        let list = Self::list();
        for i in list {
            if i.name == name.to_string() {
                return Some(i)
            }
        }
        None
    }

    pub fn r#use(&self, who: Entity, mut tar: Entity) -> (String, Entity) {
        tar.get_dmg(self.dmg);
        return (format!("{} used {} and deal {} damage", who.character, self.name, self.dmg), tar)
    }
}
