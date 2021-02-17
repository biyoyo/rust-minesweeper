extern crate gtk;
use crate::board::Board;
use glib::Continue;
use gtk::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct SidePanel {
    pub container: Box,
}

impl SidePanel {
    pub fn new(board: Rc<RefCell<Board>>) -> SidePanel {
        let container = Box::new(Orientation::Vertical, 30);
        container.set_margin_top(20);
        container.set_margin_start(20);
        container.set_margin_end(20);

        let bc = board.clone();

        let b = Button::with_label("restart game");
        b.connect_clicked(move |_| {
            bc.borrow_mut().init_fields();
            bc.borrow_mut().seconds_elapsed = 0;
            bc.borrow_mut().flags_placed = 0;
            bc.borrow_mut().game_over = false;
        });
        container.pack_start(&b, false, false, 0);

        let label = Label::new(Some("00:00"));
        container.pack_start(&label, false, false, 0);

        let flags_label = Label::new(Some("0/10"));
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
                label.set_markup(&time_elapsed);
                flags_label.set_label(format!("{}/10", b.flags_placed).as_str());
                b.seconds_elapsed += 1;
            }
            Continue(true)
        });

        SidePanel { container }
    }
}
