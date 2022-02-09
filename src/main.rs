#![allow(dead_code)]
#![feature(array_chunks)]
use engine::{piece::Kind as PieceKind, Color, Engine, Matrix};

mod engine;
mod interface;
fn main() {
    let mut matrix = Matrix::blank();
    for col in 0..=6 {
        matrix[(col, 0).into()] = Some(Color::Green);
    }
    let mut engine = Engine::with_matrix(matrix);
    engine.DEBUG_test_cursor_location(PieceKind::T, (5, 15).into());
    interface::run(engine)
}
