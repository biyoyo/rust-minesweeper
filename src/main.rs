extern crate gtk;
use gtk::*;
use std::process;

pub struct App{
    pub window:Window,
    pub header: Header,
    pub board: Board,
}

pub struct Header{
    pub cont: HeaderBar,
}

pub struct Board{
    pub board: Box,
}

impl App {
    fn new() -> App{
        let window = Window::new(WindowType::Toplevel);
        let header = Header::new();
        window.set_titlebar(&header.cont);
        window.set_title("Minesweeper");
    
        let board = Board::new();
        window.add(&board.board);

        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });
        App{window, header, board}
    }
}

impl Header{
    fn new() -> Header{
        let cont = HeaderBar::new();
        cont.set_title("Minesweeper");
        cont.set_show_close_button(true);
        Header{cont}
    }
}

impl Board{
    fn new() -> Board{

        let board = Box::new(Orientation::Vertical, 0);
        for _x in 0..10
        {
            let row = Box::new(Orientation::Horizontal, 0);
            board.pack_start(&row, false, false, 0);
            for _y in 0..10
            {
                let button = Button::new_with_label("1");
                row.pack_start(&button, false, false, 0);
                button.connect_button_press_event(move |widget, button| {
                    if button.get_button() == 1
                    {
                        widget.set_label("2");
                    }
                    else if button.get_button() == 3
                    {
                        widget.set_label("3");
                    }
                    Inhibit(false)
                });
            }
        }
        Board{board}
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
