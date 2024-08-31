use std::f64::consts::PI;

pub enum FunctionType {
    Sine,
    Cosine,
    Tangent,
    Secant,
    Cosecant,
    Cotangent,
    Parametric,
    Inequality { expr: fn(f64) -> bool },
}

pub struct Function {
    pub name: String,
    pub func_type: FunctionType,
    pub data: Vec<(f64, f64)>,
}

impl Function {
    pub fn new(name: String, func_type: FunctionType) -> Self {
        let data = match &func_type {
            FunctionType::Sine => Self::generate_sine_wave(1.0, 1.0),
            FunctionType::Cosine => Self::generate_cosine_wave(1.0, 1.0),
            FunctionType::Tangent => Self::generate_tangent_wave(1.0, 1.0),
            FunctionType::Secant => Self::generate_secant_wave(1.0, 1.0),
            FunctionType::Cosecant => Self::generate_cosecant_wave(1.0, 1.0),
            FunctionType::Cotangent => Self::generate_cotangent_wave(1.0, 1.0),
            FunctionType::Parametric => Self::generate_parametric(1.0),
            FunctionType::Inequality { .. } => Vec::new(),
        };

        Function { name, func_type, data }
    }

    pub fn generate_sine_wave(amplitude: f64, frequency: f64) -> Vec<(f64, f64)> {
        (-50..=50).map(|x| {
            let x = x as f64 / 5.0;
            (x, amplitude * (frequency * x).sin())
        }).collect()
    }

    pub fn generate_cosine_wave(amplitude: f64, frequency: f64) -> Vec<(f64, f64)> {
        (-50..=50).map(|x| {
            let x = x as f64 / 5.0;
            (x, amplitude * (frequency * x).cos())
        }).collect()
    }

    pub fn generate_tangent_wave(amplitude: f64, frequency: f64) -> Vec<(f64, f64)> {
        (-50..=50).map(|x| {
            let x = x as f64 / 5.0;
            (x, amplitude * (frequency * x).tan())
        }).collect()
    }

    pub fn generate_secant_wave(amplitude: f64, frequency: f64) -> Vec<(f64, f64)> {
        (-50..=50).map(|x| {
            let x = x as f64 / 5.0;
            (x, amplitude / (frequency * x).cos())
        }).collect()
    }

    pub fn generate_cosecant_wave(amplitude: f64, frequency: f64) -> Vec<(f64, f64)> {
        (-50..=50).map(|x| {
            let x = x as f64 / 5.0;
            (x, amplitude / (frequency * x).sin())
        }).collect()
    }

    pub fn generate_cotangent_wave(amplitude: f64, frequency: f64) -> Vec<(f64, f64)> {
        (-50..=50).map(|x| {
            let x = x as f64 / 5.0;
            (x, amplitude / (frequency * x).tan())
        }).collect()
    }

    pub fn generate_parametric(amplitude: f64) -> Vec<(f64, f64)> {
        (0..=100).map(|t| {
            let t = t as f64 / 10.0 * 2.0 * PI;
            (amplitude * t.cos(), amplitude * t.sin())
        }).collect()
    }

    pub fn update_data(&mut self, amplitude: f64, frequency: f64) {
        self.data = match &self.func_type {
            FunctionType::Sine => Self::generate_sine_wave(amplitude, frequency),
            FunctionType::Cosine => Self::generate_cosine_wave(amplitude, frequency),
            FunctionType::Tangent => Self::generate_tangent_wave(amplitude, frequency),
            FunctionType::Secant => Self::generate_secant_wave(amplitude, frequency),
            FunctionType::Cosecant => Self::generate_cosecant_wave(amplitude, frequency),
            FunctionType::Cotangent => Self::generate_cotangent_wave(amplitude, frequency),
            FunctionType::Parametric => Self::generate_parametric(amplitude),
            FunctionType::Inequality { .. } => Vec::new(),
        };
    }
}

