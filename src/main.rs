#![allow(dead_code)]
use engine::{Color, Engine, Matrix};

mod engine;
mod interface;
fn main() {
    let mut matrix = Matrix::blank();
    matrix[(1, 0).into()] = Some(Color::Green);
    let engine = Engine::with_matrix(matrix);
    interface::run(engine)
}
