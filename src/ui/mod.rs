use gtk::ApplicationWindow;
use gtk::prelude::WidgetExt;
use log::{error, info};

pub fn create_display(window: &ApplicationWindow) {
    if !window.is_visible() {
        error!("Window is not visible!");
        return;
    }

    info!("Loaded the display module!");
}