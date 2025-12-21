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

#[derive(Debug)]
pub enum WeekDay {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
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

impl WeekDay {
    pub fn from_datetime(value: chrono::DateTime<chrono::Local>) -> Option<WeekDay> {
        let numeric = chrono::Datelike::weekday(&value).num_days_from_sunday();
        Self::from_u32(numeric)
    }

    pub fn from_u32(value: u32) -> Option<WeekDay> {
        match value {
            0 => Some(WeekDay::Sunday),
            1 => Some(WeekDay::Monday),
            2 => Some(WeekDay::Tuesday),
            3 => Some(WeekDay::Wednesday),
            4 => Some(WeekDay::Thursday),
            5 => Some(WeekDay::Friday),
            6 => Some(WeekDay::Saturday),
            _ => None,
        }
    }
}

pub type DayOfMonth = u32;

#[derive(Debug)]
pub struct AllTime {
    month: Month,
    day_of_month: DayOfMonth,
    #[allow(dead_code)]
    week_day: WeekDay,
}

impl AllTime {
    #[allow(dead_code)]
    pub fn new(month: Month, day_of_month: DayOfMonth, week_day: WeekDay) -> AllTime {
        AllTime {
            month,
            day_of_month,
            week_day,
        }
    }

    pub fn from_datetime(value: chrono::DateTime<chrono::Local>) -> Option<AllTime> {
        let month = Month::from_datetime(value)?;
        let day_of_month = chrono::Datelike::day(&value);
        let week_day = WeekDay::from_datetime(value)?;
        Some(AllTime {
            month,
            day_of_month,
            week_day,
        })
    }

    pub fn month(&self) -> &Month {
        &self.month
    }

    pub fn day_of_month(&self) -> DayOfMonth {
        self.day_of_month
    }

    #[allow(dead_code)]
    pub fn week_day(&self) -> &WeekDay {
        &self.week_day
    }

    #[allow(dead_code)]
    pub fn must_get_current_time() -> AllTime {
        let now = chrono::Local::now();
        return AllTime::from_datetime(now).expect("Failed to get current time");
    }
}
