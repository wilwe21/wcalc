use std::sync::Mutex;
use std::collections::HashMap;
use rand::seq::SliceRandom;

use crate::legend;
use crate::tentity::Entity;
use crate::tattacks::Attack;
use crate::game;
use crate::map;
use crate::fui;
use crate::tbutton::Button;
use crate::bag::Item;
use crate::bag::Bag;

// make ui button as struct

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

pub fn create_status(position: Option<(u8,u8)>, selected: Option<String>, dialog: Option<String>, menu: Option<String>, turn: Option<bool>) -> HashMap<String,String> {
    let mut sta = HashMap::new();
    match position {
        Some(p) => sta.insert("position".to_string(),format!("{}x{}",p.0,p.1)),
        _ => {
            let st = get_status();
            match st {
                Some(stat) => sta.insert("position".to_string(), stat.get("position").unwrap().to_string()),
                _ => sta.insert("position".to_string(), "none".to_string())
            }
        }
    };
    match selected {
        Some(s) => sta.insert("selected".to_string(), s.to_string()),
        _ => sta.insert("selected".to_string(), "0".to_string())
    };
    match dialog {
        Some(d) => sta.insert("dialog".to_string(), d.to_string()),
        _ => sta.insert("dialog".to_string(), "None".to_string())
    };
    match menu {
        Some(m) => sta.insert("menu".to_string(), m.to_string()),
        _ => {
            let st = get_status();
            match st {
                Some(stat) => sta.insert("menu".to_string(), stat.get("menu").unwrap().to_string()),
                _ => sta.insert("menu".to_string(), "base".to_string())
            }
        }
    };
    match turn {
        Some(t) => sta.insert("turn".to_string(), t.to_string()),
        _ => sta.insert("turn".to_string(), "true".to_string())
    };
    return sta
}

pub fn start(en: Entity, pos: (u8,u8)) -> String {
    let pl = game::get_player().clone();
    set_status(Some(create_status(Some(pos), None, Some(format!("{} appeard from dark", en.character).to_string()), Some("base".to_string()), None)));
    set_enemy(Some(en.clone()));
    fui::update(pl.clone(), en.clone());
    fui::open();
    return format!("{} appeard from dark", en.character);
}

pub fn end(def: bool) {
    let mut pl = game::get_player().clone();
    if def {
        match get_status() {
            Some(st) => {
                let flo = legend::floor;
                let pos = st.get("position").unwrap().split("x").map(|r| r.parse::<usize>().unwrap()).collect::<Vec<usize>>();
                let roomid = pl.clone().room;
                let mut glst = game::get_global_stats().clone();
                let mut room = glst.get(&format!("room{}",roomid)).unwrap().clone();
                let row = room.get(&(pos[1]).to_string()).unwrap();
                let row2 = row.split("").enumerate().map(|(i,r)| if i == (pos[0]+1) { flo.to_string() } else { r.to_string() }).collect::<String>();
                room.insert((pos[1]).to_string(), row2.to_string());
                glst.insert(format!("room{}", roomid), room);
                game::init_global_stats(glst);
                map::update();
            },
            _ => {}
        };
    }
    pl.change_mode("None".to_string());
    game::update_player(pl);
    fui::close();
}

pub fn moves(text: String, button: String) -> String {
    let mut pl = game::get_player().clone();
    match get_enemy() {
        Some(_) => {},
        _ => {
            end(false);
            return "No enemy".to_string()
        }
    }
    let mut en = get_enemy().unwrap();
    if en.health == 0 {
        end(true);
        return "Enemy defeated".to_string()
    }
    let stat = get_status();
    match stat {
        Some(s) => {
            let turn = s.get("turn").unwrap().parse::<bool>().unwrap();
            if turn {
                if button == "^" || (button == "=" && text.ends_with("^")) {
                    end(false);
                    return "Escaped from battle".to_string()
                } else if button == "4" || (button == "=" && text.ends_with("4")) {
                    let stat = get_status();
                    match stat {
                        Some(s) => {
                            let sel = s.get("selected").unwrap();
                            if sel == "2" {
                                set_status(Some(create_status(None, Some("0".to_string()), None, None, None)))
                            } else if sel == "3" {
                                set_status(Some(create_status(None, Some("1".to_string()), None, None, None)))
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
                                set_status(Some(create_status(None, Some("2".to_string()), None, None, None)))
                            } else if sel == "1" {
                                set_status(Some(create_status(None, Some("3".to_string()), None, None, None)))
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
                                set_status(Some(create_status(None, Some("0".to_string()), None, None, None)))
                            } else if sel == "3" {
                                set_status(Some(create_status(None, Some("2".to_string()), None, None, None)))
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
                                set_status(Some(create_status(None, Some("1".to_string()), None, None, None)))
                            } else if sel == "2" {
                                set_status(Some(create_status(None, Some("3".to_string()), None, None, None)))
                            }
                        },
                        _ => {}
                    }
                    fui::update(pl.clone(),en.clone());
                } else if button == "5" || (button == "=" && text.ends_with("5")) {
                    let stat = get_status();
                    match stat {
                        Some(s) => {
                            let sel = s.get("selected").unwrap().parse::<usize>().unwrap();
                            let men = s.get("menu").unwrap();
                            let button = Button::get_position(men, sel);
                            if button.action.starts_with("use") {
                                let atname = button.action.replace("use ", "");
                                if atname != "" {
                                    let at = Attack::get_by_id(&atname).unwrap();
                                    let (s, t) = at.r#use(pl.clone(), en.clone());
                                    if t.health == 0 {
                                        end(true);
                                        return "Enemy defeated".to_string()
                                    }
                                    set_enemy(Some(t.clone()));
                                    set_status(Some(create_status(None, None, Some(s.clone()), Some("base".to_string()), Some(false))));
                                    fui::update(pl.clone(),t.clone());
                                    return s
                                }
                            } else if button.action.starts_with("item ") {
                                let itname = button.action.replace("item ", "");
                                let it = Item::get_by_id(&itname).unwrap();
                                let (s, w, t) = it.r#use(pl.clone(), en.clone());
                                let mut plbag = pl.clone().bag.unwrap();
                                let itid = Bag::find_index_by_item(&plbag, it.clone()).unwrap();
                                plbag.rm_item(itid, 1);
                                let mut plne = w.clone();
                                plne.change_bag(plbag);
                                game::update_player(plne.clone());
                                if t.health == 0 {
                                    end(true);
                                    return "Enemy defeated".to_string()
                                }
                                if plne.health == 0 {
                                    end(false);
                                    game::end_silent();
                                    return "You dead".to_string()
                                }
                                set_enemy(Some(t.clone()));
                                set_status(Some(create_status(None, None, Some(s.clone()), Some("base".to_string()), Some(false))));
                                fui::update(plne.clone(),t.clone());
                                return format!("used {}", it.name)
                            } else if button.action == "open_attack" {
                                set_status(Some(create_status(None, None, None, Some("Attack".to_string()), None)));
                                fui::update(pl.clone(),en.clone());
                                return "Opening attack list".to_string()
                            } else if button.action == "open_bag" {
                                set_status(Some(create_status(None, None, None, Some("Bag".to_string()), None)));
                                fui::update(pl.clone(),en.clone());
                                return "Opening bag".to_string()
                            } else if button.action == "scream" {
                                set_status(Some(create_status(None, None, Some("AAAAAAA".to_string()), None, None)));
                                fui::update(pl.clone(),en.clone());
                                return "AAAAA".to_string()
                            } else if button.action == "run" {
                                end(false);
                                return "Escaped from battle".to_string()
                            }
                        },
                        _ => {}
                    }
                } else if button == "3" || (button == "=" && text.ends_with("3")) {
                    set_status(Some(create_status(None, None, None, Some("base".to_string()), None)));
                    fui::update(pl.clone(),en.clone());
                    return "Back".to_string()
                }
            } else {
                let attacks = en.attacks.clone().into_iter().filter(|r| r != "").collect::<Vec<_>>();
                let atname = attacks.choose(&mut rand::thread_rng()).unwrap();
                let at = Attack::get_by_id(&atname).unwrap();
                let (s, t) = at.r#use(en.clone(), pl.clone());
                if t.health == 0 {
                    end(false);
                    game::end_silent();
                    return "You dead".to_string()
                }
                game::update_player(t.clone());
                set_status(Some(create_status(None, None, Some(s.clone()), Some("base".to_string()), Some(true))));
                fui::update(t.clone(),en.clone());
                return s
            }
        },
        _ => {}
    }
    return format!("player: {}hp, Enemy: {}hp", pl.health, en.health).to_string() 
}
