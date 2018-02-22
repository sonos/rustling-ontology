use std::rc::Rc;

use bidirectional_walker::*;
use walker::*;
use {Moment, Interval, last_day_in_month};
use period::*;
use std::ops;
use std::fmt;
use chrono::offset::local::Local;
use chrono::{Datelike, TimeZone, Timelike, Weekday};

#[derive(Clone, PartialEq, new)]
pub struct Context<T: TimeZone> {
    pub reference: Interval<T>,
    pub min: Interval<T>,
    pub max: Interval<T>,
}

impl<T: TimeZone> Copy for Context<T> where <T as TimeZone>::Offset: Copy {}

impl<T: TimeZone> fmt::Debug for Context<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Context {{ reference: {:?}, min: {:?}, max: {:?} }}", self.reference, self.min, self.max)
    }
}

impl Default for Context<Local> {
    fn default() -> Context<Local> {
        Self::now()
    }
}

impl Context<Local> {
    pub fn now() -> Context<Local> {
        Context::for_reference(Interval::starting_at(Moment::now(), Grain::Second))
    }
}

impl<T: TimeZone> Context<T> where <T as TimeZone>::Offset: Copy {
    pub fn for_reference(now: Interval<T>) -> Context<T> {
        // TODO: Should be refactor with the min, max date offer by chrono crate
        let now_end = now.end_moment();
        let max_year = if 2038 > now_end.year() + 30 { now_end.year() + 30 } else { 2038 };
        let min_year = if 1970 < now.start.year() - 30 { now.start.year() - 30  } else { 1970 };
        let min_interval = Interval::starting_at(Moment(now.timezone().ymd(min_year, 1, 1).and_hms(0, 0, 0)), Grain::Second);
        let max_interval = Interval::starting_at(Moment(now.timezone().ymd(max_year, 1, 1).and_hms(0, 0, 0)), Grain::Second);
        Context::new(now, min_interval, max_interval)
    }
}

pub type IntervalWalker<T> = BidirectionalWalker<Interval<T>>;

pub trait IntervalConstraint<T: TimeZone> where <T as TimeZone>::Offset: Copy {
    fn grain(&self) -> Grain;
    fn coarse_grain_step(&self) -> Grain;
    fn to_walker(&self, origin: &Interval<T>, _context: &Context<T>) -> IntervalWalker<T>;
}

#[derive(Clone)]
pub struct RcConstraint<T: TimeZone>(pub Rc<IntervalConstraint<T>>);

impl<T: TimeZone> ops::Deref for RcConstraint<T> where <T as TimeZone>::Offset: Copy {
    type Target = Rc<IntervalConstraint<T>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

macro_rules! rc {
    ($obj:expr) => (RcConstraint(Rc::new($obj)))
}


impl<T: TimeZone+'static> RcConstraint<T> where <T as TimeZone>::Offset: Copy {
    pub fn shift_by(&self, period: Period) -> RcConstraint<T> {
        ShiftBy::new(self, period)
    }

    pub fn translate_with<Offset>(&self, offset: Offset) -> RcConstraint<T>
        where Offset: Fn(&Interval<T>, &Context<T>) -> Option<Interval<T>> + 'static {
        Translate::new(self, Rc::new(offset))
    }

    pub fn take_the_nth(&self, n: i64) -> RcConstraint<T> {
       TakeTheNth::new(n, false, self)
    }

    pub fn take_the_nth_not_immediate(&self, n: i64) -> RcConstraint<T> {
       TakeTheNth::new(n, true, self)
    }

    pub fn take(&self, n: i64) -> RcConstraint<T> {
       TakeN::new(n, false, self)
    }

    pub fn take_not_immediate(&self, n: i64) -> RcConstraint<T> {
       TakeN::new(n, true, self)
    }

    pub fn span_to(&self, inner: &RcConstraint<T>) -> RcConstraint<T> {
       Span::new(self, inner, false)
    }

    pub fn span_inclusive_to(&self, inner: &RcConstraint<T>)  -> RcConstraint<T> {
       Span::new(self, inner, true)
    }

    pub fn intersect(&self, inner: &RcConstraint<T>) -> RcConstraint<T> {
        Intersection::new(self, inner)
    }

    pub fn last_of(&self, inner: &RcConstraint<T>) -> RcConstraint<T> {
        TakeLastOf::new(inner, self)
    }

    pub fn the_nth(&self, n: i64) -> NthConstraint<T> {
        NthConstraint(self.clone(), n)
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Year(pub i32);

impl Year {
    pub fn new<T: TimeZone>(y: i32) -> RcConstraint<T> where <T as TimeZone>::Offset: Copy {
        rc!(Year(y))
    }
}

impl<T: TimeZone> IntervalConstraint<T> for Year where <T as TimeZone>::Offset: Copy {
    fn grain(&self) -> Grain {
        Grain::Year
    }

    fn coarse_grain_step(&self) -> Grain {
        Grain::Year
    }

    fn to_walker(&self, origin: &Interval<T>, _context: &Context<T>) -> IntervalWalker<T> {
        let normalized_year = if self.0 <= 99 {
            (self.0 + 50) % 100 + 2000 - 50
        } else {
            self.0
        };

        if origin.start.year() <= normalized_year {
            let moment_year = Moment(origin.timezone().ymd(normalized_year, 1, 1).and_hms(0, 0, 0));
            let interval = Interval::starting_at(moment_year, Grain::Year);
            BidirectionalWalker::new().forward_values(vec![interval])
        } else {
            let moment_year = Moment(origin.timezone().ymd(normalized_year, 1, 1).and_hms(0, 0, 0));
            let interval = Interval::starting_at(moment_year, Grain::Year);
            BidirectionalWalker::new().backward_values(vec![interval])
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct YearMonthDay {
    pub year: i32, 
    pub month: u32, 
    pub day: u32,
}

impl YearMonthDay {
    pub fn new<T: TimeZone>(y: i32, m: u32, d: u32) -> RcConstraint<T> where <T as TimeZone>::Offset: Copy {
        rc!(YearMonthDay { year: y, month: m, day: d })
    }
}

impl<T: TimeZone> IntervalConstraint<T> for YearMonthDay where <T as TimeZone>::Offset: Copy {
    fn grain(&self) -> Grain {
        Grain::Day
    }

    fn coarse_grain_step(&self) -> Grain {
        Grain::Year
    }

    fn to_walker(&self, origin: &Interval<T>, _context: &Context<T>) -> IntervalWalker<T> {
        let normalized_year = if self.year < 99 {
            (self.year + 50) % 100 + 2000 - 50
        } else {
            self.year
        };
        if self.day > last_day_in_month(normalized_year, self.month, origin.timezone()) {
            BidirectionalWalker::new() 
        } else if origin.start.year() <= normalized_year {
            let moment_year = Moment(origin.timezone().ymd(normalized_year, self.month, self.day).and_hms(0, 0, 0));
            let interval = Interval::starting_at(moment_year, Grain::Day);
            BidirectionalWalker::new().forward_values(vec![interval])
        } else {
            let moment_year = Moment(origin.timezone().ymd(normalized_year, self.month, self.day).and_hms(0, 0, 0));
            let interval = Interval::starting_at(moment_year, Grain::Day);
            BidirectionalWalker::new().backward_values(vec![interval])
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MonthDay(pub u32, pub u32);

impl MonthDay {
    pub fn new<T: TimeZone + 'static>(m: u32, d: u32) -> RcConstraint<T> where <T as TimeZone>::Offset: Copy{
        rc!(MonthDay(m, d))
    }
}

impl<T: TimeZone + 'static> IntervalConstraint<T> for MonthDay where <T as TimeZone>::Offset: Copy {
    fn grain(&self) -> Grain {
        Grain::Day
    }

    fn coarse_grain_step(&self) -> Grain {
        Grain::Year
    }

    fn to_walker(&self, origin: &Interval<T>, _context: &Context<T>) -> IntervalWalker<T> {
        let rounded_moment = Moment(origin.timezone()
                                        .ymd(origin.start.year(), self.0, 1)
                                        .and_hms(0, 0, 0));
        let rounded_interval = Interval::starting_at(rounded_moment, Grain::Day);
        let offset_year = !(origin.start <= rounded_interval.end_moment()) as i64;
        let anchor = rounded_interval + PeriodComp::years(offset_year);
        let origin_copied = origin.clone();

        let day_of_month = self.1;
        let forward_walker =
            Walker::generator(anchor, |prev| prev + PeriodComp::years(1))
                .filter(move |interval| {
                            day_of_month <=
                            last_day_in_month(interval.start.year(), interval.start.month(), origin_copied.timezone())
                        })
                .map(move |interval| interval + PeriodComp::days(day_of_month as i64 - 1));

        let backward_walker =
            Walker::generator(anchor - PeriodComp::years(1),
                              |prev| prev - PeriodComp::years(1))
                    .filter(move |interval| {
                                day_of_month <=
                                last_day_in_month(interval.start.year(), interval.start.month(), origin_copied.timezone())
                            })
                    .map(move |interval| interval + PeriodComp::days(day_of_month as i64 - 1));

        BidirectionalWalker::new()
            .forward(forward_walker)
            .backward(backward_walker)
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Month(pub u32);

impl Month {
    pub fn new<T: TimeZone>(m: u32) -> RcConstraint<T> where <T as TimeZone>::Offset: Copy {
        rc!(Month(m))
    }
}

impl<T: TimeZone> IntervalConstraint<T> for Month where <T as TimeZone>::Offset: Copy {
    fn grain(&self) -> Grain {
        Grain::Month
    }

    fn coarse_grain_step(&self) -> Grain {
        Grain::Year
    }

    fn to_walker(&self, origin: &Interval<T>, _context: &Context<T>) -> IntervalWalker<T> {
        let rounded_moment = Moment(origin.timezone()
                                        .ymd(origin.start.year(), self.0, 1)
                                        .and_hms(0, 0, 0));
        let rounded_interval = Interval::starting_at(rounded_moment, Grain::Month);
        let offset_year = !(origin.start <= rounded_interval.end_moment()) as i64;
        let anchor = rounded_interval + PeriodComp::years(offset_year);

        BidirectionalWalker::new()
            .forward_with(anchor, |prev| prev + PeriodComp::years(1))
            .backward_with(anchor - PeriodComp::years(1),
                           |prev| prev - PeriodComp::years(1))
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DayOfMonth(pub u32);

impl DayOfMonth {
    pub fn new<T: TimeZone + 'static>(dom: u32) -> RcConstraint<T> where <T as TimeZone>::Offset: Copy {
        rc!(DayOfMonth(dom))
    }
}

impl<T: TimeZone + 'static> IntervalConstraint<T> for DayOfMonth where <T as TimeZone>::Offset: Copy {
    fn grain(&self) -> Grain {
        Grain::Day
    }

    fn coarse_grain_step(&self) -> Grain {
        Grain::Month
    }

    fn to_walker(&self, origin: &Interval<T>, _context: &Context<T>) -> IntervalWalker<T> {
        let offset_month = (origin.start.0.day() > self.0) as i64;
        let anchor = origin.start_round_to(Grain::Month) + PeriodComp::months(offset_month);
        let origin_copied = origin.clone();
        let day_of_month = self.0;
        let forward_walker =
            Walker::generator(anchor, |prev| prev + PeriodComp::months(1))
                .filter(move |interval| {
                            day_of_month <=
                            last_day_in_month(interval.start.year(), interval.start.month(), origin_copied.timezone())
                        })
                .map(move |interval| interval + PeriodComp::days(day_of_month as i64 - 1));

        let backward_walker =
            Walker::generator(anchor - PeriodComp::months(1),
                              |prev| prev - PeriodComp::months(1))
                    .filter(move |interval| {
                                day_of_month <=
                                last_day_in_month(interval.start.year(), interval.start.month(), origin_copied.timezone())
                            })
                    .map(move |interval| interval + PeriodComp::days(day_of_month as i64 - 1));

        BidirectionalWalker::new()
            .forward(forward_walker)
            .backward(backward_walker)
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DayOfWeek(pub Weekday);

impl DayOfWeek {
    pub fn new<T: TimeZone>(dow: Weekday) -> RcConstraint<T> where <T as TimeZone>::Offset: Copy {
        rc!(DayOfWeek(dow))
    }
}

impl<T: TimeZone> IntervalConstraint<T> for DayOfWeek where <T as TimeZone>::Offset: Copy {
    fn grain(&self) -> Grain {
        Grain::Day
    }

    fn coarse_grain_step(&self) -> Grain {
        Grain::Week
    }

    fn to_walker(&self, origin: &Interval<T>, _context: &Context<T>) -> IntervalWalker<T> {
        // number_from_monday is u32 -> use i64
        let offset = (self.0.number_from_monday() as i64 -
                      origin.start.weekday().number_from_monday() as i64 + 7) % 7;
        let anchor = origin.start_round_to(Grain::Day) + PeriodComp::days(offset);

        BidirectionalWalker::new()
            .forward_with(anchor, |prev| prev + PeriodComp::weeks(1))
            .backward_with(anchor - PeriodComp::weeks(1),
                           |prev| prev - PeriodComp::weeks(1))
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HourMinute {
    pub hour: u32,
    pub minute: u32,
    pub is_12_clock: bool,
}

impl HourMinute {
    pub fn clock_12<T: TimeZone>(hour: u32, minute: u32) -> RcConstraint<T> where <T as TimeZone>::Offset: Copy {
        rc!(HourMinute {
            hour: hour,
            minute: minute,
            is_12_clock: true,
        })
    }

    pub fn clock_24<T: TimeZone>(hour: u32, minute: u32) -> RcConstraint<T> where <T as TimeZone>::Offset: Copy {
        rc!(HourMinute {
            hour: hour,
            minute: minute,
            is_12_clock: false,
        })
    }
}


impl<T: TimeZone> IntervalConstraint<T> for HourMinute where <T as TimeZone>::Offset: Copy {
    fn grain(&self) -> Grain {
        Grain::Minute
    }

    fn coarse_grain_step(&self) -> Grain {
        Grain::Day
    }

    fn to_walker(&self, origin: &Interval<T>, _context: &Context<T>) -> IntervalWalker<T> {
        let clock_step = if self.hour <= 12 && self.is_12_clock {
            12
        } else {
            24
        };
        let offset_hour = (self.hour as i64 - origin.start.hour() as i64 + clock_step) % clock_step;
        let offset_minute = self.minute as i64 % 60;
        let anchor = origin.start_round_to(Grain::Hour) + PeriodComp::hours(offset_hour)+ PeriodComp::minutes(offset_minute);
        BidirectionalWalker::new()
            .forward_with(anchor, move |prev| prev + PeriodComp::hours(clock_step))
            .backward_with(anchor - PeriodComp::hours(clock_step),
                           move |prev| prev - PeriodComp::hours(clock_step))
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Hour {
    pub quantity: u32,
    pub is_12_clock: bool,
}

impl Hour {
    pub fn clock_12<T: TimeZone>(quantity: u32) -> RcConstraint<T> where <T as TimeZone>::Offset: Copy {
        rc!(Hour {
            quantity: quantity,
            is_12_clock: true,
        })
    }

    pub fn clock_24<T: TimeZone>(quantity: u32) -> RcConstraint<T> where <T as TimeZone>::Offset: Copy {
        rc!(Hour {
            quantity: quantity,
            is_12_clock: false,
        })
    }
}

impl<T: TimeZone> IntervalConstraint<T> for Hour where <T as TimeZone>::Offset: Copy {
    fn grain(&self) -> Grain {
        Grain::Hour
    }

    fn coarse_grain_step(&self) -> Grain {
        Grain::Day
    }

    fn to_walker(&self, origin: &Interval<T>, _context: &Context<T>) -> IntervalWalker<T> {
        let clock_step = if self.quantity <= 12 && self.is_12_clock {
            12
        } else {
            24
        };
        let offset = (self.quantity as i64 - origin.start.hour() as i64 + clock_step) % clock_step;
        let anchor = origin.start_round_to(Grain::Hour) + PeriodComp::hours(offset);

        BidirectionalWalker::new()
            .forward_with(anchor, move |prev| prev + PeriodComp::hours(clock_step))
            .backward_with(anchor - PeriodComp::hours(clock_step),
                           move |prev| prev - PeriodComp::hours(clock_step))
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Minute(pub u32);

impl Minute {
    pub fn new<T: TimeZone>(m: u32) -> RcConstraint<T> where <T as TimeZone>::Offset: Copy {
        rc!(Minute(m))
    }
}

impl<T: TimeZone> IntervalConstraint<T> for Minute where <T as TimeZone>::Offset: Copy {
    fn grain(&self) -> Grain {
        Grain::Minute
    }

    fn coarse_grain_step(&self) -> Grain {
        Grain::Hour
    }

    fn to_walker(&self, origin: &Interval<T>, _context: &Context<T>) -> IntervalWalker<T> {
        let offset = (self.0 as i64 - origin.start.minute() as i64) % 60;
        let anchor = origin.start_round_to(Grain::Minute) + PeriodComp::minutes(offset);

        BidirectionalWalker::new()
            .forward_with(anchor, |prev| prev + PeriodComp::hours(1))
            .backward_with(anchor - PeriodComp::hours(1),
                           |prev| prev - PeriodComp::hours(1))
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Second(pub u32);

impl Second {
    pub fn new<T: TimeZone>(s: u32) -> RcConstraint<T> where <T as TimeZone>::Offset: Copy {
        rc!(Second(s))
    }
}

impl<T: TimeZone> IntervalConstraint<T> for Second where <T as TimeZone>::Offset: Copy {
    fn grain(&self) -> Grain {
        Grain::Second
    }

    fn coarse_grain_step(&self) -> Grain {
        Grain::Minute
    }

    fn to_walker(&self, origin: &Interval<T>, _context: &Context<T>) -> IntervalWalker<T> {
        let offset = (self.0 as i64 - origin.start.second() as i64 + 60) % 60;
        let anchor = origin.start_round_to(Grain::Second) + PeriodComp::seconds(offset);

        BidirectionalWalker::new()
            .forward_with(anchor, |prev| prev + PeriodComp::minutes(1))
            .backward_with(anchor - PeriodComp::minutes(1),
                           |prev| prev - PeriodComp::minutes(1))
    }
}



pub struct NthConstraint<T: TimeZone>(RcConstraint<T>, i64);

impl<T: TimeZone+'static> NthConstraint<T> where <T as TimeZone>::Offset: Copy {
    pub fn after(&self, inner: &RcConstraint<T>) -> RcConstraint<T> {
        TakeTheNthAfter::new(self.1, false, inner, &self.0)
    }

    pub fn after_not_immediate(&self, inner: &RcConstraint<T>) -> RcConstraint<T> {
        TakeTheNthAfter::new(self.1, true, inner, &self.0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Cycle(pub Grain);

impl Cycle {
    pub fn rc<T: TimeZone>(grain: Grain) -> RcConstraint<T> where <T as TimeZone>::Offset: Copy {
        rc!(Cycle(grain))
    }
}

impl<T: TimeZone> IntervalConstraint<T> for Cycle where <T as TimeZone>::Offset: Copy {
    fn grain(&self) -> Grain {
        self.0
    }

    fn coarse_grain_step(&self) -> Grain {
        self.0
    }

    fn to_walker(&self, origin: &Interval<T>, _context: &Context<T>) -> IntervalWalker<T> {
        let anchor = origin.start_round_to(self.0);
        let grain = self.0;
        BidirectionalWalker::new()
            .forward_with(anchor, move |prev| prev + PeriodComp::new(grain, 1))
            .backward_with(anchor - PeriodComp::new(grain, 1),
                           move |prev| prev - PeriodComp::new(grain, 1))
    }
}


#[derive(Clone)]
pub struct TakeTheNth<T: TimeZone> {
    n: i64,
    not_immediate: bool,
    inner: RcConstraint<T>,
}

impl<T: TimeZone + 'static> TakeTheNth<T> where <T as TimeZone>::Offset: Copy {
    pub fn new(n: i64, not_immediate: bool, inner: &RcConstraint<T>) -> RcConstraint<T> {
        rc!(TakeTheNth {
            n: n,
            not_immediate: not_immediate,
            inner: inner.clone(),
        })
    }
}

impl<T: TimeZone+'static> IntervalConstraint<T> for TakeTheNth<T> where <T as TimeZone>::Offset: Copy {
    fn grain(&self) -> Grain {
        self.inner.grain()
    }

    fn coarse_grain_step(&self) -> Grain {
        self.inner.coarse_grain_step()
    }

    fn to_walker(&self, origin: &Interval<T>, context: &Context<T>) -> IntervalWalker<T> {
        let base_interval = context.reference;
        let interval_walker = self.inner.to_walker(&base_interval, context);

        let match_interval: Option<Interval<T>> = if self.n >= 0 {
            let head = interval_walker.forward.clone().next();
            let mut forward_walker = if head.is_some() && self.not_immediate &&
                                        head.and_then(move |x| x.intersect(base_interval)).is_some() {
                interval_walker
                    .forward
                    .clone()
                    .skip((self.n + 1) as usize)
            } else {
                interval_walker.forward.clone().skip(self.n as usize)
            };
            forward_walker.next()
        } else {
            interval_walker
                .backward
                .clone()
                .skip((-(self.n + 1)) as usize)
                .next()
        };

        if let Some(interval) = match_interval {
            if origin.start < interval.end_moment() {
                BidirectionalWalker::new().forward_values(vec![interval])
            } else {
                BidirectionalWalker::new().backward_values(vec![interval])
            }
        } else {
            BidirectionalWalker::new()
        }
    }
}


#[derive(Clone)]
pub struct TakeN<T: TimeZone> {
    n: i64,
    not_immediate: bool,
    inner: RcConstraint<T>,
}

impl<T: TimeZone + 'static> TakeN<T> where <T as TimeZone>::Offset: Copy {
    pub fn new(n: i64, not_immediate: bool, inner: &RcConstraint<T>) -> RcConstraint<T> {
        rc!(TakeN {
            n: n,
            not_immediate: not_immediate,
            inner: inner.clone(),
        })
    }
}

impl<T: TimeZone + 'static> IntervalConstraint<T> for TakeN<T> where <T as TimeZone>::Offset: Copy {
    fn grain(&self) -> Grain {
        self.inner.grain()
    }

    fn coarse_grain_step(&self) -> Grain {
        self.inner.coarse_grain_step()
    }

    fn to_walker(&self, origin: &Interval<T>, context: &Context<T>) -> IntervalWalker<T> {
        let base_interval = context.reference;
        let interval_walker = self.inner.to_walker(&base_interval, context);

        let match_interval: Option<Interval<T>> = if self.n >= 0 {
            let head = interval_walker.forward.clone().next();
            let forward_walker = if head.is_some() && self.not_immediate &&
                                    head.and_then(move |x| x.intersect(base_interval)).is_some() {
                interval_walker.forward.skip(1)
            } else {
                interval_walker.forward
            };
            let start = forward_walker.clone().next();
            let end = forward_walker.clone().skip(self.n as usize).next();
            if let (Some(s), Some(e)) = (start, end) {
                Some(s.interval_to(e))
            } else {
                None
            }
        } else {
            let end = interval_walker.backward.clone().next();
            let start = interval_walker
                .backward
                .skip((-(self.n + 1)) as usize)
                .next();
            if let (Some(s), Some(e)) = (start, end) {
                Some(s.union(e))
            } else {
                None
            }
        };

        if let Some(interval) = match_interval {
            if origin.start < interval.end_moment() {
                BidirectionalWalker::new().forward_values(vec![interval])
            } else {
                BidirectionalWalker::new().backward_values(vec![interval])
            }
        } else {
            BidirectionalWalker::new()
        }
    }
}


#[derive(Clone)]
pub struct TakeTheNthAfter<T: TimeZone> {
    n: i64,
    not_immediate: bool,
    after: RcConstraint<T>,
    cycle: RcConstraint<T>,
}

impl<T: TimeZone + 'static> TakeTheNthAfter<T>  where <T as TimeZone>::Offset: Copy {
    pub fn new(n: i64, not_immediate: bool, after: &RcConstraint<T>, cycle: &RcConstraint<T>) -> RcConstraint<T> {
        rc!(TakeTheNthAfter {
            n: n,
            not_immediate: not_immediate,
            after: after.clone(),
            cycle: cycle.clone(),
        })
    }
}

impl<T: TimeZone + 'static> IntervalConstraint<T> for TakeTheNthAfter<T>  where <T as TimeZone>::Offset: Copy {
    fn grain(&self) -> Grain {
        self.after.grain()
    }

    fn coarse_grain_step(&self) -> Grain {
        self.after.coarse_grain_step()
    }

    fn to_walker(&self, origin: &Interval<T>, context: &Context<T>) -> IntervalWalker<T> {
        let cycle = self.cycle.clone();
        let n = self.n;
        let not_immediate = self.not_immediate;
        let translate = Translate {
            generator: self.after.clone(),
            offset: Rc::new(move |after: &Interval<T>, c: &Context<T>| -> Option<Interval<T>> {
                let walker = cycle.to_walker(after, c);
                if n >= 0 {
                    let head = walker.forward.clone().next();
                    if not_immediate && head.is_some() && head.map(|h| h.start < after.start).unwrap_or(false) {
                        walker.forward.skip((n + 1) as usize).next()
                    } else {
                        walker.forward.skip(n as usize).next()
                    }
                } else {
                    walker.backward.skip((-(n + 1)) as usize).next()
                }
            }),
        };
        translate.to_walker(&origin, context)
    }
}


#[derive(Clone)]
pub struct TakeLastOf<T: TimeZone> {
    base: RcConstraint<T>,
    cycle: RcConstraint<T>,
}

impl<T: TimeZone+'static> TakeLastOf<T>  where <T as TimeZone>::Offset: Copy {
    pub fn new(base: &RcConstraint<T>, cycle: &RcConstraint<T>) -> RcConstraint<T> {
        rc!(TakeLastOf {
            base: base.clone(),
            cycle: cycle.clone(),
        })
    }
}

impl<T: TimeZone+'static> IntervalConstraint<T> for TakeLastOf<T>  where <T as TimeZone>::Offset: Copy {
    fn grain(&self) -> Grain {
        self.base.grain()
    }

    fn coarse_grain_step(&self) -> Grain {
        self.base.coarse_grain_step()
    }

    fn to_walker(&self, origin: &Interval<T>, context: &Context<T>) -> IntervalWalker<T> {
        let cycle = self.cycle.clone();
        let translate = Translate {
            generator: self.base.clone(),
            offset: Rc::new(move |i: &Interval<T>, c: &Context<T>| -> Option<Interval<T>> {
                let pivot = i.after();
                let walker = cycle.to_walker(&pivot, c);
                walker.backward.clone().next()
            }),
        };
        translate.to_walker(&origin, context)
    }

}


#[derive(Clone)]
pub struct Intersection<T: TimeZone> {
    lhs: RcConstraint<T>,
    rhs: RcConstraint<T>,
}

impl<T: TimeZone+'static> Intersection<T>  where <T as TimeZone>::Offset: Copy {
    fn new(lhs: &RcConstraint<T>, rhs: &RcConstraint<T>) -> RcConstraint<T> {
        rc!(Intersection { lhs: lhs.clone(), rhs: rhs.clone() })
    }
}

impl<T: TimeZone+'static> IntervalConstraint<T> for Intersection<T>  where <T as TimeZone>::Offset: Copy {
    fn grain(&self) -> Grain {
        ::std::cmp::max(self.lhs.grain(), self.rhs.grain())
    }

    fn coarse_grain_step(&self) -> Grain {
        ::std::cmp::min(self.lhs.grain(), self.rhs.grain())
    }

    fn to_walker(&self, origin: &Interval<T>, context: &Context<T>) -> IntervalWalker<T> {

        fn walk_from<U: TimeZone+'static>(origin: &Interval<U>,
                     context: Context<U>,
                     constraint: RcConstraint<U>)
                     -> Walker<Interval<U>> where <U as TimeZone>::Offset: Copy 
        {
            let context = Context::new(context.reference, *origin, *origin);
            let max_moment = origin.end_moment();
            let origin_copied = origin.clone();
            constraint
                .to_walker(origin, &context)
                .forward
                .take(183)
                .take_while(move |i| i.start < max_moment)
                .filter_map(move |i| origin_copied.intersect(i))
        }

        fn combine<U: TimeZone+'static>(origin: &Interval<U>,
                                 context: Context<U>,
                                 fine: RcConstraint<U>,
                                 coarse: RcConstraint<U>)
                                 -> IntervalWalker<U> where <U as TimeZone>::Offset: Copy
        {
            let coarse_walker = coarse.to_walker(origin, &context);
            let max_moment = context.max.end_moment();
            let fine_for_walker = fine.clone();
            let fore = coarse_walker
                .forward
                .take_while(move |i| i.start <= max_moment)
                .take(183)
                .flat_map(move |i| walk_from(&i, context, fine_for_walker.clone()));
            let back = coarse_walker
                .backward
                .take_while(move |i| i.end_moment() >= context.min.start)
                .take(183)
                .flat_map(move |i| walk_from(&i, context, fine.clone()));
            IntervalWalker::new().forward(fore).backward(back)
        }

        if self.lhs.coarse_grain_step() <= self.rhs.coarse_grain_step() {
            combine(origin, *context, self.rhs.clone(), self.lhs.clone())
        } else {
            combine(origin, *context, self.lhs.clone(), self.rhs.clone())
        }
    }
}


#[derive(Clone)]
pub struct Translate<T: TimeZone> {
    generator: RcConstraint<T>,
    offset: Rc<Fn(&Interval<T>, &Context<T>) -> Option<Interval<T>>>,
}

impl<T: TimeZone+'static> Translate<T>  where <T as TimeZone>::Offset: Copy {
    pub fn new(generator: &RcConstraint<T>,
               offset: Rc<Fn(&Interval<T>, &Context<T>) -> Option<Interval<T>>>)
               -> RcConstraint<T> {
        rc!(Translate {
            generator: generator.clone(),
            offset: offset,
        })
    }
}


impl<T: TimeZone+'static> IntervalConstraint<T> for Translate<T>  where <T as TimeZone>::Offset: Copy {
    fn grain(&self) -> Grain {
        self.generator.grain()
    }

    fn coarse_grain_step(&self) -> Grain {
        self.generator.coarse_grain_step()
    }

    fn to_walker(&self, origin: &Interval<T>, context: &Context<T>) -> IntervalWalker<T> {
        let generator_walker = self.generator.to_walker(origin, context);
        let context = *context;

        let offset = self.offset.clone();
        let origin = *origin;
        let prepend_to_fore = generator_walker
            .backward
            .take(12)
            .filter_map(move |i| offset(&i, &context))
            .take_while(move |i| origin.start <= i.end_moment());
        let mut prepend_to_fore: Vec<Interval<T>> = prepend_to_fore.into_iter().collect();
        prepend_to_fore.reverse();

        let offset = self.offset.clone();
        let still_fore = generator_walker
            .forward
            .take(12)
            .filter_map(move |i| offset(&i, &context))
            .skip_while(move |i| origin.start > i.end_moment())
            .take_while(move |i| i.start <= context.max.end_moment());

        let offset = self.offset.clone();
        let prepend_to_back = generator_walker
            .forward
            .take(12)
            .filter_map(move |i| offset(&i, &context))
            .take_while(move |i| origin.start > i.end_moment());
        let mut prepend_to_back: Vec<Interval<T>> = prepend_to_back.into_iter().collect();
        prepend_to_back.reverse();

        let offset = self.offset.clone();
        let still_back = generator_walker
            .backward
            .take(12)
            .filter_map(move |i| offset(&i, &context))
            .skip_while(move |i| origin.start <= i.end_moment())
            .take_while(move |i| context.min.start <= i.end_moment());

        IntervalWalker::new()
            .forward(Walker::vec(prepend_to_fore).chain(&still_fore))
            .backward(Walker::vec(prepend_to_back).chain(&still_back))
    }
}


#[derive(Clone)]
pub struct Span<T: TimeZone> {
    from: RcConstraint<T>,
    to: RcConstraint<T>,
    inclusive: bool,
}

impl<T: TimeZone+'static> Span<T>  where <T as TimeZone>::Offset: Copy {
    pub fn new(from: &RcConstraint<T>, to: &RcConstraint<T>, inclusive: bool) -> RcConstraint<T> {
        rc!(Span { from: from.clone(), to: to.clone(), inclusive })
    }
}

impl<T: TimeZone+'static> IntervalConstraint<T> for Span<T>  where <T as TimeZone>::Offset: Copy {
    fn grain(&self) -> Grain {
        ::std::cmp::max(self.from.grain(), self.to.grain())
    }

    fn coarse_grain_step(&self) -> Grain {
        self.from.coarse_grain_step()
    }

    fn to_walker(&self, origin: &Interval<T>, context: &Context<T>) -> IntervalWalker<T> {
        let inclusive = self.inclusive;
        let to = self.to.clone();
        let translate = Translate {
            generator: self.from.clone(),
            offset: Rc::new(move |start: &Interval<T>, c: &Context<T>| -> Option<Interval<T>> {
                to.to_walker(start, c)
                    .forward
                    .next()
                    .map(|end| {
                        if inclusive {
                             start.union(end)
                         } else {
                             start.interval_to(end)
                         }
                     })
            }),
        };
        translate.to_walker(origin, context)
    }
}

#[derive(Clone)]
pub struct ShiftBy<T: TimeZone> {
    base: RcConstraint<T>,
    period: Period,
}

impl<T: TimeZone+'static> ShiftBy<T>  where <T as TimeZone>::Offset: Copy {
    pub fn new(base: &RcConstraint<T>, period: Period) -> RcConstraint<T> {
        rc!(ShiftBy { base:base.clone(), period })
    }
}

impl<T: TimeZone+'static> IntervalConstraint<T> for ShiftBy<T>  where <T as TimeZone>::Offset: Copy {
    fn grain(&self) -> Grain {
        self.base.grain()
    }

    fn coarse_grain_step(&self) -> Grain {
        self.base.coarse_grain_step()
    }

    fn to_walker(&self, origin: &Interval<T>, context: &Context<T>) -> IntervalWalker<T> {
        if let Some(period_grain) = self.period.finer_grain() {
            let period = self.period.clone();
            let next_grain = period_grain.next();
            let translate = Translate {
                generator: self.base.clone(),
                offset: Rc::new(move |i: &Interval<T>, _: &Context<T>| -> Option<Interval<T>> {
                    Some(i.interval_round_to(next_grain) + &period)
                }),
            };
            translate.to_walker(origin, context)
        } else { // If no finer grain -> empty period
            self.base.to_walker(origin, context)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Weekday, FixedOffset, NaiveDate, NaiveDateTime, LocalResult};
    use ::*;

    fn build_context(moment: Moment<Paris>) -> Context<Paris> {
        let now = Interval::starting_at(moment, Grain::Second);

        Context::for_reference(now)
    }

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
    fn test_year() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let year = Year(2015);
        let walker = year.to_walker(&context.reference, &context);
        let mut backward = walker.backward.clone();
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2015, 1, 1).and_hms(0, 0, 0)),
                                              Grain::Year)),
                   backward.next());

        assert_eq!(None, backward.next());
        assert_eq!(None, walker.forward.clone().next());

        let year = Year(2018);
        let walker = year.to_walker(&context.reference, &context);
        assert_eq!(None, walker.backward.clone().next());

        let walker = year.to_walker(&context.reference, &context);
        let mut forward = walker.forward.clone();
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2018, 1, 1).and_hms(0, 0, 0)),
                                              Grain::Year)),
                   forward.next());
        assert_eq!(None, forward.next());

        assert_eq!(None, walker.backward.clone().next());
    }

       
    #[test]
    fn test_year_month_day() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let ymd = YearMonthDay::new(2015, 6, 5);
        let walker = ymd.to_walker(&context.reference, &context);
        let mut backward = walker.backward.clone();
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2015, 6, 5).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   backward.next());

        assert_eq!(None, backward.next());
        assert_eq!(None, walker.forward.clone().next());

        let ymd = YearMonthDay::new(2018, 6, 5);
        let walker = ymd.to_walker(&context.reference, &context);
        assert_eq!(None, walker.backward.clone().next());

        let mut forward = walker.forward.clone();
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2018, 6, 5).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   forward.next());
        assert_eq!(None, forward.next());
        assert_eq!(None, walker.backward.clone().next());

        let ymd = YearMonthDay::new(2018, 2, 30);
        let walker = ymd.to_walker(&context.reference, &context);
        assert_eq!(None, walker.backward.clone().next());
        assert_eq!(None, walker.forward.clone().next());
    }


    #[test]
    fn test_cycle() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let cycle = Cycle(Grain::Day);
        let walker = cycle.to_walker(&context.reference, &context);
        let mut backward = walker.backward.clone();
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 24).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   backward.next());
        let mut forward = walker.forward.clone();
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 25).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   forward.next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 26).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   forward.next());
    }

    #[test]
    fn test_take_the_nth_forward_positive() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let take_the_nth = Month::new(5).take_the_nth(3);
        let walker = take_the_nth.to_walker(&context.reference, &context);
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2020, 05, 1).and_hms(0, 0, 0)),
                                              Grain::Month)),
                   walker.forward.clone().next());
        assert_eq!(None, walker.forward.clone().skip(1).next());
        assert_eq!(None, walker.backward.clone().next());

    }

    

    #[test]
    fn test_take_the_nth_backward_positive() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let take_the_nth = Month::new(3).take_the_nth(3);
        let walker = take_the_nth.to_walker(&context.reference, &context);
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2021, 03, 1).and_hms(0, 0, 0)),
                                              Grain::Month)),
                   walker.forward.clone().next());
        assert_eq!(None, walker.forward.clone().skip(1).next());
        assert_eq!(None, walker.backward.clone().next());
    }

    #[test]
    fn test_take_the_nth_backward_negative() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let take_the_nth = Month::new(3).take_the_nth(-3);
        let walker = take_the_nth.to_walker(&context.reference, &context);
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2015, 03, 1).and_hms(0, 0, 0)),
                                              Grain::Month)),
                   walker.backward.clone().next());
        assert_eq!(None, walker.backward.clone().skip(1).next());
        assert_eq!(None, walker.forward.clone().next());
    }

    
    #[test]
    fn test_take_the_nth_after_positive() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let take_the_nth_after = Cycle::rc(Grain::Day).the_nth(3).after(&rc!(DayOfMonth(20)));

        let walker = take_the_nth_after.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 05, 23).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 06, 23).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 23).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 03, 23).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().skip(1).next());
    }

    
    #[test]
    fn test_take_the_nth_after_negative() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let take_the_nth_after = Cycle::rc(Grain::Day).the_nth(-3).after(&rc!(DayOfMonth(20)));

        let walker = take_the_nth_after.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 05, 17).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 06, 17).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 17).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 03, 17).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_take_the_last_week_of_month() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let take_last_of = Cycle::rc(Grain::Week).last_of(&rc!(Month(5)));

        let walker = take_last_of.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 05, 22).and_hms(0, 0, 0)),
                                              Grain::Week)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2018, 05, 21).and_hms(0, 0, 0)),
                                              Grain::Week)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2016, 05, 23).and_hms(0, 0, 0)),
                                              Grain::Week)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2015, 05, 25).and_hms(0, 0, 0)),
                                              Grain::Week)),
                   walker.backward.clone().skip(1).next());
    }

    
    #[test]
    fn test_take_the_last_day_of_month() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let take_last_of = Cycle::rc(Grain::Day).last_of(&rc!(Month(5)));

        let walker = take_last_of.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 05, 31).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2018, 05, 31).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2016, 05, 31).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2015, 05, 31).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().skip(1).next());
    }

    
    #[test]
    fn test_month_day_above_current_month() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let month = MonthDay(5, 10);
        let walker = month.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 05, 10).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2018, 05, 10).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2016, 05, 10).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2015, 05, 10).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().skip(1).next());
    }


    #[test]
    fn test_month_day_under_current_month() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let month = MonthDay(3, 20);
        let walker = month.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2018, 03, 20).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2019, 03, 20).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 03, 20).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2016, 03, 20).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_month_above_current_month() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let month = Month(5);
        let walker = month.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 05, 1).and_hms(0, 0, 0)),
                                              Grain::Month)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2018, 05, 1).and_hms(0, 0, 0)),
                                              Grain::Month)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2016, 05, 1).and_hms(0, 0, 0)),
                                              Grain::Month)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2015, 05, 1).and_hms(0, 0, 0)),
                                              Grain::Month)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_month_under_current_month() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let month = Month(3);
        let walker = month.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2018, 03, 1).and_hms(0, 0, 0)),
                                              Grain::Month)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2019, 03, 1).and_hms(0, 0, 0)),
                                              Grain::Month)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 03, 1).and_hms(0, 0, 0)),
                                              Grain::Month)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2016, 03, 1).and_hms(0, 0, 0)),
                                              Grain::Month)),
                   walker.backward.clone().skip(1).next());
    }


    #[test]
    fn test_day_of_month_backward() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day_of_month = DayOfMonth(10);
        let walker = day_of_month.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 05, 10).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 06, 10).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 10).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 03, 10).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().skip(1).next());
    }


    #[test]
    fn test_day_of_month_forward() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day_of_month = DayOfMonth(31);
        let walker = day_of_month.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 05, 31).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 07, 31).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 03, 31).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 01, 31).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().skip(1).next());
    }


    #[test]
    fn test_day_of_week_after() {
        // Day of week => Tuesday
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day_of_week = DayOfWeek(Weekday::Wed);
        let walker = day_of_week.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 26).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 05, 03).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 19).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 12).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().skip(1).next());
    }


    #[test]
    fn test_day_of_week_before() {
        // Day of week => Tuesday
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day_of_week = DayOfWeek(Weekday::Mon);
        let walker = day_of_week.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 05, 01).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 05, 08).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 24).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 17).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_hour_minute_24_clock_under_12() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day = HourMinute::clock_24(11, 24);
        let walker = day.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 25).and_hms(11, 24, 0)),
                                              Grain::Minute)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 26).and_hms(11, 24, 0)),
                                              Grain::Minute)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 24).and_hms(11, 24, 0)),
                                              Grain::Minute)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 23).and_hms(11, 24, 0)),
                                              Grain::Minute)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_hour_minute_24_clock_above_12() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day = HourMinute::clock_24(15, 24);
        let walker = day.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 25).and_hms(15, 24, 0)),
                                              Grain::Minute)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 26).and_hms(15, 24, 0)),
                                              Grain::Minute)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 24).and_hms(15, 24, 0)),
                                              Grain::Minute)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 23).and_hms(15, 24, 0)),
                                              Grain::Minute)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_hour_24_clock_under_12() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day = Hour::clock_24(11);
        let walker = day.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 25).and_hms(11, 0, 0)),
                                              Grain::Hour)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 26).and_hms(11, 0, 0)),
                                              Grain::Hour)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 24).and_hms(11, 0, 0)),
                                              Grain::Hour)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 23).and_hms(11, 0, 0)),
                                              Grain::Hour)),
                   walker.backward.clone().skip(1).next());
    }


    #[test]
    fn test_hour_24_clock_above_12() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day = Hour::clock_24(15);
        let walker = day.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 25).and_hms(15, 0, 0)),
                                              Grain::Hour)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 26).and_hms(15, 0, 0)),
                                              Grain::Hour)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 24).and_hms(15, 0, 0)),
                                              Grain::Hour)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 23).and_hms(15, 0, 0)),
                                              Grain::Hour)),
                   walker.backward.clone().skip(1).next());
    }


    #[test]
    fn test_hour_24_clock_under_current_hour() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day = Hour::clock_24(4);
        let walker = day.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 26).and_hms(4, 0, 0)),
                                              Grain::Hour)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 27).and_hms(4, 0, 0)),
                                              Grain::Hour)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 25).and_hms(4, 0, 0)),
                                              Grain::Hour)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 24).and_hms(4, 0, 0)),
                                              Grain::Hour)),
                   walker.backward.clone().skip(1).next());
    }


    #[test]
    fn test_hour_12_clock_under_12() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day = Hour::clock_12(11);
        let walker = day.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 25).and_hms(11, 0, 0)),
                                              Grain::Hour)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 25).and_hms(23, 0, 0)),
                                              Grain::Hour)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 24).and_hms(23, 0, 0)),
                                              Grain::Hour)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 24).and_hms(11, 0, 0)),
                                              Grain::Hour)),
                   walker.backward.clone().skip(1).next());
    }


    #[test]
    fn test_hour_12_clock_above_12() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day = Hour::clock_12(14);
        let walker = day.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 25).and_hms(14, 0, 0)),
                                              Grain::Hour)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 26).and_hms(14, 0, 0)),
                                              Grain::Hour)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 24).and_hms(14, 0, 0)),
                                              Grain::Hour)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 23).and_hms(14, 0, 0)),
                                              Grain::Hour)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_hour_12_clock_under_current_hour() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day = Hour::clock_12(8);
        let walker = day.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 25).and_hms(20, 0, 0)),
                                              Grain::Hour)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 26).and_hms(8, 0, 0)),
                                              Grain::Hour)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 25).and_hms(8, 0, 0)),
                                              Grain::Hour)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 24).and_hms(20, 0, 0)),
                                              Grain::Hour)),
                   walker.backward.clone().skip(1).next());
    }


    #[test]
    fn test_minute_above_current_minute() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day = Minute(15);
        let walker = day.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 15, 0)),
                                              Grain::Minute)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 25).and_hms(10, 15, 0)),
                                              Grain::Minute)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 25).and_hms(8, 15, 0)),
                                              Grain::Minute)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 25).and_hms(7, 15, 0)),
                                              Grain::Minute)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_minute_under_current_minute() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day = Minute(5);
        let walker = day.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 5, 0)),
                                              Grain::Minute)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 25).and_hms(10, 5, 0)),
                                              Grain::Minute)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 25).and_hms(8, 5, 0)),
                                              Grain::Minute)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 25).and_hms(7, 5, 0)),
                                              Grain::Minute)),
                   walker.backward.clone().skip(1).next());
    }


    #[test]
    fn test_second_above_current_second() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day = Second(30);
        let walker = day.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 30)),
                                              Grain::Second)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 11, 30)),
                                              Grain::Second)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 9, 30)),
                                              Grain::Second)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 8, 30)),
                                              Grain::Second)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_intersect_dom_month() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));

        let inter = DayOfMonth::new(12).intersect(&rc!(Month(3)));
        let walker = inter.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2018, 03, 12).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2019, 03, 12).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 03, 12).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2016, 03, 12).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_intersect_dow_dom_month() {
        let context = build_context(Moment(Paris.ymd(2013, 02, 12).and_hms(4, 30, 0)));

        let weekday = rc!(DayOfWeek(Weekday::Mon));
        let month_day = DayOfMonth::new(1).intersect(&rc!(Month(11)));
        let inter = weekday.intersect(&month_day);
        let walker = inter.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2021, 11, 1).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2027, 11, 1).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2010, 11, 1).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2004, 11, 1).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().skip(1).next());
    }

    
    #[test]
    fn test_intersect_dow_dom() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));

        let inter = DayOfMonth::new(12).intersect(&rc!(DayOfWeek(Weekday::Wed)));
        let walker = inter.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 07, 12).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2018, 09, 12).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 12).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2016, 10, 12).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().skip(1).next());
    }

    
    #[test]
    fn test_translate_date_by_days_fore() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));

        // every 32th day after the 12 of any month
        fn offset(i: &Interval<Paris>, _: &Context<Paris>) -> Option<Interval<Paris>> {
            Some(*i + PeriodComp::days(32))
        }
        let walker = DayOfMonth::new(12).translate_with(offset)
                .to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 05, 14).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 06, 13).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 13).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 03, 16).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_translate_date_by_days_back() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));

        // every 32th day before the 12 of any month
        fn offset(i: &Interval<Paris>, _: &Context<Paris>) -> Option<Interval<Paris>> {
            Some(*i - PeriodComp::days(32))
        }
        let walker = DayOfMonth::new(12).translate_with(offset).to_walker(&context.reference,
                                                                             &context);

        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 05, 11).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 06, 10).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 10).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 03, 11).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_translate_date_by_days_back_() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));

        fn offset(i: &Interval<Paris>, _: &Context<Paris>) -> Option<Interval<Paris>> {
            Some(*i + PeriodComp::days(100))
        }
        let walker = DayOfMonth::new(12).translate_with(offset).to_walker(&context.reference,
                                                                             &context);

        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 05, 23).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 06, 20).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().skip(1).next());
    }

    #[test]
    fn test_inclusive_span() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let walker = DayOfWeek::new(Weekday::Mon).span_to(&rc!(DayOfWeek(Weekday::Wed)))
            .to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::new(Moment(Paris.ymd(2017, 04, 24).and_hms(0, 0, 0)),
                                      Some(Moment(Paris.ymd(2017, 04, 26).and_hms(0, 0, 0))),
                                      Grain::Day)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::new(Moment(Paris.ymd(2017, 05, 01).and_hms(0, 0, 0)),
                                      Some(Moment(Paris.ymd(2017, 05, 03).and_hms(0, 0, 0))),
                                      Grain::Day)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::new(Moment(Paris.ymd(2017, 04, 17).and_hms(0, 0, 0)),
                                      Some(Moment(Paris.ymd(2017, 04, 19).and_hms(0, 0, 0))),
                                      Grain::Day)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::new(Moment(Paris.ymd(2017, 04, 10).and_hms(0, 0, 0)),
                                      Some(Moment(Paris.ymd(2017, 04, 12).and_hms(0, 0, 0))),
                                      Grain::Day)),
                   walker.backward.clone().skip(1).next());
    }

    
    #[test]
    fn test_shift_by_days() {
        let context = build_context(Moment(Paris.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let walker = DayOfMonth::new(12).shift_by(PeriodComp::days(2).into())
            .to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 05, 14).and_hms(0, 0, 0)),
                                              Grain::Hour)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 06, 14).and_hms(0, 0, 0)),
                                              Grain::Hour)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 04, 14).and_hms(0, 0, 0)),
                                              Grain::Hour)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Paris.ymd(2017, 03, 14).and_hms(0, 0, 0)),
                                              Grain::Hour)),
                   walker.backward.clone().skip(1).next());

    }
}
