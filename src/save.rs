use std::fs;
use std::fs::File;
use dirs::config_dir;
use std::io::prelude::*;
use std::collections::HashMap;

pub fn gen() {
}

pub fn str_to_conf(config: String) -> HashMap<String, HashMap<String, String>> {
    let mut fin = HashMap::new();
    let mut conf = HashMap::new();
    let keys: Vec<_> = config.split("[").collect();
    for key in keys {
        let mut settings: Vec<_> = key.split('\n').collect();
        let k = settings[0];
        let setting: Vec<_> = key.split("\n")
            .filter(|x| x.to_string().contains("=") && !(x.to_string().contains("//")))
            .map(|x| {
                let spl = x.split("=").collect::<Vec<_>>();
                let xxx = vec![spl[0].to_string(),spl[1..].join("=").to_string()];
                xxx.into_iter()
                    .map(|y| y.to_string().trim().to_owned()).collect::<Vec<_>>()
            }
            ).collect();
        let confi = conf.clone();
        for i in setting {
            conf.insert(i[0].clone(), i[1].clone());
        }
        fin.insert(k.to_string(), confi);
    }
    return fin
}

pub fn get() {
    match config_dir() {
        Some(h) => {
            let path = format!("{}/wcalc/game.cfg", h.display());
            match File::open(&path) {
                Ok(f) => {
                    let cont = fs::read_to_string(path).expect("config file");
                    let i = str_to_conf(cont.to_string());
                },
                _ => {
                }
            }
        },
        None => println!("no home path find")
    }
}

pub fn save() {
    println!("save game state");
}
