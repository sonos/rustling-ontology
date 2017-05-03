use std::rc::Rc;

use bidirectional_walker::*;
use walker::*;
use {Moment, Interval, last_day_in_month};
use period::*;
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Year(i32);

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
pub struct Month(u32);

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
pub struct DayOfMonth(u32);

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
pub struct DayOfWeek(Weekday);

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

#[derive(Debug, Copy, Clone, PartialEq, new)]
pub struct Hour {
    quantity: i64,
    is_12_clock: bool,
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
        let offset = (self.quantity - origin.start.hour() as i64) % clock_step;
        let anchor = origin.round_to(Grain::Hour) + PeriodComp::hours(offset);

        BidirectionalWalker::new()
            .forward_with(anchor, move |prev| prev + PeriodComp::hours(clock_step))
            .backward_with(anchor - PeriodComp::hours(clock_step),
                           move |prev| prev - PeriodComp::hours(clock_step))
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Minute(i64);

impl IntervalConstraint for Minute {
    fn grain(&self) -> Grain {
        Grain::Minute
    }
    fn to_walker(&self, origin: &Interval, _context: &Context) -> IntervalWalker {
        let offset = (self.0 - origin.start.minute() as i64) % 60;
        let anchor = origin.round_to(Grain::Minute) + PeriodComp::minutes(offset);

        BidirectionalWalker::new()
            .forward_with(anchor, |prev| prev + PeriodComp::hours(1))
            .backward_with(anchor - PeriodComp::hours(1),
                           |prev| prev - PeriodComp::hours(1))
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Second(i64);

impl IntervalConstraint for Second {
    fn grain(&self) -> Grain {
        Grain::Second
    }

    fn to_walker(&self, origin: &Interval, _context: &Context) -> IntervalWalker {
        let offset = (self.0 - origin.start.second() as i64) % 60;
        let anchor = origin.round_to(Grain::Second) + PeriodComp::seconds(offset);

        BidirectionalWalker::new()
            .forward_with(anchor, |prev| prev + PeriodComp::minutes(1))
            .backward_with(anchor - PeriodComp::minutes(1),
                           |prev| prev - PeriodComp::minutes(1))
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Cycle(Grain);

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

#[derive(Clone,new)]
pub struct TakeTheNth<Inner>
    where Inner: IntervalConstraint + 'static
{
    n: i64,
    not_immediate: bool,
    inner: Inner,
}

impl<Inner> IntervalConstraint for TakeTheNth<Inner>
    where Inner: IntervalConstraint + 'static
{
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

#[derive(Clone, new)]
pub struct TakeN<Inner>
    where Inner: IntervalConstraint + 'static
{
    n: i64,
    not_immediate: bool,
    inner: Inner,
}

impl<Inner> IntervalConstraint for TakeN<Inner>
    where Inner: IntervalConstraint + 'static
{
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
pub struct Intersection<LHS, RHS>
    where LHS: IntervalConstraint + 'static,
          RHS: IntervalConstraint + 'static
{
    lhs: Rc<LHS>,
    rhs: Rc<RHS>,
}

impl<LHS, RHS> Intersection<LHS, RHS>
    where LHS: IntervalConstraint + 'static,
          RHS: IntervalConstraint + 'static
{
    pub fn new(lhs: LHS, rhs: RHS) -> Intersection<LHS, RHS> {
        Intersection {
            lhs: Rc::new(lhs),
            rhs: Rc::new(rhs),
        }
    }
}

impl<LHS, RHS> IntervalConstraint for Intersection<LHS, RHS>
    where LHS: IntervalConstraint + 'static,
          RHS: IntervalConstraint + 'static
{
    fn grain(&self) -> Grain {
        ::std::cmp::max(self.lhs.grain(), self.rhs.grain())
    }

    fn to_walker(&self, origin: &Interval, context: &Context) -> IntervalWalker {

        fn walk_from(origin: &Interval,
                     context: Context,
                     constraint: Rc<IntervalConstraint>)
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

        fn combine<Fine, Coarse>(origin: &Interval,
                                 context: Context,
                                 fine: Rc<Fine>,
                                 coarse: Rc<Coarse>)
                                 -> IntervalWalker
            where Fine: IntervalConstraint + 'static,
                  Coarse: IntervalConstraint + 'static
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
pub struct Translate<Generator, Offset>
    where Generator: IntervalConstraint + 'static,
          Offset: Fn(&Interval, &Context) -> Option<Interval> + 'static
{
    generator: Rc<Generator>,
    offset: Rc<Offset>,
}

impl<Generator, Offset> Translate<Generator, Offset>
    where Generator: IntervalConstraint + 'static,
          Offset: Fn(&Interval, &Context) -> Option<Interval> + 'static
{
    pub fn new(generator: Generator,
               offset: Offset)
               -> Translate<Generator, Offset> {
        Translate {
            generator: Rc::new(generator),
            offset: Rc::new(offset),
        }
    }
}

impl<Generator, Offset> IntervalConstraint for Translate<Generator, Offset>
    where Generator: IntervalConstraint + 'static,
          Offset: Fn(&Interval, &Context) -> Option<Interval> + 'static
{
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
pub struct Span<From, To>
    where From: IntervalConstraint + 'static,
          To: IntervalConstraint + 'static
{
    from: Rc<From>,
    to: Rc<To>,
    inclusive: bool,
}

impl<From, To> Span<From, To>
    where From: IntervalConstraint + 'static,
          To: IntervalConstraint + 'static
{
    pub fn new(from: From, to: To, inclusive: bool) -> Span<From, To> {
        Span {
            from: Rc::new(from),
            to: Rc::new(to),
            inclusive,
        }
    }
}

impl<From, To> IntervalConstraint for Span<From, To>
    where From: IntervalConstraint + 'static,
          To: IntervalConstraint + 'static
{
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

        let month = Month(5);
        let take_the_nth = TakeTheNth::new(3, false, month);
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

        let month = Month(3);
        let take_the_nth = TakeTheNth::new(3, false, month);
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

        let month = Month(3);
        let take_the_nth = TakeTheNth::new(-3, false, month);
        let walker = take_the_nth.to_walker(&context.reference, &context);
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2015, 03, 1).and_hms(0, 0, 0)),
                                              Grain::Month)),
                   walker.backward.clone().next());
        assert_eq!(None, walker.backward.clone().skip(1).next());
        assert_eq!(None, walker.forward.clone().next());
    }

    #[test]
    fn test_month() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        // Test case 1
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

        // Test case 2
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
    fn test_hour_not_12_clock_under_12() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day = Hour::new(11, false);
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
    fn test_hour_not_12_clock_above_12() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day = Hour::new(15, false);
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
    fn test_hour_not_12_clock_under_current_hour() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day = Hour::new(4, false);
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
    fn test_hour_is_12_clock_under_12() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day = Hour::new(11, true);
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
    fn test_hour_is_12_clock_above_12() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day = Hour::new(14, true);
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
    fn test_hour_is_12_clock_under_current_hour() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let day = Hour::new(8, true);
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

        let inter = Intersection::new(DayOfMonth(12), Month(3));
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

        let inter = Intersection::new(DayOfMonth(12), DayOfWeek(Weekday::Wed));
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
        let walker = Translate::new(DayOfMonth(12), offset).to_walker(&context.reference,
                                                                             &context);

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
        let walker = Translate::new(DayOfMonth(12), offset).to_walker(&context.reference,
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
        let walker = Translate::new(DayOfMonth(12), offset).to_walker(&context.reference,
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
        let walker = Span::new(DayOfWeek(Weekday::Mon), DayOfWeek(Weekday::Wed), false)
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

}
