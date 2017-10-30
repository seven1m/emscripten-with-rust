extern crate emscripten_sys;

use emscripten_sys::emscripten_run_script_string;

use std::ffi::CString;

extern "C" {
  fn hello_from_rust();
}

fn main() {
    println!("Hello, Emscripten!");
    let script = CString::new("console.log('eval string')").unwrap();
    let script_str_ptr = script.as_ptr();
    unsafe {
        hello_from_rust();
        emscripten_run_script_string(script_str_ptr);
    }
}
