use gtk::prelude::*;

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
        } else if menu == "Bag".to_string() {
            let pl = game::get_player().clone();
            let bag = pl.bag.unwrap();
            let at0 = bag.get_by_id(0);
            let mut at0b = Self::new("", "", 0, "Don't have this item");
            match at0 {
                Some(a0) => {
                    at0b = Self::new(&a0.0.name, &format!("item {}", a0.0.id), 0, &format!("{}\nAmount: {}",a0.0.desc,a0.1));
                },
                _ => {}
            };
            let at1 = bag.get_by_id(1);
            let mut at1b = Self::new("", "", 1, "Don't have this item");
            match at1 {
                Some(a1) => {
                    at1b = Self::new(&a1.0.name, &format!("item {}", a1.0.id), 2, &format!("{}\nAmount: {}",a1.0.desc,a1.1));
                },
                _ => {}
            }
            let at2 = bag.get_by_id(2);
            let mut at2b = Self::new("", "", 2, "Don't have this item");
            match at2 {
                Some(a2) => {
                    at2b = Self::new(&a2.0.name, &format!("item {}", a2.0.id), 1, &format!("{}\nAmount: {}",a2.0.desc,a2.1));
                },
                _ => {}
            }
            let at3 = bag.get_by_id(3);
            let mut at3b = Self::new("", "", 3, "Don't have this item");
            match at3 {
                Some(a3) => {
                    at3b = Self::new(&a3.0.name, &format!("item {}", a3.0.id), 3, &format!("{}\nAmount: {}",a3.0.desc,a3.1));
                },
                _ => {}
            }
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
    pub fn r#box(menu:&str, selected: &str) -> gtk::Box {
        let menu = Self::button_list(menu.to_string());
        let op0 = gtk::Box::new(gtk::Orientation::Vertical, 1);
        let op00 = gtk::Box::new(gtk::Orientation::Vertical, 1);
        op00.set_valign(gtk::Align::Center);
        op00.set_vexpand(true);
        let op0lab = gtk::Label::builder().label(menu[0].label.clone()).build();
        op0.set_hexpand(true);
        op0.set_vexpand(true);
        op0.append(&op00);
        op00.append(&op0lab);
        op0.add_css_class("option0");
        let op1 = gtk::Box::new(gtk::Orientation::Vertical, 1);
        let op11 = gtk::Box::new(gtk::Orientation::Vertical, 1);
        op11.set_valign(gtk::Align::Center);
        op11.set_vexpand(true);
        let op1lab = gtk::Label::builder().label(menu[1].label.clone()).build();
        op1.set_hexpand(true);
        op1.set_vexpand(true);
        op1.append(&op11);
        op11.append(&op1lab);
        op1.add_css_class("option1");
        let op2 = gtk::Box::new(gtk::Orientation::Vertical, 1);
        let op22 = gtk::Box::new(gtk::Orientation::Vertical, 1);
        op22.set_valign(gtk::Align::Center);
        op22.set_vexpand(true);
        let op2lab = gtk::Label::builder().label(menu[2].label.clone()).build();
        op2.set_hexpand(true);
        op2.set_vexpand(true);
        op2.append(&op22);
        op22.append(&op2lab);
        op2.add_css_class("option2");
        let op3 = gtk::Box::new(gtk::Orientation::Vertical, 1);
        let op33 = gtk::Box::new(gtk::Orientation::Vertical, 1);
        op33.set_valign(gtk::Align::Center);
        op33.set_vexpand(true);
        let op3lab = gtk::Label::builder().label(menu[3].label.clone()).build();
        op3.set_hexpand(true);
        op3.set_vexpand(true);
        op3.append(&op33);
        op33.append(&op3lab);
        op3.add_css_class("option3");
        match selected {
            _ if selected.to_string() == "0".to_string() => op0.add_css_class("selected"),
            _ if selected.to_string() == "1".to_string() => op1.add_css_class("selected"),
            _ if selected.to_string() == "2".to_string() => op2.add_css_class("selected"),
            _ if selected.to_string() == "3".to_string() => op3.add_css_class("selected"),
            _ => {}
        };
        let b = gtk::Box::new(gtk::Orientation::Vertical, 1);
        let bb1 = gtk::Box::new(gtk::Orientation::Horizontal, 1);
        let bb2 = gtk::Box::new(gtk::Orientation::Horizontal, 1);
        bb1.set_homogeneous(true);
        bb2.set_homogeneous(true);
        bb1.append(&op0);
        bb1.append(&op2);
        bb2.append(&op1);
        bb2.append(&op3);
        b.append(&bb1);
        b.append(&bb2);
        return b
    }
}
