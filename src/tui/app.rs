use crate::tui::plot::PlotWidget;
use crossterm::event::{KeyCode, KeyEvent};
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Paragraph};
use tui::Frame;

pub struct App {
    pub functions: Vec<String>,
    pub current_input: String,
    pub selected_function: usize,
    pub display_data: Vec<(f64, f64)>,
}

impl App {
    pub fn new() -> App {
        App {
            functions: vec!["x^2".to_string(), "sin(x)".to_string()],
            current_input: String::new(),
            selected_function: 0,
            display_data: vec![],
        }
    }

    pub fn add_function(&mut self, function: String) {
        self.functions.push(function);
        self.selected_function = self.functions.len() - 1;
        self.update_plot();
    }

    pub fn remove_function(&mut self) {
        if !self.functions.is_empty() {
            self.functions.remove(self.selected_function);
            if self.selected_function > 0 {
                self.selected_function -= 1;
            }
            self.update_plot();
        }
    }

    pub fn update_plot(&mut self) {
        if let Some(func) = self.functions.get(self.selected_function) {
            self.display_data = self.calculate_plot_points(func);
        }
    }

    pub fn calculate_plot_points(&self, function: &str) -> Vec<(f64, f64)> {
        let mut points = Vec::new();
        for x in -50..=50 {
            let x_val = x as f64 / 10.0;
            let y_val = match function.as_str() {
                "x^2" => x_val.powi(2),
                "sin(x)" => x_val.sin(),
                _ => 0.0,
            };
            points.push((x_val, y_val));
        }
        points
    }

    pub fn on_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => {
                self.current_input.push(c);
            }
            KeyCode::Backspace => {
                self.current_input.pop();
            }
            KeyCode::Enter => {
                self.add_function(self.current_input.clone());
                self.current_input.clear();
            }
            KeyCode::Up => {
                if self.selected_function > 0 {
                    self.selected_function -= 1;
                    self.update_plot();
                }
            }
            KeyCode::Down => {
                if self.selected_function < self.functions.len() - 1 {
                    self.selected_function += 1;
                    self.update_plot();
                }
            }
            KeyCode::Delete => {
                self.remove_function();
            }
            _ => {}
        }
    }

    pub fn draw<B: tui::backend::Backend>(&self, f: &mut Frame<B>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
            .split(f.size());

        let input = Paragraph::new(self.current_input.as_ref())
            .block(Block::default().borders(Borders::ALL).title("Input"));

        let plot_widget = PlotWidget::new(self.display_data.clone())
            .block(Block::default().borders(Borders::ALL).title("Plot"));

        f.render_widget(plot_widget, chunks[0]);
        f.render_widget(input, chunks[1]);
    }
}

