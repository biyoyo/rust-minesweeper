extern crate gtk;
use rand::distributions::{Distribution, Uniform};

pub struct BadGuy {
    pub position: (usize, usize),
    pub is_active: bool,
} 

impl BadGuy {
    pub fn new(dimension: i8) -> BadGuy {
        let mut rng = rand::thread_rng();
        let num = Uniform::from(0..dimension);
        let x = num.sample(&mut rng);
        let y = num.sample(&mut rng);
        println!("bad guy at: {}, {}", x, y);

        let position = (x as usize, y as usize);

        BadGuy{position, is_active: false}
    }
}