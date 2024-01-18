use chrono::{NaiveDate, Datelike};

#[derive(Hash, Eq, PartialEq, Debug, PartialOrd, Ord)]
pub struct YearAndMonth {
    year: u16,
    month: u8,
}

impl From<NaiveDate> for YearAndMonth {
    fn from(date: NaiveDate) -> Self {
        YearAndMonth {
            year: date.year() as u16,
            month: date.month() as u8,
        }
    }
}