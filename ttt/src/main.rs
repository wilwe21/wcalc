use gtk::prelude::*;

#[derive(Debug, Clone)]
struct Grid {
    grid: Vec<Vec<String>>,
    len: u32,
    window: gtk::ApplicationWindow,
}

impl Grid {
    fn new(len: u32, app: &gtk::Application) -> Self {
        let mut grid: Vec<Vec<String>> = Vec::new();
        for _i in 0..len {
            let mut row: Vec<String> = Vec::new();
            for _j in 0..len {
                row.push("".to_string());
            }
            grid.push(row);
        }
        let window = gtk::ApplicationWindow::builder()
            .title("WTick Tack Toe")
            .application(app)
            .build();
        window.show();
        let boX = gtk::Box::new(gtk::Orientation::Vertical, 1);
        window.set_child(Some(&boX));
        Self {
            grid,
            len,
            window: window,
        }
    }
    fn build(self) {
        let boX = self.window.observe_children();
        if let Some(child) = boX.item(0) {
            if let Ok(box_) = child.downcast::<gtk::Box>() {
                for (a, i) in self.grid.clone().into_iter().enumerate() {
                    let row = gtk::Box::new(gtk::Orientation::Horizontal, 1);
                    for (b, _) in i.into_iter().enumerate() {
                        let cell = gtk::Button::builder()
                            .label(self.grid[a][b].clone())
                            .build();
                        row.append(&cell);
                        let win = self.window.clone();
                        cell.connect_clicked(move |button| {
                            let cur = check(&win);
                            button.set_label(&cur);
                        });
                    };
                    row.set_hexpand(true);
                    row.set_vexpand(true);
                    row.set_homogeneous(true);
                    box_.append(&row);
                }
            } else {
                println!("child is not a box");
            }
        } else {
            println!("Non child");
        }
    }
    fn display(&self) {
        for i in &self.grid {
            for j in i.iter() {
                print!("{}", j);
            }
            println!("");
        }
    }
}
fn check(win: &gtk::ApplicationWindow) -> String {
   let mut x = 0;
   let mut o = 0;
   let boX = win.observe_children();
   if let Some(child) = boX.item(0) {
        if let Ok(box_) = child.downcast::<gtk::Box>() {
            let iter = box_.observe_children();
            for ch in &iter {
                if let Ok(inch) = ch.expect("chuj").downcast::<gtk::Box>() {
                    let init = inch.observe_children();
                    for inchit in &init {
                        if let Ok(butt) = inchit.expect("chuj").downcast::<gtk::Button>() {
                            if let Some(lab) = butt.label() {
                                if lab == "X"{
                                    x += 1;
                                } else if lab == "O" {
                                    o += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
   }
   if x > o {
        return "O".to_string();
   } else {
        return "X".to_string();
   }
}

fn on_active(app: &gtk::Application) {
    let gir = Grid::new(3, app);
    gir.display();
    gir.clone().build();
}

fn main() {
    let app = gtk::Application::builder()
        .application_id("com.gitihub.wilwe21.wttt")
        .build();
    app.connect_activate(on_active);
    app.run();
}
