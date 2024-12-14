use gtk::prelude::*;
use std::sync::Mutex;

use crate::tentity::Entity;
use crate::game;

static mut charwin: Option<Mutex<gtk::Window>> = None;
static mut vis: Option<Mutex<bool>> = None;
static mut selected: Mutex<usize> = Mutex::new(0);

pub fn set_char_win(win: Option<gtk::Window>) {
    unsafe {
        match win {
            Some(wm) => charwin = Some(Mutex::new(wm)),
            _ => {
                let win = gtk::Window::builder()
                    .title("Wcalc character Chooser")
                    .resizable(false)
                    .deletable(false)
                    .build();
                win.set_default_size(500, 200);
                charwin = Some(Mutex::new(win));
            }
        }
    }
}

pub fn get_char_win() -> gtk::Window {
    unsafe {
        match charwin.as_ref() {
            Some(s) => return s.lock().unwrap().clone(),
            _ => {
                let win = gtk::Window::builder()
                    .title("Wcalc character Chooser")
                    .resizable(false)
                    .deletable(false)
                    .build();
                win.set_default_size(500, 200);
                set_char_win(Some(win.clone()));
                return win
            }
        }
    }
}

pub fn set_selected(sel: Option<usize>) {
    unsafe {
        match sel {
            Some(s) => selected = Mutex::new(s),
            _ => selected = Mutex::new(0)
        }
    }
}

pub fn get_selected() -> usize {
    unsafe {
        return selected.lock().unwrap().clone()
    }
}

pub fn update() {
    let win = get_char_win();
    let mbox = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let label = gtk::Label::new(Some(&"Select Your Character"));
    mbox.append(&label);
    let plist = Entity::players_list();
    let players_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    mbox.append(&players_box);
    let sel = get_selected();
    let mut leftlist = plist.clone().into_iter().enumerate()
        .filter(|(i,x)| *i > sel.clone() && *i < sel.clone()+3)
        .map(|(i,x)| x).collect::<Vec<_>>();
    if sel+2 > plist.clone().len() {
        leftlist.push(plist[0].clone());
        if plist.clone().len() > 1 {
            leftlist.push(plist[1].clone());
        }
    } else if sel+3 > plist.clone().len() {
        leftlist.push(plist[0].clone());
    }
    let mut rightlist = plist.clone().into_iter().enumerate()
        .filter(|(i,x)| (sel.clone() > 2 && *i < sel.clone() && *i > sel.clone()-3) ||
            (sel.clone() == 1 && *i == 0) || (sel.clone() == 2 && *i < 2))
        .map(|(i,x)| x).collect::<Vec<_>>();
    if sel == 0 {
        if plist.clone().len() > 1 {
            rightlist.push(plist[plist.clone().len()-2].clone());
        }
        rightlist.push(plist[plist.clone().len()-1].clone());
    } else if sel == 1 {
        rightlist.insert(0, plist[plist.clone().len()-1].clone());
    }
    for (i, p) in rightlist.into_iter().enumerate() {
        let li = char_box(p, false);
        players_box.append(&li);
    }
    let sel = char_box(plist[sel.clone()].clone(), true);
    players_box.append(&sel);
    for (i, p) in leftlist.into_iter().enumerate() {
        let li = char_box(p, false);
        players_box.append(&li);
    }
    win.set_child(Some(&mbox));
    set_char_win(Some(win));
}

pub fn char_box(ch: Entity,  sel: bool) -> gtk::Box {
    let bbb = gtk::Box::new(gtk::Orientation::Vertical, 4);
    bbb.set_hexpand(true);
    bbb.set_vexpand(true);
    let label = gtk::Label::new(Some(&ch.character));
    let image = gtk::Picture::for_filename(ch.image.clone());
    if sel {
        image.add_css_class("selected");
    }
    image.set_size_request(-1, 60);
    image.add_css_class("option");
    bbb.append(&label);
    bbb.append(&image);
    return bbb
}

pub fn moves(text: String, button: String) -> String {
    if !get_vis() {
        open()
    }
    let mut sel = get_selected();
    let plist = Entity::players_list();
    if button == "6" || (button == "=" && text.ends_with("6")) {
        if sel+1 > plist.len()-1 {
            sel = 0;
            set_selected(Some(0));
        } else {
            sel += 1;
            set_selected(Some(sel.clone()));
        }
    } else if button == "4" || (button == "=" && text.ends_with("4")) {
        if sel == 0 {
            sel = plist.len()-1;
            set_selected(Some(sel.clone()));
        } else {
            sel -= 1;
            set_selected(Some(sel.clone()));
        }
    } else if button == "5" || (button == "=" && text.ends_with("5")) {
        close();
        let pl = Entity::player_from_entity(plist[sel].clone(), &format!("{}x{}", game::spawn, game::spawn));
        game::init_player(pl.clone());
        game::set_mode(Some("move".to_string()));
        return format!("selected: {}", plist[sel].character.clone())
    }
    update();
    return plist[sel].character.clone().to_string()
}

pub fn set_vis(val: Option<bool>) {
    unsafe {
        vis = Some(Mutex::new(val.unwrap_or(false)))
    }
}

pub fn get_vis() -> bool {
    unsafe {
        if let Some(ref mut v) = vis {
            v.lock().unwrap().clone()
        } else {
            set_vis(Some(false));
            false
        }
    }
}

pub fn toggle() {
    if get_vis() {
        close();
        set_vis(Some(false));
    } else {
        open();
        set_vis(Some(true));
    }
}

pub fn open() {
    update();
    get_char_win().show();
}

pub fn close() {
    get_char_win().hide();
}
