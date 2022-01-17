use std::ops::{Index, IndexMut};

use rand::{
    prelude::{SliceRandom, ThreadRng},
    thread_rng,
};

use self::piece::{Kind as PieceKind, Piece};

mod piece;

type Coordinate = cgmath::Point2<usize>;
type Offset = cgmath::Vector2<isize>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MoveKind {
    Left,
    Right,
}

impl MoveKind {
    fn offset(&self) -> Offset {
        match self {
            MoveKind::Left => Offset::new(-1, 0),
            MoveKind::Right => Offset::new(1, 0),
        }
    }
}

pub struct Engine {
    matrix: Matrix,
    bag: Vec<PieceKind>,
    rng: ThreadRng,
    cursor: Option<Piece>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            matrix: Matrix::blank(),
            bag: Vec::new(),
            rng: thread_rng(),
            cursor: None,
        }
    }

    fn refill_bag(&mut self) {
        // Pull all pieces in bag
        // shuffle bag
        debug_assert!(self.bag.is_empty());
        self.bag.extend_from_slice(PieceKind::ALL.as_slice());
        self.bag.shuffle(&mut self.rng);
    }

    fn place_cursor(&mut self) {
        // Assert that the piece does not overlap filled cells
        let cursor = self
            .cursor
            .take()
            .expect("Called place_cursor without a cursor");

        debug_assert!(
            self.matrix.is_placeable(&cursor),
            "The cursor was placed in an unplaceable location {:?}",
            cursor
        );

        for coord in cursor.cells().unwrap() {
            self.matrix[coord] = true;
        }
    }

    fn move_cursor(&mut self, kind: MoveKind) -> Result<(), ()> // Ok(()) , Err(())
    {
        let cursor = match self.cursor.as_mut() {
            Some(cursor) => cursor,
            None => return Ok(()),
        };

        let new = cursor.moved_by(kind.offset());
        if self.matrix.is_clipping(&new) {
            return Err(());
        }
        Ok(self.cursor = Some(new))
    }
}

struct Matrix([bool; Self::SIZE]);

impl Matrix {
    const WIDTH: usize = 10;
    const HEIGHT: usize = 20;
    const SIZE: usize = Self::WIDTH * Self::HEIGHT;

    fn on_matrix(coord: Coordinate) -> bool {
        Self::valid_coord(coord) && coord.y < Self::HEIGHT
    }

    fn valid_coord(coord: Coordinate) -> bool {
        coord.x < Self::WIDTH
    }

    fn in_bounds(Coordinate { x, y }: Coordinate) -> bool {
        x < Self::WIDTH && y < Self::HEIGHT
    }

    fn indexing(Coordinate { x, y }: Coordinate) -> usize {
        y * Self::WIDTH + x
    }

    fn blank() -> Self {
        Self([false; Self::SIZE])
    }
    fn is_clipping(&self, piece: &Piece) -> bool {
        let cells = match piece.cells() {
            Some(value) => value,
            None => return false,
        };
        cells
            .into_iter()
            .all(|coord| !Matrix::on_matrix(coord) || self[coord] == false)
    }

    fn is_placeable(&self, piece: &Piece) -> bool {
        let cells = match piece.cells() {
            Some(value) => value,
            None => return false,
        };
        cells
            .into_iter()
            .all(|coord| Matrix::on_matrix(coord) && self[coord] == false)
    }
}

impl Index<Coordinate> for Matrix {
    type Output = bool;
    fn index(&self, coord: Coordinate) -> &Self::Output {
        assert!(Self::on_matrix(coord));
        &self.0[Self::indexing(coord)]
    }
}
impl IndexMut<Coordinate> for Matrix {
    fn index_mut(&mut self, coord: Coordinate) -> &mut Self::Output {
        assert!(Self::on_matrix(coord));
        &mut self.0[Self::indexing(coord)]
    }
}
