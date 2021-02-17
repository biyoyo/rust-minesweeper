extern crate gtk;
use crate::gtk::prelude::Cast;
use gdk_pixbuf::Pixbuf;
use gtk::*;
use rand::distributions::{Distribution, Uniform};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::Field;

pub struct Board {
    pub container: Box,
    pub dimension: i8,
    pub fields: Vec<Vec<Field>>,
    pub pixbufs: Vec<Pixbuf>,
    pub game_over: bool,
    pub seconds_elapsed: i32,
}

impl Board {
    pub fn new(dimension: i8) -> Rc<RefCell<Board>> {
        let container = Box::new(Orientation::Vertical, 0);

        let board_rc = Rc::new(RefCell::new(Board {
            container,
            dimension,
            fields: Vec::new(),
            pixbufs: Board::load_icons(),
            game_over: true,
            seconds_elapsed: 0,
        }));

        let mut board = board_rc.borrow_mut();
        board.create_fields();
        board.init_fields();

        for x in 0..dimension as usize {
            for y in 0..dimension as usize {
                //let value_clone = board.fields[x][y].value.clone();
                //let i_clone = board.pixbufs[(value_clone + 1) as usize].clone();

                let board_clone = board_rc.clone();

                board.fields[x][y].button.connect_clicked(move |button| {
                    let mut board = board_clone.borrow_mut();

                    let value_on_button = board.fields[x][y].value;

                    if value_on_button == -1 {
                        //open all bombs, game over
                        Board::reveal_all_mines(&board);
                        board.game_over = true;
                        println!("Issa bomb");
                    }

                    button
                        .get_icon_widget()
                        .unwrap()
                        .downcast::<gtk::Image>()
                        .unwrap()
                        //.set_from_pixbuf(Some(&i_clone));
                        .set_from_pixbuf(Some(&board.pixbufs[(value_on_button + 1) as usize]));

                    board.fields[x][y].is_clicked = true;

                    //if all adjacent mines are flagged
                    if board.fields[x][y].is_clicked || value_on_button == 0 {
                        //recursive reveal adjacent fields without bombs
                        let mut fields_to_traverse = VecDeque::new();
                        fields_to_traverse.push_back((x, y));

                        while let Some((x, y)) = fields_to_traverse.pop_front() {
                            let current_field_value = board.fields[x][y].value;

                            if current_field_value == 0 {
                                for k in &[-1 as i8, 0, 1] {
                                    for l in &[-1 as i8, 0, 1] {
                                        let r = *k + x as i8;
                                        let c = *l + y as i8;
                                        if board.is_valid_field(r, c)
                                            && !board.fields[r as usize][c as usize].is_clicked
                                        {
                                            fields_to_traverse.push_back((r as usize, c as usize));
                                            board.fields[r as usize][c as usize].is_clicked = true;
                                        }
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

                board.fields[x][y]
                    .button
                    .connect_button_press_event(move |button, event| {
                        let pb_clone = board_clone.borrow().pixbufs.last().unwrap().clone();
                        if event.get_button() == 3 {
                            button
                                .get_icon_widget()
                                .unwrap()
                                .downcast::<gtk::Image>()
                                .unwrap()
                                .set_from_pixbuf(Some(&pb_clone));
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
                self.fields[i][j].value = -1;
                mines += 1;
            }
        }
        //calculate values of fields
        for i in 0..self.dimension {
            for j in 0..self.dimension {
                if !self.is_mine_on_field(i, j) {
                    for k in &[-1, 0, 1] {
                        for l in &[-1, 0, 1] {
                            let r = *k + i;
                            let c = *l + j;
                            if self.is_valid_field(r, c) && self.is_mine_on_field(r, c) {
                                self.fields[i as usize][j as usize].value += 1;
                            }
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

    fn is_mine_on_field(&mut self, x: i8, y: i8) -> bool {
        self.fields[x as usize][y as usize].value == -1
    }

    fn reveal_all_mines(board: &Board) {
        for row in &board.fields {
            for field in row {
                if field.value == -1 {
                    field
                        .button
                        .get_icon_widget()
                        .unwrap()
                        .downcast::<gtk::Image>()
                        .unwrap()
                        .set_from_pixbuf(Some(&board.pixbufs[0]));
                }
            }
        }
    }
}
