extern crate gtk;
use crate::gtk::prelude::Cast;
use gdk_pixbuf::Pixbuf;
use glib::Continue;
use gtk::*;
use rand::distributions::{Distribution, Uniform};
use std::cell::RefCell;
use std::process;
use std::rc::Rc;
//use gtk::prelude::*;

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
}

#[derive(Clone)]
pub struct Field {
    pub button: ToolButton,
    pub is_clicked: bool,
    pub value: i8,
}

pub struct SidePanel {
    pub container: Box,
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

        let sp = SidePanel::new();
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
        }));

        let mut board = board_rc.borrow_mut();
        let mut fields = board.init_mines();
        Board::check_neighbours(&mut fields);

        for x in 0..dimension as usize {
            for y in 0..dimension as usize {
                let value_clone = fields[x][y].value.clone();
                let i_clone = board.pixbufs[(value_clone + 1) as usize].clone();

                let board_clone = board_rc.clone();

                fields[x][y].button.connect_clicked(move |button| {
                    let mut board = board_clone.borrow_mut();
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
                    if board.fields[x][y].is_clicked == true {
                        //recursive reveal adjacent fields without bombs
                    }
                    board.fields[x][y].is_clicked = true;
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

    fn init_mines(&mut self) -> Vec<Vec<Field>> {
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

        //place mines on random places
        let mut rng = rand::thread_rng();
        let num = Uniform::from(0..self.dimension);
        let mut mines = 0;
        while mines != 10 {
            let i = num.sample(&mut rng) as usize;
            let j = num.sample(&mut rng) as usize;
            if fields[i][j].value != -1 {
                fields[i][j].value = -1;
                mines += 1;
            }
        }

        fields
    }

    fn check_neighbours(fields: & mut Vec<Vec<Field>>) {
        let dimension = fields.len() as i8;
        for i in 0..dimension {
            for j in 0..dimension {
                if !Board::mine_on_field(fields.to_vec(), i, j) {
                    for k in &[-1, 0, 1] {
                        for l in &[-1, 0, 1] {
                            let r = *k + i;
                            let c = *l + j;
                            if Board::check_valid_field(dimension, r, c) && Board::mine_on_field(fields.to_vec(), r, c) {
                                fields[i as usize][j as usize].value += 1;
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

    fn check_valid_field(dimension: i8, x: i8, y: i8) -> bool {
        x >= 0 && x < dimension as i8 && y >= 0 && y < dimension as i8
    }

    fn mine_on_field(fields: Vec<Vec<Field>>, x: i8, y: i8) -> bool {
        fields[x as usize][y as usize].value == -1
    }

    fn explode(board: &Board) {
        for (i, _) in board.fields.iter().enumerate() {
            for (j, _) in board.fields[i].iter().enumerate() {
                if board.fields[i][j].value == -1 {
                    board.fields[i][j]
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
    fn new() -> Field {
        let im = Image::from_pixbuf(Some(
            &Pixbuf::from_file_at_size("icons/unopened.svg", 48, 48).unwrap(),
        ));
        let button = ToolButton::new::<Image>(Some(&im), Some("field"));
        Field {
            button,
            is_clicked: false,
            value: 0,
        }
    }
}

impl SidePanel {
    fn new() -> SidePanel {
        let container = Box::new(Orientation::Vertical, 30);
        container.set_margin_top(20);
        container.set_margin_start(20);
        container.set_margin_end(20);
        let b = Button::with_label("hello");
        container.pack_start(&b, false, false, 0);

        let mut a = 0;
        let label = Label::new(None);
        container.pack_start(&label, false, false, 0);

        glib::timeout_add_seconds_local(
            1,
            move || {
                let seconds = if a >= 10 {(a%60).to_string()} else {format!("0{a}", a = a).to_string()};
                let time_elapsed = format!("<span font-family='monospace'>{d}{m}:{s}</span>", d = a / 600, m = a / 60, s = seconds).to_string();
                label.set_markup(&time_elapsed);
                a += 1;
                Continue(true)
            },
        );

        SidePanel { container }
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
