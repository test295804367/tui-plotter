
use std::ffi::{CStr, CString};

pub struct Calculator {
    calc: *mut std::ffi::c_void,
}

impl Calculator {
    pub fn new() -> Calculator {
        unsafe {
            Calculator {
                calc: calculator_new(),
            }
        }
    }

    pub fn evaluate(&self, expression: &str) -> String {
        let c_expression = CString::new(expression).unwrap();
        unsafe {
            let c_result = calculator_evaluate(self.calc, c_expression.as_ptr());
            CStr::from_ptr(c_result).to_string_lossy().into_owned()
        }
    }
}

impl Drop for Calculator {
    fn drop(&mut self) {
        unsafe {
            calculator_free(self.calc);
        }
    }
}

extern "C" {
    pub fn calculator_new() -> *mut std::ffi::c_void;
    pub fn calculator_evaluate(calc: *mut std::ffi::c_void, expression: *const std::ffi::c_char) -> *const std::ffi::c_char;
    pub fn calculator_free(calc: *mut std::ffi::c_void);
}
