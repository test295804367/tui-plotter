use std::io;
use tui::backend::CrosstermBackend;
use tui::terminal::Terminal;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use tui::widgets::{Block, Borders, Chart, Dataset, GraphType, Axis, Paragraph};
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::symbols;
use std::f64::consts::PI;

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
                .constraints([Constraint::Percentage(60), Constraint::Percentage(20), Constraint::Percentage(20)].as_ref())
                .split(size);

            let block = Block::default().borders(Borders::ALL);
            f.render_widget(block, chunks[0]);

            let datasets: Vec<Dataset<'_>> = app.functions.iter().map(|(name, data)| {
                Dataset::default()
                    .name(name)
                    .marker(symbols::Marker::Dot)
                    .style(Style::default().fg(Color::Cyan))
                    .graph_type(GraphType::Line)
                    .data(data)
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
                KeyCode::Char('s') => app.add_sine_wave(),
                KeyCode::Char('c') => app.add_cosine_wave(),
                KeyCode::Char('p') => app.add_parametric(),
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

struct App {
    functions: Vec<(String, Vec<(f64, f64)>)>,
    amplitude: f64,
    frequency: f64,
}

impl App {
    fn new() -> Self {
        App {
            functions: Vec::new(),
            amplitude: 1.0,
            frequency: 1.0,
        }
    }

    fn add_sine_wave(&mut self) {
        let data: Vec<(f64, f64)> = (-50..=50).map(|x| {
            let x = x as f64 / 5.0;
            (x, self.amplitude * (self.frequency * x).sin())
        }).collect();
        self.functions.push(("sin(x)".to_string(), data));
    }

    fn add_cosine_wave(&mut self) {
        let data: Vec<(f64, f64)> = (-50..=50).map(|x| {
            let x = x as f64 / 5.0;
            (x, self.amplitude * (self.frequency * x).cos())
        }).collect();
        self.functions.push(("cos(x)".to_string(), data));
    }

    fn add_parametric(&mut self) {
        let data: Vec<(f64, f64)> = (0..=100).map(|t| {
            let t = t as f64 / 10.0 * 2.0 * PI;
            (self.amplitude * t.cos(), self.amplitude * t.sin())
        }).collect();
        self.functions.push(("Parametric: (cos(t), sin(t))".to_string(), data));
    }

    fn reset_graph(&mut self) {
        self.functions.clear();
    }

    fn increase_amplitude(&mut self) {
        self.amplitude += 0.1;
        self.update_functions();
    }

    fn decrease_amplitude(&mut self) {
        if self.amplitude > 0.1 {
            self.amplitude -= 0.1;
        }
        self.update_functions();
    }

    fn increase_frequency(&mut self) {
        self.frequency += 0.1;
        self.update_functions();
    }

    fn decrease_frequency(&mut self) {
        if self.frequency > 0.1 {
            self.frequency -= 0.1;
        }
        self.update_functions();
    }

    fn update_functions(&mut self) {
        let mut updated_functions = Vec::new();
        for (name, _) in &self.functions {
            if name == "sin(x)" {
                updated_functions.push(("sin(x)".to_string(), self.generate_sine_wave()));
            } else if name == "cos(x)" {
                updated_functions.push(("cos(x)".to_string(), self.generate_cosine_wave()));
            } else if name.starts_with("Parametric") {
                updated_functions.push(("Parametric: (cos(t), sin(t))".to_string(), self.generate_parametric()));
            }
        }
        self.functions = updated_functions;
    }

    fn generate_sine_wave(&self) -> Vec<(f64, f64)> {
        (-50..=50).map(|x| {
            let x = x as f64 / 5.0;
            (x, self.amplitude * (self.frequency * x).sin())
        }).collect()
    }

    fn generate_cosine_wave(&self) -> Vec<(f64, f64)> {
        (-50..=50).map(|x| {
            let x = x as f64 / 5.0;
            (x, self.amplitude * (self.frequency * x).cos())
        }).collect()
    }

    fn generate_parametric(&self) -> Vec<(f64, f64)> {
        (0..=100).map(|t| {
            let t = t as f64 / 10.0 * 2.0 * PI;
            (self.amplitude * t.cos(), self.amplitude * t.sin())
        }).collect()
    }

    fn on_key(&mut self, _key: KeyEvent) {
        // Handle key events for additional features
    }
}

