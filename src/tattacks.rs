use crate::tentity::Entity;
use crate::fight;

#[derive(Clone, Debug)]
pub struct Attack {
    pub name: String,
    pub id: String,
    pub dmg: usize,
    pub desc: String,
    pub dial: String,
    //pub effect: String
}
impl Attack {
    pub fn new(name: &str, id: &str, dmg: usize, desc: &str, dial: &str/*, effect: String*/) -> Self {
        Self {
            name: name.to_string(),
            id: id.to_string(),
            dmg,
            desc: desc.to_string(),
            dial: dial.to_string(),
            //effect
        }
    }

    pub fn list() -> Vec<Self> {
        let bite = Self::new("Bite", "bite", 10, "Bite your enemy for 10hp", "{} bite {} and deal {} damage");
        let divide = Self::new("Divide", "divide", 20, "Divide enemy by 20hp", "{} divide {} and deal {} damage");
        let substract = Self::new("Substract", "sub", 15, "Substract from your enemy 15 hp", "{} substract from {} {}hp");
        let venom = Self::new("Venom", "venom", 5, "Deal 5 dmg and give emeny poison", "{} applay venom on {} and deal {} damage");
        let kick = Self::new("Kick", "kick", 25, "Deal 25 dmg to your oponent", "{} kick {} and deal {} damage");
        let stand = Self::new("Stand Still", "standStill", 0, "Sand and stare at your oponent", "{} stare at {}");
        let none = Self::new("", "", 0, "You don't have this attack", "");
        return vec!(bite, divide, substract, venom, kick, stand, none)
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

    pub fn r#use(&self, who: Entity, mut tar: Entity) -> (String, Entity) {
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
        return (formated, tar)
    }
}
