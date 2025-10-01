use std::cmp::Ordering;

use chrono::{DateTime, Datelike, Local, Month, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Get the abbreviated name of the month (e.g. "Jan."")
pub const fn ieee_abbreviated_month_name(month: &Month) -> &'static str {
    match month {
        Month::January => "Jan.",
        Month::February => "Feb.",
        Month::March => "Mar.",
        Month::April => "Apr.",
        Month::May => "May",
        Month::June => "Jun.",
        Month::July => "Jul.",
        Month::August => "Aug.",
        Month::September => "Sep.",
        Month::October => "Oct.",
        Month::November => "Nov.",
        Month::December => "Dec.",
    }
}

#[derive(Error, Debug)]
pub enum PublishDateParamError {
    #[error("The provided day does not exist in the provided year/month combination.")]
    InvalidDayForMonth,
    #[error("The provided year is outside of the accepted range.")]
    InvalidYear,
}

/// This data model doesn't accommodate ranges of dates, like
/// what would be seen in a conference.
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

    pub fn from_year_month_day(
        year: i32,
        month: Month,
        day: u32,
    ) -> Result<Self, PublishDateParamError> {
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
                Result::Err(PublishDateParamError::InvalidDayForMonth)
            }
        } else {
            Result::Err(PublishDateParamError::InvalidYear)
        }
    }

    pub fn from_chrono_utc_datetime(datetime: DateTime<Utc>) -> Self {
        Self {
            year: datetime.year(),
            month: Month::try_from(u8::try_from(datetime.month()).unwrap()).ok(),
            day: Some(datetime.day()),
        }
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub const fn month(&self) -> Option<Month> {
        self.month
    }

    pub const fn day(&self) -> Option<u32> {
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccessDate {
    accessed: DateTime<Utc>,
}

impl AccessDate {
    pub fn year(&self) -> i32 {
        self.accessed.year()
    }

    pub fn month(&self) -> Month {
        Month::try_from(self.accessed.month() as u8).unwrap()
    }

    pub fn day(&self) -> u32 {
        self.accessed.day()
    }
}

impl Default for AccessDate {
    fn default() -> Self {
        Self {
            accessed: Utc::now(),
        }
    }
}

impl From<DateTime<Utc>> for AccessDate {
    fn from(value: DateTime<Utc>) -> Self {
        Self { accessed: value }
    }
}

impl From<DateTime<Local>> for AccessDate {
    fn from(value: DateTime<Local>) -> Self {
        Self {
            accessed: value.to_utc(),
        }
    }
}

impl From<NaiveDate> for AccessDate {
    fn from(value: NaiveDate) -> Self {
        let accessed_utc = value.and_hms_opt(0, 0, 0).unwrap().and_utc();
        Self {
            accessed: accessed_utc,
        }
    }
}

impl Ord for AccessDate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.accessed.cmp(&other.accessed)
    }
}

impl PartialOrd for AccessDate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for AccessDate {}
