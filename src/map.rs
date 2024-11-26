use gtk::prelude::*;
use std::sync::Mutex;
use std::collections::HashMap;
use std::time::Duration;

use crate::game;
use crate::legend;
use crate::save;

static mut map: Option<Mutex<gtk::Window>> = None;
static mut vis: Option<Mutex<bool>> = None;

pub fn init_map() {
    unsafe {
        let mwin = gtk::Window::builder()
            .title("Map")
            .resizable(false)
            .build();
        mwin.set_default_size(200, 200);
        let mbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        map = Some(Mutex::new(mwin));
    }
}

pub fn up_map(win: gtk::Window) {
    unsafe {
        map = Some(Mutex::new(win));
    }
}

pub fn use_map() -> gtk::Window {
    unsafe {
        map.as_ref().unwrap().lock().unwrap().clone()
    }
}

pub fn update() {
    let stats = game::get_global_stats();
    let rid = stats.get("player").unwrap().get("room").unwrap();
    let pos: Vec<usize> = stats.get("player").unwrap().get("position").unwrap().split("x").map(|r| r.parse::<usize>().unwrap()).collect();
    let room = stats.get(&format!("room{}",rid)).unwrap();
    let player = legend::statics()[3];
    let mut sus = HashMap::new();
    let mut display = room.clone();
    let pro = room.get(&pos[1].to_string()).unwrap();
    let pru: String = pro.chars().enumerate().map(|(i,r)| if i == pos[0] { player } else { r }).collect();
    display.insert(pos[1].to_string(), pru);
    sus.insert("roomcur".to_string(), display);
    let s: String = save::save(sus).split("\n").enumerate().map(|(i, r)| if i != 0 { format!("{}\n",r) } else { "".to_string() } ).collect();
    let m = use_map();
    let mbox = gtk::Box::new(gtk::Orientation::Vertical, 1);
    mbox.add_css_class("Map");
    let name = gtk::Label::builder().label("Room Map").build();
    let lab = gtk::Label::builder().label(&s).build();
    mbox.append(&name);
    mbox.append(&lab);
    m.set_child(Some(&mbox));
    up_map(m);
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
            set_vis(None);
            false
        }
    }
}

pub fn toggle_map() -> String {
    update();
    if get_vis() {
        close_map();
        set_vis(Some(false));
        "closing map".to_string()
    } else {
        open_map();
        set_vis(Some(true));
        "openning map".to_string()
    }
}

pub fn open_map() {
    use_map().show();
}

pub fn close_map() {
    use_map().hide();
}
