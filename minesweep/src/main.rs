use gtk::prelude::*;

fn on_active(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .title("wmine")
        .application(app)
        .build();
    window.show();
}

fn main() {
    let app = gtk::Application::builder()
        .application_id("com.github.wilwe21.wmine")
        .build();
    app.connect_activate(on_active);
    app.run();
}
