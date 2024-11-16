use std::{io::{self, stdout}, time::Duration};

use futures::{FutureExt, select, StreamExt};
use futures_timer::Delay;
use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode},
    event::{EventStream, KeyCode, Event}
};

use settings::Settings;

use crate::tui;

mod settings;

pub struct Game {
    x: i16,
    y: i16,
    settings: Settings
}

impl Game {
    pub fn new(ix: i16, iy: i16) -> Game {
        return Game {
            x: ix,
            y: iy, 
            settings: Settings::new(),
        };
    }

    pub fn calculate_canvas_size(&self) -> (i16, i16) {
        let ibx = (self.x - 1) * self.settings.get_inner_border_vertical();
        let obx = 2 * self.settings.get_outer_border_vertical();
        let iby = (self.y - 1) * self.settings.get_inner_border_horizontal();
        let oby = 2 * self.settings.get_outer_border_horizontal();
        let size_x = (self.x * self.settings.get_field_width()) + ibx + obx;
        let size_y = (self.y * self.settings.get_field_height()) + iby + oby;

        return (size_x, size_y);
    }

    async fn read_events(&self) {
        let mut reader = EventStream::new();

        loop {
            let mut delay = Delay::new(Duration::from_secs(1_000)).fuse();
            let mut event = reader.next().fuse();

            select! {
                _ = delay => { println!(".\r"); },
                maybe_event = event => {
                    match maybe_event {
                        Some(Ok(event)) => {
                            //println!("Event: {:?}\r", event);

                            if event == Event::Key(KeyCode::Char('c').into()) {
                                //println!("C pressed!");
                                tui::draw_vertical_line(&std::io::stdout(), '|', 7, 3, 5);
                                tui::flush(&std::io::stdout());
                            }

                            if event == Event::Key(KeyCode::Esc.into()) {
                                break;
                            }
                        }
                        Some(Err(e)) => {
                            println!("Error: {:?}\r", e)
                        }
                        None => break,
                    }
                }
            };
        }
    }

    pub fn start(&self) -> io::Result<()> {
        async_std::task::block_on(self.read_events());
        Ok(())
    }
}

