use std::{iter::FilterMap, ops::{Add, Div, Index, IndexMut, Mul}};

// TODO: think about storing a flat array of 81 elements because then we can deref to a iter thanks to impl<T> [T] pub fn iter(&self) -> Iter<'_, T>
#[derive(Debug, Clone, Copy)]
pub struct SquareArray<const S: usize> {
    pub data: [[u32; S]; S],
    steps: u32
}

impl<const S: usize> PartialEq for SquareArray<S> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl SquareArray<9> {
    pub fn load_game(data: &str, game_id: usize) -> Result<Self, String> {
        let data = data.lines().collect::<Vec<_>>();

        let game = data.get(game_id).ok_or(format!("Game with index {game_id} does not exist"))?.chars();
        // Create empty board
        let mut board: SquareArray<9> = Default::default();
        // Load all elements into board
        for (el, ch) in board.into_iter().zip(game) {
            board[el.0] = ch.to_digit(10).unwrap_or(0);
        }

        Ok(board)
    }

    pub fn set(&mut self, p: Point, n: u32) {
        self[p] = n;
        self.steps += 1;
    }

    pub fn get_steps(&self) -> u32 {
        self.steps
    }
}

impl<const S: usize> Default for SquareArray<S> {
    fn default() -> Self {
        Self { data: [[0; S]; S], steps: 0 }
    }
}

impl<const S: usize> Index<Point> for SquareArray<S> {
    type Output = u32;

    fn index(&self, index: Point) -> &Self::Output {
        &self.data[index.x][index.y]
    }
}

impl<const S: usize> IndexMut<Point> for SquareArray<S> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.data[index.x][index.y]
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