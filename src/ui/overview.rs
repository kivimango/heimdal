use std::io::Stdout;

use byte_unit::{Byte, ByteUnit};
use sysinfo::{CpuExt, System, SystemExt};
use termion::raw::RawTerminal;
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

use super::color_for_percent;

pub struct Overview {
    system_info: System,
}

impl Overview {
    pub fn new() -> Self {
        // TODO: refresh only memory and cpu
        let mut system_info = System::new();
        system_info.refresh_all();

        Overview { system_info }
    }

    /// Renders the system resources overview: cpu, memory, disks, network infos
    pub fn render_overview(&mut self, frame: &mut Frame<TermionBackend<RawTerminal<Stdout>>>) {
        let overview_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                ]
                .as_ref(),
            )
            .split(frame.size());

        self.render_cpu(frame, overview_layout[0]);
        self.render_memory(frame, overview_layout[1]);
    }

    /// Renders CPU basic information with an usage bar
    fn render_cpu(&self, frame: &mut Frame<TermionBackend<RawTerminal<Stdout>>>, area: Rect) {
        let cpu_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(area);

        let cpu_name = self.system_info.global_cpu_info().brand();
        let cpu_freq = self.system_info.global_cpu_info().frequency().to_string();
        let cpu_cores = self.system_info.physical_core_count().unwrap_or(0);
        let cpu_usage = self.system_info.global_cpu_info().cpu_usage() as u16;

        let cpu_block = Block::default()
            .title("CPU")
            .borders(Borders::ALL)
            .border_type(tui::widgets::BorderType::Plain);

        let cpu_text = Text::from(format!(
            "Name: {}\nFreq: {} Mhz\nCores: {}",
            cpu_name, cpu_freq, cpu_cores
        ));
        let cpu_label = Paragraph::new(cpu_text).block(cpu_block);

        let gauge_bar_color = color_for_percent(cpu_usage);
        let cpu_usage_bar = Gauge::default()
            .percent(cpu_usage)
            .gauge_style(Style::default().fg(gauge_bar_color))
            .block(
                Block::default()
                    .title("CPU usage")
                    .borders(Borders::ALL)
                    .border_type(tui::widgets::BorderType::Plain),
            );

        frame.render_widget(cpu_label, cpu_layout[0]);
        frame.render_widget(cpu_usage_bar, cpu_layout[1]);
    }

    /// Renders memory statistics with an usage bar
    fn render_memory(&self, frame: &mut Frame<TermionBackend<RawTerminal<Stdout>>>, area: Rect) {
        let (total_memory, available_memory) = (
            Byte::from_unit(self.system_info.total_memory() as f64, ByteUnit::KB).unwrap(),
            Byte::from_unit(self.system_info.available_memory() as f64, ByteUnit::KB).unwrap(),
        );

        let one_percent = total_memory.get_bytes() / 100;
        let used = total_memory.get_bytes() - available_memory.get_bytes();
        let usage_percent = used / one_percent;

        let memory_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(area);

        let memory_box = Block::default()
            .title("Memory")
            .borders(Borders::ALL)
            .border_type(tui::widgets::BorderType::Plain);

        let memory_label = Paragraph::new(Text::from(format!(
            "Total memory: {}\nAvailable memory: {}\n",
            total_memory.get_adjusted_unit(ByteUnit::GB),
            available_memory.get_adjusted_unit(ByteUnit::GB)
        )))
        .block(memory_box);

        frame.render_widget(memory_label, memory_layout[0]);

        let memory_usage_bar = Gauge::default()
            .percent(usage_percent as u16)
            .gauge_style(Style::default().fg(Color::Cyan))
            .block(
                Block::default()
                    .title("Memory usage")
                    .borders(Borders::ALL)
                    .border_type(tui::widgets::BorderType::Plain),
            );
        frame.render_widget(memory_usage_bar, memory_layout[1]);
    }

    pub fn tick(&mut self) {
        //TODO: refresh only dynamic stats: freq, usage
        self.system_info.refresh_cpu();
        self.system_info.refresh_memory();
    }
}
