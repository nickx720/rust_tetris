use cgmath::EuclideanSpace;

use super::{Coordinate, Matrix, Offset};

pub(super) struct Piece {
    pub kind: Kind,
    pub position: Offset,
    pub rotation: Rotation,
}

impl Piece {
    const CELL_COUNT: usize = 4;

    // todo is a divergent type, returns automatically any type
    pub fn cells(&self) -> Option<[Coordinate; Self::CELL_COUNT]> {
        let offsets = self.kind.cells().map(self.rotator()).map(self.positioner());
        let mut coords = [Coordinate::origin(); Self::CELL_COUNT];
        for (offset, coord_slot) in offsets.into_iter().zip(&mut coords) {
            let positive_offset = offset.cast::<usize>()?;
            let coord = Coordinate::from_vec(positive_offset);
            if Matrix::in_bounds(coord) {
                *coord_slot = coord;
            } else {
                return None;
            }
        }

        Some(coords)
    }

    fn rotator(&self) -> impl Fn(Offset) -> Offset + '_ {
        |cell| match self.kind {
            Kind::O => cell,
            _ => cell * self.rotation,
        }
    }

    fn positioner(&self) -> impl Fn(Offset) -> Offset {
        let position = self.position;
        move |cell| cell + position
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
    pub fn cells(&self) -> [Offset; Piece::CELL_COUNT] {
        match self {
            Kind::O => &[(1, 1), (1, 2), (2, 1), (2, 2)],
            Kind::I => &[(0, 2), (1, 2), (2, 2), (3, 2)],
            Kind::T => &[(0, 1), (1, 1), (2, 1), (1, 2)],
            Kind::L => &[(0, 1), (1, 1), (2, 1), (2, 2)],
            Kind::J => &[(0, 2), (0, 1), (1, 1), (2, 1)],
            Kind::S => &[(1, 1), (1, 1), (1, 2), (2, 2)],
            Kind::Z => &[(0, 2), (1, 2), (1, 1), (2, 1)],
        }
        .map(Offset::from)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Rotation {
    N,
    S,
    E,
    W,
}

impl std::ops::Mul<Rotation> for Offset {
    type Output = Self;

    fn mul(self, rotation: Rotation) -> Self::Output {
        match rotation {
            Rotation::N => self,
            Rotation::S => Offset::new(-self.x, -self.y),
            Rotation::E => Offset::new(self.y, -self.x),
            Rotation::W => Offset::new(-self.y, self.x),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn s_piece_positioning() {
        let z = Piece {
            kind: Kind::Z,
            position: Offset::new(5, 6),
            rotation: Rotation::W,
        };

        assert_eq!(
            z.cells(),
            Some([(4, 5), (4, 6), (5, 6), (5, 7)].map(Coordinate::from)),
        )
    }
}
