//#[cfg(test)]
//mod tests {
//    #[test]
//    fn it_works() {
//        assert_eq!(2 + 2, 4);
//    }
//}
//
extern crate libc;

use std::fmt;
use std::ffi::CString;

extern crate rand;

use rand::Rng;

pub struct VirtualDOM {
    name: String,
    children: Vec<VirtualDOM>,
    attributes: Vec<String>,
}

impl fmt::Display for VirtualDOM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut children_string = String::new();
        for child in self.children.iter() {
            children_string.push_str(&format!("{}", child))
        }
        write!(f, "[{}({}){}]", self.name, "", children_string)
    }
}

pub struct Component {
    root_dom: VirtualDOM,
}
impl Component {
    fn render(&self) -> VirtualDOM {
        VirtualDOM {
            name: "".to_string(),
            children: vec![],
            attributes: vec![],
        }
    }
    fn update(&self) {
        //root_dom.children = vec![self.render()]
    }
}

pub struct Renderer;
impl Renderer {
    pub fn patch(dom_id: &str, virtual_dom: VirtualDOM) {
        eval(&format!(
            "
            (function() {{
                var parentDOMId = '{}';
                document.getElementById(parentDOMId).innerHTML = null;
            }})();
        ",
            dom_id
        ));
        Renderer::render_dom(dom_id, &virtual_dom)
    }

    fn render_dom(dom_id: &str, virtual_dom: &VirtualDOM) {
        let mut rng = rand::thread_rng();
        let new_dom_id: &str = &rng.gen::<i32>().to_string();

        let hoge: String;
        if virtual_dom.attributes.len() > 0 {
            hoge = virtual_dom.attributes[0].clone();
        } else {
            hoge = "".to_string();
        }

        eval(&format!(
            "
            (function() {{
                var domName = '{}';
                var parentDOMId = '{}';
                var newDOMId = '{}';
                var domAttributes = \"{}\";
                var dom = document.createElement(domName);
                dom.id = newDOMId;
                dom.textContent = '[' + domName + '](' + domAttributes + ')';
                document.getElementById(parentDOMId).appendChild(dom);
            }})();
        ",
            virtual_dom.name,
            dom_id,
            new_dom_id,
            hoge
        ));
        for child in virtual_dom.children.iter() {
            Renderer::render_dom(new_dom_id, child);
        }
    }
}

pub fn h(tagname: &str, children: Vec<VirtualDOM>, attributes: Vec<&str>) -> VirtualDOM {
    return VirtualDOM {
        name: tagname.to_string(),
        children: children,
        attributes: attributes
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>(),
    };
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
        pub fn emscripten_set_main_loop(m: extern "C" fn(), fps: c_int, infinite: c_int);
    }
    pub extern "C" fn hoge() {
        //println!("loop");
    }
}
