use std::io::Stdout;

use termion::raw::RawTerminal;
use tui::backend::TermionBackend;
use tui::layout::Rect;
use tui::style::Color;
use tui::Frame;

mod cpu;
mod overview;
mod processes;

pub use self::cpu::Cpuview;
pub use self::overview::Overview;
pub use self::processes::ProcessesView;

/// Returns a color for Gauge widget's bar based on a percentage
pub fn color_for_percent(percentage: u16) -> Color {
    match percentage {
        0..=50 => Color::LightGreen,
        51..=74 => Color::Yellow,
        75..=90 => Color::LightRed,
        91..=100 => Color::Red,
        _ => Color::White,
    }
}

#[derive(Copy, Clone)]
pub(crate) enum Tab {
    Overview,
    CPU,
    Memory,
    Processes,
    Storage,
    Network,
}

impl From<Tab> for usize {
    fn from(tab: Tab) -> Self {
        match tab {
            Tab::Overview => 0,
            Tab::CPU => 1,
            Tab::Memory => 2,
            Tab::Processes => 3,
            Tab::Storage => 4,
            Tab::Network => 5,
        }
    }
}

impl Tab {
    pub(crate) fn next(&mut self) {
        match self {
            Tab::Overview => *self = Tab::CPU,
            Tab::CPU => *self = Tab::Memory,
            Tab::Memory => *self = Tab::Processes,
            Tab::Processes => *self = Tab::Storage,
            Tab::Storage => *self = Tab::Network,
            Tab::Network => *self = Tab::Overview,
        }
    }

    pub(crate) fn previous(&mut self) {
        match self {
            Tab::Overview => *self = Tab::Network,
            Tab::CPU => *self = Tab::Overview,
            Tab::Memory => *self = Tab::CPU,
            Tab::Processes => *self = Tab::Memory,
            Tab::Storage => *self = Tab::Processes,
            Tab::Network => *self = Tab::Storage,
        }
    }
}

pub trait View {
    /// Renders the visual representation of a system's resources.
    fn render(&self, frame: &mut Frame<TermionBackend<RawTerminal<Stdout>>>, area: Rect);

    /// Updates the system information of the system's resources to render them in the next frame.
    /// This method should only update the system's information related only to a specific resources.
    fn update(&mut self);
}
