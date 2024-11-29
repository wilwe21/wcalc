use std::sync::Mutex;

use crate::tentity::Entity;
use crate::tattacks::Attack;
use crate::game;

static mut enemy: Option<Mutex<Entity>> = None;

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

pub fn start() -> String {
    let en = Entity::get_by_name("Snake").unwrap();
    set_enemy(Some(en.clone()));
    return format!("{} appeard from dark", en.character);
}

pub fn moves(text: String, button: String) -> String {
    let mut pl = game::get_player().clone();
    match get_enemy() {
        Some(_) => {},
        _ => {
            pl.change_mode("None".to_string());
            game::update_player(pl);
            return "No enemy".to_string()
        }
    }
    let mut en = get_enemy().unwrap();
    if en.health == 0 {
            pl.change_mode("None".to_string());
            game::update_player(pl);
            return "Enemy defeated".to_string()
    }
    if button == "^" || (button == "=" && text.ends_with("^")) {
        pl.change_mode("None".to_string());
        game::update_player(pl);
        return "escaping from battle".to_string()
    } else if button == "1" || (button == "=" && text.ends_with("1")) {
        let bite = Attack::get_by_name("Bite").unwrap();
        let (s, t) = bite.r#use(pl.clone(), en.clone());
        if t.health == 0 {
            pl.change_mode("None".to_string());
            game::update_player(pl);
            return "Enemy defeated".to_string()
        }
        set_enemy(Some(t));
        return s
    }
    return format!("player: {}hp, Enemy: {}hp", pl.health, en.health).to_string() 
}
