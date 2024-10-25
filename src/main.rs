#![windows_subsystem = "windows"]
use gtk::prelude::*;

mod conf;
mod calc;

fn on_activate(app: &gtk::Application) {
    let mainBox = calc::wind();
    let window = gtk::ApplicationWindow::builder()
        .title("WCalc")
        .resizable(false)
        .application(app)
        .build();
    conf::conf_css();
    window.set_child(Some(&mainBox));
    window.show();
}

fn main() {
    let app = gtk::Application::builder()
        .application_id("com.github.wilwe21.Calc")
        .build();
    app.connect_activate(on_activate);
    app.run();
}
