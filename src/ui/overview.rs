use super::color_for_percent;
use byte_unit::{Byte, ByteUnit};
use std::io::Stdout;
use sysinfo::{CpuExt, DiskExt, System, SystemExt};
use termion::raw::RawTerminal;
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Span, Spans, Text},
    widgets::{BarChart, Block, BorderType, Borders, Gauge, Paragraph},
    Frame,
};

#[derive(Default)]
pub struct Overview {
    system_info: System,
    os: String,
    os_version: String,
    kernel_version: String,
    host_name: String,
    uptime: u64,
    cpu_load: usize,
    memory_total: u64,
    memory_used: u64,
    disk_space_total: u64,
    disk_space_used: u64,
    process_count: u64,
    network_status: String,
    network_sent: u64,
    network_received: u64,
}

impl Overview {
    pub fn new() -> Self {
        // TODO: refresh only memory and cpu
        let system_info = System::new_all();
        let os = system_info.name().unwrap_or("N/A".to_string());
        let os_version = system_info.os_version().unwrap_or("N/A".to_string());
        let kernel_version = system_info.kernel_version().unwrap_or("N/A".to_string());
        let host_name = system_info.host_name().unwrap_or("N/A".to_string());

        Overview {
            os,
            os_version,
            kernel_version,
            host_name,
            ..Default::default()
        }
    }

    /// Renders the system resources overview: cpu, memory, disks, network infos
    pub fn render_overview(
        &mut self,
        frame: &mut Frame<TermionBackend<RawTerminal<Stdout>>>,
        area: Rect,
    ) {
        self.system_info.refresh_cpu();

        // Layout
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
            .split(area);

        self.render_system_info(frame, &overview_layout);
        self.render_cpu(frame, &overview_layout);
        //self.render_memory(frame, &overview_layout);
        //self.render_disks(frame, overview_layout[2]);
    }

    fn render_system_info(
        &self,
        frame: &mut Frame<TermionBackend<RawTerminal<Stdout>>>,
        layout: &Vec<Rect>,
    ) {
        let system_info_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .margin(1)
            .split(layout[0]);

        // Data
        let overview = overview(&self.system_info);
        let uptime = overview.uptime.to_string();
        let load_data = self.system_info.load_average();
        let average_load = format!(
            "1m: {}% 5m: {}% 15m: {}%",
            load_data.one, load_data.five, load_data.fifteen
        );

        // Widgets
        //let system_info_area = Rect::new(area.x, area.y, area.width, area.height);
        let block = Block::default()
            .borders(Borders::ALL)
            .title("System information");

        let spans = vec![
            Spans::from(vec![
                Span::raw("Operating system: "),
                Span::raw(overview.os),
            ]),
            Spans::from(vec![Span::raw("Version: "), Span::raw(overview.os_version)]),
            Spans::from(vec![
                Span::raw("Kernel version: "),
                Span::raw(overview.kernel_version),
            ]),
        ];
        let os_text = Text::from(spans);
        let os_label = Paragraph::new(os_text);

        let spans2 = vec![
            Spans::from(vec![Span::raw("Uptime: "), Span::raw(uptime)]),
            Spans::from(vec![Span::raw("Hostname: "), Span::raw(overview.host_name)]),
            Spans::from(vec![Span::raw("Avg Load: "), Span::raw(average_load)]),
        ];
        let uptime_text = Text::from(spans2);
        let uptime_label = Paragraph::new(uptime_text);

        frame.render_widget(block, layout[0]);
        frame.render_widget(
            os_label.alignment(tui::layout::Alignment::Left),
            system_info_layout[0],
        );
        frame.render_widget(uptime_label, system_info_layout[1]);
    }

    /// Renders CPU basic information with an usage bar
    fn render_cpu(
        &mut self,
        frame: &mut Frame<TermionBackend<RawTerminal<Stdout>>>,
        layout: &Vec<Rect>,
    ) {
        let cpu_memory_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(layout[1]);
        let cpu_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .margin(1)
            .split(cpu_memory_layout[0]);

        self.render_memory(frame, cpu_memory_layout[1]);

        let cpu_name = self.system_info.global_cpu_info().brand();
        let cpu_freq = self.system_info.global_cpu_info().frequency().to_string();
        let cpu_cores = self.system_info.physical_core_count().unwrap_or(0);
        let cpu_usage = self.system_info.global_cpu_info().cpu_usage() as u16;

        let cpu_block = Block::default()
            .title("CPU")
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);
        frame.render_widget(cpu_block, cpu_memory_layout[0]);

        let cpu_text = Text::from(format!(
            "Name: {}\nFreq: {} Mhz\nCores: {}\nUsage: {}%",
            cpu_name, cpu_freq, cpu_cores, cpu_usage
        ));
        let cpu_label = Paragraph::new(cpu_text);

        let gauge_bar_color = color_for_percent(cpu_usage);
        let cpu_usage_bar = Gauge::default()
            .percent(cpu_usage)
            .gauge_style(Style::default().fg(gauge_bar_color));
        frame.render_widget(cpu_label, cpu_layout[0]);
        frame.render_widget(cpu_usage_bar, cpu_layout[1]);
    }

    /// Renders memory statistics with an usage bar
    fn render_memory(
        &mut self,
        frame: &mut Frame<TermionBackend<RawTerminal<Stdout>>>,
        area: Rect,
    ) {
        self.system_info.refresh_memory();
        let memory_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .margin(1)
            .split(area);

        let block = Block::default()
            .title("Memory usage")
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);

        let (total_memory, used_memory, available_memory) = (
            Byte::from_unit(self.system_info.total_memory() as f64, ByteUnit::KB).unwrap(),
            Byte::from_unit(self.system_info.used_memory() as f64, ByteUnit::KB).unwrap(),
            Byte::from_unit(self.system_info.available_memory() as f64, ByteUnit::KB).unwrap(),
        );

        let one_percent = total_memory.get_bytes() / 100;
        let used_percent = used_memory.get_bytes() / one_percent;

        let memory_label = Paragraph::new(Text::from(format!(
            "Total memory: {}\nUsed Memory: {}\nAvailable memory: {}\n",
            total_memory.get_adjusted_unit(ByteUnit::GB),
            used_memory.get_adjusted_unit(ByteUnit::GB),
            available_memory.get_adjusted_unit(ByteUnit::GB)
        )));

        frame.render_widget(block, area);
        frame.render_widget(memory_label, memory_layout[0]);

        let memory_usage_bar = Gauge::default()
            .percent(used_percent as u16)
            .gauge_style(Style::default().fg(color_for_percent(used_percent as u16)));
        frame.render_widget(memory_usage_bar, memory_layout[1]);
    }

    fn render_disks(&self, frame: &mut Frame<TermionBackend<RawTerminal<Stdout>>>, area: Rect) {
        let disks = self.system_info.disks();
        let mut total_space = 0;
        let mut available_space = 0;

        for disk in disks {
            available_space += disk.available_space();
            total_space += disk.total_space();
        }

        let total_space_gb = Byte::from_bytes(total_space).get_adjusted_unit(ByteUnit::GB);
        let used_space_gb =
            Byte::from_bytes(total_space - available_space).get_adjusted_unit(ByteUnit::GB);
        let available_space_gb = Byte::from_bytes(available_space).get_adjusted_unit(ByteUnit::GB);

        let disks_text = Text::from(format!(
            "Total space: {}\nUsed space: {}\nAvailable space: {}",
            total_space_gb, used_space_gb, available_space_gb
        ));
        let disks_label = Paragraph::new(disks_text).block(
            Block::default()
                .title("Disks")
                .borders(Borders::ALL)
                .border_type(BorderType::Plain),
        );

        let one_percent = total_space / 100;
        let used = total_space - available_space;
        let usage_percent = used / one_percent;

        let disk_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(area);
        let gauge_bar_color = color_for_percent(usage_percent as u16);
        let disk_usage_bar = Gauge::default()
            .percent(usage_percent as u16)
            .gauge_style(Style::default().fg(gauge_bar_color))
            .block(
                Block::default()
                    .title("Storage space usage")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain),
            );

        frame.render_widget(disks_label, disk_layout[0]);
        frame.render_widget(disk_usage_bar, disk_layout[1]);
    }

    pub fn tick(&mut self) {
        //TODO: refresh only dynamic stats: freq, usage
        self.system_info.refresh_cpu();
        self.system_info.refresh_memory();
    }
}

impl Overview {
    pub fn os(mut self, os_string: String) -> Self {
        self.os = os_string;
        self
    }

    pub fn os_version(mut self, os_version: String) -> Self {
        self.os_version = os_version;
        self
    }

    pub fn kernel_version(mut self, kernel_version: String) -> Self {
        self.kernel_version = kernel_version;
        self
    }

    pub fn host_name(mut self, host_name: String) -> Self {
        self.host_name = host_name;
        self
    }

    pub fn uptime(mut self, uptime: u64) -> Self {
        self.uptime = uptime;
        self
    }

    pub fn cpu_load(mut self, cpu_load: usize) -> Self {
        self.cpu_load = cpu_load;
        self
    }

    pub fn memory_total(mut self, memory_total: u64) -> Self {
        self.memory_total = memory_total;
        self
    }

    pub fn memory_used(mut self, memory_used: u64) -> Self {
        self.memory_used = memory_used;
        self
    }

    pub fn disk_space_total(mut self, disk_space_total: u64) -> Self {
        self.disk_space_total = disk_space_total;
        self
    }

    pub fn disk_space_used(mut self, disk_space_used: u64) -> Self {
        self.disk_space_used = disk_space_used;
        self
    }

    pub fn process_count(mut self, process_count: u64) -> Self {
        self.process_count = process_count;
        self
    }

    pub fn network_status(mut self, network_status: String) -> Self {
        self.network_status = network_status;
        self
    }

    pub fn network_sent(mut self, network_sent: u64) -> Self {
        self.network_sent = network_sent;
        self
    }

    pub fn network_received(mut self, network_received: u64) -> Self {
        self.network_received = network_received;
        self
    }

    pub fn update(&mut self) {
        self.system_info.refresh_cpu();
    }
}

fn overview(system_info: &System) -> Overview {
    const N_A: &'static str = "N/A";
    Overview::new().uptime(system_info.uptime())
}
