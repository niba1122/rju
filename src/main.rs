#![feature(proc_macro)]
extern crate rju;
extern crate rju_macro;

use rju::{h, Renderer, DOMType, Attribute, Component, VirtualDOM};
use rju_macro::{html};


#[macro_use]
extern crate lazy_static;

#[derive(Hash)]
pub struct OriginalComponent {
    id: i32,
}

trait IOriginalComponent : Component {
    fn handle_click(&self) {
        self.update()
    }
}

struct State {
    hoge: &'static str
}

impl Component for OriginalComponent {
    fn create() -> OriginalComponent {
        OriginalComponent {
            id: 12345,
        }
    }
    fn render(&self) -> VirtualDOM {
        let hoge = &self;
        html!(r#"
            <div>
                <h1 bind:class='self.id.to_string()'>
                    Hello World!
                </h1>
                <p>
                    new component system!
                </p>
                <button on:click='Box::new(||{println!("clicked!!!!!");})'>Add</button>
            </div>
        "#)
    }
}

impl IOriginalComponent for OriginalComponent {}

fn main() {
    Renderer::render("test", OriginalComponent::create);
}

#[no_mangle]
pub extern fn sum(a: i32, b: i32) -> i32 {
    a * b
}