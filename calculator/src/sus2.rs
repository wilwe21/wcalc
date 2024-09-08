use gtk::prelude::*;
use gtk::gdk;

fn load_css() {
    let display = gdk::Display::default().expect("Could not get default display.");
    let provider = gtk::CssProvider::new();
    let priority = gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;

    provider.load_from_data(include_str!("../css/main.css"));
    gtk::StyleContext::add_provider_for_display(&display, &provider, priority);
}

fn on_activate(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    let label = gtk::Label::new(Some("Hello, world!"));
    let width = 275;
    let height = 50;

    load_css();
    window.set_child(Some(&label));
    window.set_default_size(width, height);
    window.show();
} 

fn main() {
    let app = gtk::Application::builder()
        .application_id("com.github.wilwe21.Calc")
        .build();
    app.connect_activate(on_activate);
    app.run();
}
