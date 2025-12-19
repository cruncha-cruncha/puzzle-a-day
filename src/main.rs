use chrono::Local;
use rand::rng;
use rand::seq::SliceRandom;
use std::collections::HashSet;

fn main() -> Result<(), u32> {
    let now = get_current_time();
    let board = NovaScotiaCalendarBoard {};
    run(now, board)
}

const TILE_MARKERS: [char; 16] = [
    'I', 'O', 'B', 'S', 'X', '2', 'N', 'V', 'Y', '7', 'J', 'T', '9', 'W', '*', '%',
];
const NO_SOLUTION_FOUND: u32 = 3;

fn get_current_time() -> AllTime {
    let now = Local::now();
    return AllTime {
        month: Month::from_datetime(now).unwrap_or(Month::January),
        day_of_month: chrono::Datelike::day(&now),
    };
}

fn run(now: AllTime, board: impl CustomBoard) -> Result<(), u32> {
    println!("Using board: {}", board.name());
    println!("Today is: {:?}", now);
    println!("Solving for: {:?}", board.point_in_time(&now));

    let mut active_board = ActiveBoard::from_custom(&board, &now);

    let mut solution: Vec<Tile> = Vec::with_capacity(board.tiles().len());
    let found_solution = solve(&mut active_board, &mut solution);
    if !found_solution {
        return Err(NO_SOLUTION_FOUND);
    }

    println!("found:");
    print_solution(&solution);

    Ok(())
}

fn print_solution(tiles: &Vec<Tile>) {
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

    for (i, tile) in tiles.iter().enumerate() {
        let marker = TILE_MARKERS[i % TILE_MARKERS.len()];
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

fn solve(active_board: &mut ActiveBoard, solution: &mut Vec<Tile>) -> bool {
    let mut tile_set = match active_board.get_next_tile_set() {
        Some(index) => index,
        None => {
            return true; // no unplaced tiles left -> board is solved!
        }
    };

    let mut open_coor = active_board.get_next_open_coor(None);

    while let Some(coor) = open_coor {
        let offset = TileHelper::calc_offset(&tile_set.tiles[0], &coor);
        for tile in tile_set.tiles.iter_mut() {
            TileHelper::translate(tile, &offset);
        }

        for tile in tile_set.tiles.iter() {
            if active_board.place_tile(tile_set.key, tile) {
                if solve(active_board, solution) {
                    solution.push(tile.clone());
                    return true;
                } else {
                    active_board.remove_tile(tile_set.key, tile);
                }
            }
        }

        open_coor = active_board.get_next_open_coor(Some(&coor));
    }

    false
}

#[derive(Debug)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    pub fn from_datetime(value: chrono::DateTime<Local>) -> Option<Month> {
        let numeric = chrono::Datelike::month(&value);
        match numeric {
            1 => Some(Month::January),
            2 => Some(Month::February),
            3 => Some(Month::March),
            4 => Some(Month::April),
            5 => Some(Month::May),
            6 => Some(Month::June),
            7 => Some(Month::July),
            8 => Some(Month::August),
            9 => Some(Month::September),
            10 => Some(Month::October),
            11 => Some(Month::November),
            12 => Some(Month::December),
            _ => None,
        }
    }
}

pub type DayOfMonth = u32;

#[derive(Debug)]
pub struct AllTime {
    month: Month,
    day_of_month: DayOfMonth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate {
    x: i32,
    y: i32,
}

pub fn c(x: i32, y: i32) -> Coordinate {
    Coordinate { x, y }
}

pub type Tile = Vec<Coordinate>;

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

pub struct ActiveBoard {
    tile_sets: Vec<Vec<Tile>>, // each tile can have eight orientations; one tile set is contains all orientations for that tile
    tile_set_placed: Vec<bool>, // a tile from the set has been placed on the board
    open_coors: Vec<Vec<bool>>, // 16x16 grid of open (true) and closed (false) coordinates
}

pub struct ActiveTileSet {
    key: usize,
    tiles: Vec<Tile>,
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
        board_tiles.shuffle(&mut rng());

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
}

pub trait CustomBoard {
    fn name(&self) -> String;
    fn tiles(&self) -> Vec<Tile>;
    fn coors(&self) -> Vec<Coordinate>;
    fn point_in_time(&self, pit: &AllTime) -> Option<HashSet<Coordinate>>;
}

pub struct NovaScotiaCalendarBoard {}
impl CustomBoard for NovaScotiaCalendarBoard {
    fn name(&self) -> String {
        "Nova Scotia Calendar Board".to_string()
    }

    fn tiles(&self) -> Vec<Tile> {
        vec![
            vec![c(0, 0), c(1, 0), c(2, 0), c(0, 1), c(2, 1)], // horseshoe
            vec![c(0, 0), c(1, 0), c(2, 0), c(0, 1), c(0, 2)], // angle bracket
            vec![c(0, 0), c(1, 0), c(2, 0), c(0, 1), c(1, 1), c(2, 1)], // big rectangle
            vec![c(0, 0), c(1, 0), c(2, 0), c(0, 1), c(1, 1)], // chipped rectangle,
            vec![c(0, 0), c(1, 0), c(2, 0), c(3, 0), c(0, 1)], // lowercase L
            vec![c(0, 0), c(1, 0), c(2, 0), c(2, 1), c(3, 1)], // lighting bolt
            vec![c(0, 0), c(1, 0), c(1, 1), c(1, 2), c(2, 2)], // zig zag
            vec![c(0, 0), c(1, 0), c(2, 0), c(3, 0), c(1, 1)], // weird club
        ]
    }

    #[rustfmt::skip]
    fn coors(&self) -> Vec<Coordinate> {
        vec![
            c(0,0), c(1,0), c(2,0), c(3,0), c(4,0), c(5,0),
            c(0,1), c(1,1), c(2,1), c(3,1), c(4,1), c(5,1),
            c(0,2), c(1,2), c(2,2), c(3,2), c(4,2), c(5,2), c(6,2),
            c(0,3), c(1,3), c(2,3), c(3,3), c(4,3), c(5,3), c(6,3),
            c(0,4), c(1,4), c(2,4), c(3,4), c(4,4), c(5,4), c(6,4),
            c(0,5), c(1,5), c(2,5), c(3,5), c(4,5), c(5,5), c(6,5),
            c(0,6), c(1,6), c(2,6)
        ]
    }

    fn point_in_time(&self, pit: &AllTime) -> Option<HashSet<Coordinate>> {
        let mut out: HashSet<Coordinate> = HashSet::new();

        match pit.month {
            Month::January => out.insert(c(0, 0)),
            Month::February => out.insert(c(1, 0)),
            Month::March => out.insert(c(2, 0)),
            Month::April => out.insert(c(3, 0)),
            Month::May => out.insert(c(4, 0)),
            Month::June => out.insert(c(5, 0)),
            Month::July => out.insert(c(0, 1)),
            Month::August => out.insert(c(1, 1)),
            Month::September => out.insert(c(2, 1)),
            Month::October => out.insert(c(3, 1)),
            Month::November => out.insert(c(4, 1)),
            Month::December => out.insert(c(5, 1)),
        };

        match pit.day_of_month {
            1 => out.insert(c(0, 2)),
            2 => out.insert(c(1, 2)),
            3 => out.insert(c(2, 2)),
            4 => out.insert(c(3, 2)),
            5 => out.insert(c(4, 2)),
            6 => out.insert(c(5, 2)),
            7 => out.insert(c(6, 2)),
            8 => out.insert(c(0, 3)),
            9 => out.insert(c(1, 3)),
            10 => out.insert(c(2, 3)),
            11 => out.insert(c(3, 3)),
            12 => out.insert(c(4, 3)),
            13 => out.insert(c(5, 3)),
            14 => out.insert(c(6, 3)),
            15 => out.insert(c(0, 4)),
            16 => out.insert(c(1, 4)),
            17 => out.insert(c(2, 4)),
            18 => out.insert(c(3, 4)),
            19 => out.insert(c(4, 4)),
            20 => out.insert(c(5, 4)),
            21 => out.insert(c(6, 4)),
            22 => out.insert(c(0, 5)),
            23 => out.insert(c(1, 5)),
            24 => out.insert(c(2, 5)),
            25 => out.insert(c(3, 5)),
            26 => out.insert(c(4, 5)),
            27 => out.insert(c(5, 5)),
            28 => out.insert(c(6, 5)),
            29 => out.insert(c(0, 6)),
            30 => out.insert(c(1, 6)),
            _ => return None,
        };

        Some(out)
    }
}
