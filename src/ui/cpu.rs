use std::io::Stdout;
use sysinfo::{Component, ComponentExt, CpuExt, System, SystemExt};
use termion::raw::RawTerminal;
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Layout, Rect},
    text::Text,
    widgets::{BarChart, Block, BorderType, Borders, Paragraph},
    Frame,
};

pub struct Cpuview {
    cpu_brand: String,
}

impl Cpuview {
    pub fn new() -> Self {
        Cpuview {
            cpu_brand: String::new(),
        }
    }

    pub fn render_cpu(
        &self,
        frame: &mut Frame<TermionBackend<RawTerminal<Stdout>>>,
        area: Rect,
        system: &System,
    ) {
        let cpu_layout = Layout::default()
            .direction(tui::layout::Direction::Horizontal)
            .constraints([Constraint::Percentage(35), Constraint::Percentage(65)].as_ref())
            .split(area);

        let cpu_temp_layout = Layout::default()
            .direction(tui::layout::Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(cpu_layout[0]);

        let cpu_block = Block::default()
            .title("CPU")
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);

        let core_count = system.physical_core_count().unwrap_or(1);
        let cpu_name = system.global_cpu_info().brand();
        let cpu_freq = system.global_cpu_info().frequency().to_string();

        let cpu_text = Text::from(format!(
            "Name: {}\nFreq: {} Mhz\nNumber of cores: {}",
            cpu_name, cpu_freq, core_count
        ));
        let cpu_label = Paragraph::new(cpu_text).block(cpu_block);

        frame.render_widget(cpu_label, cpu_temp_layout[0]);

        let cpu_temp_block = Block::default()
            .title("Temperatures")
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);

        let sensors: Vec<&Component> = system
            .components()
            .iter()
            .filter(|component| component.label().contains("Core").clone())
            .collect();

        let mut sensor_labels = Text::from("");
        for s in sensors {
            let span = Text::raw(format!("{}: {}°C", s.label(), s.temperature()));
            sensor_labels.extend(span);
        }

        let sensors = Paragraph::new(sensor_labels).block(cpu_temp_block);

        frame.render_widget(sensors, cpu_temp_layout[1]);

        let mut data = Vec::new();
        let mut core_titles = Vec::new();
        let mut i = 0;
        for _ in system.cpus() {
            // CPU Core title with index
            let mut core_title = String::from("Core ");
            let index_str = i.to_string();
            core_title.push_str(&index_str);
            core_titles.push(core_title);
            i += 1;
        }

        i = 0;
        for cpu in system.cpus() {
            let usage = cpu.cpu_usage();
            data.push((core_titles[i].as_str(), usage as u64));
            i += 1;
        }

        let cpu_cores_chart = BarChart::default()
            .block(Block::default().title("CPU Usage").borders(Borders::ALL))
            .bar_width(8)
            .data(&data);

        frame.render_widget(cpu_cores_chart, cpu_layout[1]);
    }
}
