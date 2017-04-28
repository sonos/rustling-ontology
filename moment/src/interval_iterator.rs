use ::Interval;
use std::vec::IntoIter;
use std::rc;

#[derive(Clone)]
pub struct IntervalIterator {
    anchor: Interval,
    transform: rc::Rc<Fn(Interval) -> Interval>,
}

impl IntervalIterator {
    pub fn new<F: Fn(Interval) -> Interval+'static>(anchor: Interval, transform: F) -> IntervalIterator {
        IntervalIterator {
            anchor: anchor,
            transform: rc::Rc::new(transform),
        }
    }
}

impl Iterator for IntervalIterator {
    type Item = Interval;

    fn next(&mut self) -> Option<Interval> {
        let current = self.anchor;
        self.anchor = (self.transform)(current);
        Some(current)
    }
}

#[derive(Debug,Clone, PartialEq)]
struct EmptyIntervalIterator;

impl Iterator for EmptyIntervalIterator {
    type Item = Interval;

    fn next(&mut self) -> Option<Interval> {
        None
    }
}

pub trait BidirectionalIterator {
    fn forward_iter(&self) -> Box<Iterator<Item=Interval>>;
    fn backward_iter(&self) -> Box<Iterator<Item=Interval>>;
}


#[derive(Debug,Clone, PartialEq)]
pub struct BidirectionalIter<F, B> 
    where F: Iterator<Item=Interval>+Clone,
          B: Iterator<Item=Interval>+Clone,
    {
    forward: F,
    backward: B,
}

impl<F, B> BidirectionalIterator for BidirectionalIter<F, B>
    where F: Iterator<Item=Interval>+Clone+ 'static,
          B: Iterator<Item=Interval>+Clone+ 'static,
    {
    fn forward_iter(&self) -> Box<Iterator<Item=Interval>> {
        Box::new(self.forward.clone())
    }

    fn backward_iter(&self) -> Box<Iterator<Item=Interval>> {
        Box::new(self.backward.clone())
    }
}

impl BidirectionalIter<EmptyIntervalIterator, EmptyIntervalIterator> {
    pub fn new() -> BidirectionalIter<EmptyIntervalIterator, EmptyIntervalIterator> {
        BidirectionalIter {
            forward: EmptyIntervalIterator,
            backward: EmptyIntervalIterator,
        }  
    }
}

impl<F, B> BidirectionalIter<F, B>
    where F: Iterator<Item=Interval>+Clone,
          B: Iterator<Item=Interval>+Clone,
    {

    pub fn forward(self, iterator: IntervalIterator) -> BidirectionalIter<IntervalIterator, B> {
        BidirectionalIter {
            forward: iterator,
            backward: self.backward,
        } 
    }

    pub fn forward_values(self, values: Vec<Interval>) -> BidirectionalIter<IntoIter<Interval>, B> {
        BidirectionalIter {
            forward: values.into_iter(),
            backward: self.backward,
        } 
    }

    pub fn forward_with<FP>(self, anchor: Interval, transform: FP) -> BidirectionalIter<IntervalIterator, B> 
    where FP: Fn(Interval) -> Interval + 'static {
        BidirectionalIter {
            forward: IntervalIterator::new(anchor, transform),
            backward: self.backward,
        }   
    }

    pub fn backward(self, iterator: IntervalIterator) -> BidirectionalIter<F, IntervalIterator> {
        BidirectionalIter {
            forward: self.forward,
            backward: iterator,
        } 
    }

    pub fn backward_values(self, values: Vec<Interval>) -> BidirectionalIter<F, IntoIter<Interval>> {
        BidirectionalIter {
            forward: self.forward,
            backward: values.into_iter(),
        }
    }

    pub fn backward_with<BP: Fn(Interval) -> Interval+'static>(self, anchor: Interval, transform: BP) -> BidirectionalIter<F, IntervalIterator> {
        BidirectionalIter {
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

        let mut bidirectional = BidirectionalIter::new()
                    .forward_with(anchor, |prev| prev + PeriodComp::days(1))
                    .backward_with(anchor - PeriodComp::days(1), |prev| prev - PeriodComp::days(1));

        assert_eq!(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)), bidirectional.forward.next().unwrap().start);
        assert_eq!(Moment(Local.ymd(2017, 04, 26).and_hms(9, 10, 11)), bidirectional.forward.next().unwrap().start);
        assert_eq!(Moment(Local.ymd(2017, 04, 24).and_hms(9, 10, 11)), bidirectional.backward.next().unwrap().start);
        assert_eq!(Moment(Local.ymd(2017, 04, 23).and_hms(9, 10, 11)), bidirectional.backward.next().unwrap().start);
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

        let mut only_forward = BidirectionalIter::new()
                    .forward_values(values);

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

        let mut only_forward = BidirectionalIter::new().forward_with(anchor, |prev| prev + PeriodComp::days(1));

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

        let mut only_backward = BidirectionalIter::new()
                    .backward_values(values);

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

        let mut only_backward = BidirectionalIter::new().backward_with(anchor, |prev| prev - PeriodComp::days(1));

        assert_eq!(None, only_backward.forward.next());
        assert_eq!(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)), only_backward.backward.next().unwrap().start);
        assert_eq!(Moment(Local.ymd(2017, 04, 24).and_hms(9, 10, 11)), only_backward.backward.next().unwrap().start);
    }
}
