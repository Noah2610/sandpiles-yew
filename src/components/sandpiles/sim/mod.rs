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

pub fn step(grid: &mut CellGrid, alternate: bool) {
    let mut value_additions: HashMap<Pos, i32> = HashMap::new();

    for (pos, cell) in grid.0.iter() {
        let neighboring_positions = neighboring_positions_for(pos, alternate);
        let split_value = neighboring_positions.len() as i32;

        if cell.value >= split_value {
            let addition = value_additions
                .entry(pos.clone())
                .or_insert_with(Default::default);
            *addition -= split_value;
            for neighboring_pos in neighboring_positions {
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

fn neighboring_positions_for(target: &Pos, alternate: bool) -> Vec<Pos> {
    const CORNERS: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
    const ADJACENT_4: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    const ADJACENT_8: [(i32, i32); 8] = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];
    const ALTERNATING_2: [[(i32, i32); 2]; 2] =
        [[(-1, -1), (1, 1)], [(-1, 1), (1, -1)]];

    let alternate_idx = if alternate { 0 } else { 1 };

    // CORNERS
    ALTERNATING_2[alternate_idx]
        .into_iter()
        .map(|(x, y)| Pos {
            x: target.x + x,
            y: target.y + y,
        })
        .collect()
}
