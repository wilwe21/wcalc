use gtk::prelude::*;
use crate::calc;
use crate::conf;

fn on_activate(app: &gtk::Application) {
    let mainBox = calc::wind();
    let window = gtk::ApplicationWindow::builder()
        .title("wcalc")
        .resizable(false)
        .application(app)
        .build();
    conf::conf_css();
    window.set_child(Some(&mainBox));
    window.show();
}

pub fn load() {
    let app = gtk::Application::builder()
        .application_id("com.github.wilwe21.Calc")
        .build();
    app.connect_activate(on_activate);
    app.run();

}
