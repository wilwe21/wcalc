use std::collections::HashMap;

use crate::game;
use crate::legend;
use crate::map;
use crate::charchoo;
use crate::change;

use crate::tentity::Entity;
use crate::tattacks::Attack;
use crate::fight;

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
    if room == "a".to_string() {
        let mut ac = 1;
        if le != 1 {
            for i in 1..(le+1) {
                if i != le {
                    let sl = map.get(&i.to_string()).unwrap().chars().filter(|c| *c == 'a').count();
                    ac += sl;
                } else {
                    let sl = map.get(&i.to_string()).unwrap().chars().enumerate()
                        .filter(|(j,r)| *j < ri)
                        .filter(|(j,r)| *r == 'a')
                        .count();
                    ac += sl;
                }
            }
        } else {
            let sl = map.get(&"1".to_string()).unwrap().chars().enumerate()
                .filter(|(j,r)| *j < ri)
                .filter(|(j,r)| *r == 'a')
                .count();
            ac += sl;
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

pub fn check_entity(nex: Option<char>, pos: (u8,u8)) -> Option<String> {
    if nex.unwrap() == legend::trap {
        let mut pl = game::get_player();
        let fl = pl.floor.unwrap().clone() + 1;
        pl.change_floor(fl);
        pl.move_room("0");
        pl.move_to(&format!("{}x{}",game::spawn, game::spawn));
        game::update_player(pl.clone());
        game::init_global_stats(game::new_map());
        map::update();
        return Some("entering new floor".to_string())
    } else {
        let p = Entity::get_by_display(&nex.unwrap().to_string());
        match p {
            Some(en) => {
                let mut pl = game::get_player().clone();
                game::set_mode(Some("fight".to_string()));
                game::update_player(pl).clone();
                return Some(fight::start(en, pos))
            },
            _ => return None
        }
    }
}

pub fn rpginp(text: String, button: String) -> String {
    match game::get_mode() {
        Some(m) => { 
            if m == "fight".to_string() {
                return fight::moves(text.clone(), button.clone())
            } else if m == "move".to_string() {
                return map_move(text.clone(), button.clone())
            } else if m.starts_with("change") {
                if m.strip_prefix("change ").unwrap().starts_with("attack") {
                    change::set_who(Some(m.strip_prefix("change attack ").unwrap().to_string()));
                    return change::moves(text.clone(), button.clone(), "Attack")
                } else {
                    return change::moves(text.clone(), button.clone(), "Bag")
                }
            }
            return "".to_string()
        },
        _ => return charchoo::moves(text.clone(), button.clone())
    };
}

pub fn map_move(text: String, button: String) -> String{
    let mut s = game::get_global_stats();
    let mut player = game::get_player();
    let mut roomid = player.room.clone().clone();
    let orid = roomid.clone();
    let room = s.get(&format!("room{}",roomid).to_string()).unwrap().clone();
    let mut pos: Vec<u8> = player.position.split("x").map(|r| r.parse::<u8>().unwrap()).collect();
    let mx: u8 = (room.get(&"0".to_string()).unwrap().len() -1).try_into().unwrap();
    let my: u8 = (room.len()-1).try_into().unwrap();
    if button == "8" || (button == "=" && text.ends_with("8")) {
        if pos[1] > 0 {
            pos[1] -= 1;
            let nex = movevalid(pos.clone(),room.clone());
            if nex == Some(legend::wall) {
                return "can't move".to_string()
            } else if nex == Some(legend::door) {
                let map = s.get("map").unwrap();
                let cur = player.room.clone();
                if !(cur.starts_with("a")) {
                    for i in 1..(map.len()+1) {
                        let row = map.get(&i.to_string()).unwrap();
                        if let Some(ind) = row.find(&cur) {
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
            } else {
                match check_entity(nex, (pos[0],pos[1])) {
                    Some(s) => return s,
                    _ => {}
                };
            }
            player.move_to(&format!("{}x{}",pos[0],pos[1]));
            player.move_room(&roomid);
            game::update_player(player);
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
            if nex == Some(legend::wall) {
                return "can't move".to_string()
            } else if nex == Some(legend::door) {
                let map = s.get("map").unwrap();
                let cur = player.room.clone();
                if !(cur.starts_with("a")) {
                    for i in 1..(map.len()+1) {
                        if let Some(ind) = map.get(&i.to_string()).unwrap().find(&cur) {
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
            } else {
                match check_entity(nex, (pos[0],pos[1])) {
                    Some(s) => return s,
                    _ => {}
                };
            }
            player.move_to(&format!("{}x{}",pos[0],pos[1]));
            player.move_room(&roomid);
            game::update_player(player);
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
            if nex == Some(legend::wall) {
                return "can't move".to_string()
            } else if nex == Some(legend::door) {
                let map = s.get("map").unwrap();
                let cur = player.room.clone();
                if !(cur.starts_with("a")) {
                    for i in 1..(map.len()+1) {
                        if let Some(ind) = map.get(&i.to_string()).unwrap().find(&cur) {
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
            } else {
                match check_entity(nex, (pos[0],pos[1])) {
                    Some(s) => return s,
                    _ => {}
                };
            }
            player.move_to(&format!("{}x{}",pos[0],pos[1]));
            player.move_room(&roomid);
            game::update_player(player);
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
            if nex == Some(legend::wall) {
                return "can't move".to_string()
            } else if nex == Some(legend::door) {
                let map = s.get("map").unwrap();
                let cur = player.room.clone();
                if !(cur.starts_with("a")) {
                    for i in 1..(map.len()+1) {
                        if let Some(ind) = map.get(&i.to_string()).unwrap().find(&cur) {
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
            } else {
                match check_entity(nex, (pos[0],pos[1])) {
                    Some(s) => return s,
                    _ => {}
                };
            }
            player.move_to(&format!("{}x{}",pos[0],pos[1]));
            player.move_room(&roomid);
            game::update_player(player);
            map::update();
            if roomid == orid {
                return "move right".to_string()
            } else {
                return format!("entering room {}", roomid)
            }
        } else {
            return "can't move".to_string()
        }
    } else if button == "√" || (button == "=" && text.ends_with("map")) {
        return map::toggle_map()
    }
    return "".to_string()
}
