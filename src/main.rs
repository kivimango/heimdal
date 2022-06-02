use std::{io, error::Error};
use tui::{backend::TermionBackend, Terminal};
use termion::{raw::IntoRawMode, input::MouseTerminal, screen::AlternateScreen};

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut _terminal = Terminal::new(backend)?;
    Ok(())
}
