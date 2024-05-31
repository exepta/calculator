use std::cell::Cell;
use gtk::{glib, prelude::*, subclass::prelude::*};

#[derive(Default, gtk::CompositeTemplate, glib::Properties)]
#[template(file = "display.ui")]
#[properties(wrapper_type = super::Display)]
pub struct Display {

    #[property(get, set)]
    counter: Cell<i32>,
    #[template_child]
    pub count_label: TemplateChild<gtk::Label>,
    #[template_child]
    pub plus: TemplateChild<gtk::Button>,
    #[template_child]
    pub minus: TemplateChild<gtk::Button>,
    #[template_child]
    pub dialog: TemplateChild<gtk::Dialog>,

}

#[glib::object_subclass]
impl ObjectSubclass for Display {
    const NAME: &'static str = "Display";
    type Type = super::Display;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(_class: &mut Self::Class) {
        _class.bind_template();
        _class.bind_template_instance_callbacks();
    }

    fn instance_init(_obj: &glib::subclass::InitializingObject<Self>) {
        _obj.init_template();
    }
}

#[glib::derived_properties]
impl ObjectImpl for Display {}
impl WidgetImpl for Display {}
impl WindowImpl for Display {}
impl ApplicationWindowImpl for Display {}