mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Represents the state of a cell
/// Default value of 0/1 used to calculate the state of neighbors with addition.
pub enum Cell {
    Dead = 0,
    Alive = 1
}

/// This impl block is for Rust side testing
impl Universe {
    /// Get the cells array from Universe
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().clone() {
            let idx = self.get_index(*row, *col);
            self.cells[idx] = Cell::Alive;
        }
    }
}

#[wasm_bindgen]
/// Board state of the wasm game
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>
}

#[wasm_bindgen]
impl Universe {
    /// Create a new Universe
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells
        }
    }

    /// Return width of Universe
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Set width of Universe
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height)
            .map(|_| Cell::Dead)
            .collect();
    }

    /// Return height of Universe
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Set height of Universe
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height)
        .map(|_| Cell::Dead)
        .collect();
    }

    // Return a pointer to the Cells from Universe
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    /// Calculates and returns the next state of the bored after
    /// one tick.
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbors
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbors
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbors dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbors
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    /// Draws the Universe
    pub fn render(&self) -> String {
        self.to_string()
    }

    /// Retrives the index number of a given cells location.
    /// Converted to usize for the purposes of passing through wasm.
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    /// Uses the row and column of a Cell's position to index each
    /// neighbor then generate and return a count of all the number
    /// live cells adjacent to it.
    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;

        // [self.height/width - 1, 0, 1] is a on the fly made tuple
        // turned iterator in order to avoid indexing
        // out of bounds when using the module operator to find
        // the neighbor count later in the expression.
        for delta_row in [self.height - 1, 0, 1].iter().clone() {
            for delta_col in [self.width - 1, 0, 1].iter().clone() {
                if *delta_row == 0 && *delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (col + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}