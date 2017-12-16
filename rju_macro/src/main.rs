#![feature(proc_macro)]
extern crate rju;
extern crate rju_macro;
    
use rju::*;
use rju_macro::{html};

mod components;

fn main() {
    Renderer::render("test", components::main_component::factory)
}