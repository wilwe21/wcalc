use rand::Rng;
use std::sync::Mutex;
use std::collections::HashMap;

use crate::tentity::Entity;
use crate::conf;
use crate::save;
use crate::generate;
use crate::map;

pub static roomwidth: usize = 7;
pub static mapwidth: usize = 6;
pub static spawn: usize = roomwidth/2;

static mut gm: Option<Mutex<u8>> = None;

static mut stats: Option<Mutex<HashMap<String, HashMap<String, String>>>> = None;

static mut player: Option<Mutex<Entity>> = None;

pub fn init_global_stats(conf: HashMap<String, HashMap<String, String>>) {
    unsafe {
        stats = Some(Mutex::new(conf))
    }
}

pub fn get_global_stats() -> HashMap<String, HashMap<String, String>> {
    unsafe {
        if let Some(ref mut s) = stats {
            s.lock().unwrap().clone()
        } else {
            let d = new_conf();
            init_global_stats(d.clone());
            d
        }
    }
}

pub fn init_player(pl: Entity) {
    unsafe {
        player = Some(Mutex::new(pl));
    }
}

pub fn update_player(pl: Entity) {
    unsafe {
        player = Some(Mutex::new(pl));
    }
}

pub fn get_player() -> Entity {
    unsafe {
        if let Some(ref mut p) = player {
            p.lock().unwrap().clone()
        } else {
            let plist = Entity::players_list();
            let pp = Entity::player_from_entity(plist[0].clone(), &format!("{}x{}", spawn,spawn));
            init_player(pp.clone());
            pp
        }
    }
}

pub fn new_conf() -> HashMap<String, HashMap<String, String>> {
    let plist = Entity::players_list();
    let pp = Entity::player_from_entity(plist[0].clone(), &format!("{}x{}", spawn,spawn));
    init_player(pp.clone());
    let s = new_map();
    map::init_map();
    s
}

pub fn new_map() -> HashMap<String, HashMap<String, String>>{
    let mut d = String::new();
    d += &"[map]\n".to_string();
    d += &generate::generate_map(mapwidth).to_string();
    let mut s = save::str_to_conf(d.to_string());
    let rooms = generate::generate_rooms(s.get("map").unwrap().clone(), roomwidth.clone());
    s.extend(rooms);
    s
}

pub fn start() -> String {
    let mut conf = conf::get_conf();
    conf.insert("game".to_string(),"rpg".to_string());
    conf::save_conf(conf);
    init_global_stats(new_conf());
    "".to_string()
}

pub fn end() -> String {
    let mut conf = conf::get_conf();
    conf.insert("game".to_string(),false.to_string());
    conf::save_conf(conf);
    "Quit game".to_string()
}

pub fn end_silent() {
    let mut conf = conf::get_conf();
    map::close_map();
    conf.insert("game".to_string(),false.to_string());
    conf::save_conf(conf);
}

pub fn init_global_rng() {
    unsafe {
        let mut ran = rand::thread_rng();
        gm = Some(Mutex::new(ran.gen::<u8>()))
    }
}

pub fn get_global_rng() -> u8 {
    unsafe {
        gm.as_ref().unwrap().lock().unwrap().clone()
    }
}

pub fn numble() -> String {
    init_global_rng();
    let mut conf = conf::get_conf();
    conf.insert("game".to_string(),"numble".to_string());
    conf::save_conf(conf);
    let nu = get_global_rng();
    "Guess number".to_string()
}

pub fn numbinp(text: String, button: String) -> String {
    let rng = get_global_rng();
    if button == "=" {
        match text.parse::<u8>() {
            Ok(s) => if s > rng {
                return "number is lower".to_string()
            } else if s < rng {
                return "number is bigger".to_string()
            } else {
                end_silent();
                return "win".to_string()
            },
            _ =>return "not u8".to_string()
            }
    } else {
        match button.parse::<u8>() {
            Ok(s) => if text != "" {
                return format!("{}{}", text, button).to_string()
            } else {
                return button.to_string()
            },
            _ => return text.to_string()
        }
    }  
}
