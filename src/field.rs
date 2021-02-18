extern crate gtk;
use gdk_pixbuf::Pixbuf;
use gtk::{ToolButton, Image};

#[derive(Clone)]
pub struct Field {
    pub button: ToolButton,
    pub is_clicked: bool,
    pub is_flagged: bool,
    pub value: i8,
}

impl Field {
    pub fn new(pixbuf: &Pixbuf) -> Field {
        let im = Image::from_pixbuf(Some(&pixbuf));
        let button = ToolButton::new::<Image>(Some(&im), Some("field"));
        Field {
            button,
            is_clicked: false,
            is_flagged: false,
            value: 0,
        }
    }
}