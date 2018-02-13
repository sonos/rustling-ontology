extern crate chrono;
#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate enum_primitive;
extern crate vec_map;

mod period;
pub mod interval_constraints;
pub mod bidirectional_walker;
pub mod walker;

use std::ops;
use std::cmp::Ordering;
use std::fmt;

use chrono::{Duration, Datelike, Timelike};
pub use chrono::{Weekday, Local, TimeZone};
use chrono::datetime::DateTime;
pub use interval_constraints::*;
pub use period::*;


#[derive(Clone)]
pub struct Moment<T: TimeZone>(pub DateTime<T>);

impl<T: TimeZone> Copy for Moment<T> where <T as TimeZone>::Offset: Copy {}

impl<T1: TimeZone, T2: TimeZone> PartialEq<Moment<T2>> for Moment<T1> {
    fn eq(&self, other: &Moment<T2>) -> bool { 
        self.0 == other.0 
    }
}

impl<T: TimeZone> Eq for Moment<T> {}

impl<T: TimeZone> PartialOrd for Moment<T> {
    fn partial_cmp(&self, other: &Moment<T>) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: TimeZone> Ord for Moment<T> {
    fn cmp(&self, other: &Moment<T>) -> Ordering { 
        self.0.cmp(&other.0) 
    }
}

impl<T: TimeZone> fmt::Debug for Moment<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<T: TimeZone> fmt::Display for Moment<T> where T::Offset: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn last_day_in_month<T: TimeZone>(y: i32, m: u32, tz: T) -> u32 {
    assert!(m >= 1 && m <= 12);
    for d in 28..31 {
        if (tz.ymd_opt(y, m, d + 1)).single().is_none() {
            return d as u32;
        }
    }
    31
}

impl<T: TimeZone> ops::Deref for Moment<T> {
    type Target = DateTime<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Moment<Local> {
    pub fn now() -> Moment<Local> {
        Moment(Local::now())
    }

    pub fn year(&self) -> i32 {
        self.0.year()
    }

    pub fn month(&self) -> u32 {
        self.0.month()
    }

    pub fn day(&self) -> u32 {
        self.0.day()
    }

    pub fn ymd(y: i32, m: u32, d: u32) -> Moment<Local> {
        Moment(Local.ymd(y, m, d).and_hms(0, 0, 0))
    }
}

impl<T: TimeZone> Moment<T> where <T as TimeZone>::Offset: Copy {

    fn add_months(self, n: i32) -> Moment<T> {
        let (year, month0) = if n >= 0 {
            let n = n as u32;
            let carry = ((self.month0() + n % 12) >= 12) as i32;
            (self.year() + (n / 12) as i32 + carry, (self.month0() + n) % 12)
        } else {
            let n = -n as u32;
            let borrow = (self.month0() < n % 12) as i32;
            (self.year() - (n / 12) as i32 - borrow as i32, (12 + self.month0() - (n % 12)) % 12)
        };
        let target_month_days = last_day_in_month(year, month0 + 1, self.timezone());
        let day = ::std::cmp::min(target_month_days, self.day());
        Moment(self.timezone().ymd(year, month0 + 1, day).and_hms(self.hour(), self.minute(), self.second()))
    }

    fn round_to(self, g: Grain) -> Moment<T> {
        match g {
            Grain::Year => Moment(self.timezone().ymd(self.year(), 1, 1).and_hms(0, 0, 0)),
            Grain::Month => Moment(self.timezone().ymd(self.year(), self.month(), 1).and_hms(0, 0, 0)),
            Grain::Day => Moment(self.date().and_hms(0, 0, 0)),
            Grain::Hour => Moment(self.date().and_hms(self.hour(), 0, 0)),
            Grain::Minute => Moment(self.date().and_hms(self.hour(), self.minute(), 0)),
            Grain::Second => self,
            Grain::Week => {
                // shift to monday morning
                let day_offset = self.weekday().num_days_from_monday(); // monday is 0 here
                self.round_to(Grain::Day) - PeriodComp::days(day_offset as i64)
            }
            Grain::Quarter => {
                self.round_to(Grain::Month) - PeriodComp::months(self.month0() as i64 % 3)
            }
        }
    }

    fn adjust_for_daylight_saving(self) -> Moment<T> {
        Moment(self.timezone()
                   .ymd(self.year(), self.month(), self.day())
                   .and_hms(self.hour(), self.minute(), self.second()))
    }
}

impl<T: TimeZone> ops::Add<Period> for Moment<T>  where <T as TimeZone>::Offset: Copy {
    type Output = Moment<T>;
    fn add(self, p: Period) -> Moment<T> {
        self + &p
    }
}

impl<'a, T: TimeZone> ops::Add<&'a Period> for Moment<T>  where <T as TimeZone>::Offset: Copy {
    type Output = Moment<T>;
    fn add(self, p: &'a Period) -> Moment<T> {
        use enum_primitive::FromPrimitive;
        let mut result = self;
        for (g, q) in p.0.iter() {
            result = result +
                     PeriodComp {
                         grain: Grain::from_usize(g).unwrap(), // checked
                         quantity: *q,
                     };
        }
        result
    }
}

impl<T: TimeZone> ops::Add<PeriodComp> for Moment<T>  where <T as TimeZone>::Offset: Copy {
    type Output = Moment<T>;
    fn add(self, p: PeriodComp) -> Moment<T> {
        self + &p
    }
}

impl<'a, T: TimeZone> ops::Add<&'a PeriodComp> for Moment<T>  where <T as TimeZone>::Offset: Copy {
    type Output = Moment<T>;
    fn add(self, p: &'a PeriodComp) -> Moment<T> {
        match p.grain {
            Grain::Year => self.add_months(12 * p.quantity as i32),
            Grain::Quarter => self.add_months(3 * p.quantity as i32),
            Grain::Month => self.add_months(p.quantity as i32),
            Grain::Week => {
                Moment(self.0 + Duration::weeks(p.quantity)).adjust_for_daylight_saving()
            }
            Grain::Day => Moment(self.0 + Duration::days(p.quantity)).adjust_for_daylight_saving(),
            Grain::Hour => Moment(self.0 + Duration::hours(p.quantity)),
            Grain::Minute => Moment(self.0 + Duration::minutes(p.quantity)),
            Grain::Second => Moment(self.0 + Duration::seconds(p.quantity)),
        }
    }
}

impl<T: TimeZone> ops::Sub<PeriodComp> for Moment<T>  where <T as TimeZone>::Offset: Copy {
    type Output = Moment<T>;
    fn sub(self, p: PeriodComp) -> Moment<T> {
        self + -p
    }
}

impl<'a, T: TimeZone> ops::Sub<&'a PeriodComp> for Moment<T>  where <T as TimeZone>::Offset: Copy {
    type Output = Moment<T>;
    fn sub(self, p: &'a PeriodComp) -> Moment<T> {
        self + -p
    }
}

#[derive(Clone,new)]
pub struct Interval<T: TimeZone> {
    pub start: Moment<T>,
    pub end: Option<Moment<T>>,
    pub grain: Grain,
}

impl<T: TimeZone> Interval<T> {
    fn timezone(&self) -> T {
        self.start.0.timezone()
    }
}

impl Interval<Local> {
    pub fn ymd(y: i32, m: u32, d: u32) -> Interval<Local> {
        Interval::starting_at(Moment(Local.ymd(y, m, d).and_hms(0, 0, 0)), Grain::Day)
    }
}

impl<T: TimeZone> fmt::Debug for Interval<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Interval {{ start: {:?}, end: {:?}, grain: {:?} }}", self.start, self.end, self.grain)
    }
}

impl<T: TimeZone> Copy for Interval<T> where <T as TimeZone>::Offset: Copy {}

impl<T: TimeZone> PartialEq for Interval<T> {
    fn eq(&self, other: &Interval<T>) -> bool {
        self.start == other.start && self.end == other.end && self.grain == other.grain
    }
}

impl<T: TimeZone> Interval<T> where <T as TimeZone>::Offset: Copy {
    fn start_round_to(self, g: Grain) -> Interval<T> {
        Interval {
            start: self.start.round_to(g),
            grain: g,
            end: None,
        }
    }

    fn interval_round_to(self, g: Grain) -> Interval<T> {
        Interval {
            start: self.start.round_to(g),
            grain: g,
            end: self.end.map(|it| it.round_to(g)),
        }
    }

    pub fn starting_at(start: Moment<T>, grain: Grain) -> Interval<T> {
        Interval {
            start: start,
            grain: grain,
            end: None,
        }
    }

    pub fn end_moment(self) -> Moment<T> {
        self.end
            .unwrap_or_else(|| {
                                self.start +
                                PeriodComp {
                                    quantity: 1,
                                    grain: self.grain,
                                }
                            })
    }

    pub fn after(self) -> Interval<T> {
        Interval {
            start: self.end_moment(),
            grain: self.grain,
            end: None,
        }
    }

    pub fn to(self, other: Interval<T>) -> Interval<T> {
        Interval {
            start: self.start,
            grain: ::std::cmp::max(self.grain, other.grain),
            end: Some(other.start),
        }
    }

    pub fn union(self, other: Interval<T>) -> Interval<T> {
        Interval {
            start: self.start,
            grain: ::std::cmp::max(self.grain, other.grain),
            end: Some(other.end_moment()),
        }
    }

    pub fn interval_to(self, other: Interval<T>) -> Interval<T> {
        Interval {
            start: self.start,
            grain: ::std::cmp::max(self.grain, other.grain),
            end: Some(other.start)
        }
    }

    pub fn intersect(self, other: Interval<T>) -> Option<Interval<T>> {
        if self.start <= other.start {
            let self_end = self.end_moment();
            let other_end = other.end_moment();
            if other.start >= self_end {
                None
            } else if other_end <= self_end {
                Some(other)
            } else if self.start == other.start && self_end < other_end {
                Some(self)
            } else {
                Some(Interval {
                         start: other.start,
                         grain: ::std::cmp::max(self.grain, other.grain),
                         end: Some(self_end),
                     })
            }
        } else {
            other.intersect(self)
        }
    }

    pub fn seconds(self) -> i64 {
        self.end_moment()
            .0
            .signed_duration_since(self.start.0)
            .num_seconds()
    }
}

impl<T: TimeZone> ops::Add<PeriodComp> for Interval<T> where <T as TimeZone>::Offset: Copy {
    type Output = Interval<T>;
    fn add(self, p: PeriodComp) -> Interval<T> {
        Interval {
            start: self.start + p,
            end: self.end.map(|it| it + p),
            grain: ::std::cmp::max(self.grain, p.grain),
        }
    }
}

impl<T: TimeZone> ops::Sub<PeriodComp> for Interval<T>  where <T as TimeZone>::Offset: Copy {
    type Output = Interval<T>;
    fn sub(self, p: PeriodComp) -> Interval<T> {
        self + -p
    }
}

impl<T: TimeZone> ops::Add<Period> for Interval<T> where <T as TimeZone>::Offset: Copy {
    type Output = Interval<T>;
    fn add(self, p: Period) -> Interval<T> {
        self + &p
    }
}

impl<'a, T: TimeZone> ops::Add<&'a Period> for Interval<T> where <T as TimeZone>::Offset: Copy {
    type Output = Interval<T>;
    fn add(self, p: &'a Period) -> Interval<T> {
        use enum_primitive::FromPrimitive;
        let mut result = self;
        for (g, q) in p.0.iter() {
            result = result +
                     PeriodComp {
                         grain: Grain::from_usize(g).unwrap(), // checked
                         quantity: *q,
                     };
        }
        result
    }
}

impl<T: TimeZone> ops::Sub<Period> for Interval<T> where <T as TimeZone>::Offset: Copy {
    type Output = Interval<T>;
    fn sub(self, p: Period) -> Interval<T> {
        self + -p
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, FixedOffset, NaiveDate, NaiveDateTime, LocalResult};

    #[derive(Copy, Clone, PartialEq, Eq)]
    struct Paris;
    
    impl TimeZone for Paris {
        type Offset = FixedOffset;
        fn from_offset(_: &FixedOffset) -> Paris { Paris }
    
        fn offset_from_local_date(&self, _local: &NaiveDate) -> LocalResult<FixedOffset> {
            LocalResult::Single(FixedOffset::east(2*3600))
        }
        fn offset_from_local_datetime(&self, _local: &NaiveDateTime) -> LocalResult<FixedOffset> {
            LocalResult::Single(FixedOffset::east(2*3600))
        }
    
        fn offset_from_utc_date(&self, _utc: &NaiveDate) -> FixedOffset { FixedOffset::east(2*3600) }
        fn offset_from_utc_datetime(&self, _utc: &NaiveDateTime) -> FixedOffset { FixedOffset::east(2*3600) }
    }
    
    #[test]
    fn test_last_day_in_month() {
        assert_eq!(last_day_in_month(2015, 2, Paris), 28);
        assert_eq!(last_day_in_month(2016, 1, Paris), 31);
        assert_eq!(last_day_in_month(2016, 2, Paris), 29);
        assert_eq!(last_day_in_month(2016, 3, Paris), 31);
        assert_eq!(last_day_in_month(2016, 4, Paris), 30);
        assert_eq!(last_day_in_month(2016, 5, Paris), 31);
        assert_eq!(last_day_in_month(2016, 6, Paris), 30);
        assert_eq!(last_day_in_month(2016, 7, Paris), 31);
        assert_eq!(last_day_in_month(2016, 8, Paris), 31);
        assert_eq!(last_day_in_month(2016, 9, Paris), 30);
        assert_eq!(last_day_in_month(2016, 10, Paris), 31);
        assert_eq!(last_day_in_month(2016, 11, Paris), 30);
        assert_eq!(last_day_in_month(2016, 12, Paris), 31);
    }

    #[test]
    fn add_months_to_moment() {
        let now = Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11));
        assert_eq!(Moment(Paris.ymd(2017, 05, 25).and_hms(9, 10, 11)),
                   now.add_months(1));
        assert_eq!(Moment(Paris.ymd(2018, 04, 25).and_hms(9, 10, 11)),
                   now.add_months(12));
        assert_eq!(Moment(Paris.ymd(2018, 01, 25).and_hms(9, 10, 11)),
                   Moment(Paris.ymd(2017, 12, 25).and_hms(9, 10, 11)).add_months(1));
        assert_eq!(Moment(Paris.ymd(2017, 06, 30).and_hms(9, 10, 11)),
                   Moment(Paris.ymd(2017, 05, 31).and_hms(9, 10, 11)).add_months(1));
        // daylight saving brainfuck
        assert_eq!(Moment(FixedOffset::east(2*3600).ymd(2017, 03, 26).and_hms(3, 30, 00)),
                   Moment(FixedOffset::east(1*3600).ymd(2017, 02, 26).and_hms(2, 30, 00)).add_months(1));
    }

    #[test]
    fn add_period_comp_to_moment() {
        let now = Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11));
        assert_eq!(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 21)),
                   now + PeriodComp::seconds(10));
        assert_eq!(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 20, 11)),
                   now + &PeriodComp::minutes(10));
        assert_eq!(Moment(Paris.ymd(2017, 04, 25).and_hms(19, 10, 11)),
                   now + PeriodComp::hours(10));
        assert_eq!(Moment(Paris.ymd(2017, 05, 5).and_hms(9, 10, 11)),
                   now + &PeriodComp::days(10));
        assert_eq!(Moment(Paris.ymd(2017, 05, 2).and_hms(9, 10, 11)),
                   now + PeriodComp::weeks(1));
        assert_eq!(Moment(Paris.ymd(2018, 02, 25).and_hms(9, 10, 11)),
                   now + &PeriodComp::months(10));
        assert_eq!(Moment(Paris.ymd(2017, 07, 25).and_hms(9, 10, 11)),
                   now + PeriodComp::quarters(1));
        assert_eq!(Moment(Paris.ymd(2027, 04, 25).and_hms(9, 10, 11)),
                   now + &PeriodComp::years(10));
    }

    #[test]
    fn add_period_to_moment() {
        let now = Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11));

        let mut period = Period::default();
        period.0.insert(Grain::Year as usize, 2);
        period.0.insert(Grain::Month as usize, 3);

        assert_eq!(Moment(Paris.ymd(2019, 07, 25).and_hms(9, 10, 11)),
                   now + &period);

        period.0.insert(Grain::Hour as usize, 5);

        assert_eq!(Moment(Paris.ymd(2019, 07, 25).and_hms(14, 10, 11)),
                   now + period.clone());
    }

    #[test]
    fn sub_months_to_moment() {
        let now = Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11));
        assert_eq!(Moment(Paris.ymd(2017, 03, 25).and_hms(9, 10, 11)),
                   now.add_months(-1));
        assert_eq!(Moment(Paris.ymd(2016, 04, 25).and_hms(9, 10, 11)),
                   now.add_months(-12));
        assert_eq!(Moment(Paris.ymd(2017, 12, 25).and_hms(9, 10, 11)),
                   Moment(Paris.ymd(2018, 01, 25).and_hms(9, 10, 11)).add_months(-1));
        assert_eq!(Moment(Paris.ymd(2017, 06, 30).and_hms(9, 10, 11)),
                   Moment(Paris.ymd(2017, 07, 31).and_hms(9, 10, 11)).add_months(-1));
        // daylight saving brainfuck
        assert_eq!(Moment(FixedOffset::east(2*3600).ymd(2017, 03, 26).and_hms(3, 30, 00)),
                   Moment(FixedOffset::east(1*3600).ymd(2017, 04, 26).and_hms(2, 30, 00)).add_months(-1));
    }

    #[test]
    fn daylight_saving_aware() {
        // TODO Take a look at the offset shifting due to a period addition        // 1st March -> +1 and 31 Match -> +2        // 1st March + 30 days -> +1 instead of +2
        assert_eq!(Moment(Paris.ymd(2017, 03, 31).and_hms(0, 0, 0)),
                   Moment(Paris.ymd(2017, 03, 20).and_hms(0, 0, 0)) + PeriodComp::days(11))
    }

    #[test]
    fn moment_round_to() {
        let now = Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11));
        assert_eq!(Moment(Paris.ymd(2017, 01, 01).and_hms(0, 0, 0)),
                   now.round_to(Grain::Year));
        assert_eq!(Moment(Paris.ymd(2017, 04, 01).and_hms(0, 0, 0)),
                   now.round_to(Grain::Month));
        assert_eq!(Moment(Paris.ymd(2017, 04, 25).and_hms(0, 0, 0)),
                   now.round_to(Grain::Day));
        assert_eq!(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 0, 0)),
                   now.round_to(Grain::Hour));
        assert_eq!(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 0)),
                   now.round_to(Grain::Minute));
        assert_eq!(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)),
                   now.round_to(Grain::Second));
        assert_eq!(Moment(Paris.ymd(2017, 04, 24).and_hms(0, 0, 0)),
                   now.round_to(Grain::Week));
        assert_eq!(Moment(Paris.ymd(2017, 04, 01).and_hms(0, 0, 0)),
                   now.round_to(Grain::Quarter));
    }

    #[test]
    fn interval_add_period() {
        let now = Moment(Paris.ymd(2017, 04, 25).and_hms(0, 0, 0));
        let interval = Interval {
            start: now,
            grain: Grain::Day,
            end: None,
        };
        let plus_one_hour = interval + PeriodComp::hours(1);
        assert_eq!(now + PeriodComp::hours(1), plus_one_hour.start);
        assert_eq!(Grain::Hour, plus_one_hour.grain);
    }

    #[test]
    fn interval_binary() {
        let interval = Interval {
            start: Moment(Paris.ymd(2017, 04, 25).and_hms(0, 0, 0)),
            grain: Grain::Day,
            end: None,
        };
        let other = Interval {
            start: Moment(Paris.ymd(2017, 04, 26).and_hms(9, 0, 0)),
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
            start: Moment(Paris.ymd(2017, 04, 25).and_hms(0, 0, 0)),
            grain: Grain::Day,
            end: Some(Moment(Paris.ymd(2017, 04, 30).and_hms(0, 0, 0))),
        };
        assert_eq!(interval, interval.intersect(interval).unwrap());
        let other = Interval {
            start: Moment(Paris.ymd(2017, 04, 26).and_hms(9, 0, 0)),
            grain: Grain::Hour,
            end: Some(Moment(Paris.ymd(2017, 04, 26).and_hms(11, 0, 0))),
        };
        assert_eq!(other, interval.intersect(other).unwrap());
        let other = Interval {
            start: Moment(Paris.ymd(2017, 04, 26).and_hms(9, 0, 0)),
            grain: Grain::Hour,
            end: Some(Moment(Paris.ymd(2017, 05, 08).and_hms(11, 0, 0))),
        };
        assert_eq!(Interval {
                       start: Moment(Paris.ymd(2017, 04, 26).and_hms(9, 0, 0)),
                       grain: Grain::Hour,
                       end: Some(Moment(Paris.ymd(2017, 04, 30).and_hms(0, 0, 0))),
                   },
                   interval.intersect(other).unwrap());
        assert_eq!(Interval {
                       start: Moment(Paris.ymd(2017, 04, 26).and_hms(9, 0, 0)),
                       grain: Grain::Hour,
                       end: Some(Moment(Paris.ymd(2017, 04, 30).and_hms(0, 0, 0))),
                   },
                   other.intersect(interval).unwrap());
        let other = Interval {
            start: Moment(Paris.ymd(2017, 05, 26).and_hms(9, 0, 0)),
            grain: Grain::Hour,
            end: Some(Moment(Paris.ymd(2017, 06, 08).and_hms(11, 0, 0))),
        };
        assert_eq!(None, interval.intersect(other));
    }

    #[test]
    fn seconds() {
        let interval = Interval {
            start: Moment(Paris.ymd(2017, 04, 25).and_hms(0, 0, 0)),
            grain: Grain::Day,
            end: Some(Moment(Paris.ymd(2017, 04, 30).and_hms(0, 0, 0))),
        };
        assert_eq!(5 * 86400, interval.seconds());
    }
}
