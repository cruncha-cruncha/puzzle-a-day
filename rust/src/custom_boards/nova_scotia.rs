use std::collections::HashSet;

use crate::common::{AllTime, Coordinate, Month, Tile, c};

pub struct NovaScotiaCalendarBoard {}
impl crate::common::CustomBoard for NovaScotiaCalendarBoard {
    fn name(&self) -> String {
        "Nova Scotia Calendar Board".to_string()
    }

    fn tiles(&self) -> Vec<Tile> {
        vec![
            vec![c(0, 0), c(1, 0), c(2, 0), c(0, 1), c(1, 1), c(2, 1)], // big rectangle
            vec![c(0, 0), c(1, 0), c(2, 0), c(0, 1), c(2, 1)],          // horseshoe
            vec![c(0, 0), c(1, 0), c(2, 0), c(0, 1), c(0, 2)],          // angle bracket
            vec![c(0, 0), c(1, 0), c(2, 0), c(0, 1), c(1, 1)],          // chipped rectangle
            vec![c(0, 0), c(1, 0), c(2, 0), c(3, 0), c(0, 1)],          // lowercase L
            vec![c(0, 0), c(1, 0), c(2, 0), c(2, 1), c(3, 1)],          // lighting bolt
            vec![c(0, 0), c(1, 0), c(1, 1), c(1, 2), c(2, 2)],          // zig zag
            vec![c(0, 0), c(1, 0), c(2, 0), c(3, 0), c(1, 1)],          // weird club
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

        match pit.month() {
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

        match pit.day_of_month() {
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
            31 => out.insert(c(2, 6)),
            _ => return None,
        };

        Some(out)
    }
}
