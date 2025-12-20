use crate::{
    active_board::ActiveBoard,
    common::{AllTime, CustomBoard, Tile},
    tile_helper::TileHelper,
};

const NO_SOLUTION_FOUND: u32 = 3;

pub fn run(now: &AllTime, board: &dyn CustomBoard) -> Result<Vec<Tile>, u32> {
    let mut active_board = ActiveBoard::from_custom(board, now);

    let mut solution: Vec<Tile> = Vec::new();
    let found_solution = solve(&mut active_board, &mut solution);
    if !found_solution {
        return Err(NO_SOLUTION_FOUND);
    }

    Ok(solution)
}

// try to place a tile, then recurse to place the next tile
pub fn solve(active_board: &mut ActiveBoard, solution: &mut Vec<Tile>) -> bool {
    let mut tile_set = match active_board.get_next_tile_set() {
        Some(index) => index,
        None => {
            return true; // no unplaced tiles left -> board is solved!
        }
    };

    let islands = active_board.find_islands();
    let smallest_area = active_board.smallest_unplaced_tile_size();
    for island in islands {
        if island.len() < smallest_area {
            return false; // found an island too small to fit any remaining tiles -> no solution possible
        }
    }

    // get the first open coordinate on the board
    let mut open_coor = active_board.get_next_open_coor(None);

    while let Some(coor) = open_coor {
        // move tile set to current open_coor
        let offset = TileHelper::calc_offset(&tile_set.tiles[0], &coor);
        for tile in tile_set.tiles.iter_mut() {
            TileHelper::translate(tile, &offset);
        }

        // try placing each orientation of the tile set at the open coordinate
        for tile in tile_set.tiles.iter() {
            if active_board.place_tile(tile_set.key(), tile) {
                // successfully placed tile, continue solving recursively (try placing the next tile)
                if solve(active_board, solution) {
                    // found a solution!
                    solution.push(tile.clone());
                    return true;
                } else {
                    // backtrack - remove the tile and try the next orientation
                    active_board.remove_tile(tile_set.key(), tile);
                }
            }
        }

        // get the next open coordinate
        open_coor = active_board.get_next_open_coor(Some(&coor));
    }

    false
}
