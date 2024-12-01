use crate::game;
use crate::tattacks::Attack;

#[derive(Clone, Debug)]
pub struct Button {
    pub label: String,
    pub action: String,
    pub position: usize,
    pub desc: String,
}

impl Button {
    pub fn new(label: &str, action: &str, position: usize, desc: &str) -> Self {
        Self {
            label: label.to_string(),
            action: action.to_string(),
            position,
            desc: desc.to_string()
        }
    } 
    pub fn button_list(menu: String) -> Vec<Self> {
        let mut buttons = vec!();
        if menu == "base".to_string() {
            let attack = Self::new("Attack", "open_attack", 0, "Chose Atack to fight enemy");
            let bag = Self::new("Bag", "open_bag", 1, "Open Bag");
            let AAA = Self::new("AAA", "scream", 2, "Scream");
            let run = Self::new("Run", "run", 3, "Run from enemy");
            buttons.push(attack);
            buttons.push(bag);
            buttons.push(AAA);
            buttons.push(run);
        } else if menu == "Attack".to_string() {
            let pl = game::get_player().clone();
            let ats = pl.attacks;
            let at0 = Attack::get_by_id(&ats[0]).unwrap();
            let at1 = Attack::get_by_id(&ats[2]).unwrap();
            let at2 = Attack::get_by_id(&ats[1]).unwrap();
            let at3 = Attack::get_by_id(&ats[3]).unwrap();
            let at0b = Self::new(&at0.name, &format!("use {}", at0.id), 0, &at0.desc);
            let at1b = Self::new(&at1.name, &format!("use {}", at1.id), 1, &at1.desc);
            let at2b = Self::new(&at2.name, &format!("use {}", at2.id), 2, &at2.desc);
            let at3b = Self::new(&at3.name, &format!("use {}", at3.id), 3, &at3.desc);
            buttons.push(at0b);
            buttons.push(at1b);
            buttons.push(at2b);
            buttons.push(at3b);
        }
        return buttons
    }
    pub fn get_position(menu: &str, position: usize) -> Self {
        Button::button_list(menu.to_string()).into_iter().filter(|r| r.position == position).collect::<Vec<_>>()[0].clone()
    }
}
