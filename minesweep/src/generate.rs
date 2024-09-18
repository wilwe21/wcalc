use gtk::prelude::*;
use rand::prelude::*;

pub fn map_gen(main: gtk::Box, size: u32) {
    let mut map: Vec<Vec<String>> = Vec::new();
    for _i in 0..size {
        let mut line: Vec<String> = Vec::new();
        for _j in 0..size {
            line.push("X".to_string());
        }
        map.push(line);
    }
    let mut rng = rand::thread_rng();
    let y: u32 = rng.gen();
    println!("{}", y);
    let mapbox = gtk::Box::new(gtk::Orientation::Vertical, 1);
    for i in map.iter() {
        let l = gtk::Box::new(gtk::Orientation::Horizontal, 1);
        for _j in i.iter() {
            let but = gtk::Button::builder()
                .label("X")
                .build();
            l.append(&but);
        }
        mapbox.append(&l);
    }
    main.append(&mapbox);
}
