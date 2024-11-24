use gtk::prelude::*;

use rand::Rng;
use std::sync::Mutex;
use std::collections::HashMap;

use crate::conf;
use crate::save;
use crate::legend;

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
    for i in 0..(size) {
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

pub fn generate_room(size: usize, dors: String, room_id: String) -> String {
    let mut room = String::new();
    let st = legend::statics();
    let wall = st[0].clone();
    let door = st[1].clone();
    let floor = st[2].clone();
    for i in 0..(size) {
        if i > 0 && i < (size-1) {
            let s2: usize = size - 2;
            if i == size / 2 {
                let mut left = String::new();
                let mut right = String::new();
                if dors.chars().collect::<Vec<_>>()[3] == '1' {
                    left = door.clone().to_string()
                } else {
                    left = wall.clone().to_string()
                }
                if dors.chars().collect::<Vec<_>>()[2] == '1'{
                    right = door.clone().to_string()
                } else {
                    right = wall.clone().to_string()
                }
                room += &format!("{}{}{}\n", left,floor.clone().to_string().repeat(s2),right).to_string();
                continue
            }
            room += &format!("{}{}{}\n",wall.clone(),floor.clone().to_string().repeat(s2),wall.clone()).to_string();
        } else {
            let w = (size - 1) / 2;
            if i == 0 && dors.chars().collect::<Vec<_>>()[0] == '1' {
                room += &format!("{}{}{}\n", wall.clone().to_string().repeat(w),door.clone(),wall.clone().to_string().repeat(w)).to_string();
                continue
            } 
            if i == (size-1) && dors.chars().collect::<Vec<_>>()[1] == '1' {
                room += &format!("{}{}{}\n", wall.clone().to_string().repeat(w),door.clone(),wall.clone().to_string().repeat(w)).to_string();
                continue
            }
            room += &format!("{}\n",wall.clone().to_string().repeat(size)).to_string();
        }
    }
    room.to_string()
}

pub fn check_doors(map: HashMap<String, String>, y: usize, x: usize) -> String{
    let mut vec = map.into_iter().collect::<Vec<_>>();
    vec.sort_by(|x,y| x.0.cmp(&y.0));
    let lines = vec.into_iter().map(|r| r.1).collect::<Vec<_>>();
    let vecvec = lines.iter()
        .map(|z| z.split("").filter(|&r| r != "").collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let checkx = |vecv: Vec<Vec<&str>>, x: usize, y: usize| -> u8 {
        let mut dor: u8 = 0;
        if x != 0 && vecv[y][x-1] != "#" {
            dor += 1
        }
        if x != (vecv[0].len()-1) && vecv[y][x+1] != "#" {
            dor += 2
        }
        return dor
    };
    let checky = |vecv: Vec<Vec<&str>>, x: usize, y: usize| -> u8{
        let mut dor: u8 = 0;
        if y != (vecv.len()-1) && vecvec[y+1][x] != "#" {
            dor += 4
        }
        if y != 0 && vecvec[y-1][x] != "#" {
            dor += 8
        }
        return dor
    };
    let lr = checkx(vecvec.clone(), y.clone(), x.clone());
    let ud = checky(vecvec.clone(), y.clone(), x.clone());
    return format!("{:<04b}", (lr+ud)).to_string()
}

pub fn generate_rooms(map: HashMap<String, String>) -> HashMap<String, HashMap<String, String>>{
    let mut acount = 0;
    let mut rooms: HashMap<String, HashMap<String, String>> = HashMap::new();
    for i in 1..(map.len()+1) {
        let line = map.get(&i.to_string()).unwrap().clone();
        let livec: Vec<_> = line.split("").filter(|&x| x != "").collect::<Vec<_>>();
        for z in &livec {
            let ind = livec.iter().position(|&r| r == z.clone()).unwrap();
            if *z == "a" {
                acount += 1;
                let mut r: HashMap<String,String> = HashMap::new();
                let dr = check_doors(map.clone(), (ind).clone(), (i-1).clone());
                let room = generate_room(7, dr,format!("a{}",acount).to_string());
                for (c,d) in room.split("\n").enumerate() {
                    if d != "" {
                        r.insert(c.to_string(), d.to_string());
                    }
                }
                rooms.insert(format!("rooma{}",acount).to_string(), r);
            } else if *z != "#"{
                let mut r: HashMap<String,String> = HashMap::new();
                let dr = check_doors(map.clone(), (ind).clone(), (i-1).clone());
                let room = generate_room(7, dr, z.to_string());
                for (c,d) in room.split("\n").enumerate() {
                    if d != "" {
                        r.insert(c.to_string(), d.to_string());
                    }
                }
                rooms.insert(format!("room{}",z).to_string(), r);
            }
        }
    }
    return rooms
}

pub fn start() -> String {
    let mut conf = conf::get_conf();
    conf.insert("game".to_string(),"rpg".to_string());
    conf::save_conf(conf);
    init_global_stats(new_conf());
    let mut s = get_global_stats();
    let rooms = generate_rooms(s.get("map").unwrap().clone());
    s.extend(rooms);
    init_global_stats(s);
    "".to_string()
}

pub fn end() -> String {
    let mut conf = conf::get_conf();
    conf.insert("game".to_string(),false.to_string());
    conf::save_conf(conf);
    "Game Ended".to_string()
}

pub fn end_silent() {
    let mut conf = conf::get_conf();
    conf.insert("game".to_string(),false.to_string());
    conf::save_conf(conf);
}

pub fn rpginp(text: String, button: String) -> String {
    if button == "8" || text.ends_with("8") {
        return "poruszasz się do przodu".to_string()
    } else if button == "2" || text.ends_with("2") {
        return "poruszasz się do tyłu".to_string()
    } else if button == "4" || text.ends_with("4") {
        return "poruszasz się w lewo".to_string()
    } else if button == "6" || text.ends_with("6") {
        return "poruszasz się w prawo".to_string()
    } else if button == "√" || text.ends_with("map") {
        return "otwierasz mape".to_string()
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
