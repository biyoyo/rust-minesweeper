extern crate gtk;
use gtk::{ToolButton, Image};

#[derive(Clone, Debug)]
pub struct Field {
    pub button: ToolButton,
    pub is_clicked: bool,
    pub is_flagged: bool,
    pub value: i8,
}

impl Field {
    pub fn new() -> Field {
        let button = ToolButton::new::<Image>(None, Some("field"));
        Field {
            button,
            is_clicked: false,
            is_flagged: false,
            value: 0,
        }
    }
}