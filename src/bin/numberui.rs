use glib::clone;
use gtk4::prelude::*;
use gtk4::{self, Application, ApplicationWindow, Button, Label};

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
    // Init `gtk::Builder` from file
    let builder = gtk4::Builder::from_string(include_str!("number.ui"));

    // get window from glade file (gtk builder)
    let window: ApplicationWindow = builder
        .object("wnd_number")
        .expect("Could not get object `wnd_number` from builder.");

    // get the label from builder
    let label: Label = builder
        .object("lbl_number")
        .expect("Could not get object `lbl_number` from builder.");

    // get the buttons from builder
    let button_increase: Button = builder
        .object("btn_increase")
        .expect("Could not get object `btn_increase` from builder.");
    let button_decrease: Button = builder
        .object("btn_decrease")
        .expect("Could not get object `btn_decrease` from builder.");
    let button_random: Button = builder
        .object("btn_random")
        .expect("Could not get object `btn_random` from builder.");

    // Set application
    window.set_application(Some(application));

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
    button_random.connect_clicked(clone!(@strong label =>
        move |_| {
            let mut rng = rand::thread_rng();
            number.set(rng.gen_range(-10..10));
            label.set_label(&number.get().to_string());
    }));

    window.present();
}
