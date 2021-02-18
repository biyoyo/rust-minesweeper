extern crate gtk;
use crate::gtk::prelude::Cast;
use gtk::*;
use rand::distributions::{Distribution, Uniform};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::adjacent::Adjacent;
use crate::field::Field;
use crate::pixbufs::Pixbufs;

pub struct Board {
    pub container: Box,
    pub dimension: i8,
    pub fields: Vec<Vec<Field>>,
    pub pixbufs_container: Pixbufs,
    pub game_over: bool,
    pub seconds_elapsed: i32,
    pub flags_placed: i8,
    pub click_counter: i32,
}

impl Board {
    pub fn new(dimension: i8) -> Rc<RefCell<Board>> {
        let container = Box::new(Orientation::Vertical, 0);

        let board_rc = Rc::new(RefCell::new(Board {
            container,
            dimension,
            fields: Vec::new(),
            pixbufs_container: Pixbufs::new(),
            game_over: false,
            seconds_elapsed: 0,
            flags_placed: 0,
            click_counter: 0,
        }));

        let mut board = board_rc.borrow_mut();
        board.create_fields();
        board.init_fields();

        for x in 0..dimension as usize {
            for y in 0..dimension as usize {
                let board_clone = board_rc.clone();

                board.fields[x][y].button.connect_clicked(move |_button| {
                    let mut board = board_clone.borrow_mut();

                    while board.click_counter == 0 && board.fields[x][y].value != 0 {
                        board.init_fields();
                    }

                    if board.fields[x][y].is_flagged {
                        return;
                    }

                    let value_on_button = board.fields[x][y].value;

                    if value_on_button == -1 && !board.fields[x][y].is_flagged {
                        //open all bombs, game over
                        board.game_over = true;
                        board.reveal_all_mines();
                        println!("Issa bomb");
                        return;
                    }

                    board.fields[x][y].is_clicked = true;
                    board.click_counter += 1;
                    board.change_pixbuf(x, y);

                    //if all adjacent mines are flagged, reveal,
                    //if more than there are mines are flagged, do nothing
                    //if less than there are mines are flagged, do nothing
                    //if adjacent field has a mine that is not flagged, explode and end game
                    let mut mines_flagged = 0;
                    for adj in Adjacent::new(board.dimension, x, y) {
                        mines_flagged += board.fields[adj.0][adj.1].is_flagged as i8;
                    }
                    if mines_flagged == value_on_button || value_on_button == 0 {
                        //recursive reveal adjacent fields without bombs
                        let mut fields_to_traverse = VecDeque::new();
                        fields_to_traverse.push_back((x, y));

                        for adj in Adjacent::new(board.dimension, x, y) {
                            if !board.fields[adj.0][adj.1].is_clicked
                                && !board.is_mine_on_field(adj.0, adj.1)
                            {
                                fields_to_traverse.push_back(adj);
                                board.fields[adj.0][adj.1].is_clicked = true;
                                board.click_counter += 1;
                            } else if board.is_mine_on_field(adj.0, adj.1)
                                && !board.fields[adj.0][adj.1].is_flagged
                            {
                                board.game_over = true;
                                board.reveal_all_mines();
                                println!("not flagged rigth bih");
                                return;
                            }
                        }

                        while let Some((x, y)) = fields_to_traverse.pop_front() {
                            let current_field_value = board.fields[x][y].value;
                            board.fields[x][y].is_clicked = true;
                            board.click_counter += 1;

                            if current_field_value == 0 {
                                for adj in Adjacent::new(board.dimension, x, y) {
                                    if !board.fields[adj.0][adj.1].is_clicked {
                                        fields_to_traverse.push_back(adj);
                                        board.fields[adj.0][adj.1].is_clicked = true;
                                        board.click_counter += 1;
                                    }
                                }
                            }
                            board.change_pixbuf(x, y);
                        }
                    }
                });

                let board_clone = board_rc.clone();

                //handles right click
                board.fields[x][y]
                    .button
                    .connect_button_press_event(move |_button, event| {
                        let mut board = board_clone.borrow_mut();

                        if event.get_button() == 3
                            && board.flags_placed < 10
                            && !board.fields[x][y].is_clicked
                        {
                            let flag = board.fields[x][y].is_flagged;
                            //if unflag reduce count
                            board.flags_placed += -1 * flag as i8 + !flag as i8;
                            board.fields[x][y].is_flagged = !flag;
                            board.change_pixbuf(x, y);
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
                let field = Field::new();
                fields[x as usize].push(field.clone());
                row.pack_start(&field.button, false, false, 0);
            }
        }

        self.fields = fields;
    }

    pub fn init_fields(&mut self) {
        //add starting images
        for x in 0..self.dimension {
            for y in 0..self.dimension {
                let field = &mut self.fields[x as usize][y as usize];
                field.value = 0;
                field.is_clicked = false;
                field.is_flagged = false;

                self.change_pixbuf(x as usize, y as usize);
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
                if !self.is_mine_on_field(i as usize, j as usize) {
                    for adj in Adjacent::new(self.dimension, i as usize, j as usize) {
                        if self.is_mine_on_field(adj.0, adj.1) {
                            self.fields[i as usize][j as usize].value += 1;
                        }
                    }
                }
            }
        }
    }

    fn is_mine_on_field(&mut self, x: usize, y: usize) -> bool {
        self.fields[x][y].value == -1
    }

    fn reveal_all_mines(&mut self) {
        for x in 0..self.dimension {
            for y in 0..self.dimension {
                if self.fields[x as usize][y as usize].value == -1 {
                    self.change_pixbuf(x as usize, y as usize);
                }
            }
        }
    }

    pub fn change_pixbuf(&mut self, x: usize, y: usize) {
        let field = &self.fields[x][y];
        let pb = if field.value == -1 && (self.game_over || field.is_clicked) && !field.is_flagged {
            self.pixbufs_container.get_bomb()
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tests() {
        let mut adj = Adjacent::new(4, 1, 1);
        assert_eq!(adj.next(), Some((0, 0)));
    }
}
