use std::rc::Rc;

use bidirectional_walker::*;
use walker::*;
use {Moment, Interval, last_day_in_month};
use period::*;
use std::ops;
use chrono::offset::local::Local;
use chrono::{Datelike, TimeZone, Timelike, Weekday};

#[derive(Debug, Copy, Clone, PartialEq, new)]
pub struct Context {
    reference: Interval,
    min: Interval,
    max: Interval,
}

impl Context {
    pub fn now() -> Context {
        Context::for_reference(Interval::starting_at(Moment::now(), Grain::Second))
    }

    pub fn for_reference(now: Interval) -> Context {
        Context::new(now,
                     now - PeriodComp::years(50),
                     now + PeriodComp::years(50))
    }
}

pub type IntervalWalker = BidirectionalWalker<Interval>;

pub trait IntervalConstraint {
    fn grain(&self) -> Grain;
    fn to_walker(&self, origin: &Interval, _context: &Context) -> IntervalWalker;
}

#[derive(Clone)]
pub struct RcConstraint(pub Rc<IntervalConstraint>);

impl ops::Deref for RcConstraint {
    type Target = Rc<IntervalConstraint>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<C: IntervalConstraint + 'static> From<C> for RcConstraint {
    fn from(v: C) -> RcConstraint {
        RcConstraint(Rc::new(v))
    }
}

impl RcConstraint {
    pub fn shift_by(&self, period: Period) -> RcConstraint {
        ShiftBy::new(self, period)
    }

    pub fn translate_with<Offset>(&self, offset: Offset) -> RcConstraint
        where Offset: Fn(&Interval, &Context) -> Option<Interval> + 'static {
        Translate::new(self, Rc::new(offset))
    }

    pub fn take_the_nth(&self, n: i64) -> RcConstraint {
       TakeTheNth::new(n, false, self)
    }

    pub fn take_the_nth_not_immediate(&self, n: i64) -> RcConstraint {
       TakeTheNth::new(n, true, self)
    }

    pub fn take(&self, n: i64) -> RcConstraint {
       TakeN::new(n, false, self)
    }

    pub fn take_not_immediate(&self, n: i64) -> RcConstraint {
       TakeN::new(n, true, self)
    }

    pub fn span_to(&self, inner: &RcConstraint) -> RcConstraint {
       Span::new(self, inner, false)
    }

    pub fn span_inclusive_to(&self, inner: &RcConstraint)  -> RcConstraint {
       Span::new(self, inner, true)
    }

    pub fn intersect(&self, inner: &RcConstraint) -> RcConstraint {
        Intersection::new(self, inner)
    }

    pub fn last_of(&self, inner: &RcConstraint) -> RcConstraint {
        TakeLastOf::new(inner, self)
    }

    pub fn the_nth(&self, n: i64) -> NthConstraint {
        NthConstraint(self.clone(), n)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Year(pub i32);

impl Year {
    pub fn new(y: i32) -> RcConstraint {
        Year(y).into()
    }
}

impl IntervalConstraint for Year {
    fn grain(&self) -> Grain {
        Grain::Year
    }

    fn to_walker(&self, origin: &Interval, _context: &Context) -> IntervalWalker {
        let normalized_year = if self.0 < 99 {
            (self.0 + 50) % 100 + 2000 - 50
        } else {
            self.0
        };

        if origin.start.year() <= normalized_year {
            let moment_year = Moment(Local.ymd(normalized_year, 1, 1).and_hms(0, 0, 0));
            let interval = Interval::starting_at(moment_year, Grain::Year);
            BidirectionalWalker::new().forward_values(vec![interval])
        } else {
            let moment_year = Moment(Local.ymd(normalized_year, 1, 1).and_hms(0, 0, 0));
            let interval = Interval::starting_at(moment_year, Grain::Year);
            BidirectionalWalker::new().backward_values(vec![interval])
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Month(pub u32);

impl Month {
    pub fn new(m: u32) -> RcConstraint {
        Month(m).into()
    }
}

impl IntervalConstraint for Month {
    fn grain(&self) -> Grain {
        Grain::Month
    }

    fn to_walker(&self, origin: &Interval, _context: &Context) -> IntervalWalker {
        let rounded_moment = Moment(Local
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
    pub fn new(dom: u32) -> RcConstraint {
        DayOfMonth(dom).into()
    }
}

impl IntervalConstraint for DayOfMonth {
    fn grain(&self) -> Grain {
        Grain::Day
    }

    fn to_walker(&self, origin: &Interval, _context: &Context) -> IntervalWalker {
        let offset_month = (origin.start.0.day() > self.0) as i64;
        let anchor = origin.round_to(Grain::Month) + PeriodComp::months(offset_month);

        let day_of_month = self.0;
        let forward_walker =
            Walker::generator(anchor, |prev| prev + PeriodComp::months(1))
                .filter(move |interval| {
                            day_of_month <=
                            last_day_in_month(interval.start.year(), interval.start.month())
                        })
                .map(move |interval| interval + PeriodComp::days(day_of_month as i64 - 1));

        let backward_walker =
            Walker::generator(anchor - PeriodComp::months(1),
                              |prev| prev - PeriodComp::months(1))
                    .filter(move |interval| {
                                day_of_month <=
                                last_day_in_month(interval.start.year(), interval.start.month())
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
    pub fn new(dow: Weekday) -> RcConstraint {
        DayOfWeek(dow).into()
    }
}

impl IntervalConstraint for DayOfWeek {
    fn grain(&self) -> Grain {
        Grain::Day
    }

    fn to_walker(&self, origin: &Interval, _context: &Context) -> IntervalWalker {
        // number_from_monday is u32 -> use i64
        let offset = (self.0.number_from_monday() as i64 -
                      origin.start.weekday().number_from_monday() as i64) % 7;
        let anchor = origin.round_to(Grain::Day) + PeriodComp::days(offset);

        BidirectionalWalker::new()
            .forward_with(anchor, |prev| prev + PeriodComp::weeks(1))
            .backward_with(anchor - PeriodComp::weeks(1),
                           |prev| prev - PeriodComp::weeks(1))
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Hour {
    pub quantity: u32,
    pub is_12_clock: bool,
}

impl Hour {
    pub fn clock_12(quantity: u32) -> RcConstraint {
        Hour {
            quantity: quantity,
            is_12_clock: true,
        }.into()
    }

    pub fn clock_24(quantity: u32) -> RcConstraint {
        Hour {
            quantity: quantity,
            is_12_clock: false,
        }.into()
    }
}

impl IntervalConstraint for Hour {
    fn grain(&self) -> Grain {
        Grain::Hour
    }

    fn to_walker(&self, origin: &Interval, _context: &Context) -> IntervalWalker {
        let clock_step = if self.quantity <= 12 && self.is_12_clock {
            12
        } else {
            24
        };
        let offset = (self.quantity as i64 - origin.start.hour() as i64) % clock_step;
        let anchor = origin.round_to(Grain::Hour) + PeriodComp::hours(offset);

        BidirectionalWalker::new()
            .forward_with(anchor, move |prev| prev + PeriodComp::hours(clock_step))
            .backward_with(anchor - PeriodComp::hours(clock_step),
                           move |prev| prev - PeriodComp::hours(clock_step))
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Minute(pub u32);

impl Minute {
    pub fn new(m: u32) -> RcConstraint {
        Minute(m).into()
    }
}

impl IntervalConstraint for Minute {
    fn grain(&self) -> Grain {
        Grain::Minute
    }
    fn to_walker(&self, origin: &Interval, _context: &Context) -> IntervalWalker {
        let offset = (self.0 as i64 - origin.start.minute() as i64) % 60;
        let anchor = origin.round_to(Grain::Minute) + PeriodComp::minutes(offset);

        BidirectionalWalker::new()
            .forward_with(anchor, |prev| prev + PeriodComp::hours(1))
            .backward_with(anchor - PeriodComp::hours(1),
                           |prev| prev - PeriodComp::hours(1))
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Second(pub u32);

impl Second {
    pub fn new(s: u32) -> RcConstraint {
        Second(s).into()
    }
}

impl IntervalConstraint for Second {
    fn grain(&self) -> Grain {
        Grain::Second
    }

    fn to_walker(&self, origin: &Interval, _context: &Context) -> IntervalWalker {
        let offset = (self.0 as i64 - origin.start.second() as i64) % 60;
        let anchor = origin.round_to(Grain::Second) + PeriodComp::seconds(offset);

        BidirectionalWalker::new()
            .forward_with(anchor, |prev| prev + PeriodComp::minutes(1))
            .backward_with(anchor - PeriodComp::minutes(1),
                           |prev| prev - PeriodComp::minutes(1))
    }
}

pub struct NthConstraint(RcConstraint, i64);

impl NthConstraint {
    pub fn after(&self, inner: &RcConstraint) -> RcConstraint {
        TakeTheNthAfter::new(self.1, false, inner, &self.0)
    }

    pub fn after_not_immediate(&self, inner: &RcConstraint) -> RcConstraint {
        TakeTheNthAfter::new(self.1, true, inner, &self.0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Cycle(pub Grain);

impl Cycle {
    pub fn rc(grain: Grain) -> RcConstraint {
        Cycle(grain).into()
    }
}

impl IntervalConstraint for Cycle {
    fn grain(&self) -> Grain {
        self.0
    }

    fn to_walker(&self, origin: &Interval, _context: &Context) -> IntervalWalker {
        let anchor = origin.round_to(self.0);
        let grain = self.0;
        BidirectionalWalker::new()
            .forward_with(anchor, move |prev| prev + PeriodComp::new(grain, 1))
            .backward_with(anchor - PeriodComp::new(grain, 1),
                           move |prev| prev - PeriodComp::new(grain, 1))
    }
}

#[derive(Clone)]
pub struct TakeTheNth {
    n: i64,
    not_immediate: bool,
    inner: RcConstraint,
}

impl TakeTheNth {
    pub fn new(n: i64, not_immediate: bool, inner: &RcConstraint) -> RcConstraint {
        TakeTheNth {
            n: n,
            not_immediate: not_immediate,
            inner: inner.clone(),
        }.into()
    }
}

impl IntervalConstraint for TakeTheNth {
    fn grain(&self) -> Grain {
        self.inner.grain()
    }

    fn to_walker(&self, origin: &Interval, context: &Context) -> IntervalWalker {
        let base_interval = context.reference;
        let interval_walker = self.inner.to_walker(&base_interval, context);

        let match_interval: Option<Interval> = if self.n >= 0 {
            let head = interval_walker.forward.clone().next();
            let mut forward_walker = if head.is_some() && self.not_immediate &&
                                        head.map(move |x| x.intersect(base_interval)).is_some() {
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
pub struct TakeN {
    n: i64,
    not_immediate: bool,
    inner: RcConstraint,
}

impl TakeN {
    pub fn new(n: i64, not_immediate: bool, inner: &RcConstraint) -> RcConstraint {
        TakeN {
            n: n,
            not_immediate: not_immediate,
            inner: inner.clone(),
        }.into()
    }
}

impl IntervalConstraint for TakeN {
    fn grain(&self) -> Grain {
        self.inner.grain()
    }

    fn to_walker(&self, origin: &Interval, context: &Context) -> IntervalWalker {
        let base_interval = context.reference;
        let interval_walker = self.inner.to_walker(&base_interval, context);

        let match_interval: Option<Interval> = if self.n >= 0 {
            let head = interval_walker.forward.clone().next();
            let forward_walker = if head.is_some() && self.not_immediate &&
                                    head.map(move |x| x.intersect(base_interval)).is_some() {
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
pub struct TakeTheNthAfter {
    n: i64,
    not_immediate: bool,
    after: RcConstraint,
    cycle: RcConstraint,
}

impl TakeTheNthAfter {
    pub fn new(n: i64, not_immediate: bool, after: &RcConstraint, cycle: &RcConstraint) -> RcConstraint {
        TakeTheNthAfter {
            n: n,
            not_immediate: not_immediate,
            after: after.clone(),
            cycle: cycle.clone(),
        }.into()
    }
}

impl IntervalConstraint for TakeTheNthAfter {
    fn grain(&self) -> Grain {
        self.after.grain()
    }

    fn to_walker(&self, origin: &Interval, context: &Context) -> IntervalWalker {
        let cycle = self.cycle.clone();
        let n = self.n;
        let not_immediate = self.not_immediate;
        let translate = Translate {
            generator: self.after.clone(),
            offset: Rc::new(move |after: &Interval, c: &Context| -> Option<Interval> {
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
pub struct TakeLastOf {
    base: RcConstraint,
    cycle: RcConstraint,
}

impl TakeLastOf {
    pub fn new(base: &RcConstraint, cycle: &RcConstraint) -> RcConstraint {
        TakeLastOf {
            base: base.clone(),
            cycle: cycle.clone(),
        }.into()
    }
}

impl IntervalConstraint for TakeLastOf {
    fn grain(&self) -> Grain {
        self.base.grain()
    }

    fn to_walker(&self, origin: &Interval, context: &Context) -> IntervalWalker {
        let cycle = self.cycle.clone();
        let translate = Translate {
            generator: self.base.clone(),
            offset: Rc::new(move |i: &Interval, c: &Context| -> Option<Interval> {
                let pivot = i.after();
                let walker = cycle.to_walker(&pivot, c);
                walker.backward.clone().next()
            }),
        };
        translate.to_walker(&origin, context)
    }

}

#[derive(Clone)]
pub struct Intersection {
    lhs: RcConstraint,
    rhs: RcConstraint,
}

impl Intersection {
    fn new(lhs: &RcConstraint, rhs: &RcConstraint) -> RcConstraint {
        Intersection { lhs: lhs.clone(), rhs: rhs.clone() }.into()
    }
}

impl IntervalConstraint for Intersection {
    fn grain(&self) -> Grain {
        ::std::cmp::max(self.lhs.grain(), self.rhs.grain())
    }

    fn to_walker(&self, origin: &Interval, context: &Context) -> IntervalWalker {

        fn walk_from(origin: &Interval,
                     context: Context,
                     constraint: RcConstraint)
                     -> Walker<Interval> {
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

        fn combine(origin: &Interval,
                                 context: Context,
                                 fine: RcConstraint,
                                 coarse: RcConstraint)
                                 -> IntervalWalker
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

        if self.lhs.grain() < self.rhs.grain() {
            combine(origin, *context, self.rhs.clone(), self.lhs.clone())
        } else {
            combine(origin, *context, self.lhs.clone(), self.rhs.clone())
        }
    }
}

#[derive(Clone)]
pub struct Translate {
    generator: RcConstraint,
    offset: Rc<Fn(&Interval, &Context) -> Option<Interval>>,
}

impl Translate{
    pub fn new(generator: &RcConstraint,
               offset: Rc<Fn(&Interval, &Context) -> Option<Interval>>)
               -> RcConstraint {
        Translate {
            generator: generator.clone(),
            offset: offset,
        }.into()
    }
}


impl IntervalConstraint for Translate {
    fn grain(&self) -> Grain {
        self.generator.grain()
    }

    fn to_walker(&self, origin: &Interval, context: &Context) -> IntervalWalker {
        let generator_walker = self.generator.to_walker(origin, context);
        let context = *context;

        let offset = self.offset.clone();
        let origin = *origin;
        let prepend_to_fore = generator_walker
            .backward
            .take(12)
            .filter_map(move |i| offset(&i, &context))
            .take_while(move |i| origin.start <= i.end_moment());
        let mut prepend_to_fore: Vec<Interval> = prepend_to_fore.into_iter().collect();
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
        let mut prepend_to_back: Vec<Interval> = prepend_to_back.into_iter().collect();
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
pub struct Span {
    from: RcConstraint,
    to: RcConstraint,
    inclusive: bool,
}

impl Span {
    pub fn new(from: &RcConstraint, to: &RcConstraint, inclusive: bool) -> RcConstraint {
        Span { from: from.clone(), to: to.clone(), inclusive }.into()
    }
}

impl IntervalConstraint for Span {
    fn grain(&self) -> Grain {
        self.from.grain()
    }

    fn to_walker(&self, origin: &Interval, context: &Context) -> IntervalWalker {
        let inclusive = self.inclusive;
        let to = self.to.clone();
        let translate = Translate {
            generator: self.from.clone(),
            offset: Rc::new(move |start: &Interval, c: &Context| -> Option<Interval> {
                to.to_walker(start, c)
                    .forward
                    .next()
                    .map(|end| if inclusive {
                             start.union(end)
                         } else {
                             start.interval_to(end)
                         })
            }),
        };
        translate.to_walker(origin, context)
    }
}

#[derive(Clone)]
pub struct ShiftBy {
    base: RcConstraint,
    period: Period,
}

impl ShiftBy {
    pub fn new(base: &RcConstraint, period: Period) -> RcConstraint {
        ShiftBy { base:base.clone(), period }.into()
    }
}

impl IntervalConstraint for ShiftBy {
    fn grain(&self) -> Grain {
        self.base.grain()
    }

    fn to_walker(&self, origin: &Interval, context: &Context) -> IntervalWalker {
        if let Some(period_grain) = self.period.finer_grain() {
            let period = self.period.clone();
            let next_grain = period_grain.next();
            let translate = Translate {
                generator: self.base.clone(),
                offset: Rc::new(move |i: &Interval, _: &Context| -> Option<Interval> {
                    Some((*i).round_to(next_grain) + &period)
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
    use chrono::{TimeZone, Weekday};
    use chrono::offset::local::Local;
    use ::*;

    fn build_context(moment: Moment) -> Context {
        let now = Interval::starting_at(moment, Grain::Second);

        Context::for_reference(now)
    }

    #[test]
    fn test_year() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let year = Year(2015);
        let walker = year.to_walker(&context.reference, &context);
        let mut backward = walker.backward.clone();
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2015, 1, 1).and_hms(0, 0, 0)),
                                              Grain::Year)),
                   backward.next());

        assert_eq!(None, backward.next());
        assert_eq!(None, walker.forward.clone().next());

        let year = Year(2018);
        let walker = year.to_walker(&context.reference, &context);
        assert_eq!(None, walker.backward.clone().next());

        let walker = year.to_walker(&context.reference, &context);
        let mut forward = walker.forward.clone();
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2018, 1, 1).and_hms(0, 0, 0)),
                                              Grain::Year)),
                   forward.next());
        assert_eq!(None, forward.next());

        assert_eq!(None, walker.backward.clone().next());
    }

    #[test]
    fn test_cycle() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let cycle = Cycle(Grain::Day);
        let walker = cycle.to_walker(&context.reference, &context);
        let mut backward = walker.backward.clone();
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 24).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   backward.next());
        let mut forward = walker.forward.clone();
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 25).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   forward.next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 26).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   forward.next());
    }

    #[test]
    fn test_take_the_nth_forward_positive() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let take_the_nth = Month::new(5).take_the_nth(3);
        let walker = take_the_nth.to_walker(&context.reference, &context);
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2020, 05, 1).and_hms(0, 0, 0)),
                                              Grain::Month)),
                   walker.forward.clone().next());
        assert_eq!(None, walker.forward.clone().skip(1).next());
        assert_eq!(None, walker.backward.clone().next());

    }

    #[test]
    fn test_take_the_nth_backward_positive() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let take_the_nth = Month::new(3).take_the_nth(3);
        let walker = take_the_nth.to_walker(&context.reference, &context);
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2021, 03, 1).and_hms(0, 0, 0)),
                                              Grain::Month)),
                   walker.forward.clone().next());
        assert_eq!(None, walker.forward.clone().skip(1).next());
        assert_eq!(None, walker.backward.clone().next());
    }

    #[test]
    fn test_take_the_nth_backward_negative() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let take_the_nth = Month::new(3).take_the_nth(-3);
        let walker = take_the_nth.to_walker(&context.reference, &context);
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2015, 03, 1).and_hms(0, 0, 0)),
                                              Grain::Month)),
                   walker.backward.clone().next());
        assert_eq!(None, walker.backward.clone().skip(1).next());
        assert_eq!(None, walker.forward.clone().next());
    }

    #[test]
    fn test_take_the_nth_after_positive() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let take_the_nth_after = Cycle::rc(Grain::Day).the_nth(3).after(DayOfMonth(20).into());

        let walker = take_the_nth_after.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 05, 23).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 06, 23).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 23).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 03, 23).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_take_the_nth_after_negative() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let take_the_nth_after = Cycle::rc(Grain::Day).the_nth(-3).after(DayOfMonth(20).into());

        let walker = take_the_nth_after.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 05, 17).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 06, 17).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 17).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 03, 17).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_take_the_last_week_of_month() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let take_last_of = Cycle::rc(Grain::Week).last_of(Month(5).into());

        let walker = take_last_of.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 05, 22).and_hms(0, 0, 0)),
                                              Grain::Week)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2018, 05, 21).and_hms(0, 0, 0)),
                                              Grain::Week)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2016, 05, 23).and_hms(0, 0, 0)),
                                              Grain::Week)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2015, 05, 25).and_hms(0, 0, 0)),
                                              Grain::Week)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_take_the_last_day_of_month() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let take_last_of = Cycle::rc(Grain::Day).last_of(Month(5).into());

        let walker = take_last_of.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 05, 31).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2018, 05, 31).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2016, 05, 31).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2015, 05, 31).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_month_above_current_month() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let month = Month(5);
        let walker = month.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 05, 1).and_hms(0, 0, 0)),
                                              Grain::Month)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2018, 05, 1).and_hms(0, 0, 0)),
                                              Grain::Month)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2016, 05, 1).and_hms(0, 0, 0)),
                                              Grain::Month)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2015, 05, 1).and_hms(0, 0, 0)),
                                              Grain::Month)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_month_under_current_month() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let month = Month(3);
        let walker = month.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2018, 03, 1).and_hms(0, 0, 0)),
                                              Grain::Month)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2019, 03, 1).and_hms(0, 0, 0)),
                                              Grain::Month)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 03, 1).and_hms(0, 0, 0)),
                                              Grain::Month)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2016, 03, 1).and_hms(0, 0, 0)),
                                              Grain::Month)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_day_of_month_backward() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day_of_month = DayOfMonth(10);
        let walker = day_of_month.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 05, 10).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 06, 10).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 10).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 03, 10).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_day_of_month_forward() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day_of_month = DayOfMonth(31);
        let walker = day_of_month.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 05, 31).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 07, 31).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 03, 31).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 01, 31).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_day_of_week_after() {
        // Day of week => Tuesday
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day_of_week = DayOfWeek(Weekday::Wed);
        let walker = day_of_week.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 26).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 05, 03).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 19).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 12).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_day_of_week_before() {
        // Day of week => Tuesday
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day_of_week = DayOfWeek(Weekday::Mon);
        let walker = day_of_week.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 24).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 05, 01).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 17).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 10).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_hour_24_clock_under_12() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day = Hour::clock_24(11);
        let walker = day.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 25).and_hms(11, 0, 0)),
                                              Grain::Hour)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 26).and_hms(11, 0, 0)),
                                              Grain::Hour)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 24).and_hms(11, 0, 0)),
                                              Grain::Hour)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 23).and_hms(11, 0, 0)),
                                              Grain::Hour)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_hour_24_clock_above_12() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day = Hour::clock_24(15);
        let walker = day.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 25).and_hms(15, 0, 0)),
                                              Grain::Hour)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 26).and_hms(15, 0, 0)),
                                              Grain::Hour)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 24).and_hms(15, 0, 0)),
                                              Grain::Hour)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 23).and_hms(15, 0, 0)),
                                              Grain::Hour)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_hour_24_clock_under_current_hour() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day = Hour::clock_24(4);
        let walker = day.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 25).and_hms(4, 0, 0)),
                                              Grain::Hour)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 26).and_hms(4, 0, 0)),
                                              Grain::Hour)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 24).and_hms(4, 0, 0)),
                                              Grain::Hour)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 23).and_hms(4, 0, 0)),
                                              Grain::Hour)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_hour_12_clock_under_12() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day = Hour::clock_12(11);
        let walker = day.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 25).and_hms(11, 0, 0)),
                                              Grain::Hour)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 25).and_hms(23, 0, 0)),
                                              Grain::Hour)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 24).and_hms(23, 0, 0)),
                                              Grain::Hour)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 24).and_hms(11, 0, 0)),
                                              Grain::Hour)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_hour_12_clock_above_12() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day = Hour::clock_12(14);
        let walker = day.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 25).and_hms(14, 0, 0)),
                                              Grain::Hour)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 26).and_hms(14, 0, 0)),
                                              Grain::Hour)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 24).and_hms(14, 0, 0)),
                                              Grain::Hour)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 23).and_hms(14, 0, 0)),
                                              Grain::Hour)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_hour_12_clock_under_current_hour() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day = Hour::clock_12(8);
        let walker = day.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 25).and_hms(8, 0, 0)),
                                              Grain::Hour)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 25).and_hms(20, 0, 0)),
                                              Grain::Hour)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 24).and_hms(20, 0, 0)),
                                              Grain::Hour)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 24).and_hms(8, 0, 0)),
                                              Grain::Hour)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_minute_above_current_minute() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day = Minute(15);
        let walker = day.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 25).and_hms(9, 15, 0)),
                                              Grain::Minute)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 25).and_hms(10, 15, 0)),
                                              Grain::Minute)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 25).and_hms(8, 15, 0)),
                                              Grain::Minute)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 25).and_hms(7, 15, 0)),
                                              Grain::Minute)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_minute_under_current_minute() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day = Minute(5);
        let walker = day.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 25).and_hms(9, 5, 0)),
                                              Grain::Minute)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 25).and_hms(10, 5, 0)),
                                              Grain::Minute)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 25).and_hms(8, 5, 0)),
                                              Grain::Minute)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 25).and_hms(7, 5, 0)),
                                              Grain::Minute)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_second_above_current_second() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day = Second(30);
        let walker = day.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 30)),
                                              Grain::Second)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 25).and_hms(9, 11, 30)),
                                              Grain::Second)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 25).and_hms(9, 9, 30)),
                                              Grain::Second)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 25).and_hms(9, 8, 30)),
                                              Grain::Second)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_intersect_dom_month() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));

        let inter = DayOfMonth::new(12).intersect(Month(3).into());
        let walker = inter.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2018, 03, 12).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2019, 03, 12).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 03, 12).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2016, 03, 12).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_intersect_dow_dom() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));

        let inter = DayOfMonth::new(12).intersect(DayOfWeek(Weekday::Wed).into());
        let walker = inter.to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 07, 12).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2018, 09, 12).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 12).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2016, 10, 12).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_translate_date_by_days_fore() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));

        // every 32th day after the 12 of any month
        fn offset(i: &Interval, _: &Context) -> Option<Interval> {
            Some(*i + PeriodComp::days(32))
        }
        let walker = DayOfMonth::new(12).translate_with(offset)
                .to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 05, 14).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 06, 13).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 13).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 03, 16).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_translate_date_by_days_back() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));

        // every 32th day before the 12 of any month
        fn offset(i: &Interval, _: &Context) -> Option<Interval> {
            Some(*i - PeriodComp::days(32))
        }
        let walker = DayOfMonth::new(12).translate_with(offset).to_walker(&context.reference,
                                                                             &context);

        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 05, 11).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 06, 10).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 10).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 03, 11).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_translate_date_by_days_back_() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));

        fn offset(i: &Interval, _: &Context) -> Option<Interval> {
            Some(*i + PeriodComp::days(100))
        }
        let walker = DayOfMonth::new(12).translate_with(offset).to_walker(&context.reference,
                                                                             &context);

        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 05, 23).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 06, 20).and_hms(0, 0, 0)),
                                              Grain::Day)),
                   walker.forward.clone().skip(1).next());
    }

    #[test]
    fn test_inclusive_span() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let walker = DayOfWeek::new(Weekday::Mon).span_to(DayOfWeek(Weekday::Wed).into())
            .to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::new(Moment(Local.ymd(2017, 04, 24).and_hms(0, 0, 0)),
                                      Some(Moment(Local.ymd(2017, 04, 26).and_hms(0, 0, 0))),
                                      Grain::Day)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::new(Moment(Local.ymd(2017, 05, 01).and_hms(0, 0, 0)),
                                      Some(Moment(Local.ymd(2017, 05, 03).and_hms(0, 0, 0))),
                                      Grain::Day)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::new(Moment(Local.ymd(2017, 04, 17).and_hms(0, 0, 0)),
                                      Some(Moment(Local.ymd(2017, 04, 19).and_hms(0, 0, 0))),
                                      Grain::Day)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::new(Moment(Local.ymd(2017, 04, 10).and_hms(0, 0, 0)),
                                      Some(Moment(Local.ymd(2017, 04, 12).and_hms(0, 0, 0))),
                                      Grain::Day)),
                   walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_shift_by_days() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let walker = DayOfMonth::new(12).shift_by(PeriodComp::days(2).into())
            .to_walker(&context.reference, &context);

        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 05, 14).and_hms(0, 0, 0)),
                                              Grain::Hour)),
                   walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 06, 14).and_hms(0, 0, 0)),
                                              Grain::Hour)),
                   walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 14).and_hms(0, 0, 0)),
                                              Grain::Hour)),
                   walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 03, 14).and_hms(0, 0, 0)),
                                              Grain::Hour)),
                   walker.backward.clone().skip(1).next());

    }

}
