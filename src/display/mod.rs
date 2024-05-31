#[allow(deprecated)]
mod imp;

use gtk::{glib, prelude::*, subclass::prelude::*, template_callbacks};

glib::wrapper! {
    pub struct Display(ObjectSubclass<imp::Display>)
    @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow;
}

#[gtk::template_callbacks]
impl Display {
    pub fn new<P: IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    #[template_callback]
    fn on_counter_notify(&self, _p: &glib::ParamSpec) {
        // Check counter property and create a Dialog.
        if self.counter() == 3 {
            self.imp().dialog.present();
        }
    }

    #[template_callback]
    fn counter_choice(&self, response: i32) {
        self.imp().dialog.set_visible(false);

        match gtk::ResponseType::from(response) {
            gtk::ResponseType::Ok => self.set_counter(0),
            gtk::ResponseType::Other(35) => self.set_counter(6),
            gtk::ResponseType::DeleteEvent => (),
            _ => unimplemented!(),
        }
    }

    /// Callback handler for gtk::Button plus.
    #[template_callback]
    fn add_to_counter(&self, _button: &gtk::Button) {
        let n = self.counter() + 1;
        self.set_counter(n);
    }

    /// Callback handler for gtk::Button minus.
    #[template_callback]
    fn sub_to_counter(&self, _button: &gtk::Button) {
        let n = self.counter() - 1;
        self.set_counter(n);
    }
}