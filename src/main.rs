use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Button, Window, WindowType, Application, ApplicationWindow};

fn main() {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        // We create the main window.
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hello, World!")
            .build();

        let button = Button::with_label("Click me!");
        window.add(&button);
        window.show_all();

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        button.connect_clicked(|_| {
            println!("Clicked!");
        });

        // Show the window.
        window.show();
    });
    app.run();
}
