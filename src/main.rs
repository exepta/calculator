mod ui;
mod logic;

use log::{error, info};
use simple_logger::SimpleLogger;
use time::macros::format_description;

use gtk::{Application, ApplicationWindow, glib};
use gtk::prelude::*;
use crate::ui::create_display;

const VERSION :&str = env!("CARGO_PKG_VERSION");
const APP_ID :&str = "com.vogeez.calculator";

/**
 * Initialized all needed things in this project!
 */
fn init() {
    SimpleLogger::new()
        .env()
        .with_timestamp_format(format_description!("[hour]:[minute]:[second]"))
        .init()
        .unwrap();
}

/**
 * Creating the GTK4 window for handle our calculator.
 * This window is created aat the main function.
 */
fn create_window_ui(application: &Application) {
    info!("Creating GTK4 Window...");
    let window = &ApplicationWindow::builder()
        .application(application)
        .title("Calculator")
        .build();

    window.present();
    create_display(window);

    if window.is_visible() {
        info!("Window Successfully created!");
        return;
    }
    error!("Window creation failed!");
}

fn main() -> glib::ExitCode {
    init();
    info!("Starting Calculator App Version [{}]", VERSION);

    let app = Application::builder()
        .application_id(APP_ID)
        .build();
    app.connect_activate(create_window_ui);

    return app.run()
}
