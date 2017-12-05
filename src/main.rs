#![feature(proc_macro)]
extern crate rju;
extern crate rju_macro;

use rju::{h, Renderer, DOMType, Attribute, Component, VirtualDOM, State, Mutex, Arc, Any};
use rju_macro::{html};

#[macro_use]
extern crate lazy_static;

struct MainState {
    count: i32
}

impl State for MainState {
    fn as_any(&mut self) -> &mut Any {
        self
    }
}

pub fn render(component: Arc<Mutex<Component>>) -> VirtualDOM {
    let mut count: i32 = 0;
    let mut c = component.lock().unwrap();
    let mut sa = c.state.lock().unwrap();
    let mut s = sa.as_any().downcast_mut::<MainState>().unwrap();
    html!(r#"
        <div>
            <h1 bind:class='s.count.to_string()'>
                Hello World!
            </h1>
            <p>
                count: {s.count.to_string()}<br />
            </p>
            <button on:click="handle_click">click me!</button>
        </div>
    "#)
}

fn handle_click(component: Arc<Mutex<Component>>) {
    let mut c = component.lock().unwrap();
    let mut sa = c.state.lock().unwrap();
    let mut s = sa.as_any().downcast_mut::<MainState>().unwrap();
    s.count = s.count + 3;
    component.lock().unwrap().update();
}

fn factory() -> Component {
    Component {
        render: render,
        state: Arc::new(Mutex::new(MainState {
            count: 0
        }))
    }
}

fn main() {
    Renderer::render("test", factory)
}

#[no_mangle]
pub extern fn sum(a: i32, b: i32) -> i32 {
    a * b
}