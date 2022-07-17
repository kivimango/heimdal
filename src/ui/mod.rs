use tui::style::Color;

mod overview;

pub use self::overview::Overview;

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
