use crate::tentity::Entity;

#[derive(Clone, Debug)]
pub struct Attack {
    name: String,
    dmg: usize,
    //effect: String
}
impl Attack {
    pub fn new(name: &str, dmg: usize/*, effect: String*/) -> Self {
        Self {
            name: name.to_string(),
            dmg,
            //effect
        }
    }

    pub fn list() -> Vec<Self> {
        let bite = Self::new("Bite", 10);
        let divide = Self::new("Divide", 20);
        let substract = Self::new("Substract", 15);
        return vec!(bite, divide, substract)
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

    pub fn r#use(&self, who: Entity, mut tar: Entity) -> String {
        tar.get_dmg(self.dmg);
        return format!("{} used {} and deal {} damage", who.character, self.name, self.dmg)
    }
}
