use gtk::prelude::*;
use crate::generate::map_gen;

pub fn get_size(main: gtk::Box) {
    let entry = gtk::Entry::builder()
        .placeholder_text("Enter Size like 32")
        .build();
    let window = gtk::Window::builder()
        .title("size")
        .child(&entry)
        .build();
    window.show();
    entry.clone().connect_activate(move |_| {
        match entry.text().parse::<u32>() {
            Ok (int) => {
                if int > 16 {
                    entry.set_placeholder_text(Some("Need to be below 16"));
                    entry.set_text("");
                } else {
                    map_gen(main.clone(), int.clone());
                    window.hide();
                }
            },
            Err (_) => {
                entry.set_placeholder_text(Some("Need to be u32"));
                entry.set_text("");
            }
        }
    });
}
