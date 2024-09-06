use std::io;
use gtk::prelude::*;

fn getv() -> [String; 3] {
    println!("Enter value 1:");
    let mut v1 = String::new();
    io::stdin()
        .read_line(&mut v1)
        .expect("fail to read");
    println!("what to do [+-*/^√]:");
    let mut op = String::new();
    io::stdin()
        .read_line(&mut op)
        .expect("fail to read");
    println!("Enter value 2:");
    let mut v2 = String::new();
    io::stdin()
        .read_line(&mut v2)
        .expect("fail to read");
    v1 = v1.replace("\n", "");
    v2 = v2.replace("\n", "");
    op = op.replace("\n", "");
    [v1, op, v2]
}

fn domath(op: String, o1: u32, o2: u32) {
    if &op == "+" {
        let wyn = o1 + o2;
        println!("{}+{}={}", o1, o2, wyn);
    } else if &op == "*" {
        let wyn = o1 * o2;
        println!("{}*{}={}", o1, o2, wyn);
    } else if &op == "-" {
        let wyn = o1 - o2;
        println!("{}-{}={}", o1, o2, wyn);
    } else if &op == "/" {
        let wyn = o1 / o2;
        println!("{}/{}={}", o1, o2, wyn);
    } else if &op == "^" {
        let wyn = o1.pow(o2);
        println!("{}^{}={}", o1, o2, wyn);
    } else if &op == "√" {
        let f1 = o1 as f32;
        let f2 = o2 as f32;
        let wyn = f1.powf(1.0 / f2);
        println!("{}√{}={}", o1, o2, wyn);
    }

}

fn on_activate(app: &gtk::Application) {
    let mainBox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let entry = gtk::Entry::builder()
        .has_frame(true)
        .placeholder_text("chuj")
        .build();
    let buttonBox = gtk::Box::new(gtk::Orientation::Horizontal, 1);
    mainBox.append(&entry);
    mainBox.append(&buttonBox);
    let topbut = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let midbut = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let botbut = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let bbbbut = gtk::Box::new(gtk::Orientation::Vertical, 1);
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
    let buttonproc = gtk::Button::builder()
        .label("%")
        .build();
    let buttonplus = gtk::Button::builder()
        .label("+")
        .build();
    let buttonent = gtk::Button::builder()
        .label("Enter")
        .build();
    let butclick = move |button: &gtk::Button| {
        let sas = button.label().unwrap();
        let cur = entry.text();
        let endst = format!("{}{}",&cur, &sas);
        entry.set_text(&endst);
    };
    buttonplus.connect_clicked(butclick.clone());
    buttonminus.connect_clicked(butclick.clone());
    buttonproc.connect_clicked(butclick.clone());
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
    topbut.append(&button7);
    topbut.append(&button8);
    topbut.append(&button9);
    topbut.append(&buttondiv);
    midbut.append(&button4);
    midbut.append(&button5);
    midbut.append(&button6);
    midbut.append(&buttonmult);
    botbut.append(&button1);
    botbut.append(&button2);
    botbut.append(&button3);
    botbut.append(&buttonminus);
    bbbbut.append(&button0);
    bbbbut.append(&buttondot);
    bbbbut.append(&buttonproc);
    bbbbut.append(&buttonplus);
    buttonBox.append(&topbut);
    buttonBox.append(&midbut);
    buttonBox.append(&botbut);
    buttonBox.append(&bbbbut);
    buttonBox.append(&buttonent);
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
