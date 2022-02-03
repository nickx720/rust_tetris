#![allow(dead_code)]
use engine::{piece::Kind as PieceKind, Color, Engine, Matrix};

mod engine;
mod interface;
fn main() {
    let mut matrix = Matrix::blank();
    matrix[(1, 1).into()] = Some(Color::Green);
    let mut engine = Engine::with_matrix(matrix);
    engine.DEBUG_test_cursor_location(PieceKind::T, (5, 5).into());
    interface::run(engine)
}
