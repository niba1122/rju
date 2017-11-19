#![feature(proc_macro)]
extern crate rju;
extern crate rju_macro;

use rju::{h, Renderer, DOMType, Attribute, Component, VirtualDOM, COMPONENTS};
use rju_macro::{html};


#[macro_use]
extern crate lazy_static;

pub fn render(component_id: i32) -> VirtualDOM {
    let hoge = "hogestring".to_string();
    let current_state = COMPONENTS.lock().unwrap().get_mut(&component_id).unwrap().state;
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

fn handle_click(component_id: i32) {
    let current_state = COMPONENTS.lock().unwrap().get_mut(&component_id).unwrap().state;
    COMPONENTS.lock().unwrap().get_mut(&component_id).unwrap().set_state(current_state + 1);
    COMPONENTS.lock().unwrap().get_mut(&component_id).unwrap().update();
    println!("success{}", current_state);
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