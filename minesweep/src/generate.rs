use gtk::prelude::*;
use rand::prelude::*;

fn mines(mut map: Vec<Vec<String>>, size: u8, amount: u8) -> Vec<Vec<String>> {
    for _i in 0..amount {
        let mut rng = rand::thread_rng();
        let x: usize = rng.gen_range(0..size).into();
        let y: usize = rng.gen_range(0..size).into();
        map[x][y] = "X".to_string();
    }
    map
}
fn check(map: Vec<Vec<String>>, x: usize, y: usize) -> (u8, Vec<[usize; 2]>) {
    let mut mines: u8 = 0;
    let mut minescord: Vec<[usize; 2]> = Vec::new();
    if x > 0 && y > 0 && map[x - 1][y - 1] == "X" {
        mines += 1;
        minescord.push([x-1, y-1]);
    }
    if y > 0 && map[x][y - 1] == "X" {
        mines += 1;
        minescord.push([x, y-1]);
    }
    if x < map.len() - 1 && y > 0 && map[x + 1][y - 1] == "X" {
        mines += 1;
        minescord.push([x+1, y-1]);
    }
    if x > 0 && map[x - 1][y] == "X" {
        mines += 1;
        minescord.push([x-1, y]);
    }
    if map[x][y] == "X" {
        mines += 1;
        minescord.push([x, y]);
    }
    if x < map.len() - 1 && map[x + 1][y] == "X" {
        mines += 1;
        minescord.push([x+1, y]);
    }
    if x > 0 && y < map[0].len() - 1 && map[x - 1][y + 1] == "X" {
        mines += 1;
        minescord.push([x-1, y+1]);
    }
    if y < map[0].len() - 1 && map[x][y + 1] == "X" {
        mines += 1;
        minescord.push([x, y+1]);
    }
    if x < map.len() - 1 && y < map[0].len() - 1 && map[x + 1][y + 1] == "X" {
        mines += 1;
        minescord.push([x+1, y+1]);
    }

    (mines, minescord)
}

pub fn map_gen(main: gtk::Box, size: u8) {
    let mut map: Vec<Vec<String>> = Vec::new();
    for _i in 0..size {
        let mut line: Vec<String> = Vec::new();
        for _j in 0..size {
            line.push("".to_string());
        }
        map.push(line);
    }
    let amount: u8 = 4; 
    let map = mines(map, size, amount);
    let mapbox = gtk::Box::new(gtk::Orientation::Vertical, 1);
    for (x, i) in map.iter().enumerate() {
        let l = gtk::Box::new(gtk::Orientation::Horizontal, 1);
        l.add_css_class("line");
        l.set_hexpand(true);
        l.set_vexpand(true);
        l.set_homogeneous(true);
        for (y, j) in i.iter().enumerate() {
            let but = gtk::Button::builder()
                .label("")
                .build();
            but.add_css_class("cell");
            l.append(&but);
            let lab = j.clone();
            let mapclon = map.clone();
            but.connect_clicked(move |button| {
                if &lab == "X" {
                    button.set_label(&lab);
                    button.add_css_class("x");
                } else {
                    let (c, m) = check(mapclon.clone(), x, y);
                    button.set_label(&format!("{}",c));
                    match c {
                        0 => button.add_css_class("zero"),
                        1 => button.add_css_class("one"),
                        2 => button.add_css_class("two"),
                        3 => button.add_css_class("three"),
                        4 => button.add_css_class("four"),
                        5 => button.add_css_class("five"),
                        6 => button.add_css_class("six"),
                        7 => button.add_css_class("seven"),
                        8 => button.add_css_class("eight"),
                        9 => button.add_css_class("nine"),
                        10_u8..=u8::MAX => button.add_css_class("how"),
                    }
                }

            });
        }
        mapbox.append(&l);
    }
    main.append(&mapbox);
}
