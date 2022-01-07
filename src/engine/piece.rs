use cgmath::Vector2;

pub(super) struct Piece {
    pub kind: Kind,
    pub position: Vector2<usize>,
    pub rotation: Rotation,
}

impl Piece {}

pub enum Kind {
    Square,
    Line,
    T,
    J,
    S,
    Z,
}

impl Kind {}

pub enum Rotation {
    N,
    S,
    E,
    W,
}
