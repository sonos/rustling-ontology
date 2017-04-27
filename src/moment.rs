use std::ops;

use chrono::{Datelike, Duration, TimeZone, Timelike};
use chrono::offset::local::Local;
use chrono::datetime::DateTime;

#[derive(Debug,PartialEq,Copy,Clone)]
pub enum Grain {
    Year,
    Quarter,
    Month,
    Week,
    Day,
    Hour,
    Minute,
    Second,
}

#[derive(Debug,PartialEq,Copy,Clone)]
pub struct Period {
    grain: Grain,
    quantity: i64,
}

impl Period {
    pub fn years(n: i64) -> Period {
        Period {
            grain: Grain::Year,
            quantity: n,
        }
    }
    pub fn quarters(n: i64) -> Period {
        Period {
            grain: Grain::Quarter,
            quantity: n,
        }
    }
    pub fn months(n: i64) -> Period {
        Period {
            grain: Grain::Month,
            quantity: n,
        }
    }
    pub fn weeks(n: i64) -> Period {
        Period {
            grain: Grain::Week,
            quantity: n,
        }
    }
    pub fn days(n: i64) -> Period {
        Period {
            grain: Grain::Day,
            quantity: n,
        }
    }
    pub fn hours(n: i64) -> Period {
        Period {
            grain: Grain::Hour,
            quantity: n,
        }
    }
    pub fn minutes(n: i64) -> Period {
        Period {
            grain: Grain::Minute,
            quantity: n,
        }
    }
    pub fn seconds(n: i64) -> Period {
        Period {
            grain: Grain::Second,
            quantity: n,
        }
    }
}

#[derive(Debug,PartialEq,Copy,Clone)]
struct Moment(DateTime<Local>);

fn last_day_in_month(y: i32, m: u32) -> i64 {
    assert!(m >= 1 && m <= 12);
    for d in 28..31 {
        if (Local.ymd_opt(y, m, d + 1)).single().is_none() {
            return d as i64;
        }
    }
    31
}

impl ops::Deref for Moment {
    type Target = DateTime<Local>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Moment {
    fn add_months(self, n: i32) -> Moment {
        let (year, month0) = if n >= 0 {
            let n = n as u32;
            let carry = ((self.month0() + n % 12) >= 12) as i32;
            (self.year() + (n / 12) as i32 + carry, (self.month0() + n) % 12)
        } else {
            let n = -n as u32;
            let borrow = (self.month0() < n % 12) as i32;
            (self.year() - (n / 12) as i32 - borrow as i32, (12 + self.month0() - (n % 12)) % 12)
        };
        let target_month_days = last_day_in_month(year, month0 + 1);
        let day = ::std::cmp::min(target_month_days, self.day() as i64) as u32;
        Moment(Local
                   .ymd(year, month0 + 1, day)
                   .and_hms(self.hour(), self.minute(), self.second()))
    }
}

impl ops::Add<Period> for Moment {
    type Output = Moment;
    fn add(self, p: Period) -> Moment {
        match p.grain {
            Grain::Year => self.add_months(12 * p.quantity as i32),
            Grain::Quarter => self.add_months(3 * p.quantity as i32),
            Grain::Month => self.add_months(p.quantity as i32),
            Grain::Week => Moment(self.0 + Duration::weeks(p.quantity)),
            Grain::Day => Moment(self.0 + Duration::days(p.quantity)),
            Grain::Hour => Moment(self.0 + Duration::hours(p.quantity)),
            Grain::Minute => Moment(self.0 + Duration::minutes(p.quantity)),
            Grain::Second => Moment(self.0 + Duration::seconds(p.quantity)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    use chrono::TimeZone;
    use chrono::offset::local::Local;
    use chrono::datetime::DateTime;

    #[test]
    fn add_period_to_moment() {
        let now = Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11));
        assert_eq!(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 21)),
                   now + Period::seconds(10));
        assert_eq!(Moment(Local.ymd(2017, 04, 25).and_hms(9, 20, 11)),
                   now + Period::minutes(10));
        assert_eq!(Moment(Local.ymd(2017, 04, 25).and_hms(19, 10, 11)),
                   now + Period::hours(10));
        assert_eq!(Moment(Local.ymd(2017, 05, 5).and_hms(9, 10, 11)),
                   now + Period::days(10));
        assert_eq!(Moment(Local.ymd(2017, 05, 2).and_hms(9, 10, 11)),
                   now + Period::weeks(1));
        assert_eq!(Moment(Local.ymd(2018, 02, 25).and_hms(9, 10, 11)),
                   now + Period::months(10));
        assert_eq!(Moment(Local.ymd(2017, 07, 25).and_hms(9, 10, 11)),
                   now + Period::quarters(1));
        assert_eq!(Moment(Local.ymd(2027, 04, 25).and_hms(9, 10, 11)),
                   now + Period::years(10));
    }

    #[test]
    fn add_months_to_moment() {
        let now = Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11));
        assert_eq!(Moment(Local.ymd(2017, 05, 25).and_hms(9, 10, 11)),
                   now.add_months(1));
        assert_eq!(Moment(Local.ymd(2018, 04, 25).and_hms(9, 10, 11)),
                   now.add_months(12));
        assert_eq!(Moment(Local.ymd(2018, 01, 25).and_hms(9, 10, 11)),
                   Moment(Local.ymd(2017, 12, 25).and_hms(9, 10, 11)).add_months(1));
        assert_eq!(Moment(Local.ymd(2017, 06, 30).and_hms(9, 10, 11)),
                   Moment(Local.ymd(2017, 05, 31).and_hms(9, 10, 11)).add_months(1));
        // daylight saving brainfuck
        assert_eq!(Moment(Local.ymd(2017, 03, 26).and_hms(3, 30, 00)),
                   Moment(Local.ymd(2017, 02, 26).and_hms(2, 30, 00)).add_months(1));
    }

    #[test]
    fn sub_months_to_moment() {
        let now = Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11));
        assert_eq!(Moment(Local.ymd(2017, 03, 25).and_hms(9, 10, 11)),
                   now.add_months(-1));
        assert_eq!(Moment(Local.ymd(2016, 04, 25).and_hms(9, 10, 11)),
                   now.add_months(-12));
        assert_eq!(Moment(Local.ymd(2017, 12, 25).and_hms(9, 10, 11)),
                   Moment(Local.ymd(2018, 01, 25).and_hms(9, 10, 11)).add_months(-1));
        assert_eq!(Moment(Local.ymd(2017, 06, 30).and_hms(9, 10, 11)),
                   Moment(Local.ymd(2017, 07, 31).and_hms(9, 10, 11)).add_months(-1));
        // daylight saving brainfuck
        assert_eq!(Moment(Local.ymd(2017, 03, 26).and_hms(3, 30, 00)),
                   Moment(Local.ymd(2017, 04, 26).and_hms(2, 30, 00)).add_months(-1));
    }

    #[test]
    fn test_last_day_in_month() {
        assert_eq!(last_day_in_month(2015, 2), 28);
        assert_eq!(last_day_in_month(2016, 1), 31);
        assert_eq!(last_day_in_month(2016, 2), 29);
        assert_eq!(last_day_in_month(2016, 3), 31);
        assert_eq!(last_day_in_month(2016, 4), 30);
        assert_eq!(last_day_in_month(2016, 5), 31);
        assert_eq!(last_day_in_month(2016, 6), 30);
        assert_eq!(last_day_in_month(2016, 7), 31);
        assert_eq!(last_day_in_month(2016, 8), 31);
        assert_eq!(last_day_in_month(2016, 9), 30);
        assert_eq!(last_day_in_month(2016, 10), 31);
        assert_eq!(last_day_in_month(2016, 11), 30);
        assert_eq!(last_day_in_month(2016, 12), 31);
    }
}
