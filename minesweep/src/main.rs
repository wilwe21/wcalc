use gtk::prelude::*;
use gtk::gdk;

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
    load_css();
    window.show();
    get_size(main);
}

fn load_css() {
    let display = gdk::Display::default().expect("Could not get default display.");
    let provider = gtk::CssProvider::new();
    let priority = gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;

    let css_content = include_str!("../css/main.css");
    provider.load_from_data(css_content);
    gtk::StyleContext::add_provider_for_display(&display, &provider, priority);
}

fn main() {
    let app = gtk::Application::builder()
        .application_id("com.github.wilwe21.wmine")
        .build();
    app.connect_activate(on_active);
    app.run();
}
