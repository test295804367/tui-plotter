mod tui;
mod libqalculate;

use std::io;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use ratatui::widgets::{Block, Borders, Chart, Dataset, GraphType, Axis, Paragraph};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::{Span, Spans};
use ratatui::symbols;
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
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(60), Constraint::Percentage(20), Constraint::Percentage(20)].as_ref())
                .split(size);

            let block = Block::default().borders(Borders::ALL);
            f.render_widget(block, chunks[0]);

            let datasets: Vec<Dataset<'_>> = app.functions.iter().map(|function| {
                Dataset::default()
                    .name(&function.name)
                    .marker(symbols::Marker::Dot)
                    .style(Style::default().fg(Color::Cyan))
                    .graph_type(GraphType::Line)
                    .data(&function.data)
            }).collect();

            let chart = Chart::new(datasets)
                .block(Block::default().title("Graph").borders(Borders::ALL))
                .x_axis(Axis::default().title("X").bounds([-10.0, 10.0]))
                .y_axis(Axis::default().title("Y").bounds([-10.0, 10.0]));

            f.render_widget(chart, chunks[0]);

            let instructions = vec![
                Spans::from(Span::raw("Press 's' to add a sine wave.")),
                Spans::from(Span::raw("Press 'c' to add a cosine wave.")),
                Spans::from(Span::raw("Press 'p' to add a parametric function.")),
                Spans::from(Span::raw("Press 'i' to add an inequality.")),
                Spans::from(Span::raw("Press 'r' to reset the graph.")),
                Spans::from(Span::raw("Use arrows to adjust amplitude/frequency.")),
                Spans::from(Span::raw("Press 'Esc' to exit.")),
            ];

            let paragraph = Paragraph::new(instructions)
                .block(Block::default().borders(Borders::ALL).title("Instructions"));

            f.render_widget(paragraph, chunks[1]);

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
                KeyCode::Esc => return Ok(()),
                KeyCode::Char('s') => app.add_function("sin(x)".to_string(), FunctionType::Sine),
                KeyCode::Char('c') => app.add_function("cos(x)".to_string(), FunctionType::Cosine),
                KeyCode::Char('p') => app.add_function("Parametric: (cos(t), sin(t))".to_string(), FunctionType::Parametric),
                KeyCode::Char('i') => app.add_function("x > 0".to_string(), FunctionType::Inequality { expr: |x| x > 0.0 }),
                KeyCode::Char('r') => app.reset_graph(),
                KeyCode::Up => app.increase_amplitude(),
                KeyCode::Down => app.decrease_amplitude(),
                KeyCode::Right => app.increase_frequency(),
                KeyCode::Left => app.decrease_frequency(),
                _ => app.on_key(key),
            }
        }
    }
}

