use std::cmp::Ordering;

use chrono::{DateTime, Datelike, Month, Utc};
use serde::{Deserialize, Serialize};

pub struct InvalidYmdError;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PublishDate {
    year: i32,
    month: Option<Month>,
    day: Option<u32>,
}

impl PublishDate {
    pub const fn from_year(year: i32) -> Self {
        Self {
            year,
            month: None,
            day: None,
        }
    }

    pub const fn from_year_month(year: i32, month: Month) -> Self {
        Self {
            year,
            month: Some(month),
            day: None,
        }
    }

    pub fn from_year_month_day(year: i32, month: Month, day: u32) -> Result<Self, InvalidYmdError> {
        let maybe_days_in_month = month.num_days(year);
        if let Some(days_in_month) = maybe_days_in_month {
            let valid_day_range = 1..(u32::from(days_in_month));
            if valid_day_range.contains(&day) {
                Result::Ok(Self {
                    year,
                    month: Some(month),
                    day: Some(day),
                })
            } else {
                Result::Err(InvalidYmdError)
            }
        } else {
            Result::Err(InvalidYmdError)
        }
    }

    pub fn from_chrono_utc_datetime(datetime: DateTime<Utc>) -> Self {
        Self {
            year: datetime.year(),
            month: Month::try_from(u8::try_from(datetime.month()).unwrap()).ok(),
            day: Some(datetime.day()),
        }
    }

    pub fn year(&self) -> i32 {
        self.year
    }

    pub fn month(&self) -> Option<Month> {
        self.month
    }

    pub fn day(&self) -> Option<u32> {
        self.day
    }
}

impl Ord for PublishDate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.year
            .cmp(&other.year)
            .then(match (self.month, &other.month) {
                (None, None) => Ordering::Equal,
                (None, Some(_)) => Ordering::Less,
                (Some(_), None) => Ordering::Greater,
                (Some(month), Some(other_month)) => month.cmp(other_month),
            })
            .then(match (self.day, &other.day) {
                (None, None) => Ordering::Equal,
                (None, Some(_)) => Ordering::Less,
                (Some(_), None) => Ordering::Greater,
                (Some(day), Some(other_day)) => day.cmp(other_day),
            })
    }
}

impl PartialOrd for PublishDate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for PublishDate {}
