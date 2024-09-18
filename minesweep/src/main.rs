use gtk::prelude::*;

mod get_size;
mod generate;
use crate::generate::map_gen;
use crate::get_size::get_size;

fn on_active(app: &gtk::Application) {
    let main = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let window = gtk::ApplicationWindow::builder()
        .title("wmine")
        .application(app)
        .child(&main)
        .build();
    window.show();
    get_size(main);
}

fn main() {
    let app = gtk::Application::builder()
        .application_id("com.github.wilwe21.wmine")
        .build();
    app.connect_activate(on_active);
    app.run();
}
