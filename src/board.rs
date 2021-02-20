extern crate gtk;
use crate::gtk::prelude::Cast;
use crate::gtk::{Box, Image, Inhibit, Orientation};
use gtk::{WidgetExt, ToolButtonExt, BoxExt, ImageExt};
use glib::Continue;
use rand::distributions::{Distribution, Uniform};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::adjacent::Adjacent;
use crate::bad_guy::BadGuy;
use crate::field::Field;
use crate::field_gen::FieldGenerator;
use crate::pixbufs::Pixbufs;

pub struct Board {
    pub container: Box,
    pub dimension: usize,
    pub mines_count: usize,
    pub fields: Vec<Vec<Field>>,
    pub pixbufs_container: Pixbufs,
    pub game_over: bool,
    pub seconds_elapsed: i32,
    pub flags_placed: usize,
    pub click_counter: usize,
    pub bad_guy: BadGuy,
}

impl Board {
    pub fn new(dimension: usize, mines_count: usize) -> Rc<RefCell<Board>> {
        let container = Box::new(Orientation::Vertical, 0);

        let board_rc = Rc::new(RefCell::new(Board {
            container,
            dimension,
            mines_count,
            fields: Vec::new(),
            pixbufs_container: Pixbufs::new(),
            game_over: false,
            seconds_elapsed: 0,
            flags_placed: 0,
            click_counter: 0,
            bad_guy: BadGuy::new(dimension),
        }));

        let mut board = board_rc.borrow_mut();
        board.create_fields();
        board.init_fields();

        for (x, y) in FieldGenerator::new(board.dimension) {
            let board_clone = board_rc.clone();

            board.fields[x][y].button.connect_clicked(move |_button| {
                let mut board = board_clone.borrow_mut();

                Board::click_field(&mut board, x, y);
            });

            let board_clone = board_rc.clone();

            //handles right click
            board.fields[x][y]
                .button
                .connect_button_press_event(move |_button, event| {
                    let mut board = board_clone.borrow_mut();
                    if event.get_button() == 3
                        && board.flags_placed <= board.mines_count
                        && !board.fields[x][y].is_clicked
                    {
                        let flagged = board.fields[x][y].is_flagged;
                        //if unflag reduce count
                        if flagged {
                            board.flags_placed -= 1;
                        } else {
                            board.flags_placed += 1;
                        }
                        board.fields[x][y].is_flagged = !flagged;
                        board.change_pixbuf(x, y);
                    }
                    Inhibit(true)
                });
        }

        let board_clone = board_rc.clone();

        glib::timeout_add_seconds_local(1, move || {
            let mut b = board_clone.borrow_mut();
            if b.bad_guy.is_active && !b.game_over {
                let (prevx, prevy) = b.bad_guy.position;
                b.move_bad_guy();
                let (x, y) = b.bad_guy.position;
                b.change_pixbuf(prevx, prevy);
                b.change_pixbuf(x, y);
                Board::click_field(&mut b, x, y);
            }
            Continue(true)
        });

        board_rc.clone()
    }

    fn create_fields(&mut self) {
        let mut fields: Vec<Vec<Field>> = Vec::new();
        fields.resize(self.dimension as usize, Vec::new());

        for x in 0..self.dimension {
            let row = Box::new(Orientation::Horizontal, 0);
            self.container.pack_start(&row, false, false, 0);
            for _y in 0..self.dimension {
                let field = Field::new();
                fields[x as usize].push(field.clone());
                row.pack_start(&field.button, false, false, 0);
            }
        }

        self.fields = fields;
    }

    pub fn init_fields(&mut self) {
        //add starting images
        for (x, y) in FieldGenerator::new(self.dimension) {
            let field = &mut self.fields[x][y];
            field.value = 0;
            field.is_clicked = false;
            field.is_flagged = false;

            self.change_pixbuf(x, y);
        }
        //place mines on random places
        let mut rng = rand::thread_rng();
        let num = Uniform::from(0..self.dimension);
        let mut mines = 0;
        while mines != self.mines_count {
            let i = num.sample(&mut rng) as usize;
            let j = num.sample(&mut rng) as usize;
            if !self.is_mine_on_field(i,j) {
                self.fields[i][j].value = -1;
                mines += 1;
            }
        }
        //calculate values of fields
        for (x, y) in FieldGenerator::new(self.dimension) {
            if !self.is_mine_on_field(x, y) {
                let mines = Adjacent::new(self.dimension, x, y).fold(0, |acc, (x, y)| if self.is_mine_on_field(x,y) {acc +1} else {acc});
                self.fields[x][y].value = mines;
            }
        }
    }

    fn click_field(board: &mut Board, x: usize, y: usize) {
        while board.click_counter == 0 && board.fields[x][y].value != 0 {
            board.init_fields();
        }

        if board.fields[x][y].is_flagged || board.game_over {
            return;
        }

        if board.is_mine_on_field(x,y) && !board.fields[x][y].is_flagged {
            //open all bombs, game over
            board.reveal_all_mines();
            println!("It's a bomb");
            return;
        }

        if (x, y) == board.bad_guy.position {
            board.bad_guy.is_active = true;
        }

        if !board.fields[x][y].is_clicked {
            board.click_counter += 1;
        }

        //if all adjacent mines are flagged, reveal,
        //if more than there are mines are flagged, do nothing
        //if less than there are mines are flagged, do nothing
        //if adjacent field has a mine that is not flagged, explode and end game
        let mines_flagged = Adjacent::new(board.dimension, x, y).fold(0, |acc, (x, y)| if board.fields[x][y].is_flagged {acc +1} else {acc});

        if (mines_flagged == board.fields[x][y].value && board.fields[x][y].is_clicked)
            || board.fields[x][y].value == 0
        {
            //recursive reveal adjacent fields without bombs
            let mut fields_to_traverse = VecDeque::new();
            fields_to_traverse.push_back((x, y));
            board.fields[x][y].is_clicked = true;

            for adj in Adjacent::new(board.dimension, x, y) {
                if !board.fields[adj.0][adj.1].is_clicked && !board.is_mine_on_field(adj.0, adj.1) {
                    fields_to_traverse.push_back(adj);
                    board.fields[adj.0][adj.1].is_clicked = true;
                } else if board.is_mine_on_field(adj.0, adj.1)
                    && !board.fields[adj.0][adj.1].is_flagged
                {
                    board.reveal_all_mines();
                    println!("not flagged rigth");
                    return;
                }
            }

            while let Some((x, y)) = fields_to_traverse.pop_front() {
                let current_field_value = board.fields[x][y].value;
                board.fields[x][y].is_clicked = true;

                if current_field_value == 0 {
                    for adj in Adjacent::new(board.dimension, x, y) {
                        if !board.fields[adj.0][adj.1].is_clicked {
                            fields_to_traverse.push_back(adj);
                            board.fields[adj.0][adj.1].is_clicked = true;
                        }
                    }
                }
                board.change_pixbuf(x, y);
            }
        }

        board.fields[x][y].is_clicked = true;
        board.change_pixbuf(x, y);

        let mut flag = true;
        for (x, y) in FieldGenerator::new(board.dimension) {
            if !board.fields[x][y].is_clicked && !board.is_mine_on_field(x, y) {
                flag = false;
            }
        }

        if flag {
            println!("you won");
            board.game_over = true;
            return;
        }
    }

    fn is_mine_on_field(&mut self, x: usize, y: usize) -> bool {
        self.fields[x][y].value == -1
    }

    fn reveal_all_mines(&mut self) {
        self.game_over = true;
        for (x, y) in FieldGenerator::new(self.dimension) {
            if self.is_mine_on_field(x, y) {
                self.change_pixbuf(x, y);
            }
        }
    }

    pub fn change_pixbuf(&mut self, x: usize, y: usize) {
        if self.bad_guy.position == (x, y) {
            self.bad_guy.is_active = self.click_counter != 0;
        }

        let field = &self.fields[x][y];
        let pb = if field.value == -1 && (self.game_over || field.is_clicked) && !field.is_flagged {
            self.pixbufs_container.get_bomb()
        } else if (x, y) == self.bad_guy.position && self.bad_guy.is_active {
            self.pixbufs_container.get_badguy()
        } else if field.is_clicked {
            self.pixbufs_container.get_numbered(field.value)
        } else if field.is_flagged {
            self.pixbufs_container.get_flag()
        } else {
            self.pixbufs_container.get_unopened()
        };

        if field.button.get_icon_widget().is_none() {
            let im = Image::from_pixbuf(pb);
            field.button.set_icon_widget(Some(&im));
        } else {
            field
                .button
                .get_icon_widget()
                .unwrap()
                .downcast::<gtk::Image>()
                .unwrap()
                .set_from_pixbuf(pb);
        }
    }

    pub fn move_bad_guy(&mut self) {
        let adj = Adjacent::new(
            self.dimension,
            self.bad_guy.position.0,
            self.bad_guy.position.1,
        );
        let iter: Vec<(usize, usize)> = adj
            .filter(|(x, y)| !self.fields[*x][*y].is_flagged)
            .collect();

        if iter.is_empty() {
            return;
        }

        let mut rng = rand::thread_rng();
        let num = Uniform::from(0..iter.len());
        let offset = iter[num.sample(&mut rng)];

        self.bad_guy.position = (offset.0, offset.1);
    }
}
