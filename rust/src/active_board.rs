use crate::{
    common::{AllTime, Coordinate, CustomBoard, Tile},
    tile_helper::TileHelper,
};
use std::collections::HashSet;

pub struct ActiveBoard {
    tile_sets: Vec<Vec<Tile>>,  // each tile can have eight orientations; one tile set contains all orientations for that tile
    tile_set_placed: Vec<bool>, // a tile from the set has been placed on the board
    open_coors: Vec<Vec<bool>>, // 16x16 grid of open (true) and closed (false) coordinates
}

pub struct ActiveTileSet {
    key: usize,           // an index into ActiveBoard tile_sets / tile_set_placed
    pub tiles: Vec<Tile>, // all orientations of the tile
}

impl ActiveTileSet {
    pub fn key(&self) -> usize {
        self.key
    }
}

impl ActiveBoard {
    #[allow(dead_code)]
    pub fn print_to_console(&self) {
        for y in 0..16 {
            for x in 0..16 {
                if self.open_coors[x][y] {
                    print!(". ");
                } else {
                    print!("# ");
                }
            }
            println!();
        }
    }

    pub fn from_custom(board: &dyn CustomBoard, pit: &AllTime) -> ActiveBoard {
        let avoid_points = match board.point_in_time(pit) {
            Some(coors) => coors,
            None => {
                panic!("Invalid point in time for board");
            }
        };

        let mut open_coors = vec![vec![false; 16]; 16];
        for coor in board.coors() {
            open_coors[coor.x as usize][coor.y as usize] = true;
        }
        for coor in &avoid_points {
            open_coors[coor.x as usize][coor.y as usize] = false;
        }

        // shuffle tile order on start, so we get a potentially different solution each time
        let mut board_tiles = board.tiles().clone();
        rand::seq::SliceRandom::shuffle(board_tiles.as_mut_slice(), &mut rand::rng());

        let mut tiles: Vec<Vec<Tile>> = Vec::with_capacity(board_tiles.len());
        for tile in board_tiles {
            let mut orientations: Vec<Tile> = Vec::with_capacity(8);
            let mut current_tile = tile.clone();
            for _ in 0..4 {
                orientations.push(current_tile.clone());
                TileHelper::rotate(&mut current_tile);
            }
            TileHelper::flip(&mut current_tile);
            for _ in 0..4 {
                orientations.push(current_tile.clone());
                TileHelper::rotate(&mut current_tile);
            }
            tiles.push(orientations);
        }

        ActiveBoard {
            tile_set_placed: vec![false; tiles.len()],
            tile_sets: tiles,
            open_coors,
        }
    }

    pub fn get_next_tile_set(&self) -> Option<ActiveTileSet> {
        for (i, placed) in self.tile_set_placed.iter().enumerate() {
            if !placed {
                return Some(ActiveTileSet {
                    key: i,
                    tiles: self.tile_sets[i].clone(),
                });
            }
        }
        None
    }

    fn get_first_open_coor(&self) -> Option<Coordinate> {
        for x in 0..self.open_coors.len() {
            for y in 0..self.open_coors[0].len() {
                if self.open_coors[x][y] {
                    return Some(Coordinate {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }
        None
    }

    pub fn get_next_open_coor(&self, current_coor: Option<&Coordinate>) -> Option<Coordinate> {
        let current_coor = match current_coor {
            Some(coor) => coor,
            None => {
                return self.get_first_open_coor();
            }
        };

        let mut start_y = (current_coor.y as usize) + 1;
        for x in (current_coor.x as usize)..self.open_coors.len() {
            for y in start_y..self.open_coors[0].len() {
                if self.open_coors[x][y] {
                    return Some(Coordinate {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
            start_y = 0;
        }
        None
    }

    pub fn place_tile(&mut self, set_key: usize, tile: &Tile) -> bool {
        debug_assert!(!self.tile_set_placed[set_key], "Tile already placed");

        if !TileHelper::is_within_board(tile) {
            return false;
        }

        for coor in tile.iter() {
            if !self.open_coors[coor.x as usize][coor.y as usize] {
                return false;
            }
        }

        self.tile_set_placed[set_key] = true;
        for coor in tile.iter() {
            self.open_coors[coor.x as usize][coor.y as usize] = false;
        }
        true
    }

    pub fn remove_tile(&mut self, set_key: usize, tile: &Tile) {
        debug_assert!(self.tile_set_placed[set_key], "Tile not currently placed");
        debug_assert!(
            TileHelper::is_within_board(tile),
            "Tile not within board bounds"
        );

        self.tile_set_placed[set_key] = false;
        for coor in tile.iter() {
            self.open_coors[coor.x as usize][coor.y as usize] = true;
        }
    }

    pub fn smallest_unplaced_tile_size(&self) -> usize {
        self.tile_sets
            .iter()
            .enumerate()
            .filter_map(|(i, tile_set)| {
                if !self.tile_set_placed[i] {
                    Some(tile_set[0].len())
                } else {
                    None
                }
            })
            .min()
            .unwrap_or(usize::MAX)
    }

    pub fn find_islands(&self) -> Vec<Tile> {
        let mut visited: HashSet<Coordinate> = HashSet::new();
        let mut islands: Vec<Tile> = Vec::new();

        for x in 0..self.open_coors.len() {
            for y in 0..self.open_coors[0].len() {
                let coor = Coordinate {
                    x: x as i32,
                    y: y as i32,
                };
                if self.open_coors[x][y] && !visited.contains(&coor) {
                    // found start of new island
                    let mut island: Tile = Vec::new();
                    let mut stack: Vec<Coordinate> = vec![coor];

                    while let Some(current) = stack.pop() {
                        if visited.contains(&current) {
                            continue;
                        }
                        visited.insert(current);
                        island.push(current);

                        // check neighbors
                        let neighbors = vec![
                            Coordinate {
                                x: current.x + 1,
                                y: current.y,
                            },
                            Coordinate {
                                x: current.x - 1,
                                y: current.y,
                            },
                            Coordinate {
                                x: current.x,
                                y: current.y + 1,
                            },
                            Coordinate {
                                x: current.x,
                                y: current.y - 1,
                            },
                        ];
                        for neighbor in neighbors {
                            if neighbor.x >= 0
                                && neighbor.x < self.open_coors.len() as i32
                                && neighbor.y >= 0
                                && neighbor.y < self.open_coors[0].len() as i32
                                && self.open_coors[neighbor.x as usize][neighbor.y as usize]
                                && !visited.contains(&neighbor)
                            {
                                stack.push(neighbor);
                            }
                        }
                    }

                    islands.push(island);
                }
            }
        }

        islands
    }
}
