use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
mod terminal;
use terminal::Terminal;
use std::io::Error;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn new() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialise().unwrap();
        let runner = self.repl();
        Terminal::terminate().unwrap();
        runner.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }

    fn draw_tilde() -> Result<(), Error> {
        let height = Terminal::size()?.1;
        for _row in 0..(height-1) {
            Terminal::print("~")?;
            Terminal::print("\r\n")?;

        }
        Terminal::print("~")?;
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        if self.should_quit {
            Terminal::clear_screen()?;
            println!("Quitting already...");
        }
        else {
            Terminal::hide_cursor()?;
            Self::draw_tilde()?;
            Terminal::move_cursor_to(0, 0)?;
            Terminal::show_cursor()?;
        }
        Ok(())
    }
}
