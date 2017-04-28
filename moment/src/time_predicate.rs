use ::Moment;
use ::Interval;
use std::vec::IntoIter;
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

struct EmptyIntervalIterator;

impl Iterator for EmptyIntervalIterator {
    type Item = Interval;

    fn next(&mut self) -> Option<Interval> {
        None
    }
}

#[derive(Debug,Clone, PartialEq)]
struct BidirectionalIterator<F, B> 
    where F: Iterator<Item=Interval>,
          B: Iterator<Item=Interval>
    {
    forward: F,
    backward: B,
}

impl BidirectionalIterator<EmptyIntervalIterator, EmptyIntervalIterator> {
    fn new() -> BidirectionalIterator<EmptyIntervalIterator, EmptyIntervalIterator> {
        BidirectionalIterator {
            forward: EmptyIntervalIterator,
            backward: EmptyIntervalIterator,
        }  
    }
}

impl<F, B> BidirectionalIterator<F, B>
    where F: Iterator<Item=Interval>,
          B: Iterator<Item=Interval>
    {

    fn forward(self, values: Vec<Interval>) -> BidirectionalIterator<IntoIter<Interval>, B> {
        BidirectionalIterator {
            forward: values.into_iter(),
            backward: self.backward,
        } 
    }

    fn forward_with<FP: Fn(Interval) -> Interval>(self, anchor: Interval, transform: FP) -> BidirectionalIterator<IntervalIterator<FP>, B> {
        BidirectionalIterator {
            forward: IntervalIterator::new(anchor, transform),
            backward: self.backward,
        }   
    }

    fn backward(self, values: Vec<Interval>) -> BidirectionalIterator<F, IntoIter<Interval>> {
        BidirectionalIterator {
            forward: self.forward,
            backward: values.into_iter(),
        }
    }

    fn backward_with<BP: Fn(Interval) -> Interval>(self, anchor: Interval, transform: BP) -> BidirectionalIterator<F, IntervalIterator<BP>> {
        BidirectionalIterator {
            forward: self.forward,
            backward: IntervalIterator::new(anchor, transform),
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
                    .forward_with(anchor, |prev| prev + PeriodComp::days(1))
                    .backward_with(anchor, |prev| prev - PeriodComp::days(1));
    }

    #[test]
    fn test_interval_bidirectional_forward_values() {
        let values = vec![
            Interval::starting_at(
                Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)), 
                Grain::Second
            ),
            Interval::starting_at(
                Moment(Local.ymd(2017, 04, 26).and_hms(9, 10, 11)),
                Grain::Second
            )
        ];

        let mut only_forward = BidirectionalIterator::new()
                    .forward(values);

        assert_eq!(None, only_forward.backward.next());
        assert_eq!(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)), only_forward.forward.next().unwrap().start);
        assert_eq!(Moment(Local.ymd(2017, 04, 26).and_hms(9, 10, 11)), only_forward.forward.next().unwrap().start);
        assert_eq!(None, only_forward.forward.next());
    }

    #[test]
    fn test_interval_bidirectional_forward_closure() {
        let anchor = Interval::starting_at(
                Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)), 
                Grain::Second
        );

        let mut only_forward = BidirectionalIterator::new().forward_with(anchor, |prev| prev + PeriodComp::days(1));

        assert_eq!(None, only_forward.backward.next());
        assert_eq!(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)), only_forward.forward.next().unwrap().start);
        assert_eq!(Moment(Local.ymd(2017, 04, 26).and_hms(9, 10, 11)), only_forward.forward.next().unwrap().start);
    }

    #[test]
    fn test_interval_bidirectional_backward_values() {
        let values = vec![
            Interval::starting_at(
                Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)), 
                Grain::Second
            ),
            Interval::starting_at(
                Moment(Local.ymd(2017, 04, 24).and_hms(9, 10, 11)),
                Grain::Second
            )
        ];

        let mut only_backward = BidirectionalIterator::new()
                    .backward(values);

        assert_eq!(None, only_backward.forward.next());
        assert_eq!(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)), only_backward.backward.next().unwrap().start);
        assert_eq!(Moment(Local.ymd(2017, 04, 24).and_hms(9, 10, 11)), only_backward.backward.next().unwrap().start);
        assert_eq!(None, only_backward.backward.next());
    }

    #[test]
    fn test_interval_bidirectional_backward_closure() {
        let anchor = Interval::starting_at(
                Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)), 
                Grain::Second
        );

        let mut only_backward = BidirectionalIterator::new().backward_with(anchor, |prev| prev - PeriodComp::days(1));

        assert_eq!(None, only_backward.forward.next());
        assert_eq!(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)), only_backward.backward.next().unwrap().start);
        assert_eq!(Moment(Local.ymd(2017, 04, 24).and_hms(9, 10, 11)), only_backward.backward.next().unwrap().start);
    }
}
