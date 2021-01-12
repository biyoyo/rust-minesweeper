extern crate gtk;
use crate::gtk::prelude::Cast;
use gdk_pixbuf::Pixbuf;
use gtk::*;
use rand::distributions::{Distribution, Uniform};
use std::process;
use std::cell::RefCell;
use std::rc::Rc;

pub struct App {
    pub window: Window,
    pub header: Header,
    pub board: Rc<RefCell<Board>>,
}

pub struct Header {
    pub cont: HeaderBar,
}

pub struct Board {
    pub container: Box,
    pub mine_field: Vec<Vec<i8>>,
    pub dimension: i8,
    pub fields: Vec<Vec<Field>>,
    pub pixbufs: Vec<Pixbuf>,
}

#[derive(Clone)]
pub struct Field {
    pub button: ToolButton,
    pub is_clicked: bool,
    value: i8,
}

impl App {
    fn new() -> App {
        let window = Window::new(WindowType::Toplevel);
        let header = Header::new();
        window.set_titlebar(Some(&header.cont));
        window.set_title("Minesweeper");

        let board_rc = Board::new(8);
        let board = board_rc.borrow();

        for row in &board.mine_field {
            println!("{:?}", row);
        }

        window.add(&board.container);

        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });
        App {
            window,
            header,
            board: board_rc.clone(),
        }
    }
}

impl Header {
    fn new() -> Header {
        let cont = HeaderBar::new();
        cont.set_title(Some("Minesweeper"));
        cont.set_show_close_button(true);
        Header { cont }
    }
}

impl Board {
    fn new(dimension: i8) -> Rc<RefCell<Board>> {
        let container = Box::new(Orientation::Vertical, 0);
        let mine_field = Board::init_mines(dimension);
        let board_rc = Rc::new(RefCell::new(Board {
            container,
            mine_field,
            dimension,
            fields: Vec::new(),
            pixbufs: Board::load_icons(),
        }));
        let mut board = board_rc.borrow_mut();
        board.check_neighbours();

        let mut fields: Vec<Vec<Field>> = Vec::new();
        fields.resize(dimension as usize, Vec::new());
        for x in 0..dimension {
            let row = Box::new(Orientation::Horizontal, 0);
            board.container.pack_start(&row, false, false, 0);
            for y in 0..dimension {
                let value = board.mine_field[x as usize][y as usize];
                let field = Field::new(value);
                fields[x as usize].push(field.clone());
                row.pack_start(&field.button, false, false, 0);
            }
        }

        for x in 0..dimension as usize {
            for y in 0..dimension as usize {
                let value_clone = fields[x][y].value.clone();
                let i_clone = board.pixbufs[(value_clone + 1) as usize].clone();

                let board_clone = board_rc.clone();

                fields[x][y].button.connect_clicked(move |button| {
                    let mut board = board_clone.borrow_mut();
                    board.fields[x][y].is_clicked = true;
                    if value_clone == -1 {
                        //open all bombs, game over
                        Board::explode(&board);
                        println!("Issa bomb");
                    }
                    button
                        .get_icon_widget()
                        .unwrap()
                        .downcast::<gtk::Image>()
                        .unwrap()
                        .set_from_pixbuf(Some(&i_clone));
                });

                /* todo handle right click
                field[x][y].button.connect_key_press_event(|button| {
                    
                });
                */
            }
        }
        board.fields = fields;

        board_rc.clone()
    }

    fn init_mines(dimension: i8) -> Vec<Vec<i8>> {
        let mut mine_field = Vec::<Vec<i8>>::new();
        let mut vec = Vec::new();
        vec.resize(dimension as usize, 0);
        mine_field.resize(dimension as usize, vec);

        let mut rng = rand::thread_rng();
        let num = Uniform::from(0..dimension);
        let mut mines = 0;
        while mines != 10 {
            let i = num.sample(&mut rng) as usize;
            let j = num.sample(&mut rng) as usize;
            if mine_field[i][j] != -1 {
                mine_field[i][j] = -1;
                mines += 1;
            }
        }

        mine_field
    }

    fn check_neighbours(&mut self) {
        for i in 0..self.dimension {
            for j in 0..self.dimension {
                if !self.mine_on_field(i, j) {
                    for k in &[-1, 0, 1] {
                        for l in &[-1, 0, 1] {
                            let r = *k + i;
                            let c = *l + j;
                            if self.check_valid_field(r, c) && self.mine_on_field(r, c) {
                                self.mine_field[i as usize][j as usize] += 1;
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
            "unopened.svg",
            "one.svg",
            "two.svg",
            "three.svg",
            "four.svg",
            "five.svg",
            "six.svg",
        ];

        for file in &file_names {
            pixbufs.push(Pixbuf::from_file_at_size(format!("icons/{}", file), 48, 48).unwrap());
        }
        pixbufs
    }

    fn check_valid_field(&self, x: i8, y: i8) -> bool {
        x >= 0 && x < self.dimension as i8 && y >= 0 && y < self.dimension as i8
    }

    fn mine_on_field(&self, x: i8, y: i8) -> bool {
        self.mine_field[x as usize][y as usize] == -1
    }

    fn explode(board: &Board)
    {
        for (i, _) in board.mine_field.iter().enumerate()
        {
            for (j, _) in board.mine_field[i].iter().enumerate()
            {
                if board.mine_field[i][j] == -1
                {
                    board.fields[i][j].button
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

impl Field {
    fn new(value: i8) -> Field {
        let im = Image::from_pixbuf(Some(
            &Pixbuf::from_file_at_size("icons/unopened.svg", 48, 48).unwrap(),
        ));
        let button = ToolButton::new::<Image>(Some(&im), Some("field"));
        Field {
            button,
            is_clicked: false,
            value,
        }
    }
}

fn main() {
    if gtk::init().is_err() {
        eprintln!("failed to initialize GTK Application");
        process::exit(1);
    }
    let app = App::new();
    app.window.show_all();
    gtk::main();
}
