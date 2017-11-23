#![feature(proc_macro)]
extern crate rju;
extern crate rju_macro;

use rju::{h, Renderer, DOMType, Attribute, Component, VirtualDOM, State};
use rju_macro::{html};

use std::sync::Mutex;
use std::sync::Arc;
use std::any::Any;

#[macro_use]
extern crate lazy_static;

struct MainState {
    count: i32
}

impl State for MainState {
    fn as_any(&self) -> &Any {
        self
    }
}

pub fn render(component: Arc<Mutex<Component>>) -> VirtualDOM {
    let hoge = "hogestring".to_string();
    let current_state: i32 = component.lock().unwrap().state.as_any().downcast_ref::<MainState>().unwrap().count;
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
    let current_state = component.lock().unwrap().state.as_any().downcast_ref::<MainState>().unwrap().count;
    component.lock().unwrap().set_state(Box::new(MainState {
        count: current_state + 1
    }));
    component.lock().unwrap().update();
}

fn factory() -> Component {
    Component {
        render: render,
        state: Box::new(MainState {
            count: 0
        })
    }
}

fn main() {
    Renderer::render("test", factory)
}

#[no_mangle]
pub extern fn sum(a: i32, b: i32) -> i32 {
    a * b
}