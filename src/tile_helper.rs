use crate::common::{Coordinate, Tile};

pub struct TileHelper {}

impl TileHelper {
    #[allow(dead_code)]
    pub fn print_to_console(tile: &Tile) {
        let mut board = vec![vec!['.'; 16]; 16];
        for coor in tile {
            board[coor.y as usize][coor.x as usize] = '#';
        }
        for row in board {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }
    }

    #[inline]
    // find difference between tile[0] and some origin
    pub fn calc_offset(tile: &Tile, origin: &Coordinate) -> Coordinate {
        Coordinate {
            x: origin.x - tile[0].x,
            y: origin.y - tile[0].y,
        }
    }

    #[inline]
    // move entire tile by offset
    pub fn translate(tile: &mut Tile, offset: &Coordinate) {
        for coor in tile.iter_mut() {
            coor.x += offset.x;
            coor.y += offset.y;
        }
    }

    // rotate 90 ccw around tile[0]
    pub fn rotate(tile: &mut Tile) {
        let origin = tile[0];
        for coor in tile.iter_mut().skip(1) {
            let relative_x = coor.x - origin.x;
            let relative_y = coor.y - origin.y;
            coor.x = origin.x - relative_y;
            coor.y = origin.y + relative_x;
        }
    }

    // flip horizontally around tile[0]
    pub fn flip(tile: &mut Tile) {
        let origin = tile[0];
        for coor in tile.iter_mut().skip(1) {
            let relative_x = coor.x - origin.x;
            coor.x = origin.x - relative_x;
        }
    }

    #[inline]
    // is the tile completely within the 16x16 board?
    // aka will we avoid an index out of bounds error?
    pub fn is_within_board(tile: &Tile) -> bool {
        for coor in tile {
            if coor.x < 0 || coor.x >= 16 || coor.y < 0 || coor.y >= 16 {
                return false;
            }
        }
        true
    }
}