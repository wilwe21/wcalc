use gtk::prelude::*;

use crate::conf;

pub fn start() -> String {
    let mut conf = conf::get_conf();
    conf.insert("game".to_string(),true.to_string());
    conf::save_conf(conf);
    "chuj".to_string()
}

pub fn end() -> String {
    "chuj".to_string()
}
