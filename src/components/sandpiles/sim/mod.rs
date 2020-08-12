pub mod cell;
pub mod cell_grid;

pub use cell::Cell;
pub use cell_grid::CellGrid;

#[derive(Hash, PartialEq, Eq)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}
