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
                win.set_default_size(400, 400);
                set_fw(None);
                return win
            }
        }
    }
}

pub fn update(pl: Entity, en: Entity) {
    let win = get_fw();
    let b = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let b1 = gtk::Box::new(gtk::Orientation::Horizontal, 1);
    let b11 = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let b12 = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let b2 = gtk::Box::new(gtk::Orientation::Horizontal, 1);
    let b21 = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let b22 = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let b3 = gtk::Box::new(gtk::Orientation::Horizontal, 1);
    let b31 = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let b32 = gtk::Box::new(gtk::Orientation::Vertical, 1);
    b1.add_css_class("box1");
    b2.add_css_class("box2");
    b3.add_css_class("box3");
    b11.add_css_class("box11");
    b21.add_css_class("box21");
    b31.add_css_class("box31");
    b11.set_halign(gtk::Align::Start);
    b21.set_halign(gtk::Align::Start);
    b31.set_halign(gtk::Align::Start);
    b12.set_halign(gtk::Align::End);
    b22.set_halign(gtk::Align::End);
    b32.set_halign(gtk::Align::End);
    b21.set_valign(gtk::Align::End);
    b22.set_valign(gtk::Align::End);
    b12.add_css_class("box12");
    b22.add_css_class("box22");
    b32.add_css_class("box32");
    b.add_css_class("fightbox");
    b.set_hexpand(true);
    b1.set_vexpand(true);
    b2.set_vexpand(true);
    b3.set_vexpand(true);
    b11.set_hexpand(true);
    b21.set_hexpand(true);
    b31.set_hexpand(true);
    b12.set_hexpand(true);
    b22.set_hexpand(true);
    b32.set_hexpand(true);
    let plbox = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let plname = gtk::Label::builder().label(format!("{}", pl.character)).build();
    let plhp = gtk::Label::builder().label(format!("{}hp", pl.health)).build();
    plbox.add_css_class("playerbox");
    plbox.append(&plname);
    plbox.append(&plhp);
    let enbox = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let enemigoname = gtk::Label::builder().label(format!("{}", en.character)).build();
    let enemigohp = gtk::Label::builder().label(format!("{}hp", en.health)).build();
    enbox.set_width_request(150);
    plbox.set_width_request(150);
    enbox.add_css_class("enemybox");
    enbox.append(&enemigoname);
    enbox.append(&enemigohp);
    b.append(&b1);
    b.append(&b2);
    b.append(&b3);
    b1.append(&b11);
    b1.append(&b12);
    b2.append(&b21);
    b2.append(&b22);
    b3.append(&b31);
    b3.append(&b32);
    b11.append(&enbox);
    b22.append(&plbox);
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

