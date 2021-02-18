extern crate gtk;
use gdk_pixbuf::Pixbuf;

pub struct Pixbufs {
    pixbufs: Vec<Pixbuf> 
}

impl Pixbufs {
    pub fn new() -> Pixbufs {
        let mut pixbufs = Vec::new();
        let file_names = [
            "bomb",
            "zero",
            "one",
            "two",
            "three",
            "four",
            "five",
            "six",
            "unopened",
            "flag",
        ];

        for file in &file_names {
            pixbufs.push(Pixbuf::from_file_at_size(format!("icons/{}.svg", file), 48, 48).unwrap());
        }
        
        Pixbufs{pixbufs}
    }

    pub fn get_flag(&mut self) -> Option<&Pixbuf> {
        self.pixbufs.last()
    }

    pub fn get_bomb(&mut self) -> Option<&Pixbuf> {
        self.pixbufs.first()
    }

    pub fn get_numbered(&mut self, value: i8) -> Option<&Pixbuf> {
        Some(&self.pixbufs[(value + 1) as usize])
    }

    pub fn get_unopened(&mut self) -> Option<&Pixbuf> {
        Some(&self.pixbufs[8 as usize])
    }
}