use gtk::prelude::*;

pub fn map_gen(main: gtk::Box, size: u32) {
    let lab = gtk::Label::builder()
        .label(format!("{}", size))
        .build();
    main.append(&lab);
}
