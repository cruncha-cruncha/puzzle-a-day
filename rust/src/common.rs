use std::collections::HashSet;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

pub fn c(x: i32, y: i32) -> Coordinate {
    Coordinate { x, y }
}

pub type Tile = Vec<Coordinate>;

pub trait CustomBoard {
    #[allow(dead_code)]
    fn name(&self) -> String;
    fn tiles(&self) -> Vec<Tile>;
    fn coors(&self) -> Vec<Coordinate>;
    fn point_in_time(&self, pit: &AllTime) -> Option<HashSet<Coordinate>>;
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
    pub fn from_datetime(value: chrono::DateTime<chrono::Local>) -> Option<Month> {
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

    #[allow(dead_code)]
    pub fn from_u32(value: u32) -> Option<Month> {
        match value {
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

impl AllTime {
    #[allow(dead_code)]
    pub fn new(month: Month, day_of_month: DayOfMonth) -> AllTime {
        AllTime {
            month,
            day_of_month,
        }
    }

    pub fn from_datetime(value: chrono::DateTime<chrono::Local>) -> Option<AllTime> {
        let month = Month::from_datetime(value)?;
        let day_of_month = chrono::Datelike::day(&value);
        Some(AllTime {
            month,
            day_of_month,
        })
    }

    pub fn month(&self) -> &Month {
        &self.month
    }

    pub fn day_of_month(&self) -> DayOfMonth {
        self.day_of_month
    }

    #[allow(dead_code)]
    pub fn must_get_current_time() -> AllTime {
        let now = chrono::Local::now();
        return AllTime::from_datetime(now).expect("Failed to get current time");
    }
}
