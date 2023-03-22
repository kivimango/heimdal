mod app;
mod core;
mod events;
mod ui;

use crate::core::Event;
use app::App;
use events::Events;
use std::time::Duration;
use std::{error::Error, io};
use termion::event::Key;
use termion::raw::IntoRawMode;
use tui::{backend::TermionBackend, Terminal};

/// Emit a tick event by every 200 ms (60fps).
/// You can tune the responsiveness of the application.
/// But setting it too low also means that this loop will run a lot and eat up resources.
pub const DEFAULT_TICK_RATE: u64 = 200;

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let _ = terminal.clear();

    //TODO: tick_rate could be set from the args and/or from a cfg file
    let tick_rate = Duration::from_millis(DEFAULT_TICK_RATE);
    let mut should_update = false;
    let events = Events::new();
    let mut app = App::new();

    loop {
        // render the current state of the terminal on the main thread
        terminal.draw(|frame| {
            app.render(frame);
        })?;

        // input handling
        if let Ok(event) = events.recv() {
            match event {
                Event::Input(key) => match key {
                    Key::Char('q') => break,
                    // termion does not have a separate Tab Key like Backspace, it handled as a '\t' char
                    termion::event::Key::Backspace => app.previous_tab(),
                    Key::Up | Key::Down | Key::Left | Key::Right => app.handle_arrow_keys(key),
                    Key::Char(ch) => app.switch_tab(ch),
                    _ => (),
                },
                Event::Tick => {
                    app.tick();
                }
            }
        }
    }

    let _ = terminal.clear();
    return Ok(());
}
