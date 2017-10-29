//#[cfg(test)]
//mod tests {
//    #[test]
//    fn it_works() {
//        assert_eq!(2 + 2, 4);
//    }
//}
//
use std::fmt;

use std::ffi::CString;
extern crate libc;

pub struct VirtualDOM {
    name: String,
    children: Vec<VirtualDOM>,
    attributes: Vec<String>
}

impl fmt::Display for VirtualDOM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut children_string = String::new();
        for child in self.children.iter() {
            children_string.push_str(&format!("{}", child))
        }
        write!(f, "[{}({}){}]", self.name, children_string, "")
    }
}

pub struct Component {
    root_dom: VirtualDOM
}
impl Component {
    fn render(&self) -> VirtualDOM {
        VirtualDOM {name: "".to_string(), children: vec![], attributes: vec![]}
    }
    fn update(&self) {
        //root_dom.children = vec![self.render()]
    }
}

pub fn h(tagname: &str, children: Vec<VirtualDOM>, attributes: Vec<String>) -> VirtualDOM {
    return VirtualDOM {
        name: tagname.to_string(),
        children: children,
        attributes: attributes
    }
}

pub fn start() {
    unsafe {
      ffi::emscripten_set_main_loop(ffi::hoge, 0, 0);
    }
}

/// Safe rust wrapper for emscripten_run_script_int (basically, JS eval()).
pub fn eval(x: &str) -> i32 {
    let x = CString::new(x).unwrap();
    let ptr = x.as_ptr();
    unsafe { ffi::emscripten_run_script_int(ptr) }
}

// This is mostly standard Rust-C FFI stuff.
mod ffi {
    use libc::*;

    extern "C" {
        // This extern is defined in `html/library.js`.

        // This extern is built in by Emscripten.
        pub fn emscripten_run_script_int(x: *const c_char) -> c_int;
        pub fn emscripten_set_main_loop(m: extern fn(), fps: c_int, infinite: c_int);
    }
    pub extern fn hoge() {
        //println!("loop");
    }
}

