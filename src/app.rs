use crate::ui::{Cpuview, Overview, ProcessesView, Tab};
use std::{io::Stdout, rc::Rc};
use sysinfo::{System, SystemExt};
use termion::{event::Key, raw::RawTerminal};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::Tabs,
    Frame,
};

const TAB_TITLES: [&str; 6] = [
    "Overview",
    "CPU",
    "Memory",
    "Processes",
    "Storage",
    "Network",
];


pub(crate) struct App {
    active_tab: Tab,
    system_info: Rc<System>,
    overview: Overview,
    cpu_view: Cpuview,
    process_view: ProcessesView,
}

impl App {
    pub(crate) fn new() -> Self {
        let mut system_info = System::new_all();
        system_info.refresh_all();
        let system_info = Rc::new(system_info);

        App {
            active_tab: Tab::Overview,
            system_info,
            overview: Overview::new(),
            cpu_view: Cpuview::new(),
            process_view: ProcessesView::new(),
        }
    }

    pub(crate) fn render(&mut self, frame: &mut Frame<TermionBackend<RawTerminal<Stdout>>>) {
        let layout = Layout::default()
            .direction(tui::layout::Direction::Vertical)
            .constraints([Constraint::Percentage(5), Constraint::Percentage(95)].as_ref())
            .split(frame.size());

        match self.active_tab {
            Tab::Overview => self.overview.render_overview(frame, layout[1]),
            Tab::CPU => self
                .cpu_view
                .render_cpu(frame, layout[1], &self.system_info),
            Tab::Processes => {
                self.process_view
                    .render_processes(frame, layout[1], &self.system_info)
            }
            /*Tab::Memory => render_memory(),
            Tab::Storage => render_storage(),
            Tab::Network => render_network()*/
            _ => (),
        }

        let tab_menu = TAB_TITLES
            .iter()
            .map(|tab| {
                let (first, rest) = tab.split_at(1);
                Spans::from(vec![
                    Span::styled(
                        first,
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::UNDERLINED),
                    ),
                    Span::styled(rest, Style::default().fg(Color::White)),
                ])
            })
            .collect();

        let tabs = Tabs::new(tab_menu)
            .select(self.active_tab.into())
            //.block(Block::default().title("Menu").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow))
            .divider(Span::raw("|"));

        frame.render_widget(tabs, layout[0]);
    }

    pub(crate) fn handle_arrow_keys(&self, key: Key) {
        match self.active_tab {
            Tab::Overview => (),
            Tab::CPU => (),
            Tab::Memory => (),
            Tab::Processes => self.process_view.handle_arrow_keys(key),
            Tab::Storage => (),
            Tab::Network => (),
        }
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
            _ => (),
        }
    }

    pub(crate) fn previous_tab(&mut self) {
        self.active_tab.previous();
    }

    pub(crate) fn tick(&mut self) {
        match self.active_tab {
            Tab::Overview => {
                self.overview.update();
            }
            Tab::CPU => {
                self.cpu_view.update();
            }
            Tab::Memory | Tab::Processes | Tab::Storage | Tab::Network  => {}
        }
    }
}
