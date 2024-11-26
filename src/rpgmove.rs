use std::collections::HashMap;

use crate::save;
use crate::game;
use crate::legend;
use crate::map;

pub fn movevalid(pos: Vec<u8>, room: HashMap<String, String>) -> bool {
    let forw = room.get(&pos[1].to_string()).unwrap().chars().nth(pos[0].into());
    forw == Some(legend::statics()[0])
}

pub fn rpginp(text: String, button: String) -> String {
    let mut s = game::get_global_stats();
    let mut player = s.get("player").unwrap().clone();
    let roomid = player.get("room").unwrap().clone();
    let room = s.get(&format!("room{}",roomid).to_string()).unwrap().clone();
    let mut pos: Vec<u8> = player.get("position").unwrap().split("x").map(|r| r.parse::<u8>().unwrap()).collect();
    let mx: u8 = (room.get(&"0".to_string()).unwrap().len() -1).try_into().unwrap();
    let my: u8 = (room.len()-1).try_into().unwrap();
    if button == "8" || text.ends_with("8") {
        if pos[1] > 0 {
            pos[1] -= 1;
            if movevalid(pos.clone(),room.clone()) {
                return "can't move".to_string()
            }
            player.insert("position".to_string(), format!("{}x{}", pos[0], pos[1]).to_string());
            s.insert("player".to_string(), player);
            game::init_global_stats(s);
            map::update();
            return "up".to_string()
        } else {
            return "can't move".to_string()
        }
    } else if button == "2" || text.ends_with("2") {
        if pos[1] < mx {
            pos[1] += 1;
            if movevalid(pos.clone(),room.clone()) {
                return "can't move".to_string()
            }
            player.insert("position".to_string(), format!("{}x{}", pos[0], pos[1]).to_string());
            s.insert("player".to_string(), player);
            game::init_global_stats(s);
            map::update();
            return "down".to_string()
        } else {
            return "can't move".to_string();
        }
    } else if button == "4" || text.ends_with("4") {
        if pos[0] > 0 {
            pos[0] -= 1;
            if movevalid(pos.clone(),room.clone()) {
                return "can't move".to_string()
            }
            player.insert("position".to_string(), format!("{}x{}", pos[0], pos[1]).to_string());
            s.insert("player".to_string(), player);
            game::init_global_stats(s);
            map::update();
            return "left".to_string()
        } else {
            return "can't move".to_string()
        }
    } else if button == "6" || text.ends_with("6") {
        if pos[0] < my {
            pos[0] += 1;
            if movevalid(pos.clone(),room.clone()) {
                return "can't move".to_string()
            }
            player.insert("position".to_string(), format!("{}x{}", pos[0], pos[1]).to_string());
            s.insert("player".to_string(), player);
            game::init_global_stats(s);
            map::update();
            return "right".to_string()
        } else {
            return "can't move".to_string()
        }
    } else if button == "√" || text.ends_with("map") {
        return map::toggle_map()
    }
    return "".to_string()
}

