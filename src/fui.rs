use gtk::prelude::*;
use std::sync::Mutex;

use crate::tentity::Entity;

static mut fightwin: Option<Mutex<gtk::Window>> = None;
static mut vis: Option<Mutex<bool>> = None;

pub fn set_fw(win: Option<gtk::Window>) {
    unsafe {
        match win {
            Some(wn) => fightwin = Some(Mutex::new(wn)),
            _ => {
                let win = gtk::Window::builder()
                    .title("Wcalc Fight")
                    .resizable(false)
                    .build();
                win.set_default_size(200, 200);
                fightwin = Some(Mutex::new(win));
            }
        }
    }   
}

pub fn get_fw() -> gtk::Window {
    unsafe {
        match fightwin.as_ref() {
            Some(s) => return s.lock().unwrap().clone(),
            _ => { 
                let win = gtk::Window::builder()
                    .title("Wcalc Fight")
                    .resizable(false)
                    .build();
                win.set_default_size(200, 200);
                set_fw(None);
                return win
            }
        }
    }
}

pub fn update(pl: Entity, en: Entity) {
    let win = get_fw();
    let b = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let play = gtk::Label::builder().label(format!("{}: {}hp", pl.character, pl.health)).build();
    let enemigo = gtk::Label::builder().label(format!("{}: {}hp", en.character, en.health)).build();
    b.append(&play);
    b.append(&enemigo);
    win.set_child(Some(&b));
    set_fw(Some(win));
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
    get_fw().show();
}

pub fn close() {
    get_fw().hide();
}

