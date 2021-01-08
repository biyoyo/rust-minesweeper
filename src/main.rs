extern crate gtk;
extern crate gdk_pixbuf;
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
        window.set_titlebar(Some(&header.cont));
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
        cont.set_title(Some("Minesweeper"));
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
                let im = Image::from_file("image.png");
                let button = ToolButton::new::<Image>(None, Some("aa"));
                row.pack_start(&button, false, false, 0);
                //handles left click
                button.connect_clicked(move |button| {button.set_label(Some("2"));});
                button.connect_button_press_event(move |widget, button| {
                    //handles double click
                    if button.get_button() == 1
                    {
                        widget.set_label(Some("2"));
                    }
                    //handles right click
                    else if button.get_button() == 3
                    {
                        widget.set_label(Some("3"));
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
