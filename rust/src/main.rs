use crate::{
    common::{AllTime, CustomBoard, Tile},
};

mod active_board;
mod common;
mod custom_boards;
mod tile_helper;
mod solver;

fn main() -> Result<(), u32> {
    let now = AllTime::must_get_current_time();
    let board = custom_boards::nova_scotia::NovaScotiaCalendarBoard {};
    let hide_tiles = 5;

    println!("Today is: {:?}", now);
    println!("Solving for: {:?}", board.point_in_time(&now));

    match solver::run(&now, &board) {
        Ok(solution) => {
            println!("found: ");
            print_solution(&solution, hide_tiles);
            Ok(())
        }
        Err(e) => Err(e),
    }
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
