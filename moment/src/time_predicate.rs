use ::Moment;
use ::Interval;

struct Context;

#[derive(Debug,Clone, PartialEq)]
struct IntervalIterator<F> where F: Fn(Interval) -> Interval {
    anchor: Interval,
    transform: F,
}

impl<F> IntervalIterator<F> where F: Fn(Interval) -> Interval {
    fn new(anchor: Interval, transform: F) -> IntervalIterator<F> {
        IntervalIterator {
            anchor: anchor,
            transform: transform,
        }
    }
}

impl<F> Iterator for IntervalIterator<F> where F: Fn(Interval) -> Interval {
    type Item = Interval;

    fn next(&mut self) -> Option<Interval> {
        let current = self.anchor;
        self.anchor = (self.transform)(current);
        Some(current)
    }
}

#[derive(Debug,Clone, PartialEq)]
struct BidirectionalIterator<F, B> 
    where F: Iterator<Item=Interval>,
          B: Iterator<Item=Interval>
    {
    forward: Option<F>,
    backward: Option<B>,
}

impl<P> BidirectionalIterator<IntervalIterator<P>, IntervalIterator<P>> where P: Fn(Interval) -> Interval {
    fn only_forward(self, anchor: Interval, transform: P) -> BidirectionalIterator<IntervalIterator<P>, IntervalIterator<P>> {
        BidirectionalIterator {
            forward: Some(IntervalIterator::new(anchor, transform)),
            backward: None,
        }
    } 

    fn only_backward(self, anchor: Interval, transform: P) -> BidirectionalIterator<IntervalIterator<P>, IntervalIterator<P>> {
        BidirectionalIterator {
            forward: None,
            backward: Some(IntervalIterator::new(anchor, transform)),
        }
    }   
    
}

impl<FP, BP> BidirectionalIterator<IntervalIterator<FP>, IntervalIterator<BP>>
    where FP: Fn(Interval) -> Interval,
          BP: Fn(Interval) -> Interval,
    {

    fn new() -> BidirectionalIterator<IntervalIterator<FP>, IntervalIterator<BP>> {
        BidirectionalIterator {
            forward: None,
            backward: None,
        } 
    }

    fn with_forward(self, anchor: Interval, transform: FP) -> BidirectionalIterator<IntervalIterator<FP>, IntervalIterator<BP>> {
        BidirectionalIterator {
            forward: Some(IntervalIterator::new(anchor, transform)),
            backward: self.backward,
        }   
    }

    fn with_backward(self, anchor: Interval, transform: BP) -> BidirectionalIterator<IntervalIterator<FP>, IntervalIterator<BP>> {
        BidirectionalIterator {
            forward: self.forward,
            backward: Some(IntervalIterator::new(anchor, transform)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::Moment;
    use ::Interval;
    use ::period::*;
    use chrono::offset::local::Local;
    use chrono::datetime::DateTime;
    use chrono::TimeZone;

    #[test]
    fn test_interval_iterator() {
        let now = Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11));
        let interval = Interval::starting_at(now, Grain::Second);
        let mut iterator = IntervalIterator::new(interval, |prev| prev + PeriodComp::days(1));

        assert_eq!(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)), iterator.next().unwrap().start);
        assert_eq!(Moment(Local.ymd(2017, 04, 26).and_hms(9, 10, 11)), iterator.next().unwrap().start);
        assert_eq!(Moment(Local.ymd(2017, 04, 27).and_hms(9, 10, 11)), iterator.next().unwrap().start);
        assert_eq!(Moment(Local.ymd(2017, 04, 28).and_hms(9, 10, 11)), iterator.next().unwrap().start);
    }

    #[test]
    fn test_interval_bidirectional() {
        let anchor = Interval::starting_at(
                Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)), 
                Grain::Second
        );

        let bidirectional = BidirectionalIterator::new()
                    .with_forward(anchor, |prev| prev + PeriodComp::days(1))
                    .with_backward(anchor, |prev| prev - PeriodComp::days(1));

    }
}
