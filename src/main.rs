extern crate gtk;
use crate::gtk::prelude::Cast;
use gdk_pixbuf::Pixbuf;
use gtk::*;
use rand::distributions::{Distribution, Uniform};
use std::cell::RefCell;
use std::process;
use std::rc::Rc;

mod sidepanel;

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
    pub dimension: i8,
    pub fields: Vec<Vec<Field>>,
    pub pixbufs: Vec<Pixbuf>,
    pub game_over: bool,
    pub seconds_elapsed: i32,
}

#[derive(Clone)]
pub struct Field {
    pub button: ToolButton,
    pub is_clicked: bool,
    pub value: i8,
}

impl App {
    fn new() -> App {
        let window = Window::new(WindowType::Toplevel);
        let header = Header::new();
        window.set_titlebar(Some(&header.cont));
        window.set_title("Minesweeper");

        let main_container = Box::new(Orientation::Horizontal, 0);

        let board_rc = Board::new(8);
        let board = board_rc.borrow();

        main_container.pack_start(&board.container, false, false, 0);

        let sp = sidepanel::SidePanel::new(board_rc.clone());
        main_container.pack_start(&sp.container, false, false, 0);

        window.add(&main_container);

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

        let board_rc = Rc::new(RefCell::new(Board {
            container,
            dimension,
            fields: Vec::new(),
            pixbufs: Board::load_icons(),
            game_over: false,
            seconds_elapsed: 0,
        }));

        let mut board = board_rc.borrow_mut();
        board.create_fields();
        board.init_fields();

        for x in 0..dimension as usize {
            for y in 0..dimension as usize {
                let value_clone = board.fields[x][y].value.clone();
                let i_clone = board.pixbufs[(value_clone + 1) as usize].clone();

                let board_clone = board_rc.clone();

                board.fields[x][y].button.connect_clicked(move |button| {
                    let mut board = board_clone.borrow_mut();
                    if value_clone == -1 {
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
                        .set_from_pixbuf(Some(&i_clone));
                    if board.fields[x][y].is_clicked == true {
                        //recursive reveal adjacent fields without bombs
                    }
                    board.fields[x][y].is_clicked = true;
                });

                let board_clone = board_rc.clone();

                board.fields[x][y].button.connect_button_press_event(move |button, event| {
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
        //board.fields = fields;

        board_rc.clone()
    }

    fn create_fields(&mut self) {
        let mut fields: Vec<Vec<Field>> = Vec::new();
        fields.resize(self.dimension as usize, Vec::new());

        for x in 0..self.dimension {
            let row = Box::new(Orientation::Horizontal, 0);
            self.container.pack_start(&row, false, false, 0);
            for _y in 0..self.dimension {
                let field = Field::new(&self.pixbufs[self.pixbufs.len()-2]);
                fields[x as usize].push(field.clone());
                row.pack_start(&field.button, false, false, 0);
            }
        }

        self.fields = fields;
    }

    fn init_fields(&mut self) {
        //add starting images
        for row in &mut self.fields {
            for elem in row {
                elem.button.get_icon_widget()
                        .unwrap()
                        .downcast::<gtk::Image>()
                        .unwrap()
                        .set_from_pixbuf(Some(&self.pixbufs[self.pixbufs.len()-2]));
                elem.value = 0;
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

impl Field {
    fn new(pixbuf: &Pixbuf) -> Field {
        let im = Image::from_pixbuf(Some(&pixbuf));
        let button = ToolButton::new::<Image>(Some(&im), Some("field"));
        Field {
            button,
            is_clicked: false,
            value: 0,
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
