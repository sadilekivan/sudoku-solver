use std::{iter::FilterMap, ops::{Add, Div, Index, IndexMut, Mul}};

// TODO: think about storing a flat array of 81 elements because then we can deref to a iter thanks to impl<T> [T] pub fn iter(&self) -> Iter<'_, T>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SquareArray<const S: usize> (pub [[u32; S]; S]);

impl SquareArray<9> {
    pub fn load_game(data: &str) -> Self {
        let vec_board = data.lines().map(
            |row| row.chars().map(
                |ch| ch.to_digit(10).unwrap_or(0)
            ).collect::<Vec<_>>()
        ).collect::<Vec<_>>();
    
        let mut board: SquareArray<9> = Default::default();
    
        for (i, row) in vec_board.into_iter().enumerate() {
            board.0[i].copy_from_slice(&row[..9]);  // Copy first 9 elements from each row
        }
    
        board
    }

    pub fn load_puzzles(game_id: usize) -> Self {
        let game = include_str!("games/puzzles1_unbiased").lines().collect::<Vec<_>>()[game_id].chars();
        let mut board: SquareArray<9> = Default::default();
        for (el, ch) in board.into_iter().zip(game) {
            board[el.0] = ch.to_digit(10).unwrap_or(0);
        }
        board
    }
}

impl<const S: usize> Default for SquareArray<S> {
    fn default() -> Self {
        Self([[0; S]; S])
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

impl<const S: usize> SquareArrayIter<S> {
    pub fn is_filled(field: (Point, u32)) -> Option<(Point, u32)> {
        if field.1 == 0 {
            Some(field)
        } else {
            None
        }
    }

    // Filters out fields which already have a number, we only preserve fields with 0 value
    pub fn skip_filled(self) -> FilterMap<Self, fn((Point, u32)) -> Option<(Point, u32)>> {
        self.filter_map(Self::is_filled)
    }
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