use std::ops::{Add, Div, Index, IndexMut, Mul};
use crate::{Board, SudokuBoard};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SquareArray<const S: usize>(
    pub [[u32; S]; S]
);

impl Board {
    pub fn load_game(data: &str) -> Result<Vec<Self>, String> {
        let data = data.lines().collect::<Vec<_>>();

        let game_v: Vec<Self> = data.into_iter().map(
            |game_string|
            {
                let mut board: Self = Default::default();
                // Load all elements into board
                for (el, ch) in board.into_iter().zip(game_string.chars()) {
                    board[el.0] = ch.to_digit(10).unwrap_or(0);
                }
                board
            }
        ).collect();
        
        Ok(game_v)
    }
}

impl SudokuBoard for Board {
    fn write(&mut self, p: Point, n: u32) {
        self[p] = n;
    }

    fn clear(&mut self, p: Point) {
        self[p] = 0;
    }

    fn read(&mut self, p: Point) -> u32 {
        self[p]
    }
}

impl<const S: usize> Default for SquareArray<S> {
    fn default() -> Self {
        Self ([[0; S]; S])
    }
}

impl<const S: usize> Index<Point> for SquareArray<S> {
    type Output = u32;

    fn index(&self, index: Point) -> &Self::Output {
        &self.0[index.x][index.y]
    }
}

impl<const S: usize> IndexMut<Point> for SquareArray<S> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.0[index.x][index.y]
    }
}

/// 2D index point
#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    pub x: usize, // x
    pub y: usize // y
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn with_x(self, x: usize) -> Self {
        Self {x, y: self.y}
    }

    pub fn with_y(self, y: usize) -> Self {
        Self {x: self.x, y}
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Div<usize> for Point {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}

impl Mul<usize> for Point {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs
        }
    } 
}

// Iterates over the fields left to right, top to bottom
pub struct SquareArrayIter<const S: usize> {
    board: SquareArray<S>,
    p: Point
}

impl<const S: usize> IntoIterator for SquareArray<S> {
    type Item = (Point, u32);

    type IntoIter = SquareArrayIter<S>;

    fn into_iter(self) -> Self::IntoIter {
        SquareArrayIter { board: self, p: Default::default() }
    }
}

impl<const S: usize> Iterator for SquareArrayIter<S> {
    type Item = (Point, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.p.x == S {
            return None;
        }

        let p = self.p;
        let n = self.board[self.p];

        if self.p.y == S - 1 {
            self.p.x += 1;
            self.p.y = 0;
        } else {
            self.p.y += 1;
        }
        Some((p, n)) 
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tracked<T: SudokuBoard> {
    inner: T,
    write_count: u32,
    clear_count: u32,
    read_count: u32
}

impl<T: SudokuBoard> Tracked<T> {
    pub fn get_write_count(&self) -> u32 {
        self.write_count
    }

    pub fn get_clear_count(&self) -> u32 {
        self.clear_count
    }

    pub fn get_read_count(&self) -> u32 {
        self.read_count
    }
}

impl<T: SudokuBoard> PartialEq for Tracked<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T: SudokuBoard> SudokuBoard for Tracked<T> {
    fn write(&mut self, p: Point, n: u32) {
        self.write_count += 1;
        self.inner.write(p, n);
    }

    fn clear(&mut self, p: Point) {
        self.clear_count += 1;
        self.inner.clear(p);
    }

    fn read(&mut self, p: Point) -> u32 {
        self.read_count += 1;
        self.inner.read(p)
    }
}

impl<T: SudokuBoard> IntoIterator for Tracked<T> {
    type Item = (Point, u32);

    type IntoIter = SquareArrayIter<9>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}


impl From<Board> for Tracked<Board> {
    fn from(value: Board) -> Self {
        Tracked { inner: value, write_count: 0, clear_count: 0, read_count: 0 }
    }
}