#![feature(proc_macro)]
extern crate rju;
extern crate rju_macro;

use rju::{h, start, Renderer};
use rju::{eval};
use rju_macro::{html};


fn main() {
    //library_func();
    eval("
    console.log('hogehoge');
    var button = document.createElement('input');
    button.type = 'button';
    button.value = 'Dispatch event';
    document.body.appendChild(button);

    var ul = document.createElement('ul');
    ul.id = 'list';
    document.body.appendChild(ul);

    button.addEventListener('click', function () {
      var li = document.createElement('li');
      li.textContent = sum(1, 2);
      document.getElementById('list').appendChild(li);
    });
    
    ");
    //sum(1, 2);
    html!("
      <html>
        <body>
          <h1>Hello World</h1>
          <p>aiueo</p>
        </body>
      </html>
    ");

    Renderer::patch("test", html!("<strong attr='test'></strong>"));

    start();
    
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