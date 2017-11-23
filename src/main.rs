#![feature(proc_macro)]
extern crate rju;
extern crate rju_macro;

use rju::{h, Renderer, DOMType, Attribute, Component, VirtualDOM};
use rju_macro::{html};

pub use std::sync::Mutex;
pub use std::sync::Arc;

#[macro_use]
extern crate lazy_static;

pub fn render(component: Arc<Mutex<Component>>) -> VirtualDOM {
    let hoge = "hogestring".to_string();
    let current_state = component.lock().unwrap().state;
    html!(r#"
        <div>
            <h1 bind:class='current_state.to_string()'>
                Hello World!
            </h1>
            <p>
                count: {current_state}<br />
                hoge: {hoge}
            </p>
            <button on:click="handle_click">click me!</button>
        </div>
    "#)
}

fn handle_click(component: Arc<Mutex<Component>>) {
    let current_state = component.lock().unwrap().state;
    component.lock().unwrap().set_state(current_state + 1);
    component.lock().unwrap().update();
}

fn factory() -> Component {
    Component {
        render: render,
        state: 0
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