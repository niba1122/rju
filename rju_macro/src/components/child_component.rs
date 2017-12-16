#![feature(proc_macro)]
extern crate rju;
extern crate rju_macro;

use rju::*;
use rju_macro::{html};

struct State {
    count: i32,
    text: String
}

impl BaseState for State {
    fn as_any(&mut self) -> &mut Any {
        self
    }
}

fn render(component: Arc<Mutex<Component>>) -> VirtualDOM {
    let mut count: i32 = 0;
    let mut c = component.lock().unwrap();
    let mut sa = c.state.lock().unwrap();
    let mut s = sa.as_any().downcast_mut::<State>().unwrap();
    html!(r#"
        <div>
            <h2>
                child component!
            </h2>
        </div>
    "#)
}

fn handle_click(component: Arc<Mutex<Component>>) {
    let mut c = component.lock().unwrap();
    let mut sa = c.state.lock().unwrap();
    let mut s = sa.as_any().downcast_mut::<State>().unwrap();

    s.count = s.count + 2;
    c.update();
}

pub fn factory() -> InitialComponent {
    InitialComponent {
        render: render,
        state: Arc::new(Mutex::new(State {
            count: 0,
            text: String::from("child state!")
        })),
    }
}