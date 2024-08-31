use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use tui::widgets::{Block, Borders, Chart, Dataset, GraphType, Axis};
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

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

fn run_app<B: tui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(size);

            let block = Block::default().borders(Borders::ALL);
            f.render_widget(block, chunks[0]);

            let datasets = vec![
                Dataset::default()
                    .name("data")
                    .marker(tui::symbols::Marker::Dot)
                    .style(Style::default().fg(Color::Cyan))
                    .graph_type(GraphType::Line)
                    .data(&app.data),
            ];

            let chart = Chart::new(datasets)
                .block(Block::default().title("Graph").borders(Borders::ALL))
                .x_axis(Axis::default().title("X").bounds([-10.0, 10.0]))
                .y_axis(Axis::default().title("Y").bounds([-10.0, 10.0]));

            f.render_widget(chart, chunks[0]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => return Ok(()),
                _ => app.on_key(key),
            }
        }
    }
}

struct App {
    data: Vec<(f64, f64)>,
}

impl App {
    fn new() -> Self {
        let data = (-50..=50).map(|x| {
            let x = x as f64 / 5.0;
            (x, x.sin())
        }).collect();

        App { data }
    }

    fn on_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => {
                println!("Key pressed: {}", c);
            }
            _ => {}
        }
    }
}

