use crate::{
    active_board::ActiveBoard,
    common::{AllTime, CustomBoard, Tile},
    tile_helper::TileHelper,
};

mod active_board;
mod common;
mod custom_boards;
mod tile_helper;

const NO_SOLUTION_FOUND: u32 = 3;

fn main() -> Result<(), u32> {
    let now = AllTime::must_get_current_time();
    let board = custom_boards::nova_scotia::NovaScotiaCalendarBoard {};
    let hide_tiles = 0;

    println!("Using board: {}", board.name());
    println!("Today is: {:?}", now);
    println!("Solving for: {:?}", board.point_in_time(&now));

    match run(&now, &board) {
        Ok(solution) => {
            println!("found: ");
            print_solution(&solution, hide_tiles);
            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn run(now: &AllTime, board: &dyn CustomBoard) -> Result<Vec<Tile>, u32> {
    let mut active_board = ActiveBoard::from_custom(board, now);

    let mut solution: Vec<Tile> = Vec::new();
    let found_solution = solve(&mut active_board, &mut solution);
    if !found_solution {
        return Err(NO_SOLUTION_FOUND);
    }

    Ok(solution)
}

fn print_solution(tiles: &Vec<Tile>, hide_tiles: usize) {
    const TILE_MARKERS: [char; 16] = [
        'I', 'O', 'B', 'S', 'X', '2', 'N', 'V', 'Y', '7', 'J', 'T', '9', 'W', 'K', 'L',
    ];

    let max_x = tiles
        .iter()
        .flat_map(|tile| tile.iter().map(|coor| coor.x))
        .max()
        .unwrap_or(0);
    let max_y = tiles
        .iter()
        .flat_map(|tile| tile.iter().map(|coor| coor.y))
        .max()
        .unwrap_or(0);

    let mut output = vec![vec![' '; (max_x + 1) as usize]; (max_y + 1) as usize];

    let random_offset = rand::random::<u32>() as usize;
    for (i, tile) in tiles.iter().enumerate().skip(hide_tiles) {
        let marker = TILE_MARKERS[(i + random_offset) % TILE_MARKERS.len()];
        for coor in tile {
            output[coor.y as usize][coor.x as usize] = marker;
        }
    }

    for row in output {
        for cell in row {
            print!("{} ", cell);
        }
        println!();
    }
}

// try to place a tile, then recurse to place the next tile
fn solve(active_board: &mut ActiveBoard, solution: &mut Vec<Tile>) -> bool {
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
