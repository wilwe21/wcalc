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
    let plimage = gtk::Picture::for_filename(pl.image.clone());
    plimage.set_hexpand(true);
    plimage.set_vexpand(true);
    plimage.add_css_class("player_image");
    b21.append(&plimage);
    let enimage = gtk::Picture::for_filename(en.image.clone());
    enimage.set_hexpand(true);
    enimage.set_vexpand(true);
    enimage.add_css_class("enemy_image");
    b12.append(&enimage);
    if *stat.clone().unwrap().get("turn").unwrap() == "false".to_string() 
    && *stat.clone().unwrap().get("anim").unwrap() == "attack".to_string() {
        plimage.add_css_class("attacking");
        enimage.add_css_class("dmg");
    }  
    if *stat.clone().unwrap().get("turn").unwrap() == "true".to_string()
    && *stat.clone().unwrap().get("anim").unwrap() == "attack".to_string() {
        enimage.add_css_class("attacking");
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
    let menu = Button::button_list(men.to_string());
    let op0 = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let op00 = gtk::Box::new(gtk::Orientation::Vertical, 1);
    op00.set_valign(gtk::Align::Center);
    op00.set_vexpand(true);
    let op0lab = gtk::Label::builder().label(menu[0].label.clone()).build();
    op0.set_hexpand(true);
    op0.set_vexpand(true);
    op0.append(&op00);
    op00.append(&op0lab);
    op0.add_css_class("option0");
    let op1 = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let op11 = gtk::Box::new(gtk::Orientation::Vertical, 1);
    op11.set_valign(gtk::Align::Center);
    op11.set_vexpand(true);
    let op1lab = gtk::Label::builder().label(menu[1].label.clone()).build();
    op1.set_hexpand(true);
    op1.set_vexpand(true);
    op1.append(&op11);
    op11.append(&op1lab);
    op1.add_css_class("option1");
    let op2 = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let op22 = gtk::Box::new(gtk::Orientation::Vertical, 1);
    op22.set_valign(gtk::Align::Center);
    op22.set_vexpand(true);
    let op2lab = gtk::Label::builder().label(menu[2].label.clone()).build();
    op2.set_hexpand(true);
    op2.set_vexpand(true);
    op2.append(&op22);
    op22.append(&op2lab);
    op2.add_css_class("option2");
    let op3 = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let op33 = gtk::Box::new(gtk::Orientation::Vertical, 1);
    op33.set_valign(gtk::Align::Center);
    op33.set_vexpand(true);
    let op3lab = gtk::Label::builder().label(menu[3].label.clone()).build();
    op3.set_hexpand(true);
    op3.set_vexpand(true);
    op3.append(&op33);
    op33.append(&op3lab);
    op3.add_css_class("option3");
    match stat.clone() {
        Some(s) => {
            let sel = s.get("selected").unwrap();
            match sel {
                _ if *sel == "0".to_string() => op0.add_css_class("selected"),
                _ if *sel == "1".to_string() => op1.add_css_class("selected"),
                _ if *sel == "2".to_string() => op2.add_css_class("selected"),
                _ if *sel == "3".to_string() => op3.add_css_class("selected"),
                _ => {}
            }
        },
        _ => {}
    }
    let bb1 = gtk::Box::new(gtk::Orientation::Horizontal, 1);
    let bb2 = gtk::Box::new(gtk::Orientation::Horizontal, 1);
    bb1.set_homogeneous(true);
    bb2.set_homogeneous(true);
    bb1.append(&op0);
    bb1.append(&op2);
    bb2.append(&op1);
    bb2.append(&op3);
    b32.append(&bb1);
    b32.append(&bb2);
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

