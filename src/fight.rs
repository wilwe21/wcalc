use std::sync::Mutex;
use std::collections::HashMap;

use crate::tentity::Entity;
use crate::tattacks::Attack;
use crate::game;
use crate::fui;

static mut enemy: Option<Mutex<Entity>> = None;

static mut status: Option<Mutex<HashMap<String, String>>> = None;

pub fn set_enemy(en: Option<Entity>) {
    unsafe {
        match en {
            Some(en) => enemy = Some(Mutex::new(en)),
            _ => enemy = None
        }
    }
}

pub fn get_enemy() -> Option<Entity> {
    unsafe {
        match enemy.as_ref() {
            Some(en) => Some(en.lock().unwrap().clone()),
            _ => None
        }
    }
}

pub fn set_status(en: Option<HashMap<String,String>>) {
    unsafe {
        match en {
            Some(en) => status = Some(Mutex::new(en)),
            _ => status = None
        }
    }
}

pub fn get_status() -> Option<HashMap<String,String>> {
    unsafe {
        match status.as_ref() {
            Some(en) => Some(en.lock().unwrap().clone()),
            _ => None
        }
    }
}

pub fn create_status(selected: Option<String>, dialog: Option<String>, menu: Option<String>) -> HashMap<String,String> {
    let mut sta = HashMap::new();
    match selected {
        Some(s) => sta.insert("selected".to_string(), s.to_string()),
        _ => sta.insert("selected".to_string(), "0".to_string())
    };
    match dialog {
        Some(d) => sta.insert("dialog".to_string(), d.to_string()),
        _ => sta.insert("dialog".to_string(), "...".to_string())
    };
    match menu {
        Some(m) => sta.insert("menu".to_string(), m.to_string()),
        _ => sta.insert("menu".to_string(), "base".to_string())
    };
    return sta
}

pub fn get_menu() -> Vec<String> {
    let stat = get_status();
    let mut menu = vec!();
    match stat {
        Some(s) => {
            let m = s.get("menu").unwrap();
            if *m == "base".to_string() {
                menu.push("Attack".to_string());
                menu.push("A".to_string());
                menu.push("Bag".to_string());
                menu.push("Run".to_string());
            }        
            if *m == "Attack".to_string() {  
                let pl = game::get_player().clone();
                let at = pl.attacks;    
                menu.push(at[0].to_string());
                menu.push(at[2].to_string());
                menu.push(at[1].to_string());
                menu.push(at[3].to_string());
            }
        },
        _ => {
            menu.push("Attack".to_string());
            menu.push("A".to_string());
            menu.push("Bag".to_string());
            menu.push("Run".to_string());
        }
    };
    menu
}

pub fn start() -> String {
    let en = Entity::get_by_name("Snake").unwrap();
    let pl = game::get_player().clone();
    set_status(Some(create_status(None, Some(format!("{} appeard from dark", en.character).to_string()), None)));
    set_enemy(Some(en.clone()));
    fui::update(pl.clone(), en.clone());
    fui::open();
    return format!("{} appeard from dark", en.character);
}

pub fn end() {
    let mut pl = game::get_player().clone();
    pl.change_mode("None".to_string());
    game::update_player(pl);
    fui::close();
}

pub fn moves(text: String, button: String) -> String {
    let mut pl = game::get_player().clone();
    match get_enemy() {
        Some(_) => {},
        _ => {
            end();
            return "No enemy".to_string()
        }
    }
    let mut en = get_enemy().unwrap();
    if en.health == 0 {
        end();
        return "Enemy defeated".to_string()
    }
    if button == "^" || (button == "=" && text.ends_with("^")) {
        end();
        return "escaping from battle".to_string()
    } else if button == "1" || (button == "=" && text.ends_with("1")) {
        let bite = Attack::get_by_name("Bite").unwrap();
        let (s, t) = bite.r#use(pl.clone(), en.clone());
        if t.health == 0 {
            end();
            return "Enemy defeated".to_string()
        }
        set_enemy(Some(t.clone()));
        fui::update(pl.clone(),t.clone());
        return s
    } else if button == "4" || (button == "=" && text.ends_with("4")) {
        let stat = get_status();
        match stat {
            Some(s) => {
                let sel = s.get("selected").unwrap();
                if sel == "2" {
                    set_status(Some(create_status(Some("0".to_string()), Some("Attacks".to_string()), None)))
                } else if sel == "3" {
                    set_status(Some(create_status(Some("1".to_string()), Some("Bag".to_string()), None)))
                }
            },
            _ => {}
        }
        fui::update(pl.clone(),en.clone());
    } else if button == "6" || (button == "=" && text.ends_with("6")) {
        let stat = get_status();
        match stat {
            Some(s) => {
                let sel = s.get("selected").unwrap();
                if sel == "0" {
                    set_status(Some(create_status(Some("2".to_string()), Some("A".to_string()), None)))
                } else if sel == "1" {
                    set_status(Some(create_status(Some("3".to_string()), Some("Run".to_string()), None)))
                }
            },
            _ => {}
        }
        fui::update(pl.clone(),en.clone());
    } else if button == "8" || (button == "=" && text.ends_with("8")) {
        let stat = get_status();
        match stat {
            Some(s) =>  {
                let sel = s.get("selected").unwrap();
                if sel == "1" {
                    set_status(Some(create_status(Some("0".to_string()), Some("Attacks".to_string()), None)))
                } else if sel == "3" {
                    set_status(Some(create_status(Some("2".to_string()), Some("A".to_string()), None)))
                }
            },
            _ => {}
        }
        fui::update(pl.clone(),en.clone());
    } else if button == "2" || (button == "=" && text.ends_with("2")) {
        let stat = get_status();
        match stat {
            Some(s) => {
                let sel = s.get("selected").unwrap();
                if sel == "0" {
                    set_status(Some(create_status(Some("1".to_string()), Some("Bag".to_string()), None)))
                } else if sel == "2" {
                    set_status(Some(create_status(Some("3".to_string()), Some("Run".to_string()), None)))
                }
            },
            _ => {}
        }
        fui::update(pl.clone(),en.clone());
    } else if button == "5" || (button == "=" && text.ends_with("5")) {
        let stat = get_status();
        match stat {
            Some(s) => {
                let sel = s.get("selected").unwrap();
                let men = s.get("menu").unwrap();
                if *men == "base".to_string() {
                    if *sel == "0".to_string() {
                        println!("attack");
                    } else if *sel == "1".to_string() {
                        println!("bag");
                    } else if *sel == "2".to_string() {
                        println!("a");
                    } else if *sel == "3".to_string() {
                        println!("run");
                    }
                }
            },
            _ => {}
        }
    }
    return format!("player: {}hp, Enemy: {}hp", pl.health, en.health).to_string() 
}
