use crate::tui::functions::{Function, FunctionType};
use crossterm::event::KeyEvent;
use ratatui::widgets::ListState;

pub struct App {
    pub functions: Vec<Function>,
    pub available_functions: Vec<String>,
    pub amplitude: f64,
    pub frequency: f64,
    pub show_instructions: bool,
    pub show_menu: bool, // Track if the menu is shown
    pub menu_state: ListState, // Track the state of the menu
}

impl App {
    pub fn new() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        App {
            functions: Vec::new(),
            available_functions: vec![
                "sin(x)".to_string(),
                "cos(x)".to_string(),
                "tan(x)".to_string(),
                "sec(x)".to_string(),
                "csc(x)".to_string(),
                "cot(x)".to_string(),
                "Parametric: (cos(t), sin(t))".to_string(),
                "Inequality: x > 0".to_string(),
            ],
            amplitude: 1.0,
            frequency: 1.0,
            show_instructions: true,
            show_menu: false, // Menu is hidden by default
            menu_state: state, // Initialize menu state
        }
    }

    pub fn toggle_menu(&mut self) {
        self.show_menu = !self.show_menu;
    }

    pub fn menu_up(&mut self) {
        let i = match self.menu_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.available_functions.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.menu_state.select(Some(i));
    }

    pub fn menu_down(&mut self) {
        let i = match self.menu_state.selected() {
            Some(i) => {
                if i >= self.available_functions.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.menu_state.select(Some(i));
    }

    pub fn select_function(&mut self) {
        if let Some(i) = self.menu_state.selected() {
            let func_name = &self.available_functions[i];
            let func_type = match func_name.as_str() {
                "sin(x)" => FunctionType::Sine,
                "cos(x)" => FunctionType::Cosine,
                "tan(x)" => FunctionType::Tangent,
                "sec(x)" => FunctionType::Secant,
                "csc(x)" => FunctionType::Cosecant,
                "cot(x)" => FunctionType::Cotangent,
                "Parametric: (cos(t), sin(t))" => FunctionType::Parametric,
                "Inequality: x > 0" => FunctionType::Inequality { expr: |x| x > 0.0 },
                _ => return,
            };
            self.add_function(func_name.clone(), func_type);
            self.show_menu = false; // Close the menu after selection
        }
    }

    pub fn add_function(&mut self, name: String, func_type: FunctionType) {
        let function = Function::new(name, func_type);
        self.functions.push(function);
    }

    pub fn reset_graph(&mut self) {
        self.functions.clear();
    }

    pub fn increase_amplitude(&mut self) {
        self.amplitude += 0.1;
        self.update_functions();
    }

    pub fn decrease_amplitude(&mut self) {
        if self.amplitude > 0.1 {
            self.amplitude -= 0.1;
        }
        self.update_functions();
    }

    pub fn increase_frequency(&mut self) {
        self.frequency += 0.1;
        self.update_functions();
    }

    pub fn decrease_frequency(&mut self) {
        if self.frequency > 0.1 {
            self.frequency -= 0.1;
        }
        self.update_functions();
    }

    fn update_functions(&mut self) {
        for function in &mut self.functions {
            function.update_data(self.amplitude, self.frequency);
        }
    }

    pub fn on_key(&mut self, _key: KeyEvent) {
        // Handle key events for additional features
    }
}

