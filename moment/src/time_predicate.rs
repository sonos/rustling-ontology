use bidirectional_walker::*;
use walker::*;
use ::*;
use period::*;
use chrono::offset::local::Local;
use chrono::{Datelike, Duration, TimeZone, Timelike, Weekday};

struct Context;

struct IntervalWalker {
    grain: Grain,
    walker: BidirectionalWalker<Interval>,
}

impl IntervalWalker {
    fn new(grain: Grain, walker: BidirectionalWalker<Interval>) -> IntervalWalker {
        IntervalWalker {
            grain: grain,
            walker: walker,
        }
    }
}

trait IntervalPredicate {
    fn predicate(&self, origin: Interval, context: Context) -> IntervalWalker;
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Year(i32);

impl IntervalPredicate for Year {
    fn predicate(&self, origin: Interval, context: Context) -> IntervalWalker {
        let normalized_year = if self.0 < 99 {
            (self.0 + 50) % 100 + 2000 - 50
        } else {
            self.0
        };

        if origin.start.year() <= normalized_year {
            let moment_year = Moment(Local.ymd(normalized_year, 1, 1).and_hms(0, 0, 0));
            let interval = Interval::starting_at(moment_year, Grain::Year);
            IntervalWalker::new(Grain::Year,
                                   BidirectionalWalker::new().forward_values(vec![interval]))
        } else {
            let moment_year = Moment(Local.ymd(normalized_year, 1, 1).and_hms(0, 0, 0));
            let interval = Interval::starting_at(moment_year, Grain::Year);
            IntervalWalker::new(Grain::Year,
                                   BidirectionalWalker::new().backward_values(vec![interval]))
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Month(u32);

impl IntervalPredicate for Month {
    fn predicate(&self, origin: Interval, context: Context) -> IntervalWalker {
        let rounded_moment = Moment(Local
                                        .ymd(origin.start.year(), self.0, 1)
                                        .and_hms(0, 0, 0));
        let offset_year = (origin.start.0.day() > self.0) as i64;
        let anchor = Interval::starting_at(rounded_moment + PeriodComp::years(offset_year),
                                           Grain::Month);

        let walker = BidirectionalWalker::new()
            .forward_with(anchor, |prev| prev + PeriodComp::years(1))
            .backward_with(anchor - PeriodComp::years(1),
                           |prev| prev - PeriodComp::years(1));
        IntervalWalker::new(Grain::Month, walker)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct DayOfMonth(u32);

impl IntervalPredicate for DayOfMonth {
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

        let walker = BidirectionalWalker::new()
            .forward(forward_walker)
            .backward(backward_walker);

        IntervalWalker::new(Grain::Day, walker)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct DayOfWeek(Weekday);

impl IntervalPredicate for DayOfWeek {
    fn predicate(&self, origin: Interval, context: Context) -> IntervalWalker {
        let offset = (self.0.number_from_monday() - origin.start.weekday().number_from_monday()) %
                     7;
        let anchor = origin.round_to(Grain::Day) + PeriodComp::days(offset as i64);

        let walker = BidirectionalWalker::new()
            .forward_with(anchor, |prev| prev + PeriodComp::weeks(1))
            .backward_with(anchor - PeriodComp::weeks(1),
                           |prev| prev - PeriodComp::weeks(1));

        IntervalWalker::new(Grain::Day, walker)
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
    fn predicate(&self, origin: Interval, context: Context) -> IntervalWalker {
        let clock_step = if self.quantity <= 12 && self.is_12_clock {
            12
        } else {
            24
        };
        let offset = (self.quantity - origin.start.hour() as i64) % clock_step;
        let anchor = origin.round_to(Grain::Hour) + PeriodComp::hours(offset);

        let walker = BidirectionalWalker::new()
            .forward_with(anchor, move |prev| prev + PeriodComp::hours(clock_step))
            .backward_with(anchor - PeriodComp::hours(clock_step),
                           move |prev| prev - PeriodComp::hours(clock_step));

        IntervalWalker::new(Grain::Hour, walker)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Minute(i64);

impl IntervalPredicate for Minute {
    fn predicate(&self, origin: Interval, context: Context) -> IntervalWalker {
        let offset = (self.0 - origin.start.minute() as i64) % 60;
        let anchor = origin.round_to(Grain::Minute) + PeriodComp::minutes(offset);

        let walker = BidirectionalWalker::new()
            .forward_with(anchor, |prev| prev + PeriodComp::hours(1))
            .backward_with(anchor - PeriodComp::hours(1),
                           |prev| prev + PeriodComp::hours(1));

        IntervalWalker::new(Grain::Minute, walker)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Second(i64);

impl IntervalPredicate for Second {
    fn predicate(&self, origin: Interval, context: Context) -> IntervalWalker {
        let offset = (self.0 - origin.start.second() as i64) % 60;
        let anchor = origin.round_to(Grain::Second) + PeriodComp::seconds(offset);

        let walker = BidirectionalWalker::new()
            .forward_with(anchor, |prev| prev + PeriodComp::minutes(1))
            .backward_with(anchor - PeriodComp::minutes(1),
                           |prev| prev + PeriodComp::minutes(1));

        IntervalWalker::new(Grain::Second, walker)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Cycle(Grain);

impl IntervalPredicate for Cycle {
    fn predicate(&self, origin: Interval, context: Context) -> IntervalWalker {
        let anchor = origin.round_to(self.0);
        let grain = self.0;
        let walker = BidirectionalWalker::new()
            .forward_with(anchor, move |prev| prev + PeriodComp::new(grain, 1))
            .backward_with(anchor - PeriodComp::new(grain, 1), move |prev| prev - PeriodComp::new(grain, 1));

        IntervalWalker::new(grain, walker)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use chrono::offset::local::Local;
    use ::*;

    #[test]
    fn test_year_predicate_iterator() {
        let year_predicate = Year(2015);
        let now = Interval::starting_at(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)),
                                        Grain::Second);

        let walker = year_predicate.predicate(now, Context);

        let mut backward = walker.walker.backward.clone();
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2015, 1, 1).and_hms(0, 0, 0)),
                                              Grain::Year)),
                   backward.next());

        assert_eq!(None, backward.next());
        assert_eq!(None, walker.walker.forward.clone().next());

        let year_predicate = Year(2018);
        let walker = year_predicate.predicate(now, Context);
        assert_eq!(None, walker.walker.backward.clone().next());

        let walker = year_predicate.predicate(now, Context);
        let mut forward = walker.walker.forward.clone();
        assert_eq!(Some(Interval::starting_at(Moment(Local.ymd(2018, 1, 1).and_hms(0, 0, 0)),
                                              Grain::Year)),
                   forward.next());
        assert_eq!(None, forward.next());

        assert_eq!(None, walker.walker.backward.clone().next());
    }
}

