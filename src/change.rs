use gtk::prelude::*;
use std::sync::Mutex;

use crate::tbutton::Button;

static mut chwin: Option<Mutex<gtk::Window>> = None;
static mut vis: Option<Mutex<bool>> = None;
static mut selected: Mutex<usize> = Mutex::new(0);

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
    let label = gtk::Label::new(Some(&"test"));
    let ymenu = Button::r#box(typ, &sel.to_string());
    bu.append(&label);
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
    get_ch_win().show();
}

pub fn close() {
    get_ch_win().hide();
}
