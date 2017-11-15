#![feature(proc_macro)]
extern crate rju;
extern crate rju_macro;

use rju::{h, Renderer, DOMType, Attribute, Component, VirtualDOM};
use rju_macro::{html};

use std::cell::RefCell;

use std::sync::RwLock;

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

impl Component for OriginalComponent {
    fn create() -> OriginalComponent {
        OriginalComponent { id: 12345}
    }
    fn render(&self) -> VirtualDOM {
        html!(r#"
            <div>
                <h1 bind:class='self.id.to_string()'>
                    Hello World!
                </h1>
                <p>
                    new component system!
                </p>
                <button>Add</button>
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