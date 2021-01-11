extern crate gtk;
use gtk::*;
use std::process;
use rand::distributions::{Distribution, Uniform};
use gdk_pixbuf::Pixbuf;

pub struct App{
    pub window:Window,
    pub header: Header,
    pub board: Board,
}

pub struct Header{
    pub cont: HeaderBar,
}

pub struct Board{
    pub container: Box,
    pub mine_field: Vec<Vec<i8>>,
    pub dimension: i8,
    pub buttons: Vec<Vec<Field>>,
}

#[derive (Clone)]
pub struct Field{
    pub button: ToolButton,
    pub is_clicked: bool,
    images: Images,
    value: i8,
}

#[derive (Clone)]
pub struct Images{
    pub pixbufs: Vec<Pixbuf>,
}

impl App {
    fn new() -> App{
        let window = Window::new(WindowType::Toplevel);
        let header = Header::new();
        window.set_titlebar(Some(&header.cont));
        window.set_title("Minesweeper");
    
        let board = Board::new(8);

        for row in &board.mine_field {
            println!("{:?}", row);
        }

        window.add(&board.container);

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
    fn new(dimension: i8) -> Board{

        let container = Box::new(Orientation::Vertical, 0);
        let mine_field = Board::init_mines(dimension);
        let mf = Vec::new(); 
        let mut board = Board{container, mine_field, dimension, buttons: mf};
        board.check_neighbours();
        board.buttons = board.init_fields(dimension, &board.mine_field);

        /*
        for x in 0..dimension
        {
            let row = Box::new(Orientation::Horizontal, 0);
            board.container.pack_start(&row, false, false, 0);
            for y in 0..dimension
            {
                //let pb = Pixbuf::from_file_at_size("image.png", 32, 32);
                let im = Image::from_pixbuf(Some(&board.images.pixbufs[7]));
                //let im = Image::from_file("image.png");
                //let img = Image::from_file("img.png");
                let button = ToolButton::new::<Image>(Some(&im), Some("aa"));
                row.pack_start(&button, false, false, 0);
                //let file_name = Board::get_field_image(board.mine_field[x as usize][y as usize]);
                //handles left click
                button.connect_clicked(|button| {
                    //button.set_label(Some("2"));
                    //im.set_from_file(file_name);
                    let value = board.mine_field[x as usize][y as usize] + 1;
                    //todo try to cast as image
                    button.get_icon_widget().unwrap().set_from_pixbuf(Some(&board.images.pixbufs[value as usize]))
                });
            }
        }
        */
        board
    }

    fn init_mines(dimension: i8) -> Vec<Vec<i8>>
    {
        let mut mine_field = Vec::<Vec<i8>>::new();
        let mut vec = Vec::new();
        vec.resize(dimension as usize, 0);
        mine_field.resize(dimension as usize, vec);

        let mut rng = rand::thread_rng();
        let num = Uniform::from(0..dimension);
        let mut mines = 0; 
        while mines != 10
        {
            let i = num.sample(&mut rng) as usize;
            let j = num.sample(&mut rng) as usize;
            if mine_field[i][j] != -1
            {
                mine_field[i][j] = -1;
                mines += 1;
            }
        }

        mine_field
    }

    fn init_fields(&self, dimension: i8, mine_field: &Vec<Vec<i8>>) -> Vec<Vec<Field>>
    {
        let mut fields : Vec<Vec<Field>> = Vec::new();
        fields.resize(dimension as usize, Vec::new());
        for x in 0..dimension
        {
            let row = Box::new(Orientation::Horizontal, 0);
            self.container.pack_start(&row, false, false, 0);
            for y in 0..dimension
            {
                //let pb = Pixbuf::from_file_at_size("image.png", 32, 32);
                //let im = Image::from_pixbuf(Some(&board.images.pixbufs[7]));
                //let im = Image::from_file("image.png");
                //let img = Image::from_file("img.png");
                let value = mine_field[x as usize][y as usize];
                let field = Field::new(value);
                fields[x as usize].push(field.clone());
                row.pack_start(&field.button, false, false, 0);
                //let file_name = Board::get_field_image(board.mine_field[x as usize][y as usize]);
                //handles left click
                /*
                button.connect_clicked(|button| {
                    //button.set_label(Some("2"));
                    //im.set_from_file(file_name);
                    let value = board.mine_field[x as usize][y as usize] + 1;
                    //todo try to cast as image
                    button.get_icon_widget().unwrap().set_from_pixbuf(Some(&board.images.pixbufs[value as usize]))
                });
                */
            }
        }
        fields
    }

    fn check_neighbours(&mut self)
    {
        for i in 0..self.dimension
        {
            for j in 0..self.dimension
            {
                if !self.mine_on_field(i, j)
                {
                    for k in &[-1, 0, 1]
                    {
                        for l in &[-1, 0, 1]
                        {
                            let r = *k + i;
                            let c = *l + j;
                            if self.check_valid_field(r, c) && self.mine_on_field(r, c)
                            {
                                self.mine_field[i as usize][j as usize] += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    fn check_valid_field(&self, x: i8, y: i8) -> bool
    {
        x >= 0 && x < self.dimension as i8 && y >= 0 && y < self.dimension as i8
    }

    fn mine_on_field(&self, x: i8, y:i8) -> bool
    {
        self.mine_field[x as usize][y as usize] == -1
    }

    fn get_field_image(value: i8) -> &'static str
    {
        match value {
            -1 => "mine.png",
            0 => "img.png",
            1 => "one.png",
            2 => "two.png",
            3 => "three.png",
            4 => "four.png",
            _ => "image.png",
        }
    }
}

impl Images{
    fn new() ->  Images
    {
        let mut pixbufs = Vec::new();
        let file_names = ["bomb.svg", "one.svg", "two.svg", "three.svg", "four.svg", "five.svg", "six.svg", "image.png"];

        for file in &file_names{
            pixbufs.push(Pixbuf::from_file_at_size(file, 48, 48).unwrap());
        }
        Images{pixbufs}
    }
}

impl Field{
    fn new(value: i8) -> Field{
        let button = ToolButton::new::<Image>(None, Some("field"));
        Field{button, is_clicked: false, images: Images::new(), value}
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
