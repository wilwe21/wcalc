#![windows_subsystem = "windows"]
use gtk::prelude::*;
use gtk::gdk;

use sysinfo::System;

use std::fs;
use std::fs::File;
use dirs::config_dir;
use std::io::prelude::*;
use std::collections::HashMap;

pub fn def_conf() -> HashMap <String, String> {
    let mut def_conf: HashMap<String, String> = HashMap::new();
    def_conf.insert("theme".to_string(), "default".to_string());
    def_conf
}

pub fn init_conf() -> HashMap <String, String> {
    match config_dir(){
        Some(h) => {
            let path = format!("{}/wcalc/config.cfg", h.display());
            match fs::create_dir(format!("{}/wcalc", h.display())) {
                Ok(()) => println!("create"),
                _ => println!("some error")
            };
            match fs::create_dir(format!("{}/wcalc/css", h.display())) {
                Ok(()) => println!("Created path"),
                _ => println!("Can't create css path")
            }
            let mut cs = String::new();
            if System::name().unwrap() == "Windows" {
                cs = ".entry {
	margin: 2px;
	border: 2px solid blue;
}
button {
	margin: 2px;
	border: 2px solid blue;
}
box {
	background-color: @bg_color;
	color: @fg_color;
}
.botbut {
	margin-right: 5px;
}
".to_string();
            } else {
                cs = ".entry {
	margin: 2px;
	border: 2px solid @accent_bg_color;
}
button {
	margin: 2px;
	border: 2px solid @accent_bg_color;
}
box {
	background-color: @bg_color;
	color: @fg_color;
}
.botbut {
	margin-right: 5px;
}
.enter {
	color: lime;
}
".to_string();
            }
            let mut fi = File::create(format!("{}/wcalc/css/default.css", h.display())).expect("can't create file");
            fi.write(cs.as_bytes()).expect("can't create file");
            let mut f = File::create(path).expect("can't create file");
            let cont = format!("//themes {}/wcalc/css
theme = default
placeholder = 21"
, h.display());
            f.write(cont.as_bytes()).expect("can't write");
            def_conf()
        },
        None => {
            println!("No config path find");
            def_conf()
        }
    }
}

pub fn get_conf() -> HashMap<String, String> {
    match config_dir() {
        Some(h) => {
            let path = format!("{}/wcalc/config.cfg", h.display());
            match File::open(&path) {
                Ok(f) => {
                    let cont = fs::read_to_string(path).expect("config file");
                    let mut conf = HashMap::new();
                    let settings: Vec<_> = cont.split('\n').filter(|x| x.to_string().contains("=") && !(x.to_string().contains("//")))
                        .map(|x| {
                            let spl = x.split("=").collect::<Vec<_>>();
                            let xxx = vec![spl[0].to_string(),spl[1..].join("=").to_string()];
                            xxx.into_iter()
                                .map(|y| y.to_string().trim().to_owned()).collect::<Vec<_>>()
                        }
                        ).collect();
                    println!("{:?}", settings);
                    for i in settings {
                        conf.insert(i[0].clone(), i[1].clone());
                    }
                    return conf
                },
                _ => {
                    let s = init_conf();
                    return s
                }
            }
        },
        None => println!("no home path find")
    }
    def_conf()
}

pub fn save_conf(conf: HashMap <&'static str, String>) {
    match config_dir() {
        Some(h) => {
            let path = format!("{}/wcalc/config.cfg", h.display());
            match File::options().read(false).write(true).open(&path) {
                Ok(_) => {
                    let co = fs::read_to_string(path.clone()).expect("file");
                    let mut co = co.split("\n").collect::<Vec<_>>().iter()
                        .map(|x| x.to_string()).collect::<Vec<String>>();
                    for (key, value) in conf.clone().into_iter() {
                        let cci = co.clone();
                        let ind = cci.iter().enumerate()
                            .find(|(_, r)| r.to_string().split("=").collect::<Vec<_>>()[0].contains(key) && !(r.to_string().contains("//"))).map(|(i, _)| i);
                        match ind {
                            Some(i) => {
                                let form: String = format!("{} = {}", key, value).to_string();
                                co.remove(i);
                                co.insert(i,form);
                            }, _ => {
                                co.push(format!("{} = {}", key, value))
                            }
                        }
                    }
                    println!("{:?}",co);
                    println!("{}", co.join("\n"));
                    let mut f = File::create(&path).expect("file path");
                    f.write_all(co.join("\n").as_bytes());
                },
                _ => println!("sus")
            }
        },
        _ => println!("can't save becouse you don't have home")
    }
}

pub fn config() {
    let home = config_dir().expect("Home");
    let css_path = format!("{}/wcalc/css", home.display());
    let mut themes: Vec<String> = vec![];
    if let Ok(i) = fs::read_dir(css_path) {
        for j in i {
            if let Ok(s) = j {
                let file_name = s.file_name();
                let name = file_name.to_string_lossy().to_string();
                themes.push(name.strip_suffix(".css").expect("").to_string());
            }
        }
    }
    themes.sort();
    let con = gtk::Window::builder()
        .height_request(200)
        .width_request(300)
        .resizable(false)
        .title("Wcalc Config")
        .build();
    let mb = gtk::CenterBox::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    con.set_child(Some(&mb));
    let save = gtk::Button::builder()
        .label("Save")
        .hexpand(true)
        .build();
    let cancel = gtk::Button::builder()
        .label("Cancel")
        .hexpand(true)
        .build();
    let sus = vec![];
    let model = gtk::StringList::new(&sus);
    for i in themes.clone() {
        model.append(&i)
    }
    let spin = gtk::DropDown::builder().model(&model).build();
    let cur_conf = get_conf();
    let cur_theme = cur_conf.get("theme").expect("theme");
    spin.set_selected(themes.clone().iter().position(|r| *r == *cur_theme).unwrap().try_into().unwrap());
    mb.set_start_widget(Some(&spin));
    let hcsb = gtk::Box::new(gtk::Orientation::Horizontal, 1);
    hcsb.append(&save);
    hcsb.append(&cancel);
    mb.set_end_widget(Some(&hcsb));
    con.show();
    let cc = con.clone();
    save.connect_clicked(move |_| {
        let them = spin.selected() as usize;
        let t = get_conf();
        println!("changing to {}", themes[them]);
        let t = t.get("theme").expect("theme conf");
        if t.to_string() != themes[them].to_string() {
            let mut confa = HashMap::new();
            confa.insert("theme", themes[them].to_string());
            println!("{:?}",confa);
            save_conf(confa);
            conf_css();
        }
        cc.destroy();
    });
    cancel.connect_clicked(move |_| {
        con.destroy();
    });
}

pub fn conf_css() {
    let display = gdk::Display::default().expect("Could not get default display.");
    let provider = gtk::CssProvider::new();
    let priority = gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;

    let conf = get_conf();
    let theme = conf.get("theme").expect("theme");
    match config_dir() {
        Some(h) => {
            match File::open(format!("{}/wcalc/css/{}.css",h.display(), theme)) {
                Ok(_) => {
                    let css_p = format!("{}/wcalc/css/{}.css",h.display(), theme);
                    let css_content = fs::read_to_string(css_p).expect("file");
                    provider.load_from_data(&css_content);
                    gtk::StyleContext::add_provider_for_display(&display, &provider, priority);
                }
                _ => {
                    let css_content = String::new();
                    if System::name().unwrap() == "Windows" {
                        let css_content = include_str!("./css/win.css");
                    } else {
                        let css_content = include_str!("./css/linux.css");
                    }
                    provider.load_from_data(&css_content);
                    gtk::StyleContext::add_provider_for_display(&display, &provider, priority);
                }
            }
        },
        _ => {
            let css_content = String::new();
            if System::name().unwrap() == "Windows" {
                let css_content = include_str!("./css/win.css");
            } else {
                let css_content = include_str!("./css/linux.css");
            }
            provider.load_from_data(&css_content);
            gtk::StyleContext::add_provider_for_display(&display, &provider, priority);
        }
    }
}
