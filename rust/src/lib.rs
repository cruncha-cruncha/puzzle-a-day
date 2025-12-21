use common::{AllTime, Month};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen;
use wasm_bindgen::prelude::*;

use crate::common::{Coordinate, WeekDay};

mod active_board;
mod common;
mod custom_boards;
mod solver;
mod tile_helper;

#[allow(dead_code)]
pub const CUSTOM_BOARDS: [&dyn common::CustomBoard; 4] = [
    &crate::custom_boards::nova_scotia::NovaScotiaCalendarBoard {},
    &crate::custom_boards::gmdm::GMDoMBoard {},
    &crate::custom_boards::gmdmwd::GMDoMWDBoard {},
    &crate::custom_boards::gt::GTBoard {},
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsTile {
    pub coordinates: Vec<Coordinate>,
}

#[wasm_bindgen]
pub fn solve(month: u32, day: u32, week_day: u32, custom_board: &str) -> Vec<JsValue> {
    let month = Month::from_u32(month).unwrap();
    let week_day = WeekDay::from_u32(week_day).unwrap();
    let all_time = AllTime::new(month, day, week_day);
    let board = match CUSTOM_BOARDS.iter().find(|b| b.name() == custom_board) {
        Some(b) => b,
        None => return vec![],
    };

    match solver::run(&all_time, *board) {
        Ok(solution) => {
            let mut js_tiles: Vec<JsValue> = Vec::new();
            for tile in solution {
                let js_tile = JsTile { coordinates: tile };
                js_tiles.push(serde_wasm_bindgen::to_value(&js_tile).unwrap());
            }
            return js_tiles;
        }
        Err(_) => return vec![],
    }
}

#[wasm_bindgen]
pub fn get_board_options() -> JsValue {
    let mut boards = Vec::new();
    for custom_board in CUSTOM_BOARDS {
        boards.push(custom_board.name());
    }
    serde_wasm_bindgen::to_value(&boards).unwrap()
}
