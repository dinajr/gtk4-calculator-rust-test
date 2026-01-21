use gtk4::prelude::*;

mod gui;
use crate::gui::build_ui;
//use::std::env;

fn main() -> gtk4::glib::ExitCode {
    let application = gtk4::Application::builder()
        .application_id("com.gtk-calc-test")
        .build();

    application.connect_activate(build_ui);
    application.run()
}



