use glam::USizeVec2;

use crate::board::{cardinal_from_flat_index, cardinal_into_flat_index, BOARD_CELLS, BOARD_SIZE};

/// Operations to manipulate a sudoku board
pub trait BoardOps
where
    Self: std::fmt::Debug + PartialEq,
{
    fn write(&mut self, pos: USizeVec2, value: u8) {
        self.get_mut()[cardinal_into_flat_index(pos)] = value;
    }

    fn clear(&mut self, pos: USizeVec2) {
        self.get_mut()[cardinal_into_flat_index(pos)] = 0;
    }

    fn read(&self, pos: USizeVec2) -> u8 {
        self.get()[cardinal_into_flat_index(pos)]
    }

    fn is_clear(&self, pos: USizeVec2) -> bool {
        self.read(pos) == 0
    }

    fn is_valid_move(&self, pos: USizeVec2, n: u8) -> bool {
        // Check field row
        for x in 0..BOARD_SIZE {
            let pos = pos.with_x(x);
            if self.read(pos) == n {
                return false;
            }
        }

        // Check field column
        for y in 0..BOARD_SIZE {
            let pos = pos.with_y(y);
            if self.read(pos) == n {
                return false;
            }
        }

        let offset = pos / 3 * 3;
        // Check fields in subgrid
        for x in 0..3 {
            for y in 0..3 {
                let pos = USizeVec2::new(x, y) + offset;
                if self.read(pos) == n {
                    return false;
                }
            }
        }

        true
    }

    fn get(&self) -> &[u8; BOARD_CELLS];

    fn get_mut(&mut self) -> &mut [u8; BOARD_CELLS];

    fn iter(&self) -> impl Iterator<Item = (USizeVec2, &u8)> {
        self.get()
            .iter()
            .enumerate()
            .map(|(index, value)| (cardinal_from_flat_index(index), value))
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = (USizeVec2, &mut u8)> {
        self.get_mut()
            .iter_mut()
            .enumerate()
            .map(|(index, value)| (cardinal_from_flat_index(index), value))
    }
}
