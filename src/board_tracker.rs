use glam::USizeVec2;
use glam::Vec3;
use std::cell::Cell;

use crate::sudoku_board::{SudokuBoard, SudokuBoardOps};

#[derive(Debug, Clone, Copy)]
pub struct SudokuSolverTelemetry {
    pub write_count: f32,
    pub clear_count: f32,
    pub read_count: f32,
}

impl SudokuSolverTelemetry {
    fn new() -> Self {
        Vec3::ZERO.into()
    }

    fn tick_write(&mut self) {
        self.write_count += 1.0
    }

    fn tick_read(&mut self) {
        self.read_count += 1.0
    }

    fn tick_clear(&mut self) {
        self.clear_count += 1.0
    }
}

impl Default for SudokuSolverTelemetry {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec3> for SudokuSolverTelemetry {
    fn from(value: Vec3) -> Self {
        SudokuSolverTelemetry {
            write_count: value.x,
            clear_count: value.y,
            read_count: value.z,
        }
    }
}

impl From<SudokuSolverTelemetry> for Vec3 {
    fn from(val: SudokuSolverTelemetry) -> Self {
        Vec3 {
            x: val.write_count,
            y: val.clear_count,
            z: val.read_count,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TrackedSudokuBoard {
    pub inner: SudokuBoard,
    telemetry: Cell<SudokuSolverTelemetry>,
}

impl TrackedSudokuBoard {
    pub fn get_telemetry(&self) -> SudokuSolverTelemetry {
        self.telemetry.take()
    }
}

impl PartialEq for TrackedSudokuBoard {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl From<SudokuBoard> for TrackedSudokuBoard {
    fn from(value: SudokuBoard) -> Self {
        TrackedSudokuBoard {
            inner: value,
            telemetry: Cell::new(SudokuSolverTelemetry::new()),
        }
    }
}

impl SudokuBoardOps for TrackedSudokuBoard {
    fn write(&mut self, pos: USizeVec2, value: u8) {
        let mut telemetry = self.telemetry.get();
        telemetry.tick_write();
        self.telemetry.set(telemetry);
        self.inner.write(pos, value);
    }

    fn read(&self, pos: USizeVec2) -> u8 {
        let mut telemetry = self.telemetry.get();
        telemetry.tick_read();
        self.telemetry.set(telemetry);
        self.inner.read(pos)
    }

    fn clear(&mut self, pos: USizeVec2) {
        let mut telemetry = self.telemetry.get();
        telemetry.tick_clear();
        self.telemetry.set(telemetry);
        self.inner.clear(pos);
    }

    fn get(&self) -> &[u8; crate::sudoku_board::BOARD_CELLS] {
        &self.inner.0
    }

    fn get_mut(&mut self) -> &mut [u8; crate::sudoku_board::BOARD_CELLS] {
        &mut self.inner.0
    }
}
