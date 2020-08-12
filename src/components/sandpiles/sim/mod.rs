pub mod cell;
pub mod cell_grid;

pub use cell::Cell;
pub use cell_grid::CellGrid;

use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

pub fn step(grid: &mut CellGrid) {
    let mut value_additions: HashMap<Pos, i32> = HashMap::new();

    for (pos, cell) in grid.0.iter() {
        if cell.value >= 4 {
            let addition = value_additions
                .entry(pos.clone())
                .or_insert_with(Default::default);
            *addition -= 4;
            for neighboring_pos in neighboring_positions_for(pos) {
                let neighbor_addition = value_additions
                    .entry(neighboring_pos)
                    .or_insert_with(Default::default);
                *neighbor_addition += 1;
            }
        }
    }

    for (pos, addition) in value_additions.into_iter() {
        let cell = grid.0.entry(pos).or_insert_with(Default::default);
        cell.value += addition;
    }
}

fn neighboring_positions_for(target: &Pos) -> Vec<Pos> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .map(|(x, y)| Pos {
            x: target.x + x,
            y: target.y + y,
        })
        .collect()
}
