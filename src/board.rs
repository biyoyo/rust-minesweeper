extern crate gtk;
use crate::gtk::prelude::Cast;
use gdk_pixbuf::Pixbuf;
use gtk::*;
use rand::distributions::{Distribution, Uniform};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::Field;

pub struct Adjacent {
    pub coord: (i8, i8),
    offset: (i8, i8),
    dimension: i8,
}

impl Adjacent {
    pub fn new(dimension:i8, x: usize, y: usize) -> Adjacent {
        Adjacent{coord:(x as i8, y as i8), offset: (-1, -1), dimension}
    }
}

impl Iterator for Adjacent {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let mut adjacent;
        loop {
            adjacent = (self.coord.0 + self.offset.0, self.coord.1 + self.offset.1);
            if self.offset.0 < 1 && self.offset.1 <= 1 {
                self.offset.0 += 1;
                if self.offset == (0,0) {
                    self.offset.0 += 1;
                }
            }
            else if self.offset.0 >= 1 && self.offset.1 <= 1 {
                self.offset.0 = -1;
                self.offset.1 += 1;
            }
            else {
                return None;
            }
            if adjacent.0 >= 0 && adjacent.0 < self.dimension && adjacent.1 >= 0 && adjacent.1 < self.dimension {
                break;
            }
        }
        Some((adjacent.0 as usize, adjacent.1 as usize))
    }
}

pub struct Board {
    pub container: Box,
    pub dimension: i8,
    pub fields: Vec<Vec<Field>>,
    pub pixbufs: Vec<Pixbuf>,
    pub game_over: bool,
    pub seconds_elapsed: i32,
    pub flags_placed: i8,
}

impl Board {
    pub fn new(dimension: i8) -> Rc<RefCell<Board>> {
        let container = Box::new(Orientation::Vertical, 0);

        let board_rc = Rc::new(RefCell::new(Board {
            container,
            dimension,
            fields: Vec::new(),
            pixbufs: Board::load_icons(),
            game_over: false,
            seconds_elapsed: 0,
            flags_placed: 0,
        }));

        let mut board = board_rc.borrow_mut();
        board.create_fields();
        board.init_fields();

        for x in 0..dimension as usize {
            for y in 0..dimension as usize {
                let board_clone = board_rc.clone();

                board.fields[x][y].button.connect_clicked(move |button| {
                    let mut board = board_clone.borrow_mut();

                    let value_on_button = board.fields[x][y].value;

                    if value_on_button == -1 {
                        //open all bombs, game over
                        board.reveal_all_mines();
                        board.game_over = true;
                        println!("Issa bomb");
                    }

                    button
                        .get_icon_widget()
                        .unwrap()
                        .downcast::<gtk::Image>()
                        .unwrap()
                        .set_from_pixbuf(Some(&board.pixbufs[(value_on_button + 1) as usize]));

                    //if all adjacent mines are flagged, reveal,
                    //if more than there are mines are flagged, do nothing
                    //if less than there are mines are flagged, do nothing
                    //if adjacent field has a mine that is not flagged, explode and end game
                    if board.fields[x][y].is_clicked {
                        let mut mines_flagged = 0;
                        for adj in Adjacent::new(board.dimension, x, y) {
                            if board.is_mine_on_field(adj.0, adj.1) {
                                match board.fields[adj.0][adj.1].is_flagged {
                                    true => mines_flagged += 1,
                                    false => board.reveal_all_mines(),
                                    _ => unreachable!(),
                                }
                            } 
                        }
                        if mines_flagged == value_on_button {
                            for adj in Adjacent::new(board.dimension, x, y) {
                                board.fields[adj.0][adj.1].is_clicked = true;

                                let field = &board.fields[adj.0][adj.1];

                                let pb = if field.is_flagged {
                                    board.pixbufs.last()
                                }
                                else {
                                    Some(&board.pixbufs[(field.value + 1) as usize])
                                };

                                board.fields[adj.0][adj.1]
                                    .button
                                    .get_icon_widget()
                                    .unwrap()
                                    .downcast::<gtk::Image>()
                                    .unwrap()
                                    .set_from_pixbuf(pb);
                            }
                        }
                    }

                    board.fields[x][y].is_clicked = true;

                    if value_on_button == 0 {
                        //recursive reveal adjacent fields without bombs
                        let mut fields_to_traverse = VecDeque::new();
                        fields_to_traverse.push_back((x, y));

                        while let Some((x, y)) = fields_to_traverse.pop_front() {
                            let current_field_value = board.fields[x][y].value;

                            if current_field_value == 0 {
                                for adj in Adjacent::new(board.dimension, x,y) {
                                    if !board.fields[adj.0][adj.1].is_clicked {
                                        fields_to_traverse.push_back(adj);
                                        board.fields[adj.0][adj.1].is_clicked = true;
                                    }
                                }
                            }
                            let pb = &board.pixbufs[(current_field_value + 1) as usize];
                            board.fields[x][y]
                                .button
                                .get_icon_widget()
                                .unwrap()
                                .downcast::<gtk::Image>()
                                .unwrap()
                                .set_from_pixbuf(Some(&pb));
                        }
                    }
                });

                let board_clone = board_rc.clone();

                //handles right click
                board.fields[x][y]
                    .button
                    .connect_button_press_event(move |button, event| {
                        let mut board = board_clone.borrow_mut();

                        if event.get_button() == 3 && board.flags_placed < 10  && !board.fields[x][y].is_clicked{
                            let flag = board.fields[x][y].is_flagged;
                            //if unflag reduce count
                            board.flags_placed += -1 * flag as i8 + !flag as i8;

                            let pb = if flag {
                                Some(&board.pixbufs[8 as usize])
                            }
                            else {
                                board.pixbufs.last()
                            };

                            button
                                .get_icon_widget()
                                .unwrap()
                                .downcast::<gtk::Image>()
                                .unwrap()
                                .set_from_pixbuf(pb);

                            board.fields[x][y].is_flagged = !flag;
                        }
                        Inhibit(true)
                    });
            }
        }

        board_rc.clone()
    }

    fn create_fields(&mut self) {
        let mut fields: Vec<Vec<Field>> = Vec::new();
        fields.resize(self.dimension as usize, Vec::new());

        for x in 0..self.dimension {
            let row = Box::new(Orientation::Horizontal, 0);
            self.container.pack_start(&row, false, false, 0);
            for _y in 0..self.dimension {
                let field = Field::new(&self.pixbufs[self.pixbufs.len() - 2]);
                fields[x as usize].push(field.clone());
                row.pack_start(&field.button, false, false, 0);
            }
        }

        self.fields = fields;
    }

    pub fn init_fields(&mut self) {
        //add starting images
        for row in &mut self.fields {
            for elem in row {
                elem.button
                    .get_icon_widget()
                    .unwrap()
                    .downcast::<gtk::Image>()
                    .unwrap()
                    .set_from_pixbuf(Some(&self.pixbufs[self.pixbufs.len() - 2]));
                elem.value = 0;
                elem.is_clicked = false;
            }
        }
        //place mines on random places
        let mut rng = rand::thread_rng();
        let num = Uniform::from(0..self.dimension);
        let mut mines = 0;
        while mines != 10 {
            let i = num.sample(&mut rng) as usize;
            let j = num.sample(&mut rng) as usize;
            if self.fields[i][j].value != -1 {
                println!("{}", mines);
                self.fields[i][j].value = -1;
                mines += 1;
            }
        }
        //calculate values of fields
        for i in 0..self.dimension {
            for j in 0..self.dimension {
                if !self.is_mine_on_field(i as usize, j as usize) {
                    let mut iter = Adjacent::new(self.dimension, i as usize, j as usize);
                    for adj in iter {
                        if self.is_mine_on_field(adj.0, adj.1) {
                            self.fields[i as usize][j as usize].value += 1;
                        }
                    }
                }
            }
        }
    }

    fn load_icons() -> Vec<Pixbuf> {
        let mut pixbufs = Vec::new();
        let file_names = [
            "bomb.svg",
            "zero.svg",
            "one.svg",
            "two.svg",
            "three.svg",
            "four.svg",
            "five.svg",
            "six.svg",
            "unopened.svg",
            "flag.svg",
        ];

        for file in &file_names {
            pixbufs.push(Pixbuf::from_file_at_size(format!("icons/{}", file), 48, 48).unwrap());
        }
        pixbufs
    }

    fn is_valid_field(&mut self, x: i8, y: i8) -> bool {
        x >= 0 && x < self.dimension as i8 && y >= 0 && y < self.dimension as i8
    }

    fn is_mine_on_field(&mut self, x: usize, y: usize) -> bool {
        self.fields[x][y].value == -1
    }

    fn reveal_all_mines(&mut self) {
        for row in &self.fields {
            for field in row {
                if field.value == -1 {
                    field
                        .button
                        .get_icon_widget()
                        .unwrap()
                        .downcast::<gtk::Image>()
                        .unwrap()
                        .set_from_pixbuf(Some(&self.pixbufs[0]));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tests() {
        let mut adj = Adjacent::new(4, 1, 1);
        assert_eq!(adj.next(), Some((0,0)));
    }
}
