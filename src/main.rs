#![feature(proc_macro)]
extern crate rju_macro;
pub use rju_macro::{html};

use std::ffi::CString;
extern crate libc;

/// Safe rust wrapper for our JS function `alert`.
fn alert(x: &str) {
    let x = CString::new(x).unwrap();
    let ptr = x.as_ptr();
    unsafe { ffi::alert(ptr) }
}

/// Safe rust wrapper for emscripten_run_script_int (basically, JS eval()).
fn eval(x: &str) -> i32 {
    let x = CString::new(x).unwrap();
    let ptr = x.as_ptr();
    unsafe { ffi::emscripten_run_script_int(ptr) }
}

// This is mostly standard Rust-C FFI stuff.
mod ffi {
    use libc::*;

    extern "C" {
        // This extern is defined in `html/library.js`.

        pub fn alert(x: *const c_char);
        // This extern is built in by Emscripten.
        pub fn emscripten_run_script_int(x: *const c_char) -> c_int;
        pub fn emscripten_set_main_loop(m: extern fn(), fps: c_int, infinite: c_int);
    }
}

fn h(tagname: &str, children: Vec<String>, attributes: Vec<String>) -> String {
    //println!("{} generated!", tagname);
    return format!("{}[{}]", tagname, children.concat())
}

extern fn hoge() {
    //println!("loop");
}

fn main() {
    //library_func();
    eval("
    console.log('hogehoge');
    var button = document.createElement('input');
    button.type = 'button';
    button.value = 'Dispatch event';
    document.body.appendChild(button);

    var ul = document.createElement('ul');
    ul.id = 'list';
    document.body.appendChild(ul);

    button.addEventListener('click', function () {
      var li = document.createElement('li');
      li.textContent = sum(1, 2);
      document.getElementById('list').appendChild(li);
    });
    
    ");
    //sum(1, 2);
    html!("
      <html>
        <body>
          <h1>Hello World</h1>
          <p>aiueo</p>
        </body>
      </html>
    ");
    unsafe {
      ffi::emscripten_set_main_loop(hoge, 0, 0);
    }
}

#[no_mangle]
pub extern fn sum(a: i32, b: i32) -> i32 {
    println!("success: {}", html!("
      <html>
        <body>
          <h1>Hello World</h1>
        </body>
      </html>
    "));
    a * b
}
