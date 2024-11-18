use std::{io, time::Duration};

use futures::{FutureExt, select, StreamExt};
use futures_timer::Delay;
use crossterm::{
    event::{EventStream, KeyCode, Event}
};

use super::settings::Settings;

use crate::tui;

pub struct Game {
    x: u16,
    y: u16,
    cursor_x: u16,
    cursor_y: u16,
    solution: Vec<Vec<u16>>,
    input: Vec<Vec<u16>>,
    settings: Settings,
    out: std::io::Stdout,
}

impl Game {
    pub fn new(ix: u16, iy: u16, iout: std::io::Stdout) -> Game {
        return Game {
            x: ix,
            y: iy,
            cursor_x: (ix / 2) + 1,
            cursor_y: (iy / 2) + 1,
            solution: vec![vec![0; ix as usize]; iy as usize],
            input: vec![vec![0; ix as usize]; iy as usize],
            settings: Settings::new(),
            out: iout,
        };
    }

    pub fn get_x(&self) -> u16 {
        self.x
    }

    pub fn get_y(&self) -> u16 {
        self.y
    }

    pub fn get_settings(&self) -> &Settings {
        &self.settings
    }

    fn handle_keycode(&self, keycode: KeyCode) -> bool {
        return match keycode {
            KeyCode::Esc => {
                true
            },
            _ => {
                false
            }
        }
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
                                println!("C pressed!");
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
        tui::setup(&self.out)?;
        tui::draw_board(&self.out, self);
        async_std::task::block_on(self.read_events());
        tui::cleanup(&self.out)?;
        Ok(())
    }
}

