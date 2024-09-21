use gtk::prelude::*;

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<String>>,
    len: u32,
    b: gtk::Box,
}

impl Grid {
    fn new(len: u32) -> Self {
        let mut grid: Vec<Vec<String>> = Vec::new();
        for _i in 0..len {
            let mut row: Vec<String> = Vec::new();
            for _j in 0..len {
                row.push("".to_string());
            }
            grid.push(row);
        }
        let gbox = gtk::Box::new(gtk::Orientation::Vertical, 1);
        Self {
            grid,
            len,
            b: gbox,
        }
    }
    fn build(&self) {
        let mut cur = "X";
        for i in &self.grid {
            let row = gtk::Box::new(gtk::Orientation::Horizontal, 1);
            for _j in i {
                let cell = gtk::Button::builder()
                    .label("")
                    .build();
                row.append(&cell);
                cell.connect_clicked(move |button| {
                    button.set_label(cur);
                    let mut o = "O".to_string();
                    let mut x = "X".to_string();
                    cur = if cur == "X" { &mut o } else { &mut x };
                });
            };
            row.set_hexpand(true);
            row.set_vexpand(true);
            row.set_homogeneous(true);
            self.b.append(&row);
        }
    }
}

fn on_active(app: &gtk::Application) {
    let mut gir = Grid::new(3);
    gir.build();
    println!("{:?}",gir);
    let window = gtk::ApplicationWindow::builder()
        .title("WTick Tack Toe")
        .application(app)
        .build();
    window.set_child(Some(&gir.b));
    window.show();
}

fn main() {
    let app = gtk::Application::builder()
        .application_id("com.gitihub.wilwe21.wttt")
        .build();
    app.connect_activate(on_active);
    app.run();
}
