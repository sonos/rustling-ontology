use interval_iterator::*;
use ::Interval;
use ::Moment;
use period::*;
use chrono::offset::local::Local;
use chrono::{Datelike, Duration, TimeZone, Timelike};

struct Context;

struct IntervalGenerator {
    grain: Grain,
    iterator: Box<BidirectionalIterator> 
}

impl IntervalGenerator {
    fn new(grain: Grain, iterator: Box<BidirectionalIterator>) -> IntervalGenerator {
        IntervalGenerator {
            grain: grain,
            iterator: iterator,
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
           IntervalGenerator::new(Grain::Year, Box::new(BidirectionalIter::new().forward(vec![interval])))
       } else {
           let moment_year = Moment(Local.ymd(normalized_year, 1, 1).and_hms(0, 0, 0));
           let interval = Interval::starting_at(moment_year, Grain::Year);
           IntervalGenerator::new(Grain::Year, Box::new(BidirectionalIter::new().backward(vec![interval])))
       }
   }
}


/*
def month(quantity):
    def processing(time_token, context):
        rounded = time.format_time(time.year(time_token), quantity)

        if time.is_start_before_the_end_of(time_token, rounded):
            anchor = rounded
        else:
            anchor = time.plus(rounded, "year", 1)
        start = uf.build_sequence_iterator(lambda item: time.plus(item, "year", 1), anchor)
        end = uf.build_sequence_iterator(lambda item: time.minus(item, "year", 1), time.minus(anchor, "year", 1))
        return [start, end]
    return build_time_predicate(processing, "month")
*/

struct Month(u32);

impl IntervalPredicate for Month {
    fn predicate(&self, origin: Interval, context: Context) -> IntervalGenerator {
        let rounded_moment = Moment(Local.ymd(origin.start.year(), self.0, 1).and_hms(0, 0, 0));
        let anchor = if origin.start <= rounded_moment {
            Interval::starting_at(rounded_moment, Grain::Month)
        } else {
            Interval::starting_at(rounded_moment + PeriodComp::years(1), Grain::Month)
        };
        fn gen(i:Interval) -> Interval { i }
//        let gen = |prev| prev;
        let iterator = BidirectionalIter::new()
                .forward_with(anchor, gen)
                .backward_with(anchor - PeriodComp::years(1), |prev| prev);
        //IntervalGenerator::new(Grain::Year, Box::new(iterator))
        unimplemented!()
    }
}