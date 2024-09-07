use std::io;
use gtk::prelude::*;
use meval::eval_str;

fn on_activate(app: &gtk::Application) {
    let mainBox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let entry = gtk::Entry::builder()
        .has_frame(true)
        .placeholder_text("9+10=21")
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
        .label("Enter")
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
    entry.connect_activate(|entry| {
        let text = entry.text();
        match meval::eval_str(text) {
            Ok(vyl) => {
                let end = vyl.to_string();
                entry.set_text(&end);
                entry.set_position(end.len() as i32);
                return
            } Err(e) => {
                println!("Error: {}", e);
            }
        }
    });
    let butclick = move |button: &gtk::Button| {
        let sas = button.label().unwrap();
        let cur = entry.text();
        if sas == "Clr" {
            entry.set_text("");
            return
        }
        if sas == "Del" {
            if cur == "" {
                return
            }
            let new = &cur[..cur.len() -1];
            entry.set_text(new);
            return
        }
        if sas != "Enter" {
            let mut endst = String::new();
            if sas == "√" {
                endst = format!("{}^(1/",&cur);
            } else {
                endst = format!("{}{}",&cur, &sas);
            }
            entry.set_text(&endst);
            return
        }
        if sas == "Enter" {
            if cur == "" {
                return
            }
            match meval::eval_str(cur) {
                Ok(vyl) => {
                    let end = vyl.to_string();
                    entry.set_text(&end);
                    return
                } Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    };
    buttonplus.connect_clicked(butclick.clone());
    buttonminus.connect_clicked(butclick.clone());  
    buttondiv.connect_clicked(butclick.clone());
    buttonmult.connect_clicked(butclick.clone());  
    buttondot.connect_clicked(butclick.clone());
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
    let window = gtk::ApplicationWindow::builder()
        .title("Calc")
        .application(app)
        .child(&mainBox)
        .build();
    window.show();
}

fn main() {
    let app = gtk::Application::builder()
        .application_id("com.github.wilwe21.Calc")
        .build();
    app.connect_activate(on_activate);
    app.run();
}
