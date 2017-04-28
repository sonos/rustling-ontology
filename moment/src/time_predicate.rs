use interval_iterator::*;
use ::*;
use period::*;
use chrono::offset::local::Local;
use chrono::{Datelike, Duration, TimeZone, Timelike, Weekday};

struct Context;

struct IntervalGenerator {
    grain: Grain,
    iterator: Box<BidirectionalIterator> 
}

impl IntervalGenerator {
    fn new<I: BidirectionalIterator + 'static>(grain: Grain, iterator: I) -> IntervalGenerator {
        IntervalGenerator {
            grain: grain,
            iterator: Box::new(iterator),
        }
    }
}

trait IntervalPredicate {
    fn predicate(&self, origin: Interval, context: Context) -> IntervalGenerator;
}

struct Year(i32);

impl IntervalPredicate for Year {
   fn predicate(&self, origin: Interval, context: Context) -> IntervalGenerator {
       let normalized_year = if self.0 < 99 { (self.0 + 50) % 100 + 2000 - 50 } else { self.0 };

       if origin.start.year() <= normalized_year {
           let moment_year = Moment(Local.ymd(normalized_year, 1, 1).and_hms(0, 0, 0));
           let interval = Interval::starting_at(moment_year, Grain::Year);
           IntervalGenerator::new(Grain::Year, BidirectionalIter::new().forward_values(vec![interval]))
       } else {
           let moment_year = Moment(Local.ymd(normalized_year, 1, 1).and_hms(0, 0, 0));
           let interval = Interval::starting_at(moment_year, Grain::Year);
           IntervalGenerator::new(Grain::Year, BidirectionalIter::new().backward_values(vec![interval]))
       }
   }
}

struct Month(u32);

impl IntervalPredicate for Month {
    fn predicate(&self, origin: Interval, context: Context) -> IntervalGenerator {
        let rounded_moment = Moment(Local.ymd(origin.start.year(), self.0, 1).and_hms(0, 0, 0));
        let offset_year = (origin.start.0.day() > self.0) as i64;
        let anchor = Interval::starting_at(rounded_moment + PeriodComp::years(offset_year), Grain::Month);

        let iterator = BidirectionalIter::new()
                .forward_with(anchor, |prev| prev + PeriodComp::years(1))
                .backward_with(anchor - PeriodComp::years(1), |prev| prev - PeriodComp::years(1));
        IntervalGenerator::new(Grain::Month, iterator)
    }
}

struct DayOfMonth(u32);

impl IntervalPredicate for DayOfMonth {
    fn predicate(&self, origin: Interval, context: Context) -> IntervalGenerator {
        let offset_month = (origin.start.0.day() > self.0) as i64;
        let anchor = origin.round_to(Grain::Month) + PeriodComp::months(offset_month);

        let forward_iterator = IntervalIterator::new(anchor, |prev| prev + PeriodComp::months(1))
                .filter(|interval| self.0 <= last_day_in_month(interval.start.year(), interval.start.month()))
                .map(|interval| interval + PeriodComp::days(self.0 as i64 - 1));

        let backward_iterator = IntervalIterator::new(anchor - PeriodComp::months(1), |prev| prev - PeriodComp::months(1))
                .filter(|interval| self.0 <= last_day_in_month(interval.start.year(), interval.start.month()))
                .map(|interval| interval + PeriodComp::days(self.0 as i64 - 1));
        
        //let iterator = BidirectionalIter::new().forward(forward_iterator);//.backward(backward_iterator);
        //IntervalGenerator::new(Grain::Day, iterator)
        unimplemented!();
    }
}

struct DayOfWeek(Weekday);

impl IntervalPredicate for DayOfWeek {
    fn predicate(&self, origin: Interval, context: Context) -> IntervalGenerator {
        let offset = (self.0.number_from_monday() - origin.start.weekday().number_from_monday()) % 7;
        let anchor = origin.round_to(Grain::Day) + PeriodComp::days(offset as i64);

        let iterator = BidirectionalIter::new()
            .forward_with(anchor, |prev| prev + PeriodComp::weeks(1))
            .backward_with(anchor - PeriodComp::weeks(1), |prev| prev - PeriodComp::weeks(1));

        IntervalGenerator::new(Grain::Day, iterator)
    }
}

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
    fn predicate(&self, origin: Interval, context: Context) -> IntervalGenerator {
        let clock_step = if self.quantity <= 12 && self.is_12_clock { 12 } else { 24 };
        let offset = (self.quantity - origin.start.hour() as i64) % clock_step;
        let anchor = origin.round_to(Grain::Hour) + PeriodComp::hours(offset);

        let iterator = BidirectionalIter::new()
            .forward_with(anchor, move |prev| prev + PeriodComp::hours(clock_step))
            .backward_with(anchor - PeriodComp::hours(clock_step), move |prev| prev - PeriodComp::hours(clock_step));
        
        IntervalGenerator::new(Grain::Hour, iterator)
    }
}

struct Minute(i64);

impl IntervalPredicate for Minute {
    fn predicate(&self,  origin: Interval, context: Context) -> IntervalGenerator {
        let offset = (self.0 - origin.start.minute() as i64) % 60;
        let anchor = origin.round_to(Grain::Minute) + PeriodComp::minutes(offset);

        let iterator = BidirectionalIter::new()
            .forward_with(anchor, |prev| prev + PeriodComp::hours(1))
            .backward_with(anchor - PeriodComp::hours(1), |prev| prev + PeriodComp::hours(1));

        IntervalGenerator::new(Grain::Minute, iterator)
    }
}

struct Second(i64);

impl IntervalPredicate for Second {
    fn predicate(&self, origin: Interval, context: Context) -> IntervalGenerator {
        let offset = (self.0 - origin.start.second() as i64) % 60;
        let anchor = origin.round_to(Grain::Second) + PeriodComp::seconds(offset);

        let iterator = BidirectionalIter::new()
            .forward_with(anchor, |prev| prev + PeriodComp::minutes(1))
            .backward_with(anchor - PeriodComp::minutes(1), |prev| prev + PeriodComp::minutes(1));

        IntervalGenerator::new(Grain::Second, iterator)
    }
}
