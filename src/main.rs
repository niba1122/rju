#![feature(proc_macro)]
extern crate rju;
extern crate rju_macro;

use rju::{h, Renderer, DOMType, Attribute, Component, VirtualDOM};
use rju_macro::{html};


#[macro_use]
extern crate lazy_static;

pub fn render(id: i32) -> VirtualDOM {
    let hoge = "hogestring".to_string();
    html!(r#"
        <div>
            <h1 bind:class='hoge'>
                Hello World!
            </h1>
            <p>
                new component system!
            </p>
        </div>
    "#)
}

fn factory() -> Component {
    Component {
        render: render
    }
}

fn main() {
    // Renderer::render("test", OriginalComponentFactory::create);
    Renderer::render("test", factory)
}

#[no_mangle]
pub extern fn sum(a: i32, b: i32) -> i32 {
    a * b
}