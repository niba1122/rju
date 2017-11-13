#![feature(proc_macro)]
extern crate rju;
extern crate rju_macro;

use rju::{h, Renderer, DOMType, Attribute, Component, VirtualDOM};
use rju_macro::{html};

use std::cell::RefCell;


struct State {
    hoge: String,
    fuga: i32
}

impl State {
    pub fn add(&mut self) {
        self.fuga = self.fuga + 1
    }
}

fn test() {
    println!("print!!!!!!!!!!!!!!!!!");
}

fn main() {
    let component = Component {
        parent_dom_id: "test",
        render: |component| -> VirtualDOM {
            html!(r#"
                <div>
                    <strong bind:class='"foobar".to_string()'>
                        Hello World!!!!!!
                    </strong>
                    <p>
                        nemuiyo!!!!!!!!!!!
                    </p>
                    <button on:click="test">add</button>
                </div>
            "#)
        },
    };
    component.update()
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