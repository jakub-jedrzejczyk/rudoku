use std::{io::Stdout, time::Duration};

use futures::{FutureExt, select, StreamExt};
use futures_timer::Delay;
use crossterm::{
    event::{EventStream, KeyCode, Event, KeyModifiers}
};

use crate::logic::game::Game;
use crate::options::options::Options;
use crate::tui::terminal::Terminal;


pub struct App {
    game: Game,
    settings: Options,
    terminal: Terminal,
    stdout: Stdout,
}

impl App {
    pub fn new(game: Game, settings: Options, terminal: Terminal, stdout: Stdout) -> App {
        return App {
            game,
            settings,
            terminal,
            stdout
        }
    }

    fn handle_keycode(&self, keycode: KeyCode, modifiers: KeyModifiers) -> bool {
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
                _ = delay => {
                    println!(".\r");
                },
                maybe_event = event => {
                    match maybe_event {
                        Some(Ok(event)) => {
                            match event {
                                Event::Key(event) => {
                                    let close_app: bool = self.handle_keycode(event.code, event.modifiers);
                                    if close_app {
                                        break;
                                    }
                                },
                                Event::Resize(width, height) => {

                                },
                                _ => break,
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

    pub fn start(&mut self) {
        self.terminal.setup().unwrap();
        async_std::task::block_on(self.read_events());
        self.terminal.cleanup().unwrap();
    }
}
