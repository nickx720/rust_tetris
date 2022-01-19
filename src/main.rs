#![allow(dead_code)]
use engine::Engine;

mod engine;
mod interface;
fn main() {
    let engine = Engine::new();
    interface::run(engine)
}
