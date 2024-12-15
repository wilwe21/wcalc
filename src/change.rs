use gtk::prelude::*;
use std::sync::Mutex;

use crate::tbutton::Button;
use crate::tattacks::Attack;
use crate::bag::Item;

static mut chwin: Option<Mutex<gtk::Window>> = None;
static mut vis: Mutex<bool> = Mutex::new(false);
static mut selected: Mutex<usize> = Mutex::new(0);
static mut who: Option<Mutex<String>> = None;

pub fn set_ch_win(win: Option<gtk::Window>) {
    unsafe {
        match win {
            Some(wm) => chwin = Some(Mutex::new(wm)),
            _ => {
                let win = gtk::Window::builder()
                    .title("Wcalc character Chooser")
                    .resizable(false)
                    .deletable(false)
                    .build();
                win.set_default_size(500, 200);
                chwin = Some(Mutex::new(win));
            }
        }
    }
}

pub fn get_ch_win() -> gtk::Window {
    unsafe {
        match chwin.as_ref() {
            Some(s) => return s.lock().unwrap().clone(),
            _ => {
                let win = gtk::Window::builder()
                    .title("Wcalc character Chooser")
                    .resizable(false)
                    .deletable(false)
                    .build();
                win.set_default_size(500, 200);
                set_ch_win(Some(win.clone()));
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

pub fn set_who(ho: Option<String>) {
    unsafe {
        match ho {
            Some(h) => who = Some(Mutex::new(h)),
            _ => who = None
        }
    }
}

pub fn get_who() -> String {
    unsafe {
        match who.as_ref() {
            Some(w) => w.lock().unwrap().clone(),
            _ => "None".to_string()
        }
    }
}

pub fn update(typ: &str) {
    let win = get_ch_win();
    let b = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let bu = gtk::Box::new(gtk::Orientation::Horizontal, 1);
    let bd = gtk::Box::new(gtk::Orientation::Horizontal, 1);
    bu.set_hexpand(true);
    bu.set_vexpand(true);
    bd.set_hexpand(true);
    bd.set_vexpand(true);
    let sel = get_selected();
    let mut name = String::new();
    let mut desc = String::new();
    if typ == "Attack" {
        let at = Attack::get_by_id(&get_who()).unwrap();
        name = at.name;
        desc = at.desc;
    } else if typ == "Bag" {
        let it = Item::get_by_id(&get_who()).unwrap();
        name = it.name;
        desc = it.desc;
    };
    let wn = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let wnl = gtk::Label::new(Some(&name));
    wn.add_css_class("option");
    wn.set_valign(gtk::Align::Center);
    wn.append(&wnl);
    wn.set_hexpand(true);
    wn.set_vexpand(true);
    let wd = gtk::Label::builder()
        .label(&*desc)
        .hexpand(true)
        .vexpand(true)
        .valign(gtk::Align::Start)
        .justify(gtk::Justification::Center)
        .width_chars(20)
        .wrap(true)
        .wrap_mode(gtk::pango::WrapMode::WordChar)
        .build();
    let ymenu = Button::r#box(typ.clone(), &sel.to_string());
    ymenu.add_css_class("box32");
    let pos = Button::get_position(&typ, sel);
    let d = &pos.desc;
    let dilab = gtk::Label::builder()
        .label(&*d)
        .hexpand(true)
        .vexpand(true)
        .valign(gtk::Align::Start)
        .justify(gtk::Justification::Center)
        .width_chars(20)
        .wrap(true)
        .wrap_mode(gtk::pango::WrapMode::WordChar)
        .build();
    let ubox = gtk::Box::new(gtk::Orientation::Horizontal, 1);
    ubox.append(&wd);
    ubox.append(&wn);
    bd.add_css_class("box3");
    bu.add_css_class("upperbox");
    bd.append(&dilab);
    bu.append(&ubox);
    bd.append(&ymenu);
    b.append(&bu);
    b.append(&bd);
    win.set_child(Some(&b));
    set_ch_win(Some(win));
}

pub fn moves(text: String, button: String, typ: &str) -> String {
    if !get_vis() {
        open()
    }
    let mut sel = get_selected();
    if button == "6" || (button == "=" && text.ends_with("6")) {
        if sel == 0 {
            set_selected(Some(2));
        } else if sel == 1 {
            set_selected(Some(3));
        }
    } else if button == "4" || (button == "=" && text.ends_with("4")) {
        if sel == 2 {
            set_selected(Some(0));
        } else if sel == 3 {
            set_selected(Some(1));
        }
    } else if button == "2" || (button == "=" && text.ends_with("2")) {
        if sel == 0 {
            set_selected(Some(1));
        } else if sel == 2 {
            set_selected(Some(3));
        }
    } else if button == "8" || (button == "=" && text.ends_with("8")) {
        if sel == 1 {
            set_selected(Some(0));
        } else if sel == 3 {
            set_selected(Some(2));
        }
    }
    update(typ);
    return "test".to_string()
}

pub fn get_selected() -> usize {
    unsafe {
        return selected.lock().unwrap().clone()
    }
}

pub fn set_vis(val: bool) {
    unsafe {
        vis = Mutex::new(val)
    }
}

pub fn get_vis() -> bool {
    unsafe {
        vis.lock().unwrap().clone()
    }
}

pub fn toggle() {
    if get_vis() {
        close();
        set_vis(false);
    } else {
        open();
        set_vis(true);
    }
}

pub fn open() {
    get_ch_win().show();
}

pub fn close() {
    get_ch_win().hide();
}
