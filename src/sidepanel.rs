extern crate gtk;
use glib::Continue;
use gtk:: {Box, Label, Orientation, Button};
use gtk:: {BoxExt, LabelExt, ButtonExt, WidgetExt, ContainerExt};
use std::cell::RefCell;
use std::rc::Rc;

use crate::board::Board;
use crate::bad_guy::BadGuy;

pub struct SidePanel {
    pub container: Box,
}

impl SidePanel {
    pub fn new(board: Rc<RefCell<Board>>) -> SidePanel {
        let container = Box::new(Orientation::Vertical, 30);
        container.set_margin_top(20);
        container.set_margin_start(20);
        container.set_margin_end(20);

        let label = Label::new(None);
        label.set_markup(&format!("<span font-family='monospace'>New game</span>").to_string());

        let b = Button::new();
        b.add(&label);

        let bc = board.clone();
        b.connect_clicked(move |_| {
            let mut board = bc.borrow_mut();
            board.seconds_elapsed = 0;
            board.flags_placed = 0;
            board.game_over = false;
            board.click_counter = 0;
            board.bad_guy = BadGuy::new(board.dimension);
            board.init_fields();
            println!("bad_guy on: {}", board.bad_guy.is_active);
        });
        container.pack_start(&b, false, false, 0);

        let label = Label::new(None);
        label.set_markup(&format!("<span font-family='monospace'>Time elapsed:</span>").to_string());
        container.pack_start(&label, false, false, 0);

        let time_label = Label::new(None);
        time_label.set_markup(&format!("<span font-family='monospace'>00:00</span>").to_string());
        container.pack_start(&time_label, false, false, 0);

        let label = Label::new(None);
        label.set_markup(&format!("<span font-family='monospace'>Flags:</span>").to_string());
        container.pack_start(&label, false, false, 0);

        let flags_label = Label::new(None);
        flags_label.set_markup(&format!("<span font-family='monospace'>0/{mines}</span>", mines = board.borrow().mines_count).to_string());
        container.pack_start(&flags_label, false, false, 0);

        let bc = board.clone();

        glib::timeout_add_seconds_local(1, move || {
            let mut b = bc.borrow_mut();
            if !b.game_over {
                let a = b.seconds_elapsed;
                let seconds = if (a % 60) >= 10 {
                    (a % 60).to_string()
                } else {
                    format!("0{}", a % 60).to_string()
                };
                let time_elapsed = format!(
                    "<span font-family='monospace'>{d}{m}:{s}</span>",
                    d = a / 600,
                    m = a / 60,
                    s = seconds
                )
                .to_string();
                time_label.set_markup(&time_elapsed);

                let flags_placed = format!(
                    "<span font-family='monospace'>{flags}/{mines}</span>",
                    flags = b.flags_placed,
                    mines = b.mines_count
                );
                flags_label.set_markup(&flags_placed.to_string());
                b.seconds_elapsed += 1;
            }
            Continue(true)
        });

        SidePanel { container }
    }
}
