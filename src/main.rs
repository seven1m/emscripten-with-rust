extern crate emscripten_sys;

use emscripten_sys::emscripten_run_script_string;

use std::ffi::CString;

fn main() {
    println!("Hello, Emscripten!");
    let script = CString::new("alert('this is amazing')").unwrap();
    let script_str_ptr = script.as_ptr();
    unsafe {
        emscripten_run_script_string(script_str_ptr);
    }
}
