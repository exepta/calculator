use gtk::CssProvider;
use gtk::gdk::Display;
use log::info;

pub fn css() {
    info!("Loading css file...");
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("../assets/styles.css"));


    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to display!"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    info!("Finished loading css file!");
}