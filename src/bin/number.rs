use glib::clone;
use gtk4::prelude::*;
use gtk4::{self, Application, ApplicationWindow, Box, Button, Label, Orientation};

use rand::Rng;

use std::cell::Cell;
use std::rc::Rc;

fn main() {
    // Create a new application
    let app = Application::builder().application_id("counter").build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);
    // Run the application
    app.run();
}

fn build_ui(application: &Application) {
    // Create a window
    let window = ApplicationWindow::new(application);

    // Create buttons
    let button_increase = Button::builder()
        .label("+")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let button_decrease = Button::builder()
        .label("-")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let button_rnd = Button::builder()
        .label("?")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // and the label containing initial value of 0 and after that a value changed by the button presses
    let label = Label::builder()
        .label("0")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Reference-counted object with inner-mutability
    let number = Rc::new(Cell::new(0));

    // Connect callbacks
    // When a button is clicked, `number` and label contents are changed.
    button_increase.connect_clicked(clone!(@weak number, @strong label =>
        move |_| {
            number.set(number.get() + 1);
            label.set_label(&number.get().to_string());
    }));
    button_decrease.connect_clicked(clone!(@weak number, @strong label =>
        move |_| {
            number.set(number.get() - 1);
            label.set_label(&number.get().to_string());
    }));
    // When randomize button is clicked instead of decreasing or increasing the number by one select a random value
    button_rnd.connect_clicked(clone!(@strong label =>
        move |_| {
            let mut rng = rand::thread_rng();
            number.set(rng.gen_range(-10..10));
            label.set_label(&number.get().to_string());
    }));

    // Add buttons and the label
    let gtk_box = Box::new(Orientation::Vertical, 0);
    window.set_child(Some(&gtk_box));
    gtk_box.append(&button_increase);
    gtk_box.append(&label);
    gtk_box.append(&button_decrease);
    gtk_box.append(&button_rnd);
    window.present();
}
