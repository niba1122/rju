#![feature(proc_macro)]
extern crate rju;
extern crate rju_macro;

use rju::{h, Renderer, DOMType, Attribute, Component, VirtualDOM};
use rju_macro::{html};


struct State {
    hoge: String,
    fuga: i32
}

impl State {
    pub fn add(&mut self) {
        self.fuga = self.fuga + 1
    }
}

// fn add(&mut count) {
//     count = count + 1
// }

fn test() {
    println!("print!!!!!!!!!!!!!!!!!1");
}

fn main() {
    let mut count: i32 = 0;

    let hogehoge = "hogehoge";
    let component = Component {
        parent_dom_id: "test",
        render: |ref component| -> VirtualDOM {
            component.update();
            html!(r#"
                <div>
                    <strong bind:attr='125.to_string()' b:attr2='hogehoge'>
                        Hello World!!!!!!
                    </strong>
                    <p>
                        Refactor!!!!!!!!!!!!
                    </p>
                    <button on:click="test">add</button>
                </div>
            "#)
        }
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