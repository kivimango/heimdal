use std::io::Stdout;
use sysinfo::{System, SystemExt};
use termion::raw::RawTerminal;
use tui::{Frame, backend::TermionBackend, text::{Spans, Span}, style::{Style, Color, Modifier}, widgets::Tabs, layout::{Layout, Constraint}};
use crate::ui::{Cpuview, Overview, Tab};

const TAB_TITLES: [&str;6] = ["Overview", "CPU", "Memory", "Processes", "Storage", "Network"];

pub(crate) struct App {
    active_tab: Tab,
    system_info: System,
    overview: Overview,
    cpu_view: Cpuview
}

impl App {
    pub(crate) fn new() -> Self {
        let system_info = System::new_all();

        App {
            active_tab: Tab::Overview,
            system_info,
            overview: Overview::new(),
            cpu_view: Cpuview::new()
        }
    }

    pub(crate) fn render(&mut self, frame: &mut Frame<TermionBackend<RawTerminal<Stdout>>>) {
        self.system_info.refresh_all();

        let layout = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .constraints([
            Constraint::Percentage(5),
            Constraint::Percentage(95)
        ].as_ref())
        .split(frame.size());

        match self.active_tab {
            Tab::Overview => self.overview.render_overview(frame, layout[1]),
            Tab::CPU => self.cpu_view.render_cpu(frame, layout[1], &self.system_info),
            /*Tab::Memory => render_memory(),
            Tab::Processes => render_processes(),
            Tab::Storage => render_storage(),
            Tab::Network => render_network()*/
            _ => ()
        }

        let tab_menu = TAB_TITLES.iter().map(|tab| {
            let (first, rest) = tab.split_at(1);
            Spans::from(vec![
                Span::styled(first, 
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        }).collect();

        let tabs = Tabs::new(tab_menu)
            .select(self.active_tab.into())
            //.block(Block::default().title("Menu").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow))
            .divider(Span::raw("|"));

        frame.render_widget(tabs, layout[0]);
    }

    pub(crate) fn switch_tab(&mut self, ch: char) {
        match ch {
            'o' | 'O' => self.active_tab = Tab::Overview,
            'c' | 'C' => self.active_tab = Tab::CPU,
            'm' | 'M' => self.active_tab = Tab::Memory,
            'p' | 'P' => self.active_tab = Tab::Processes,
            's' | 'S' => self.active_tab = Tab::Storage,
            'n' | 'N' => self.active_tab = Tab::Network,
            '\t' => self.active_tab.next(),
            _ => ()
        }
    }
}



