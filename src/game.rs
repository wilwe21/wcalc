use rand::Rng;
use std::sync::Mutex;
use std::collections::HashMap;

use crate::tplayer::Player;
use crate::conf;
use crate::save;
use crate::generate;
use crate::map;

static mut gm: Option<Mutex<u8>> = None;

static mut stats: Option<Mutex<HashMap<String, HashMap<String, String>>>> = None;

static mut player: Option<Mutex<Player>> = None;

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
            let d = new_conf(6,7);
            init_global_stats(d.clone());
            d
        }
    }
}

pub fn init_player(c: String, pos: String, room: String) {
    unsafe {
        player = Some(Mutex::new(Player::new(c,pos,room)));
    }
}

pub fn update_player(pl: Player) {
    unsafe {
        player = Some(Mutex::new(pl));
    }
}

pub fn get_player() -> Player {
    unsafe {
        if let Some(ref mut p) = player {
            p.lock().unwrap().clone()
        } else {
            let pp = Player::new("0".to_string(), "5x5".to_string(), "0".to_string());
            init_player("0".to_string(), "5x5".to_string(), "0".to_string());
            pp
        }
    }
}

pub fn new_conf(mw: usize, rw: usize) -> HashMap<String, HashMap<String, String>> {
    let spawn = rw/2;
    init_player("0".to_string(), format!("{}x{}", spawn,spawn).to_string(), "0".to_string());
    let mut p = get_player();
    let mut d = p.to_string();
    d += &"[map]\n".to_string();
    d += &generate::generate_map(mw).to_string();
    let mut s = save::str_to_conf(d.to_string());
    let rooms = generate::generate_rooms(s.get("map").unwrap().clone(), rw.clone());
    s.get_mut("player").unwrap().insert("position".to_string(), format!("{}x{}", (rw)/2, (rw)/2));
    s.extend(rooms);
    map::init_map();
    s
}

pub fn start() -> String {
    let mut conf = conf::get_conf();
    conf.insert("game".to_string(),"rpg".to_string());
    conf::save_conf(conf);
    let rw: usize = 7;
    let mw: usize = 6;
    init_global_stats(new_conf(mw, rw));
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
