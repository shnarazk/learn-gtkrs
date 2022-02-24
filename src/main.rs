use {
    gtk4::prelude::*,
    gtk4::{Application, ApplicationWindow, Button, Orientation},
    std::{cell::Cell, rc::Rc},
};

fn main() {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let button_increase = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let button_decrease = Button::builder()
        .label("Decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let number = Rc::new(Cell::new(0i32));
    let number_inc = number.clone();
    let number_dec = number;
    // let number_value = gtk4::Label::builder()
    //     .label(&format!("{}", number.get()))
    //     .build();
    button_increase.connect_clicked(move |_| {
        number_inc.set(number_inc.get() + 1);
    });
    button_decrease.connect_clicked(move |_| {
        number_dec.set(number_dec.get() - 1);
    });
    let gtk_box = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);
    // gtk_box.append(&number_value);
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App (2nd step)")
        .child(&gtk_box)
        .build();
    window.present();
}
