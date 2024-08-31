
extern "C" {
    pub fn calculator_new() -> *mut std::ffi::c_void;
    pub fn calculator_evaluate(calc: *mut std::ffi::c_void, expression: *const std::ffi::c_char) -> *const std::ffi::c_char;
    pub fn calculator_free(calc: *mut std::ffi::c_void);
}
