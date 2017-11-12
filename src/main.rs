#![feature(proc_macro)]
extern crate rju;
extern crate rju_macro;

use rju::{h, Renderer, DOMType, Attribute};
use rju_macro::{html};


fn main() {
    html!("
      <html>
        <body>
          <h1>Hello World</h1>
          <p>aiueo</p>
        </body>
      </html>
    ");

    Renderer::patch("test", html!("
      <div>
        <strong bind:attr='(1 + 2).to_string()' b:attr2='hogehoge'>
          Hello World!!!!!!
        </strong>
      </div>"));
//    start();
    
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