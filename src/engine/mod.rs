use cgmath::EuclideanSpace;
use geometry::GridIncrement;
use std::{
    ops::{Index, IndexMut},
    slice::ArrayChunks,
    time::Duration,
};

use rand::{
    prelude::{SliceRandom, ThreadRng},
    thread_rng,
};

use self::piece::{Kind as PieceKind, Piece, Rotation};

mod geometry;
pub mod piece;

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
    level: u8,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            matrix: Matrix::blank(),
            bag: Vec::new(),
            rng: thread_rng(),
            cursor: None,
            level: 1,
        }
    }

    pub fn with_matrix(matrix: Matrix) -> Self {
        Self {
            matrix,
            ..Self::new()
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

    pub fn move_cursor(&mut self, kind: MoveKind) -> Result<(), ()> // Ok(()) , Err(())
    {
        let cursor = match self.cursor.as_mut() {
            Some(cursor) => cursor,
            None => return Ok(()),
        };

        let new = cursor.moved_by(kind.offset());
        if self.matrix.is_clipping(&new) {
            return Err(());
        }
        self.cursor = Some(new);
        Ok(())
    }

    pub fn cursor_info(&self) -> Option<([Coordinate; Piece::CELL_COUNT], Color)> {
        let cursor = self.cursor?;
        Some((cursor.cells().unwrap(), cursor.kind.color()))
    }

    pub fn DEBUG_test_cursor_location(&mut self, kind: PieceKind, position: Offset) {
        let piece = Piece {
            kind,
            rotation: Rotation::N,
            position,
        };
        self.cursor = Some(piece)
    }

    fn tick_down(&mut self) {
        // try to move it down, if it can't , it will return error
        self.cursor = Some(self.ticked_down_cursor().unwrap());
    }

    pub fn cursor_has_hit_buttom(&self) -> bool {
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

    pub fn hard_drop(&mut self) {
        // move cursor all the way down
        while let Some(new) = self.ticked_down_cursor() {
            self.cursor = Some(new);
        }
        // place cursor
        self.place_cursor();
    }

    // _ in iter() means new lifetime
    // _ as return means deduced life time
    pub fn cells(&self) -> CellIter<'_> {
        CellIter {
            position: Coordinate::origin(),
            cells: self.matrix.0.iter(),
        }
    }
    pub fn drop_time(&self) -> Duration {
        let level_index = self.level - 1;
        let seconds_per_line = (0.8 - ((self.level - 1) as f32 * 0.007)).powi(level_index as _);
        Duration::from_secs_f32(seconds_per_line)
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

    pub fn blank() -> Self {
        Self([None; Self::SIZE])
    }
    fn is_clipping(&self, piece: &Piece) -> bool {
        let cells = match piece.cells() {
            Some(value) => value,
            None => return true,
        };
        cells.into_iter().any(|coord| {
            !Matrix::valid_coord(coord) || (Matrix::on_matrix(coord) || self[coord].is_some())
        })
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

    fn lines(&self) -> ArrayChunks<'_, Option<Color>, { Self::WIDTH }> {
        self.0.array_chunks()
    }

    fn full_lines(&self) -> Vec<usize> {
        todo!()
    }

    fn line_clear(&mut self, animation: impl FnMut(&[usize])) {
        let lines: Vec<usize> = todo!("identify full lines");
        animation(lines.as_slice());
        self.matrix.clear_lines(lines.as_slice());
    }
    fn clear_lines(&mut self, indices: &[usize]) {
        todo!("clear lines");
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
    type Item = (Coordinate, Option<Color>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(&cell) = self.cells.next() {
            // increment position
            let coord = self.position;
            self.position.grid_inc();
            return Some((coord, cell));
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cell_iter() {
        let mut matrix = Matrix::blank();
        matrix[Coordinate::new(2, 0)] = Some(Color::Blue);
        matrix[Coordinate::new(3, 1)] = Some(Color::Green);

        let mut iter = CellIter {
            position: Coordinate::origin(),
            cells: matrix.0.iter(),
        };

        let first_five = (&mut iter).take(5).collect::<Vec<_>>();
        assert_eq!(
            first_five,
            [
                (Coordinate::new(0, 0), None),
                (Coordinate::new(1, 0), None),
                (Coordinate::new(2, 0), Some(Color::Blue)),
                (Coordinate::new(3, 0), None),
                (Coordinate::new(4, 0), None),
            ]
        );

        let other_item = (&mut iter).skip(8).next();
        assert_eq!(
            other_item,
            Some((Coordinate::new(3, 1), Some(Color::Green)))
        );

        assert!(iter.all(|(_, contents)| contents.is_none()));
    }
}
