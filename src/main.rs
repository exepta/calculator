mod logic;
mod load;

use log::{error, info};
use simple_logger::SimpleLogger;
use time::macros::format_description;

use gtk::{Application, ApplicationWindow, gio, glib};
use gtk::prelude::*;
use crate::load::css;

const VERSION :&str = env!("CARGO_PKG_VERSION");
const APP_ID :&str = "com.vogeez.calculator.Calculator";

/**
 * Initialized all needed things in this project!
 */
fn init() {
    SimpleLogger::new()
        .env()
        .with_timestamp_format(format_description!("[hour]:[minute]:[second]"))
        .init()
        .unwrap();

    gio::resources_register_include!("assets_templates.gresource")
        .expect("Failed to fetch ui resources...");
}

/**
 * Creating the GTK4 window for handle our calculator.
 * This window is created aat the main function.
 */
fn create_window_ui(application: &Application) {
    info!("Creating GTK4 Window...");

    let window = &ApplicationWindow::new(application);
    window.present();

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
    app.connect_startup(|_| css());
    app.connect_activate(create_window_ui);

    return app.run()
}
