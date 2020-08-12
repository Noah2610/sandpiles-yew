use std::collections::HashMap;

use super::Cell;
use super::Pos;

#[derive(Default)]
pub struct CellGrid(pub HashMap<Pos, Cell>);
