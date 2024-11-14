use std::fs;
use std::fs::File;
use dirs::config_dir;
use std::io::prelude::*;
use std::collections::HashMap;

pub fn str_to_conf(config: String) -> HashMap<String, HashMap<String, String>> {
    let mut fin = HashMap::new();
    let mut conf: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut keys: Vec<_> = config.split("[").collect();
    keys.remove(0);
    for key in keys {
        let mut hask: HashMap<String, String> = HashMap::new();
        let mut settings: Vec<_> = key.split('\n').filter(|x| x.to_string() != "").collect();
        let k = settings[0].replace("]", "");
        if k != "map" && !(k.starts_with("room")) {
            let setting: Vec<_> = key.split("\n")
                .filter(|x| x.to_string().contains("=") && !(x.to_string().contains("//")))
                .map(|x| {
                    let spl = x.split("=").collect::<Vec<_>>();
                    let xxx = vec![spl[0].to_string(),spl[1..].join("=").to_string()];
                    xxx.into_iter()
                        .map(|y| y.to_string().trim().to_owned()).collect::<Vec<_>>()
                }
                ).collect();
            for i in setting {
                hask.insert(i[0].clone(), i[1].clone());
            }
        } else if k == "map" {
            let map: Vec<_> = key.split("\n").collect();
            for i in 1..(map.len()) {
                if map[i] != "" {
                    hask.insert(i.to_string(), map[i].clone().to_string());
                }
            }
        } else if k.starts_with("room") {
            let r: Vec<_> = key.split("\n").collect();
            let id = r[1].split(" = ").collect::<Vec<_>>()[1];
            hask.insert("id".to_string(), id.to_string());
            for i in 2..(r.len()) {
                if r[i] != "" {
                    hask.insert(i.to_string(), r[i].clone().to_string());
                }
            }
        }
        if !(k.starts_with("room")) {
            fin.insert(k.to_string(), hask.clone());
        } else {
            let id = hask.get("id").unwrap().clone();
            fin.insert(format!("room{}",id).to_string(), hask.clone());
        }
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

pub fn save(config: HashMap<String, HashMap<String, String>>) -> String {
    let mut conf = String::new();
    for (k, v) in &config {
        conf += &format!("[{}]\n", k).to_string();
        if k == "map" {
            for l in 1..(v.len()+1) {
                conf += &format!("{}\n",v[&l.to_string()]).to_string();
            }
        } else if k.starts_with("room") {
            conf += &format!("id = {}\n", v.get("id").unwrap()).to_string();
            for l in 2..(v.len()+1) {
                conf += &format!("{}\n",v[&l.to_string()]).to_string();
            }
        } else {
            for (l, m) in v {
                conf += &format!("{} = {}\n", l, m).to_string();
            }
        }
    }
    conf.to_string()
}
