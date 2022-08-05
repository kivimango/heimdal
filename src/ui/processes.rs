use std::io::Stdout;

use sysinfo::{ProcessExt, System, SystemExt, Uid, UserExt};
use termion::raw::RawTerminal;
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, BorderType, Borders, Cell, Row, Table, TableState},
    Frame,
};

const CELL_HEADERS: [&str; 6] = ["PID", "Name", "User", "CPU", "Memory", "Status"];

pub struct ProcessesView {
    table_state: TableState,
}

impl ProcessesView {
    pub(crate) fn new() -> Self {
        ProcessesView {
            table_state: TableState::default(),
        }
    }

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
        //let normal_style = Style::default().bg(Color::Blue);
        let header_cells = CELL_HEADERS.iter().map(|header| Cell::from(*header));
        let table_header = Row::new(header_cells)
            //.style(normal_style)
            .height(1);
        let rows = system.processes().iter().map(|(pid, process)| {
            let cells: [Cell; 6] = [
                Cell::from(pid.to_string()),
                Cell::from(process.name()),
                Cell::from(get_username_for_id(process.user_id(), system)),
                Cell::from(process.cpu_usage().to_string()),
                Cell::from(process.memory().to_string()),
                Cell::from(process.status().to_string()),
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
