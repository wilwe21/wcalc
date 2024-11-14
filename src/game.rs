use gtk::prelude::*;

use rand::Rng;
use std::sync::Mutex;
use std::collections::HashMap;

use crate::conf;
use crate::save;

static mut gm: Option<Mutex<u8>> = None;

static mut stats: Option<Mutex<HashMap<String, HashMap<String, String>>>> = None;

pub fn init_global_stats(conf: HashMap<String, HashMap<String, String>>) {
    unsafe {
        stats = Some(Mutex::new(conf))
    }
}

pub fn get_global_stats() -> HashMap<String, HashMap<String, String>> {
    unsafe {
        stats.as_ref().unwrap().lock().unwrap().clone()
    }
}

pub fn new_conf() -> HashMap<String, HashMap<String, String>> {
    let mut d = include_str!("./stats.cfg").to_string();
    d += &"[map]\n".to_string();
    d += &generate_map(6).to_string();
    save::str_to_conf(d.to_string())
}

pub fn generate_map(size: usize) -> String {
    let mut ran = rand::thread_rng();
    let mut map = String::new();
    for i in 0..(size+1) {
        map += &format!("{:#^size$}\n","#").to_string();
    }
    let mut map2 = map.split("\n").filter(|&x| x != "")
        .map(|x| x.split("").filter(|&xx| xx != "").map(|xxx| xxx.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();
    map2[0][0] = "0".to_string();
    let mut cur = vec![0,0];
    if size-1 <= 2 {
        return map2.into_iter().map(|x| x.join("")).collect::<Vec<String>>().join("\n")
    } else {
        for i in 1..10 {
            let num = ran.gen_range(0..2);
            if num == 0 && !(cur[0]+1 > size-1) {
                if cur[1] != 0 && cur[0] != 0 {
                    let ar = ran.gen_range(0..2);
                    if ar == 0 {
                        if map2[cur[0]-1][cur[1]-1] == "#" && map2[cur[0]][cur[1]-1] == "#" {
                            map2[cur[0]][cur[1]-1] = "a".to_string();
                        } 
                    } else if ar == 1 {
                        if map2[cur[0]-1][cur[1]] == "#" && map2[cur[0]-1][cur[1]-1] == "#" {
                            map2[cur[0]-1][cur[1]] = "a".to_string();
                        } 
                    }
                }
                cur[0] += 1;
                map2[cur[0]][cur[1]] = format!("{}",i);
            } else if !(cur[1]+1 > size-1)  {
                if cur[1] != 0 && cur[0] != 0 {
                    let ar = ran.gen_range(0..3);
                    if ar == 0 {
                        if map2[cur[0]][cur[1]-1] == "#" {
                            map2[cur[0]][cur[1]-1] = "a".to_string();
                        } 
                    } else if ar == 1 {
                        if map2[cur[0]-1][cur[1]] == "#" {
                            map2[cur[0]-1][cur[1]] = "a".to_string();
                        } 
                    }
                }
                cur[1] += 1;
                map2[cur[0]][cur[1]] = format!("{}",i);
            }
        }
    }
    return map2.into_iter().map(|x| x.join("")).collect::<Vec<String>>().join("\n")
}

pub fn generate_room(size: usize, room_id: String) -> String {
    let mut room = String::new();
    for i in 0..(size+1) {
        room += &format!("{:#^size$}\n","#").to_string();
    }
    room.to_string()
}

pub fn start() -> String {
    /*let mut conf = conf::get_conf();
    conf.insert("game".to_string(),"rpg".to_string());
    conf::save_conf(conf);*/
    init_global_stats(new_conf());
    let mut s = save::save(get_global_stats());
    s += "[room0]\nid = 0\n";
    s += &generate_room(6, "0".to_string());
    s += "[room1]\nid = 1\n";
    s += &generate_room(6, "1".to_string());
    s += "[room2]\nid = 2\n";
    s += &generate_room(6, "2".to_string());
    init_global_stats(save::str_to_conf(s));
    end()
}

pub fn end() -> String {
    let mut conf = conf::get_conf();
    conf.insert("game".to_string(),false.to_string());
    conf::save_conf(conf);
    println!("{:?}", get_global_stats());
    "Game Ended".to_string()
}

pub fn end_silent() {
    let mut conf = conf::get_conf();
    conf.insert("game".to_string(),false.to_string());
    conf::save_conf(conf);
}

pub fn rpginp(text: String, button: String) -> String {
    if button == "8" {
        return "poruszasz się do przodu".to_string()
    } else if button == "2" {
        return "poruszasz się do tyłu".to_string()
    } else if button == "4" {
        return "poruszasz się w lewo".to_string()
    } else if button == "6" {
        return "poruszasz się w prawo".to_string()
    }
    return "".to_string()
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
