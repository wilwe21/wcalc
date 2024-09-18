use gtk::prelude::*;
use rand::prelude::*;

fn mines(mut map: Vec<Vec<String>>, size: u8, amount: u8) -> Vec<Vec<String>> {
    for _i in 0..amount {
        let mut rng = rand::thread_rng();
        let x: usize = rng.gen_range(0..size).into();
        let y: usize = rng.gen_range(0..size).into();
        map[x][y] = "X".to_string();
    }
    println!("{:?}", map);
    map
}
fn check(map: Vec<Vec<String>>, x: usize, y: usize) -> u8 {
    let mut mines: u8 = 0;
    if x > 0 && y > 0 && map[x - 1][y - 1] == "X" {
        mines += 1;
    }
    if y > 0 && map[x][y - 1] == "X" {
        mines += 1;
    }
    if x < map.len() - 1 && y > 0 && map[x + 1][y - 1] == "X" {
        mines += 1;
    }
    if x > 0 && map[x - 1][y] == "X" {
        mines += 1;
    }
    if map[x][y] == "X" {
        mines += 1;
    }
    if x < map.len() - 1 && map[x + 1][y] == "X" {
        mines += 1;
    }
    if x > 0 && y < map[0].len() - 1 && map[x - 1][y + 1] == "X" {
        mines += 1;
    }
    if y < map[0].len() - 1 && map[x][y + 1] == "X" {
        mines += 1;
    }
    if x < map.len() - 1 && y < map[0].len() - 1 && map[x + 1][y + 1] == "X" {
        mines += 1;
    }
    mines
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
        for (y, j) in i.iter().enumerate() {
            let but = gtk::Button::builder()
                .label("")
                .build();
            l.append(&but);
            let lab = j.clone();
            let mapclon = map.clone();
            but.connect_clicked(move |button| {
                if &lab == "X" {
                    button.set_label(&lab)
                } else {
                    let c = check(mapclon.clone(), x, y);
                    button.set_label(&format!("{}",c));
                }

            });
        }
        mapbox.append(&l);
    }
    main.append(&mapbox);
}
