use crate::core::Event;
use std::{
    io::stdin,
    sync::mpsc::{self, Receiver, TryRecvError},
    thread,
    time::{Duration, Instant},
};
use termion::event::Key;
use termion::input::TermRead;

pub struct Events {
    rx: Receiver<Event<Key>>,
}

impl Events {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let input_tx = tx.clone();

        // Off-thread input event loop
        thread::spawn(move || {
            let stdin = stdin();

            for key in stdin.keys().flatten() {
                if let Err(error) = input_tx.send(Event::Input(key)) {
                    eprint!("Error reading key from input: {}", error);
                }
            }
        });
        // Off-thread tick event loop
        thread::spawn(move || {
            let wait_time = Duration::from_secs(1);
            let mut start = Instant::now();
            loop {
                let elapsed = start.elapsed();
                if elapsed > wait_time {
                    if let Err(error) = tx.send(Event::Tick) {
                        eprint!("Error sending tick event: {}", error);
                    }
                    start = Instant::now();
                }
            }
        });
        Events { rx }
    }

    pub fn recv(&self) -> Result<Event<Key>, TryRecvError> {
        self.rx.try_recv()
    }
}
 