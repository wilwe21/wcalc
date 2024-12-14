use gtk::prelude::*;
use std::sync::Mutex;

static mut charwin: Option<Mutex<gtk::Window>> = None;
static mut vis: Option<Mutex<bool>> = None;

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
                win.set_default_size(100, 500);
                set_char_win(None);
                return win
            }
        }
    }
}

pub fn moves(text: String, button: String) -> String {
    return button
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
    get_char_win().show();
}

pub fn close() {
    get_char_win().hide();
}
