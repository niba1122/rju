#![feature(proc_macro)]
extern crate rju;
extern crate rju_macro;

use rju::{h, Renderer, DOMType, Attribute, Component, VirtualDOM};
use rju_macro::{html};

use std::cell::RefCell;

use std::sync::RwLock;

#[macro_use]
extern crate lazy_static;

fn add() {
    // let mut h = hoge.write().unwrap();
    // *h += 1;
    component.update();
}

fn render() -> VirtualDOM {
    let hoge2 = *hoge.read().unwrap();
    let hoge_string = hoge2.to_string();
    html!(r#"
        <div>
            <strong bind:class='hoge2.to_string()'>
                Hello World!!!!!!
            </strong>
            <p>
                nemukunaiyo!!!!!!!!!!!
            </p>
            <button on:click="add">Add</button>
        </div>
    "#)
}

lazy_static! {
    static ref component: Component = {
        let mut c = Component {
            parent_dom_id: "test",
            render: render,
        };
        c
    };
    static ref state: State = {
        let mut s = State {
            hoge: "aiueo".to_string(),
            fuga: 0,
            foo: RwLock::new(vec![])
        };
        s
    };
    static ref hoge: RwLock<i32> = {
        let mut m = 0;
        RwLock::new(m)
    };
}

struct State {
    hoge: String,
    fuga: i32,
    foo: RwLock<Vec<i32>>
}

impl State {
    pub fn add(&mut self) {
        self.fuga = self.fuga + 1
    }
}

fn main() {
    Renderer::initialize();
    component.update();
    Renderer::start();
}


#[no_mangle]
pub extern fn sum(a: i32, b: i32) -> i32 {
    // println!("success: {}", html!("
    //     <html>
    //         <body>
    //         <h1>Hello World</h1>
    //     </body>
    //   </html>
    // "));
    a * b
}