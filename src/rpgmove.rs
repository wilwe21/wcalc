use std::collections::HashMap;

use crate::save;
use crate::game;
use crate::legend;
use crate::map;

// fix moving when a is near a

pub fn movevalid(pos: Vec<u8>, room: HashMap<String, String>) -> Option<char> {
    let forw = room.get(&pos[1].to_string()).unwrap().chars().nth(pos[0].into());
    forw
}

pub fn validaroom(map: HashMap<String,String>, le: usize, ri: usize) -> String {
    // le = up down
    // starts with 1 becouse of stupid
    // and numering rows from 1
    // ri = left right
    // starts with 0 becouse why not
    let room = map.get(&le.clone().to_string()).unwrap().chars().enumerate().map(|(j, r)| if j == ri.clone() { r } else { '#' }).filter(|&r| r != '#').collect::<String>();
    let mut ac = 1;
    if room == "a".to_string() {
        for i in 1..le {
            if i != le {
                ac += map.get(&i.to_string()).unwrap().chars().filter(|c| *c == 'a').count()
            } else {
                ac += map.get(&i.to_string()).unwrap().chars().enumerate().filter(|(j,r)| *j > ri && r.to_string() == "a".to_string()).count()
            }
        }
        return format!("a{}",ac).to_string()
    }
    return room
}

pub fn finda(map: HashMap<String, String>, find: usize) -> Vec<usize> {
    let mut ac = 0;
    let mut fond: Vec<_> = vec!(1,0);
    for i in 1..map.len() {
        let row =  map.get(&i.to_string()).unwrap().chars();
        for (j, r) in row.enumerate(){
            if r == 'a' {
                ac += 1
            }
            if ac == find {
                fond = vec!(i,j);
                return fond
            }
        }
    }
    return fond
}

pub fn rpginp(text: String, button: String) -> String {
    let mut s = game::get_global_stats();
    let mut player = s.get("player").unwrap().clone();
    let mut roomid = player.get("room").unwrap().clone();
    let orid = roomid.clone();
    let room = s.get(&format!("room{}",roomid).to_string()).unwrap().clone();
    let mut pos: Vec<u8> = player.get("position").unwrap().split("x").map(|r| r.parse::<u8>().unwrap()).collect();
    let mx: u8 = (room.get(&"0".to_string()).unwrap().len() -1).try_into().unwrap();
    let my: u8 = (room.len()-1).try_into().unwrap();
    if button == "8" || (button == "=" && text.ends_with("8")) {
        if pos[1] > 0 {
            pos[1] -= 1;
            let nex = movevalid(pos.clone(),room.clone());
            if nex == Some(legend::statics()[0]) {
                return "can't move".to_string()
            }
            if nex == Some(legend::statics()[1]) {
                let map = s.get("map").unwrap();
                let cur = player.get("room").unwrap();
                if !(cur.starts_with("a")) {
                    for i in 1..(map.len()+1) {
                        let row = map.get(&i.to_string()).unwrap();
                        if let Some(ind) = row.find(cur) {
                            pos[1] = (map.len()-1) as u8;
                            let le = i-1;
                            let ri = ind;
                            roomid = map.get(&le.clone().to_string()).unwrap().chars().enumerate().map(|(j, r)| if j == ri.clone() { r } else { '#' }).filter(|&r| r != '#').collect::<String>();
                            roomid = validaroom(map.clone(), le.clone(), ri.clone());
                        }
                    }
                } else {
                    let num = cur.replace("a", "").parse::<usize>().unwrap(); 
                    let v: Vec<usize> = finda(map.clone(), num.clone());
                    pos[1] = (map.len()-1) as u8;
                    let le = v[0]-1;
                    let ri = v[1];
                    roomid = map.get(&le.clone().to_string()).unwrap().chars().enumerate().map(|(j, r)| if j == ri.clone() { r } else { '#' }).filter(|&r| r != '#').collect::<String>();
                    roomid = validaroom(map.clone(), le.clone(), ri.clone());
                }
            }
            player.insert("room".to_string(), roomid.to_string());
            player.insert("position".to_string(), format!("{}x{}", pos[0], pos[1]).to_string());
            s.insert("player".to_string(), player);
            game::init_global_stats(s);
            map::update();
            if roomid == orid {
                return "move up".to_string()
            } else {
                return format!("entering room {}", roomid)
            }
        } else {
            return "can't move".to_string()
        }
    } else if button == "2" || (button == "=" && text.ends_with("2")) {
        if pos[1] < mx {
            pos[1] += 1;
            let nex = movevalid(pos.clone(),room.clone());
            if nex == Some(legend::statics()[0]) {
                return "can't move".to_string()
            }
            if nex == Some(legend::statics()[1]) {
                let map = s.get("map").unwrap();
                let cur = player.get("room").unwrap();
                if !(cur.starts_with("a")) {
                    for i in 1..(map.len()+1) {
                        if let Some(ind) = map.get(&i.to_string()).unwrap().find(cur) {
                            pos[1] = 1;
                            let le = i+1;
                            let ri = ind;
                            roomid = map.get(&le.clone().to_string()).unwrap().chars().enumerate().map(|(j, r)| if j == ri.clone() { r } else { '#' }).filter(|&r| r != '#').collect::<String>();
                            roomid = validaroom(map.clone(), le.clone(), ri.clone());
                        }
                    }
                } else {
                    let num = cur.replace("a", "").parse::<usize>().unwrap(); 
                    let v: Vec<usize> = finda(map.clone(), num.clone());
                    pos[1] = 1;
                    let le = v[0]+1;
                    let ri = v[1];
                    roomid = map.get(&le.clone().to_string()).unwrap().chars().enumerate().map(|(j, r)| if j == ri.clone() { r } else { '#' }).filter(|&r| r != '#').collect::<String>();
                    roomid = validaroom(map.clone(), le.clone(), ri.clone());
                }
            }
            player.insert("room".to_string(), roomid.to_string());
            player.insert("position".to_string(), format!("{}x{}", pos[0], pos[1]).to_string());
            s.insert("player".to_string(), player);
            game::init_global_stats(s);
            map::update();
            if roomid == orid {
                return "move down".to_string()
            } else {
                return format!("entering room {}", roomid)
            }
        } else {
            return "can't move".to_string();
        }
    } else if button == "4" || (button == "=" && text.ends_with("4")) {
        if pos[0] > 0 {
            pos[0] -= 1;
            let nex = movevalid(pos.clone(),room.clone());
            if nex == Some(legend::statics()[0]) {
                return "can't move".to_string()
            }
            if nex == Some(legend::statics()[1]) {
                let map = s.get("map").unwrap();
                let cur = player.get("room").unwrap();
                if !(cur.starts_with("a")) {
                    for i in 1..(map.len()+1) {
                        if let Some(ind) = map.get(&i.to_string()).unwrap().find(cur) {
                            pos[0] = (map.get(&i.to_string()).unwrap().len()-1) as u8;
                            let le = i;
                            let ri = ind-1;
                            roomid = map.get(&le.clone().to_string()).unwrap().chars().enumerate().map(|(j, r)| if j == ri.clone() { r } else { '#' }).filter(|&r| r != '#').collect::<String>();
                            roomid = validaroom(map.clone(), le.clone(), ri.clone());
                        }
                    }
                } else {
                    let num = cur.replace("a", "").parse::<usize>().unwrap(); 
                    let v: Vec<usize> = finda(map.clone(), num.clone());
                    pos[0] = (map.get(&"1".to_string()).unwrap().len()-1) as u8;
                    let le = v[0];
                    let ri = v[1]-1;
                    roomid = map.get(&le.clone().to_string()).unwrap().chars().enumerate().map(|(j, r)| if j == ri.clone() { r } else { '#' }).filter(|&r| r != '#').collect::<String>();
                    roomid = validaroom(map.clone(), le.clone(), ri.clone());
                }
            }
            player.insert("room".to_string(), roomid.to_string());
            player.insert("position".to_string(), format!("{}x{}", pos[0], pos[1]).to_string());
            s.insert("player".to_string(), player);
            game::init_global_stats(s);
            map::update();
            if roomid == orid {
                return "move left".to_string()
            } else {
                return format!("entering room {}", roomid)
            }
        } else {
            return "can't move".to_string()
        }
    } else if button == "6" || (button == "=" && text.ends_with("6")) {
        if pos[0] < my {
            pos[0] += 1;
            let nex = movevalid(pos.clone(),room.clone());
            if nex == Some(legend::statics()[0]) {
                return "can't move".to_string()
            }
            if nex == Some(legend::statics()[1]) {
                let map = s.get("map").unwrap();
                let cur = player.get("room").unwrap();
                if !(cur.starts_with("a")) {
                    for i in 1..(map.len()+1) {
                        if let Some(ind) = map.get(&i.to_string()).unwrap().find(cur) {
                            pos[0] = 1;
                            let le = i;
                            let ri = ind+1;
                            roomid = map.get(&le.clone().to_string()).unwrap().chars().enumerate().map(|(j, r)| if j == ri.clone() { r } else { '#' }).filter(|&r| r != '#').collect::<String>();
                            roomid = validaroom(map.clone(), le.clone(), ri.clone());
                        }
                    }
                } else {
                    let num = cur.replace("a", "").parse::<usize>().unwrap(); 
                    let v: Vec<usize> = finda(map.clone(), num.clone());
                    pos[0] = 1;
                    let le = v[0];
                    let ri = v[1]+1;
                    roomid = map.get(&le.clone().to_string()).unwrap().chars().enumerate().map(|(j, r)| if j == ri.clone() { r } else { '#' }).filter(|&r| r != '#').collect::<String>();
                    roomid = validaroom(map.clone(), le.clone(), ri.clone());
                }
            }
            player.insert("room".to_string(), roomid.to_string());
            player.insert("position".to_string(), format!("{}x{}", pos[0], pos[1]).to_string());
            s.insert("player".to_string(), player);
            game::init_global_stats(s);
            map::update();
            if roomid == orid {
                return "move right".to_string()
            } else {
                return format!("entering room {}", roomid)
            }
        } else {
            return "can't move".to_string()
        }
    } else if button == "√" || text.ends_with("map") {
        return map::toggle_map()
    }
    return "".to_string()
}

