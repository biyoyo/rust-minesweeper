extern crate gtk;
use gtk::{Window, Orientation, WindowType, HeaderBar, Box};
use gtk::{GtkWindowExt, ContainerExt, WidgetExt, HeaderBarExt, BoxExt};
use gtk::{main_quit, Inhibit};
use std::cell::RefCell;
use std::process;
use std::rc::Rc;
use crate::board::Board;

mod board;
mod sidepanel;
mod field;

pub struct App {
    pub window: Window,
    pub header: Header,
    pub board: Rc<RefCell<Board>>,
}

pub struct Header {
    pub cont: HeaderBar,
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

        for x in 0..8 {
            for y in 0..8 {
                print!("{} ", board.fields[x][y].value);
            }
            println!("");
        }

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

fn main() {
    if gtk::init().is_err() {
        eprintln!("failed to initialize GTK Application");
        process::exit(1);
    }
    let app = App::new();
    app.window.show_all();
    gtk::main();
}
