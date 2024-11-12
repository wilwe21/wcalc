use gtk::prelude::*;
use rand::Rng;
use std::sync::Mutex;

use crate::conf;

static mut gm: Option<Mutex<u8>> = None;

pub fn start() -> String {
    let mut conf = conf::get_conf();
    conf.insert("game".to_string(),true.to_string());
    conf::save_conf(conf);
    "Game Started".to_string()
}

pub fn end() -> String {
    let mut conf = conf::get_conf();
    conf.insert("game".to_string(),false.to_string());
    conf::save_conf(conf);
    "Game Ended".to_string()
}

pub fn end_silent() {
    let mut conf = conf::get_conf();
    conf.insert("game".to_string(),false.to_string());
    conf::save_conf(conf);
}

pub fn init_global_rng() {
    unsafe {
        let mut ran = rand::thread_rng();
        gm = Some(Mutex::new(ran.gen::<u8>()))
    }
}

pub fn get_global_rng() -> u8 {
    unsafe {
        gm.as_ref().unwrap().lock().unwrap().clone()
    }
}

pub fn numble() -> String {
    init_global_rng();
    let mut conf = conf::get_conf();
    conf.insert("game".to_string(),"numble".to_string());
    conf::save_conf(conf);
    let nu = get_global_rng();
    "Guess number".to_string()
}
