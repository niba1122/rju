#![feature(proc_macro)]
extern crate rju;
extern crate rju_macro;

use rju::*;
use rju_macro::{html};

use components;

struct MainState {
    count: i32,
    text: String
}

impl State for MainState {
    fn as_any(&mut self) -> &mut Any {
        self
    }
}

fn render(c: Arc<Mutex<Component>>) -> VirtualDOM {
    let mut component = c.lock().unwrap();
    let mut s = component.state.lock().unwrap();
    let mut state = s.as_any().downcast_mut::<MainState>().unwrap();
    html!(r#"
        <div>
            <h1 bind:class='state.count.to_string()'>
                Hello World!
            </h1>
            <p>
                {state.text}
            </p>
            <p>
                count: {state.count.to_string()}<br />
            </p>
            <button on:click="handle_click">click me!</button>
            <component:child_factory />
        </div>
    "#)
}

fn handle_click(c: Arc<Mutex<Component>>) {
    let mut component = c.lock().unwrap();
    let mut s = component.state.lock().unwrap();
    let mut state = s.as_any().downcast_mut::<MainState>().unwrap();
    state.count = state.count + 1;
    state.text = fizzbuzz(state.count);
    component.update();
}

fn fizzbuzz(n: i32) -> String {
    match n {
        n if n % 5 == 0 && n % 3 == 0 => "fizzbuzz".to_string(),
        n if n % 5 == 0 => "buzz".to_string(),
        n if n % 3 == 0 => "fizz".to_string(),
        _ => n.to_string()
    }
}

pub fn factory() -> InitialComponent {
    InitialComponent {
        render: render,
        state: Arc::new(Mutex::new(MainState {
            count: 0,
            text: String::from("")
        }))
    }
}

fn child_factory() -> InitialComponent {
    components::child::factory()
}