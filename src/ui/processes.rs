use std::io::Stdout;

use sysinfo::{ProcessExt, System, SystemExt, Uid, UserExt};
use termion::{event::Key, raw::RawTerminal};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Cell, Row, Table, TableState},
    Frame,
};

const CELL_HEADERS: [&str; 6] = ["PID", "Name", "User", "CPU", "Memory", "Status"];

pub(crate) enum TableSort {
    Ascending,
    Descending,
}

impl Default for TableSort {
    fn default() -> Self {
        TableSort::Ascending
    }
}

impl TableSort {
    fn reverse(&mut self) {
        match self {
            TableSort::Ascending => *self = TableSort::Descending,
            TableSort::Descending => *self = TableSort::Ascending,
        }
    }
}

pub(crate) enum TableSortPredicate {
    PID,
    Name,
    User,
    CPU,
    Memory,
    Status,
}

impl Default for TableSortPredicate {
    fn default() -> Self {
        TableSortPredicate::Name
    }
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct Process {
    pid: String,
    name: String,
    user: String,
    cpu_usage: String,
    memory_usage: String,
    status: String,
}

pub struct ProcessesView {
    sort_predicate: TableSortPredicate,
    sort_order: TableSort,
    table_state: TableState,
    processes: Vec<Process>,
}

impl ProcessesView {
    pub(crate) fn new() -> Self {
        ProcessesView {
            sort_predicate: TableSortPredicate::Name,
            sort_order: TableSort::Ascending,
            table_state: TableState::default(),
            // TODO: figure out the maximum number or rows to draw
            processes: Vec::with_capacity(85),
        }
    }

    pub(crate) fn handle_arrow_keys(&self, key: Key) {}

    pub(crate) fn render_processes(
        &mut self,
        frame: &mut Frame<TermionBackend<RawTerminal<Stdout>>>,
        area: Rect,
        system: &System,
    ) {
        let process_layout = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(area);

        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let normal_style = Style::default().bg(Color::Blue);
        let header_cells = CELL_HEADERS.iter().map(|header| Cell::from(*header));
        let table_header = Row::new(header_cells).style(normal_style).height(1);
        let values = system.processes().clone();

        self.processes = values
            .iter()
            .map(|(_pid, process)| Process {
                pid: process.pid().clone().to_string(),
                name: process.name().to_string(),
                user: get_username_for_id(process.user_id(), system),
                cpu_usage: process.cpu_usage().to_string(),
                memory_usage: process.memory().to_string(),
                status: process.status().to_string(),
            })
            .collect();

        self.sort();

        /*let rows = system.processes().iter().map(|(pid, process)| {
            let cells: [Cell; 6] = [
                Cell::from(pid.to_string()),
                Cell::from(process.name()),
                Cell::from(get_username_for_id(process.user_id(), system)),
                Cell::from(process.cpu_usage().to_string()),
                Cell::from(process.memory().to_string()),
                Cell::from(process.status().to_string()),
            ];
            Row::new(cells).height(1)
        });*/
        let rows = self.processes.iter().map(|p| {
            let cells: [Cell; 6] = [
                Cell::from(p.pid.clone()),
                Cell::from(p.name.clone()),
                Cell::from(p.user.clone()),
                Cell::from(p.cpu_usage.clone()),
                Cell::from(p.memory_usage.clone()),
                Cell::from(p.status.clone()),
            ];
            Row::new(cells).height(1)
        });

        let table = Table::new(rows)
            .header(table_header)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
                    .title("Processes"),
            )
            .highlight_style(selected_style)
            .highlight_symbol(">> ")
            .widths(&[
                Constraint::Length(4),
                Constraint::Percentage(10),
                Constraint::Percentage(5),
                Constraint::Length(3),
                Constraint::Length(15),
                Constraint::Length(10),
            ]);

        frame.render_stateful_widget(table, process_layout[0], &mut self.table_state);
    }

    pub(crate) fn sort_by(&mut self, predicate: TableSortPredicate) {
        self.sort_predicate = predicate;
        self.sort();
    }

    fn sort(&mut self) {
        match self.sort_predicate {
            TableSortPredicate::PID => self.processes.sort_by(|a, b| match self.sort_order {
                TableSort::Ascending => a.pid.cmp(&b.pid),
                TableSort::Descending => a.pid.cmp(&b.pid).reverse(),
            }),
            TableSortPredicate::Name => self.processes.sort_by(|a, b| match self.sort_order {
                TableSort::Ascending => a.name.cmp(&b.name),
                TableSort::Descending => a.name.cmp(&b.name).reverse(),
            }),
            TableSortPredicate::User => self.processes.sort_by(|a, b| match self.sort_order {
                TableSort::Ascending => a.user.cmp(&b.user),
                TableSort::Descending => a.user.cmp(&b.user).reverse(),
            }),
            TableSortPredicate::CPU => self.processes.sort_by(|a, b| match self.sort_order {
                TableSort::Ascending => a.cpu_usage.cmp(&b.cpu_usage),
                TableSort::Descending => a.cpu_usage.cmp(&b.cpu_usage).reverse(),
            }),
            TableSortPredicate::Memory => self.processes.sort_by(|a, b| match self.sort_order {
                TableSort::Ascending => a.memory_usage.cmp(&b.memory_usage),
                TableSort::Descending => a.memory_usage.cmp(&b.memory_usage).reverse(),
            }),
            TableSortPredicate::Status => self.processes.sort_by(|a, b| match self.sort_order {
                TableSort::Ascending => a.status.cmp(&b.status),
                TableSort::Descending => a.status.cmp(&b.status).reverse(),
            }),
        }
    }
}

fn get_username_for_id(uid: Option<&Uid>, sysinfo: &System) -> String {
    if let Some(uid) = uid {
        if let Some(user) = sysinfo.get_user_by_id(&uid) {
            return user.name().to_string();
        } else {
            return String::from("N/A");
        }
    } else {
        return String::from("N/A");
    }
}
