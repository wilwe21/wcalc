use crate::tentity::Entity;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Item {
    pub name: String,
    pub id: String,
    pub action: String,
    pub desc: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bag {
    pub s0: Option<(Item, usize)>,
    pub s1: Option<(Item, usize)>,
    pub s2: Option<(Item, usize)>,
    pub s3: Option<(Item, usize)>,
}

impl Bag {
    pub fn new(s0: Option<(Item, usize)>, s1: Option<(Item, usize)>, s2: Option<(Item, usize)>, s3: Option<(Item, usize)>) -> Self {
        Self {
            s0,
            s1,
            s2,
            s3
        }
    }
    pub fn get_by_id(&self, id: usize) -> Option<(Item, usize)> {
        if id == 0 {
            return self.s0.clone()
        } else if id == 1 {
            return self.s1.clone()
        } else if id == 2 {
            return self.s2.clone()
        } else if id == 3 {
            return self.s3.clone()
        } else {
            return None
        }
    }
    pub fn find_index_by_item(&self, item: Item) -> Option<usize> {
        match &self.s0 {
            Some(s) => {
                if s.0.id == item.id {
                    return Some(0)
                }
            },
            _ => {}
        };
        match &self.s1 {
            Some(s) => {
                if s.0.id == item.id {
                    return Some(1)
                }
            },
            _ => {}
        };
        match &self.s2 {
            Some(s) => {
                if s.0.id == item.id {
                    return Some(2)
                }
            },
            _ => {}
        };
        match &self.s3 {
            Some(s) => {
                if s.0.id == item.id {
                    return Some(3)
                }
            },
            _ => {}
        };
        return None
    }

    pub fn rm_item(&mut self, id: usize, amount: usize) {
        if id == 0 {
            match &self.s0 {
                Some(s) => {
                    if s.1.clone() - amount.clone() > 0 {
                        self.s0 = Some((s.0.clone(), s.1.clone() - amount));
                    } else { 
                        self.s0 = None 
                    } 
                },
                _ => {}
            }
        } else if id == 1 { 
            match &self.s1 {
                Some(s) => {
                    if s.1.clone() - amount.clone() > 0 {
                        self.s1 = Some((s.0.clone(), s.1.clone() - amount));
                    } else { 
                        self.s1 = None 
                    } 
                },
                _ => {}
            }
        } else if id == 2 {
            match &self.s2 {
                Some(s) => {
                    if s.1.clone() - amount > 0 {
                        self.s2 = Some((s.0.clone(), s.1.clone() - amount));
                    } else { 
                        self.s2 = None 
                    } 
                },
                _ => {}
            }
        } else if id == 3 {
            match &self.s3 {
                Some(s) => {
                    if s.1.clone() - amount > 0 {
                        self.s3 = Some((s.0.clone(), s.1.clone() - amount));
                    } else { 
                        self.s3 = None 
                    } 
                },
                _ => {}
            }
        }
    }
}

impl Item {
    pub fn new(name: &str, id: &str, action: &str, desc: &str) -> Self {
        Self {
            name: name.to_string(),
            id: id.to_string(),
            action: action.to_string(),
            desc: desc.to_string(),
        }
    }

    pub fn list_items() -> Vec<Self> {
        let heal = Self::new("HP Potion", "potionHP", "heal 20", "Heal yourself by 20 hp");
        let bow = Self::new("Bow", "bow", "shoot", "Shoot your enemy");
        return vec!(heal, bow)
    }

    pub fn get_by_id(id: &str) -> Option<Self> {
        let items = Self::list_items();
        for i in items {
            if i.id == id.to_string() {
                return Some(i)
            }
        }
        None
    }

    pub fn r#use(&self, mut who: Entity, mut tar: Entity) -> (String, Entity, Entity) {
        let mut rval = String::new();
        let ac = self.action.clone();
        if ac.starts_with("heal ") {
            let amount = ac.replace("heal ", "").parse::<usize>().unwrap();
            who.heal(amount);
            rval = format!("{} healed by {}hp", who.character, amount);
        }
        return (rval.to_string(), who, tar)
    }
}
