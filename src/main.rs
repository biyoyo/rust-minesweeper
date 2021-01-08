extern crate gtk;
use gtk::*;
use std::process;
use rand::distributions::{Distribution, Uniform};

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
    pub dimension: u8,
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
    fn new(dimension: u8) -> Board{

        let container = Box::new(Orientation::Vertical, 0);
        let mine_field = Board::init_field(dimension);
        for x in 0..mine_field.len()
        {
            let row = Box::new(Orientation::Horizontal, 0);
            container.pack_start(&row, false, false, 0);
            for y in 0..mine_field[x].len()
            {
                let im = Image::from_file("image.png");
                //let img = Image::from_file("img.png");
                let button = ToolButton::new::<Image>(Some(&im), Some("aa"));
                row.pack_start(&button, false, false, 0);
                let mut fl = false;
                if mine_field[x][y] == -1
                {
                    fl = true;
                }
                //handles left click
                button.connect_clicked(move |_button| {
                    if fl == true
                    {
                        //button.set_label(Some("2"));
                        im.set_from_file("mine.png");
                    }
                });
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

        let mut board = Board{container, mine_field, dimension};
        board.check_neighbours();

        board
    }

    fn init_field(dimension: u8) -> Vec<Vec<i8>>
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

    fn check_neighbours(&mut self)
    {
        for i in 0..self.dimension
        {
            for j in 0..self.dimension
            {
                if !self.mine_on_field(i as i8, j as i8)
                {
                    for k in &[-1, 0, 1]
                    {
                        for l in &[-1, 0, 1]
                        {
                            let r = *k + i as i8;
                            let c = *l + j as i8;
                            if self.check_valid_field(r, c) && self.mine_on_field(r, c)
                            {
                                self.mine_field[i as usize][j as usize] += 1;
                            }
                        }
                    }
                }
            }
        }
        /*
        self.mine_field.iter().enumerate().map(|(i, x)|
        {
            x.iter().enumerate().map(|(j, _)|
            {
                if self.check_valid_field(i as i8, j as i8)
                {
                    self.mine_field[i][j] +=1;
                }
            });
        });
        */
        /*
        for (i, _) in self.mine_field.iter().enumerate()
        {
            for (j, _) in self.mine_field[i].iter().enumerate()
            {
                for k in -1..1
                {
                    for l in -1..1
                    {
                        let r = k + i as i8;
                        let c = l + j as i8;
                        if self.check_valid_field(r, c) && self.mine_on_field(r, c)
                        {
                            self.mine_field[i][j] += 1
                        }
                    }
                }
            }
        }
        */
    }

    fn check_valid_field(&self, x: i8, y: i8) -> bool
    {
        x >= 0 && x < self.dimension as i8 && y >= 0 && y < self.dimension as i8
    }

    fn mine_on_field(&self, x: i8, y:i8) -> bool
    {
        self.mine_field[x as usize][y as usize] == -1
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
