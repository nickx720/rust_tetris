use cgmath::EuclideanSpace;
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

        let color = cursor.kind.color();
        for coord in cursor.cells().unwrap() {
            self.matrix[coord] = Some(color);
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

    fn tick_down(&mut self) {
        // try to move it down, if it can't , it will return error
        self.cursor = Some(self.ticked_down_cursor().unwrap());
    }

    fn cursor_has_hit_buttom(&self) -> bool {
        self.cursor.is_some() && self.ticked_down_cursor().is_none()
    }

    fn ticked_down_cursor(&self) -> Option<Piece> {
        let cursor = match self.cursor {
            Some(value) => value,
            None => return None,
        };
        let new = cursor.moved_by(Offset::new(0, -1));
        if !self.matrix.is_clipping(&new) {
            Some(new)
        } else {
            None
        }
    }

    fn hard_drop(&mut self) {
        // move cursor all the way down
        while let Some(new) = self.ticked_down_cursor() {
            self.cursor = Some(new);
        }
        // place cursor
        self.place_cursor();
    }

    // _ in iter() means new lifetime
    // _ as return means deduced life time
    fn iter(&self) -> CellIter<'_> {
        CellIter {
            position: Coordinate::origin(),
            cells: self.matrix.0.iter(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    Yellow,
    Cyan,
    Purple,
    Orange,
    Blue,
    Green,
    Red,
}
pub struct Matrix([Option<Color>; Self::SIZE]);

impl Matrix {
    pub const WIDTH: usize = 10;
    pub const HEIGHT: usize = 20;
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
        Self([None; Self::SIZE])
    }
    fn is_clipping(&self, piece: &Piece) -> bool {
        let cells = match piece.cells() {
            Some(value) => value,
            None => return true,
        };
        cells
            .into_iter()
            .any(|coord| !Matrix::on_matrix(coord) || self[coord].is_some())
    }

    fn is_placeable(&self, piece: &Piece) -> bool {
        let cells = match piece.cells() {
            Some(value) => value,
            None => return false,
        };
        cells
            .into_iter()
            .all(|coord| Matrix::on_matrix(coord) && self[coord].is_none())
    }
}

impl Index<Coordinate> for Matrix {
    type Output = Option<Color>;
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

pub struct CellIter<'matrix> {
    position: Coordinate,
    cells: ::std::slice::Iter<'matrix, Option<Color>>,
}

impl<'matrix> Iterator for CellIter<'matrix> {
    type Item = (Coordinate, &'matrix Option<Color>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cell) = self.cells.next() {
            let coord = self.position;

            // increment position
            self.position.x += 1;
            self.position.x %= Matrix::WIDTH;
            return Some((coord, cell));
        }
        None
    }
}
