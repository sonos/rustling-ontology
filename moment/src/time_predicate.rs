use bidirectional_walker::*;
use walker::*;
use ::*;
use period::*;
use chrono::offset::local::Local;
use chrono::{Datelike, Duration, TimeZone, Timelike, Weekday};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Context {
    reference: Interval
}

impl Context {
    fn now() -> Context {
        Context {
            reference: Interval::starting_at(Moment::now(), Grain::Second),
        }
    }
}

type IntervalWalker = BidirectionalWalker<Interval>;

trait IntervalPredicate {
    fn grain(&self) -> Grain;
    fn predicate(&self, origin: Interval, context: Context) -> IntervalWalker;
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Year(i32);

impl IntervalPredicate for Year {
    fn grain(&self) -> Grain {
        Grain::Year
    }

    fn predicate(&self, origin: Interval, context: Context) -> IntervalWalker {
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
struct Month(u32);

impl IntervalPredicate for Month {
    fn grain(&self) -> Grain {
        Grain::Month
    }

    fn predicate(&self, origin: Interval, context: Context) -> IntervalWalker {
        let rounded_moment = Moment(Local
                                        .ymd(origin.start.year(), self.0, 1)
                                        .and_hms(0, 0, 0));
        let rounded_interval = Interval::starting_at(rounded_moment,Grain::Month);
        let offset_year = !(origin.start <= rounded_interval.end_moment()) as i64;
        let anchor = rounded_interval + PeriodComp::years(offset_year);

        BidirectionalWalker::new()
            .forward_with(anchor, |prev| prev + PeriodComp::years(1))
            .backward_with(anchor - PeriodComp::years(1),
                           |prev| prev - PeriodComp::years(1))
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct DayOfMonth(u32);

impl IntervalPredicate for DayOfMonth {
    fn grain(&self) -> Grain {
        Grain::Day
    }

    fn predicate(&self, origin: Interval, context: Context) -> IntervalWalker {
        let offset_month = (origin.start.0.day() > self.0) as i64;
        let anchor = origin.round_to(Grain::Month) + PeriodComp::months(offset_month);

        let day_of_month = self.0;
        let forward_walker = Walker::generator(anchor, |prev| prev + PeriodComp::months(1))
            .filter(move |interval| {
                        day_of_month <= last_day_in_month(interval.start.year(), interval.start.month())
                    })
            .map(move |interval| interval + PeriodComp::days(day_of_month as i64 - 1));

        let backward_walker = Walker::generator(anchor - PeriodComp::months(1),
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
struct DayOfWeek(Weekday);

impl IntervalPredicate for DayOfWeek {
    fn grain(&self) -> Grain {
        Grain::Day
    }
    
    fn predicate(&self, origin: Interval, context: Context) -> IntervalWalker {
        // number_from_monday is u32 -> use i64
        let offset = (self.0.number_from_monday() as i64 - origin.start.weekday().number_from_monday() as i64) %
                     7;
        let anchor = origin.round_to(Grain::Day) + PeriodComp::days(offset);

        BidirectionalWalker::new()
            .forward_with(anchor, |prev| prev + PeriodComp::weeks(1))
            .backward_with(anchor - PeriodComp::weeks(1),
                           |prev| prev - PeriodComp::weeks(1))
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Hour {
    quantity: i64,
    is_12_clock: bool,
}

impl Hour {
    fn new(quantity: i64, is_12_clock: bool) -> Hour {
        Hour {
            quantity: quantity,
            is_12_clock: is_12_clock,
        }
    }
}

impl IntervalPredicate for Hour {
    fn grain(&self) -> Grain {
        Grain::Hour
    }

    fn predicate(&self, origin: Interval, context: Context) -> IntervalWalker {
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
struct Minute(i64);

impl IntervalPredicate for Minute {
    fn grain(&self) -> Grain {
        Grain::Minute
    }
    fn predicate(&self, origin: Interval, context: Context) -> IntervalWalker {
        let offset = (self.0 - origin.start.minute() as i64) % 60;
        let anchor = origin.round_to(Grain::Minute) + PeriodComp::minutes(offset);

        BidirectionalWalker::new()
            .forward_with(anchor, |prev| prev + PeriodComp::hours(1))
            .backward_with(anchor - PeriodComp::hours(1),
                           |prev| prev + PeriodComp::hours(1))
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Second(i64);

impl IntervalPredicate for Second {
    fn grain(&self) -> Grain {
        Grain::Second
    }

    fn predicate(&self, origin: Interval, context: Context) -> IntervalWalker {
        let offset = (self.0 - origin.start.second() as i64) % 60;
        let anchor = origin.round_to(Grain::Second) + PeriodComp::seconds(offset);

        BidirectionalWalker::new()
            .forward_with(anchor, |prev| prev + PeriodComp::minutes(1))
            .backward_with(anchor - PeriodComp::minutes(1),
                           |prev| prev + PeriodComp::minutes(1))
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Cycle(Grain);

impl IntervalPredicate for Cycle {
    fn grain(&self) -> Grain {
        self.0
    }

    fn predicate(&self, origin: Interval, context: Context) -> IntervalWalker {
        let anchor = origin.round_to(self.0);
        let grain = self.0;
        BidirectionalWalker::new()
            .forward_with(anchor, move |prev| prev + PeriodComp::new(grain, 1))
            .backward_with(anchor - PeriodComp::new(grain, 1), move |prev| prev - PeriodComp::new(grain, 1))
    }
}

#[derive(Clone)]
struct TakeTheNth {
    predicate: ::std::rc::Rc<IntervalPredicate>,
    n: i64,
    not_immediate: bool,
}

impl TakeTheNth {
    fn new<P: IntervalPredicate + 'static>(n: i64, not_immediate: bool, predicate: P) -> TakeTheNth {
        TakeTheNth {
            predicate: ::std::rc::Rc::new(predicate),
            n: n,
            not_immediate: not_immediate,
        }
    }
}

impl IntervalPredicate for TakeTheNth {
    fn grain(&self) -> Grain {
        self.predicate.grain()
    }

    fn predicate(&self, origin: Interval, context: Context) -> IntervalWalker {
        let base_interval = context.reference;
        let interval_walker = self.predicate.predicate(base_interval, context);

        let match_interval: Option<Interval> = if self.n >= 0 {
            let head = interval_walker.forward.clone().next();
            let mut forward_walker = if head.is_some() 
                    && self.not_immediate 
                    && head.map(move |x| x.intersect(base_interval)).is_some() {
                interval_walker.forward.clone().skip((self.n+1) as usize)
            } else {
                interval_walker.forward.clone().skip(self.n as usize)
            };
            forward_walker.next()
        } else {
            interval_walker.backward.clone().skip((- (self.n + 1)) as usize).next()
        };

        if let Some(interval) = match_interval {
            if origin.start < interval.end_moment() {
                BidirectionalWalker::new()
                    .forward_values(vec![interval])
            } else {
                BidirectionalWalker::new()
                    .backward_values(vec![interval])
            }
        } else {
            BidirectionalWalker::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Weekday};
    use chrono::offset::local::Local;
    use chrono::offset::fixed::FixedOffset;
    use ::*;

    fn build_context(moment: Moment) -> Context {
        let now = Interval::starting_at(moment,
                                        Grain::Second);

        Context { reference: now }
    }

    #[test]
    fn test_year_predicate_iterator() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let year_predicate = Year(2015);
        let walker = year_predicate.predicate(context.reference, context);
        let mut backward = walker.backward.clone();
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2015, 1, 1).and_hms(0, 0, 0)),
                                              Grain::Year)),
                   backward.next());

        assert_eq!(None, backward.next());
        assert_eq!(None, walker.forward.clone().next());

        let year_predicate = Year(2018);
        let walker = year_predicate.predicate(context.reference, context);
        assert_eq!(None, walker.backward.clone().next());

        let walker = year_predicate.predicate(context.reference, context);
        let mut forward = walker.forward.clone();
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2018, 1, 1).and_hms(0, 0, 0)),
                                              Grain::Year)),
                   forward.next());
        assert_eq!(None, forward.next());

        assert_eq!(None, walker.backward.clone().next());
    }

    #[test]
    fn test_cycle_predicate() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        let cycle_predicate = Cycle(Grain::Day);
        let walker = cycle_predicate.predicate(context.reference, context);
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
    fn test_take_the_nth_predicate() {
        // Context
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));

        //  Test case 1
        let month_predicate = Month(5);
        let take_the_nth_predicate = TakeTheNth::new(3, false, month_predicate);
        let walker = take_the_nth_predicate.predicate(context.reference, context);
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2020, 05, 1).and_hms(0, 0, 0)),
                                              Grain::Month)), 
                    walker.forward.clone().next());
        assert_eq!(None, walker.forward.clone().skip(1).next());
        assert_eq!(None, walker.backward.clone().next());

        // Test case 2
        let month_predicate = Month(3);
        let take_the_nth_predicate = TakeTheNth::new(3, false, month_predicate);
        let walker = take_the_nth_predicate.predicate(context.reference, context);
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2021, 03, 1).and_hms(0, 0, 0)),
                                              Grain::Month)), 
                    walker.forward.clone().next());
        assert_eq!(None, walker.forward.clone().skip(1).next());
        assert_eq!(None, walker.backward.clone().next());

        //Test case 3

        let month_predicate = Month(3);
        let take_the_nth_predicate = TakeTheNth::new(-3, false, month_predicate);
        let walker = take_the_nth_predicate.predicate(context.reference, context);
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2015, 03, 1).and_hms(0, 0, 0)),
                                              Grain::Month)), 
                    walker.backward.clone().next());
        assert_eq!(None, walker.backward.clone().skip(1).next());
        assert_eq!(None, walker.forward.clone().next());
    }

    #[test]
    fn test_month_predicate() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        // Test case 1
        let month_predicate = Month(5);
        let walker = month_predicate.predicate(context.reference, context);

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
        let month_predicate = Month(3);
        let walker = month_predicate.predicate(context.reference, context);

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
    fn test_day_of_month_predicate() {
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        // Test case 1
        let day_of_month_predicate = DayOfMonth(10);
        let walker = day_of_month_predicate.predicate(context.reference, context);

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

        // Test case 2
        let day_of_month_predicate = DayOfMonth(31);
        let walker = day_of_month_predicate.predicate(context.reference, context);

        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 05, 31).and_hms(0, 0, 0)),
                                              Grain::Day)), 
                    walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 07, 31).and_hms(0, 0, 0)),
                                              Grain::Day)), 
                    walker.forward.clone().skip(1).next());
        // TODO Take a look at the offset shifting due to a period addition
        // 1st March -> +1 and 31 Match -> +2
        // 1st March + 30 days -> +1 instead of +2
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 03, 31).and_hms(0, 0, 0) + FixedOffset::east(3600)),
                                              Grain::Day)), 
                    walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 01, 31).and_hms(0, 0, 0)),
                                              Grain::Day)), 
                    walker.backward.clone().skip(1).next());
    }

    #[test]
    fn test_day_of_week() {
        // Day of week => Tuesday
        let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
        // Test case 1
        let day_of_week_predicate = DayOfWeek(Weekday::Wed);
        let walker = day_of_week_predicate.predicate(context.reference, context);

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

        // Test case 2
        let day_of_week_predicate = DayOfWeek(Weekday::Mon);
        let walker = day_of_week_predicate.predicate(context.reference, context);

        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 24).and_hms(0, 0, 0)),
                                              Grain::Day)), 
                    walker.forward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 05, 01).and_hms(0, 0, 0)),
                                              Grain::Day)), 
                    walker.forward.clone().skip(1).next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 17).and_hms(0, 0, 0) + FixedOffset::east(3600)),
                                              Grain::Day)), 
                    walker.backward.clone().next());
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2017, 04, 10).and_hms(0, 0, 0)),
                                              Grain::Day)), 
                    walker.backward.clone().skip(1).next());
    }
}

