use cgmath::Vector2;

pub(super) struct Piece {
    pub kind: Kind,
    pub position: Vector2<usize>,
    pub rotation: Rotation,
}

impl Piece {
    const CELL_COUNT: usize = 4;

    // todo is a divergent type, returns automatically any type
    pub fn cells(&self) -> Option<impl Iterator<Item = Vector2<usize>>> {
        self.kind.cells()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    O,
    I,
    T,
    L,
    J,
    S,
    Z,
}

impl Kind {
    pub const ALL: [Self; 7] = [
        Self::O,
        Self::I,
        Self::T,
        Self::L,
        Self::J,
        Self::S,
        Self::Z,
    ];

    // adding static references for references, restricts references of values to the static
    pub fn cells(&self) -> impl Iterator<Item = &'static Vector2<isize>> {
        match self {
            Kind::O => &[(0, 0), (0, 1), (1, 0), (1, 1)],
            Kind::I => &[(-1, 0), (0, 0), (1, 0), (2, 0)],
            Kind::T => &[(-1, 0), (0, 0), (1, 0), (0, 1)],
            Kind::L => &[(-1, 0), (0, 0), (1, 0), (1, 1)],
            Kind::J => &[(-1, 1), (-1, 0), (0, 0), (1, 0)],
            Kind::S => &[(-1, 0), (0, 0), (0, 1), (1, 1)],
            Kind::Z => &[(-1, 1), (0, 1), (0, 0), (1, 0)],
        }
        .iter()
        .map(From::from)
    }
}

pub enum Rotation {
    N,
    S,
    E,
    W,
}

impl<S> std::ops::Mul<Rotation> for Vector2<S>
where
    S: std::ops::Neg<Output = S>,
{
    type Output = Self;

    fn mul(self, rotation: Rotation) -> Self::Output {
        match rotation {
            Rotation::N => self,
            Rotation::S => Vector2::new(-self.x, -self.y),
            Rotation::E => Vector2::new(self.y, -self.x),
            Rotation::W => Vector2::new(-self.y, self.x),
        }
    }
}
