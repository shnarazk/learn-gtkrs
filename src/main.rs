// use gtk4 as gtk;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};

fn main() {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My new GTK App")
        .build();
    window.present();
}
