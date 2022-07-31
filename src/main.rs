mod app;
mod core;
mod ui;

use crate::core::Event;
use std::io::stdin;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use std::{error::Error, io};
use app::App;
use termion::input::TermRead;
use termion::event::Key::Char;
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

    let (tx, rx) = mpsc::channel();
    //TODO: tick_rate could be set from the args and/or from a cfg file
    let tick_rate = Duration::from_millis(DEFAULT_TICK_RATE);
    let mut should_update = false;

    // Off-thread input event loop
    thread::spawn(move || {
        let stdin = stdin();
        let mut keys = stdin.keys();
        let mut last_tick = Instant::now();

        loop {
            let key = keys.next();

            if let Some(key) = key {
                match key {
                    Ok(key) => tx.send(Event::Input(key)).unwrap(),
                    Err(error) => eprint!("Error reading key from input: {}", error),
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let mut app = App::new();

    loop {
        // render the current state of the terminal on the main thread
        terminal.draw(|frame| {
            app.render(frame);
        })?;

        // input handling
        match rx.recv()? {
            Event::Input(key) => match key {
                termion::event::Key::Char('q') => break,
                Char(ch) => app.switch_tab(ch),
                _ => (),
            },
            Event::Tick => should_update = true,
        }
    }

    let _ = terminal.clear();
    return Ok(());
}
