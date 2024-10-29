use gtk::prelude::*;

use crate::conf;

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
