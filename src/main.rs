mod tui;
mod libqalculate;

use std::io;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Dataset};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::{Span, Spans};
use crate::tui::functions::{FunctionType};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = tui::app::App::new();

    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut tui::app::App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let instructions_height = if app.show_instructions { 7 } else { 0 }; // Set instructions height to 7 lines
            let settings_height = 4;

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(size.height - (instructions_height + settings_height) as u16),
                    Constraint::Length(instructions_height),
                    Constraint::Length(settings_height),
                ].as_ref())
                .split(size);

            let block = Block::default().borders(Borders::ALL);
            f.render_widget(block, chunks[0]);

            if app.show_menu {
                let items: Vec<ListItem> = app.available_functions.iter().map(|func| {
                    ListItem::new(Span::raw(func.clone()))
                }).collect();

                let list = List::new(items)
                    .block(Block::default().title("Select Function").borders(Borders::ALL))
                    .highlight_style(Style::default().bg(Color::LightBlue))
                    .highlight_symbol(">> ");

                f.render_stateful_widget(list, chunks[0], &mut app.menu_state);
            } else {
                let datasets = app.functions.iter().map(|function| {
                    Dataset::default()
                        .name(&function.name)
                        .marker(ratatui::symbols::Marker::Dot)
                        .style(Style::default().fg(Color::Cyan))
                        .graph_type(ratatui::widgets::GraphType::Line)
                        .data(&function.data)
                }).collect::<Vec<_>>();

                let chart = ratatui::widgets::Chart::new(datasets)
                    .block(Block::default().title("Graph").borders(Borders::ALL))
                    .x_axis(ratatui::widgets::Axis::default().title("X").bounds([-10.0, 10.0]))
                    .y_axis(ratatui::widgets::Axis::default().title("Y").bounds([-10.0, 10.0]));

                f.render_widget(chart, chunks[0]);
            }

            if app.show_instructions {
                let instructions = vec![
                    Spans::from(Span::raw("Press 'f' to add a function.")),
                    Spans::from(Span::raw("Press 'r' to reset the graph.")),
                    Spans::from(Span::raw("Use arrows to adjust amplitude/frequency.")),
                    Spans::from(Span::raw("Press 'h' to show/hide instructions.")),
                    Spans::from(Span::raw("Press 'Esc' or 'q' to exit.")),
                ];

                let paragraph = Paragraph::new(instructions)
                    .block(Block::default().borders(Borders::ALL).title("Instructions"));

                f.render_widget(paragraph, chunks[1]);
            }

            let settings = vec![
                Spans::from(Span::raw(format!("Amplitude: {:.2}", app.amplitude))),
                Spans::from(Span::raw(format!("Frequency: {:.2}", app.frequency))),
            ];

            let settings_paragraph = Paragraph::new(settings)
                .block(Block::default().borders(Borders::ALL).title("Settings"));

            f.render_widget(settings_paragraph, chunks[2]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc | KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('f') => app.toggle_menu(),
                KeyCode::Up => {
                    if app.show_menu {
                        app.menu_up();
                    } else {
                        app.increase_amplitude();
                    }
                }
                KeyCode::Down => {
                    if app.show_menu {
                        app.menu_down();
                    } else {
                        app.decrease_amplitude();
                    }
                }
                KeyCode::Right => app.increase_frequency(),
                KeyCode::Left => app.decrease_frequency(),
                KeyCode::Enter => {
                    if app.show_menu {
                        app.select_function();
                    }
                }
                KeyCode::Char('r') => app.reset_graph(),
                _ => app.on_key(key),
            }
        }
    }
}

