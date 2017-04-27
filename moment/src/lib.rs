extern crate chrono;

use std::ops;

use chrono::{Datelike, Duration, TimeZone, Timelike};
use chrono::offset::local::Local;
use chrono::datetime::DateTime;

#[derive(Debug,PartialEq,Copy,Clone,Eq,Ord,PartialOrd)]
pub enum Grain {
    Year = 0,
    Quarter = 1,
    Month = 2,
    Week = 3,
    Day = 4,
    Hour = 5,
    Minute = 6,
    Second = 7,
}

#[derive(Debug,PartialEq,Copy,Clone,Eq)]
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

impl ops::Neg for Period {
    type Output = Period;

    fn neg(self) -> Period {
        Period {
            quantity: -self.quantity,
            ..self
        }
    }
}

#[derive(Debug,PartialEq,Copy,Clone,PartialOrd,Eq,Ord)]
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

    fn round_to(self, g: Grain) -> Moment {
        match g {
            Grain::Year => Moment(Local.ymd(self.year(), 1, 1).and_hms(0, 0, 0)),
            Grain::Month => Moment(Local.ymd(self.year(), self.month(), 1).and_hms(0, 0, 0)),
            Grain::Day => Moment(self.date().and_hms(0, 0, 0)),
            Grain::Hour => Moment(self.date().and_hms(self.hour(), 0, 0)),
            Grain::Minute => Moment(self.date().and_hms(self.hour(), self.minute(), 0)),
            Grain::Second => self,
            Grain::Week => {
                // shift to monday morning
                let day_offset = self.weekday().num_days_from_monday(); // monday is 0 here
                self.round_to(Grain::Day) - Period::days(day_offset as i64)
            }
            Grain::Quarter => {
                self.round_to(Grain::Month) - Period::months(self.month0() as i64 % 3)
            }
        }
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

impl ops::Sub<Period> for Moment {
    type Output = Moment;
    fn sub(self, p: Period) -> Moment {
        self + -p
    }
}

#[derive(Debug,PartialEq,Copy,Clone)]
struct Interval {
    start: Moment,
    grain: Grain,
    end: Option<Moment>,
}

impl Interval {
    pub fn end_moment(self) -> Moment {
        self.end
            .unwrap_or_else(|| {
                                self.start +
                                Period {
                                    quantity: 1,
                                    grain: self.grain,
                                }
                            })
    }

    pub fn after(self) -> Interval {
        Interval {
            start: self.end_moment(),
            grain: self.grain,
            end: None,
        }
    }

    pub fn to(self, other: Interval) -> Interval {
        Interval {
            start: self.start,
            grain: ::std::cmp::max(self.grain, other.grain),
            end: Some(other.start),
        }
    }

    pub fn union(self, other: Interval) -> Interval {
        Interval {
            start: self.start,
            grain: ::std::cmp::max(self.grain, other.grain),
            end: Some(other.end_moment()),
        }
    }

    pub fn intersect(self, other: Interval) -> Option<Interval> {
        if self.start <= other.start {
            if other.start >= self.end_moment() {
                None
            } else if other.end_moment() <= self.end_moment() {
                Some(other)
            } else {
                Some(Interval {
                         start: other.start,
                         grain: ::std::cmp::max(self.grain, other.grain),
                         end: Some(self.end_moment()),
                     })
            }
        } else {
            other.intersect(self)
        }
    }
}

impl ops::Add<Period> for Interval {
    type Output = Interval;
    fn add(self, p: Period) -> Interval {
        Interval {
            start: self.start + p,
            end: self.end.map(|it| it + p),
            grain: ::std::cmp::max(self.grain, p.grain),
        }
    }
}

impl ops::Sub<Period> for Interval {
    type Output = Interval;
    fn sub(self, p: Period) -> Interval {
        self + -p
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
    fn moment_round_to() {
        let now = Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11));
        assert_eq!(Moment(Local.ymd(2017, 01, 01).and_hms(0, 0, 0)),
                   now.round_to(Grain::Year));
        assert_eq!(Moment(Local.ymd(2017, 04, 01).and_hms(0, 0, 0)),
                   now.round_to(Grain::Month));
        assert_eq!(Moment(Local.ymd(2017, 04, 25).and_hms(0, 0, 0)),
                   now.round_to(Grain::Day));
        assert_eq!(Moment(Local.ymd(2017, 04, 25).and_hms(9, 0, 0)),
                   now.round_to(Grain::Hour));
        assert_eq!(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 0)),
                   now.round_to(Grain::Minute));
        assert_eq!(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)),
                   now.round_to(Grain::Second));
        assert_eq!(Moment(Local.ymd(2017, 04, 24).and_hms(0, 0, 0)),
                   now.round_to(Grain::Week));
        assert_eq!(Moment(Local.ymd(2017, 04, 01).and_hms(0, 0, 0)),
                   now.round_to(Grain::Quarter));
    }

    #[test]
    fn interval_add_period() {
        let now = Moment(Local.ymd(2017, 04, 25).and_hms(0, 0, 0));
        let interval = Interval {
            start: now,
            grain: Grain::Day,
            end: None,
        };
        let plus_one_hour = interval + Period::hours(1);
        assert_eq!(now + Period::hours(1), plus_one_hour.start);
        assert_eq!(Grain::Hour, plus_one_hour.grain);
    }

    #[test]
    fn interval_binary() {
        let interval = Interval {
            start: Moment(Local.ymd(2017, 04, 25).and_hms(0, 0, 0)),
            grain: Grain::Day,
            end: None,
        };
        let other = Interval {
            start: Moment(Local.ymd(2017, 04, 26).and_hms(9, 0, 0)),
            grain: Grain::Hour,
            end: None,
        };
        let result = interval.to(other);
        assert_eq!(interval.start, result.start);
        assert_eq!(Some(other.start), result.end);
        assert_eq!(Grain::Hour, result.grain);

        let result = interval.union(other);
        assert_eq!(interval.start, result.start);
        assert_eq!(Some(other.end_moment()), result.end);
        assert_eq!(Grain::Hour, result.grain);
    }

    #[test]
    fn interval_intersect() {
        let interval = Interval {
            start: Moment(Local.ymd(2017, 04, 25).and_hms(0, 0, 0)),
            grain: Grain::Day,
            end: Some(Moment(Local.ymd(2017, 04, 30).and_hms(0, 0, 0))),
        };
        assert_eq!(interval, interval.intersect(interval).unwrap());
        let other = Interval {
            start: Moment(Local.ymd(2017, 04, 26).and_hms(9, 0, 0)),
            grain: Grain::Hour,
            end: Some(Moment(Local.ymd(2017, 04, 26).and_hms(11, 0, 0))),
        };
        assert_eq!(other, interval.intersect(other).unwrap());
        let other = Interval {
            start: Moment(Local.ymd(2017, 04, 26).and_hms(9, 0, 0)),
            grain: Grain::Hour,
            end: Some(Moment(Local.ymd(2017, 05, 08).and_hms(11, 0, 0))),
        };
        assert_eq!(Interval {
                       start: Moment(Local.ymd(2017, 04, 26).and_hms(9, 0, 0)),
                       grain: Grain::Hour,
                       end: Some(Moment(Local.ymd(2017, 04, 30).and_hms(0, 0, 0))),
                   },
                   interval.intersect(other).unwrap());
        assert_eq!(Interval {
                       start: Moment(Local.ymd(2017, 04, 26).and_hms(9, 0, 0)),
                       grain: Grain::Hour,
                       end: Some(Moment(Local.ymd(2017, 04, 30).and_hms(0, 0, 0))),
                   },
                   other.intersect(interval).unwrap());
        let other = Interval {
            start: Moment(Local.ymd(2017, 05, 26).and_hms(9, 0, 0)),
            grain: Grain::Hour,
            end: Some(Moment(Local.ymd(2017, 06, 08).and_hms(11, 0, 0))),
        };
        assert_eq!(None, interval.intersect(other));
    }


}
