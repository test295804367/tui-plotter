use crate::tui::functions::{Function, FunctionType};
use crossterm::event::KeyEvent;

pub struct App {
    pub functions: Vec<Function>,
    pub amplitude: f64,
    pub frequency: f64,
    pub show_instructions: bool,  // Add this field
}

impl App {
    pub fn new() -> Self {
        App {
            functions: Vec::new(),
            amplitude: 1.0,
            frequency: 1.0,
            show_instructions: true,  // Instructions shown by default
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

