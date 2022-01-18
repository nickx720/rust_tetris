#![allow(dead_code)]
#![feature(bool_to_option)]
use engine::Engine;
use interface::Interface;

mod engine;
mod interface;
fn main() {
    let engine = Engine::new();
    Interface::run(engine)
}
