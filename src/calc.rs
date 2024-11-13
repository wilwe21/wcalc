use gtk::prelude::*;

use std::process::exit;

use meval::{eval_str_with_context, Context, eval_str};
use regex::Regex;
use rand::Rng;

use crate::conf;
use crate::game;

pub fn calc(entr: String) -> String {
    let conf = conf::get_conf();
    let mut round = 15_u32;
    match conf.get("pi") {
        Some(s) => match s.parse::<u32>() {
            Ok(d) => round = d,
            _ => println!("no pi")
        }
        _ => println!("no pi")
    }
    let zer = 10_i64.pow(round);
    let pi: f64 = (3.141592653589793 * (zer as f64)).round() / zer as f64;
    let mut ctx = Context::new();
    ctx
        .var("pi", pi)
        .var("G", 0.915965594177219)
        .func2("log", |x,y| f64::log(x, y))
        .funcn("sub", |xs| {
            let mut i = xs[0];
            for j in 1..xs.len() {
                i -= xs[j];
            } 
            i
        },..)
        .funcn("div", |xs| {
            let mut i = xs[0];
            for j in 1..xs.len() {
                i = i / xs[j];
            } 
            i
        },..)
        .func("fact", |x| {
            let x: i64 = x.round() as i64;
            let mut i = x.clone();
            for j in 1..x {
                i = i*j;
            }
            i as f64
        })
        .func2("root", |x, y| eval_str(format!("{x}^(1/{y})")).unwrap())
        .funcn("sum", |xs| xs.iter().sum(), ..);
    match eval_str_with_context(entr.clone(), ctx.clone()) {
        Ok(vyl) => {
            return vyl.to_string()
        } Err(e) => {
            let chpar = |strin: &str| {
                let one = "Parse error: Missing 1 right parenthesis.";
                let par = Regex::new(r"Parse error: Missing .* right parentheses.")
                    .map(|regex| {
                        if regex.is_match(&strin.clone()) || &strin == &one {
                            let numb = strin.to_string().chars()
                                .find(|a| a.is_digit(10))
                                .and_then(|a| a.to_digit(10)).unwrap() as usize;
                            let pars = ")".repeat(numb);
                            let mut ccc = format!("{}{}", entr.clone(), pars);
                            match eval_str_with_context(ccc.clone(), ctx.clone()) {
                                Ok(vyl) => {
                                    return vyl.to_string()
                                } Err(ee) => {
                                    println!("Error pars ee: {}", ee);
                                    return ccc
                                }
                            }
                        } else { return strin.to_string() }
                    })
                    .unwrap_or_else(|error| {
                        println!("error");
                        return entr.clone()
                    });
                return par.to_string()
            };
            let chtok = |strin: &str| {
                let untok = Regex::new(r"Parse error: Unexpected token at byte .*")
                    .map(|regex| {
                        if regex.is_match(&strin) {
                            let ccc = entr.clone();
                            let pos = strin.to_string().chars()
                                .find(|a| a.is_digit(10))
                                .and_then(|a| a.to_digit(10)).unwrap() as i32;
                            let test = |exp: String, pos: i32| {
                                let mut st = exp;
                                let pos: usize = pos.try_into().unwrap();
                                let prev = &st[pos..pos+1];
                                println!("{}", prev);
                                if prev != "*" {
                                    st.insert(pos, "*".chars().next().unwrap());
                                }
                                match eval_str_with_context(st.clone(), ctx.clone()) {
                                    Ok(vyl) => {
                                        let end = vyl.to_string();
                                        return [st, end];
                                    } Err(e) => {
                                        return [st, e.to_string()];
                                    }
                                }
                            };
                            let mut stra = test(ccc.clone().to_string(), pos);
                            let mut iter = 0;
                            loop {
                                if stra[1] != "false" {
                                    if regex.is_match(&stra[1]) && iter > 20 {
                                        let pos = stra[1].to_string().chars()
                                            .find(|a| a.is_digit(10))
                                            .and_then(|a| a.to_digit(10)).unwrap() as i32;
                                        stra = test(stra[0].clone().to_string(), pos);
                                        iter += 1;
                                    } else {
                                        return stra[0].to_string()
                                    }
                                } else { return stra[0].to_string() }
                            }
                            return strin.to_string()
                        } else { return strin.to_string() }
                    })
                    .unwrap_or_else(|error| {
                        println!("error");
                        strin.to_string()
                    });
                return untok.to_string()
            };
            let i: String = chpar(&e.to_string());
            let i: String = chtok(&i.to_string());
            println!("Error: {}", e);
            return i
        }
    }
    entr
}

pub fn ent_str(text: String, button: String) -> String {
    let conf = conf::get_conf();
    let mut game = String::new();
    match conf.get("game") {
        Some(s) => game = s.to_string(),
        _ => game = "false".to_string()
    };
    if game != "false" {
        if text.contains(":q") || button == "Del"{
            return game::end()
        } 
        if text.contains("clr") || button == "Clr"{
            return "".to_string()
        }
        if game == "numble" {
            game::numbinp(text.to_string(), button.to_string())
        } else if game == "rpg" {
            game::rpginp(text.to_string(), button.to_string())
        } else {
            game::end_silent();
            return "no game".to_string()
        }
    } else {
        if button == "Clr" {
            return "".to_string()
        }
        if button == "Del" {
            if text == "" {
                return "".to_string()
            }
            let new = &text[..text.len() -1];
            return new.to_string()
        }
        if button == "󰒓" {
            conf::config();
            return text.to_string()
        }
        if button != "=" {
            let mut endst = String::new();
            if button == "√" {
                endst = format!("{}^(1/",&text);
            } else {
                let le: usize = text.len();
                if !button.parse::<i32>().is_ok() {
                    if le > 0 {
                        if button == "(" || button == ")" {
                            endst = format!("{}{}",&text, &button); 
                        } else {
                            if le > 1 {
                                if &text[le-1..le] == button || &text[le-2..le-1] == button{
                                    endst = text.to_string();
                                } else { endst = format!("{}{}", &text, &button); }
                            } else {
                                if &text[le-1..le] == button {
                                    endst = text.to_string();
                                } else { endst = format!("{}{}", &text, &button); }
                            }
                        }
                    } else { endst = format!("{}{}",&text, &button); }
                } else { endst = format!("{}{}",&text, &button); }
            }
            return endst.to_string()
        } else {
            if text.contains(":q") {
                exit(6);
            } else if text == "conf" {
                conf::config();
                return "".to_string()
            } else if text == "numble" {
                return game::numble()
            } else if text == "start" || text == "game" {
                return game::start()
            } else if text.contains("clr") {
                return "".to_string()
            } else {
                let end = calc(text.to_string());
                return end.to_string()
            }
        }
    }
}

pub fn wind() -> gtk::Box {
    let cur_conf = conf::get_conf();
    let mut plac = String::new();
    let mut confbut = false;
    match cur_conf.get("config button") {
        Some(p) => {
            match p {
                _ if p.parse::<bool>().is_ok() => confbut = p.parse().unwrap(),
                _ => confbut = true
            }
        },
        _ => confbut = true
    };
    match cur_conf.get("placeholder") {
        Some(p) => plac = p.to_string(),
        _ => plac = "9+10=21".to_string()
    };
    let mainBox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let entry = gtk::Entry::builder()
        .has_frame(true)
        .placeholder_text(&plac)
        .build();
    let buttonBox = gtk::Box::new(gtk::Orientation::Horizontal, 1);
    mainBox.append(&entry);
    mainBox.append(&buttonBox);
    let topbut = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let midbut = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let botbut = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let bbbbut = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let entupbut = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let parbox = gtk::Box::new(gtk::Orientation::Horizontal, 1);
    let clrbox = gtk::Box::new(gtk::Orientation::Horizontal, 1);
    let porootbox = gtk::Box::new(gtk::Orientation::Horizontal, 1);
    let button1 = gtk::Button::builder()
        .label("1")
        .build();
    let button2 = gtk::Button::builder()
        .label("2")
        .build();
    let button3 = gtk::Button::builder()
        .label("3")
        .build();
    let buttonminus = gtk::Button::builder()
        .label("-")
        .build();
    let button4 = gtk::Button::builder()
        .label("4")
        .build();
    let button5 = gtk::Button::builder()
        .label("5")
        .build();
    let button6 = gtk::Button::builder()
        .label("6")
        .build();
    let buttonmult = gtk::Button::builder()
        .label("*")
        .build();
    let button7 = gtk::Button::builder()
        .label("7")
        .build();
    let button8 = gtk::Button::builder()
        .label("8")
        .build();
    let button9 = gtk::Button::builder()
        .label("9")
        .build();
    let buttondiv = gtk::Button::builder()
        .label("/")
        .build();
    let button0 = gtk::Button::builder()
        .label("0")
        .build();
    let buttondot = gtk::Button::builder()
        .label(".")
        .build();
    let buttonplus = gtk::Button::builder()
        .label("+")
        .build();
    let buttonexp = gtk::Button::builder()
        .label("^")
        .build();
    let buttonent = gtk::Button::builder()
        .label("=")
        .build();
    let buttonclr = gtk::Button::builder()
        .label("Clr")
        .build();
    let buttonlefpar = gtk::Button::builder()
        .label("(")
        .build();
    let buttonrigpar = gtk::Button::builder()
        .label(")")
        .build();
    let buttonrmlas = gtk::Button::builder()
        .label("Del")
        .build();
    let buttonroot = gtk::Button::builder()
        .label("√")
        .build();
    let butconf = gtk::Button::builder()
        .label("󰒓")
        .build();
    // css related
    topbut.add_css_class("topbut");
    midbut.add_css_class("midbut");
    botbut.add_css_class("botbut");
    bbbbut.add_css_class("bbbbut");
    entupbut.add_css_class("entupbut");
    parbox.add_css_class("parbox");
    clrbox.add_css_class("clrbox");
    porootbox.add_css_class("porootbox");
    buttonent.add_css_class("enter");
    entry.connect_activate(move |entry| {
        let text = entry.text();
        let end = ent_str(text.to_string(), "=".to_string());
        entry.set_text(&end);
        entry.set_position((entry.text().len() as usize).try_into().unwrap());
    });
    let butclick = move |button: &gtk::Button| {
        let sas = button.label().unwrap();
        let cur = entry.text();
        let i = ent_str(cur.to_string(), sas.to_string());
        entry.set_text(&i);
        entry.set_position((entry.text().len() as usize).try_into().unwrap());
    };
    buttonplus.connect_clicked(butclick.clone());
    buttonminus.connect_clicked(butclick.clone());  
    buttondiv.connect_clicked(butclick.clone());
    buttonmult.connect_clicked(butclick.clone());  
    buttondot.connect_clicked(butclick.clone());
    butconf.connect_clicked(butclick.clone());
    button0.connect_clicked(butclick.clone());
    button1.connect_clicked(butclick.clone());
    button2.connect_clicked(butclick.clone());
    button3.connect_clicked(butclick.clone());
    button4.connect_clicked(butclick.clone());
    button5.connect_clicked(butclick.clone());
    button6.connect_clicked(butclick.clone());
    button7.connect_clicked(butclick.clone());
    button8.connect_clicked(butclick.clone());
    button9.connect_clicked(butclick.clone());
    buttonent.connect_clicked(butclick.clone());
    buttonexp.connect_clicked(butclick.clone());
    buttonclr.connect_clicked(butclick.clone());
    buttonlefpar.connect_clicked(butclick.clone());
    buttonrigpar.connect_clicked(butclick.clone());
    buttonrmlas.connect_clicked(butclick.clone());
    buttonroot.connect_clicked(butclick.clone());
    topbut.append(&button7);
    midbut.append(&button8);
    botbut.append(&button9);
    bbbbut.append(&buttondiv);
    topbut.append(&button4);
    midbut.append(&button5);
    botbut.append(&button6);
    bbbbut.append(&buttonmult);
    topbut.append(&button1);
    midbut.append(&button2);
    botbut.append(&button3);
    bbbbut.append(&buttonminus);
    topbut.append(&button0);
    midbut.append(&buttondot);
    if confbut {
        botbut.append(&butconf);
    }
    bbbbut.append(&buttonplus);
    parbox.append(&buttonlefpar);
    parbox.append(&buttonrigpar);
    clrbox.append(&buttonclr);
    clrbox.append(&buttonrmlas);
    porootbox.append(&buttonexp);
    porootbox.append(&buttonroot);
    entupbut.append(&porootbox);
    entupbut.append(&parbox);
    entupbut.append(&clrbox);
    entupbut.append(&buttonent);
    buttonBox.append(&topbut);
    buttonBox.append(&midbut);
    buttonBox.append(&botbut);
    buttonBox.append(&bbbbut);
    buttonBox.append(&entupbut);
    mainBox
}
