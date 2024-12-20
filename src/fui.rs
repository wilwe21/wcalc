use gtk::prelude::*;
use std::sync::Mutex;

use crate::conf;

use crate::tentity::Entity;
use crate::fight;
use crate::tbutton::Button;

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
                    .deletable(false)
                    .build();
                win.set_default_size(400, 400);
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
                set_fw(Some(win.clone()));
                return win
            }
        }
    }
}

pub fn update(pl: Entity, en: Entity) {
    let win = get_fw();
    let stat = fight::get_status();
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
    b3.set_size_request(110, 110);
    b1.add_css_class("box1");
    b2.add_css_class("box2");
    b3.add_css_class("box3");
    b11.add_css_class("box11");
    b21.add_css_class("box21");
    b31.add_css_class("box31");
    b11.set_halign(gtk::Align::Start);
    b21.set_halign(gtk::Align::Start);
    b31.set_halign(gtk::Align::Start);
    b22.set_halign(gtk::Align::End);
    b32.set_halign(gtk::Align::End);
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
    let plimgbox = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let plimage = gtk::Picture::for_filename(pl.image.clone());
    plimage.add_css_class("player_image");
    plimgbox.add_css_class("imgbox");
    plimage.set_size_request(50, 50);
    plimage.set_hexpand(true);
    plimage.set_vexpand(true);
    plimgbox.append(&plimage);
    b21.append(&plimgbox);
    let enimgbox = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let enimage = gtk::Picture::for_filename(en.image.clone());
    enimage.add_css_class("enemy_image");
    enimgbox.add_css_class("imgbox");
    enimage.set_size_request(50, 50);
    enimage.set_hexpand(true);
    enimage.set_vexpand(true);
    enimgbox.append(&enimage);
    b12.append(&enimgbox);
    if *stat.clone().unwrap().get("turn").unwrap() == "false".to_string() 
    && *stat.clone().unwrap().get("anim").unwrap() == "attack".to_string() {
        plimage.add_css_class("attacking_p");
        enimage.add_css_class("dmg");
    }  
    if *stat.clone().unwrap().get("turn").unwrap() == "true".to_string()
    && *stat.clone().unwrap().get("anim").unwrap() == "attack".to_string() {
        enimage.add_css_class("attacking_e");
        plimage.add_css_class("dmg");
    }
    let mut men = "base".to_string();
    match stat.clone() {
        Some(sta) => {
            let dial = sta.get("dialog").unwrap();
            men = sta.get("menu").unwrap().to_string();
            if dial != "None" {
                let dilab = gtk::Label::builder()
                    .label(&*dial)
                    .hexpand(true)
                    .vexpand(true)
                    .valign(gtk::Align::Start)
                    .justify(gtk::Justification::Center)
                    .width_chars(20)
                    .wrap(true)
                    .wrap_mode(gtk::pango::WrapMode::WordChar)
                    .build();
                b31.append(&dilab);
            } else {
                let sel = sta.get("selected").unwrap().parse::<usize>().unwrap();
                let list = Button::get_position(&men, sel);
                let d = &list.desc;
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
                b31.append(&dilab);
            }
        },
        _ => {}
    }
    let mut sel = "0".to_string();
    match stat.clone() {
        Some(s) => sel = s.get("selected").unwrap().to_string(),
        _ => {}
    };
    let menu = Button::r#box(&men.to_string(),&sel);
    b32.append(&menu);
    let plbox = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let plname = gtk::Label::builder().label(format!("{}", pl.character)).build();
    plname.set_halign(gtk::Align::Start);
    let plfrac = (pl.health.clone() as f64)/(pl.maxhealth.clone() as f64);
    let plprogress = gtk::ProgressBar::builder()
        .fraction(plfrac.clone())
        .build();
    plprogress.add_css_class("playerprogress");
    if plfrac.clone() <= 0.25 {
        plprogress.add_css_class("low");
    } else if plfrac.clone() <= 0.5 {
        plprogress.add_css_class("mid");
    }
    let plhp = gtk::Label::builder().label(format!("{}/{}hp", pl.health, pl.maxhealth)).build();
    plhp.set_halign(gtk::Align::End);
    plbox.add_css_class("playerbox");
    plbox.append(&plname);
    plbox.append(&plprogress);
    plbox.append(&plhp);
    let enbox = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let enemigoname = gtk::Label::builder().label(format!("{}", en.character)).build();
    enemigoname.set_halign(gtk::Align::Start);
    let enfrac = (en.health.clone() as f64)/(en.maxhealth.clone() as f64);
    let enemigoprogress = gtk::ProgressBar::builder()
        .fraction(enfrac.clone())
        .build();
    enemigoprogress.add_css_class("enemyprogress");
    if enfrac.clone() <= 0.25 {
        enemigoprogress.add_css_class("low");
    } else if enfrac.clone() <= 0.5 {
        enemigoprogress.add_css_class("mid");
    }
    let enemigohp = gtk::Label::builder().label(format!("{}/{}hp", en.health, en.maxhealth)).build();
    enemigohp.set_halign(gtk::Align::End);
    enbox.set_width_request(150);
    plbox.set_width_request(150);
    enbox.add_css_class("enemybox");
    enbox.append(&enemigoname);
    enbox.append(&enemigoprogress);
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

