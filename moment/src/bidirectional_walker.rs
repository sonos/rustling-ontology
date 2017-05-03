use walker::*;

#[derive(Clone)]
pub struct BidirectionalWalker<V: Copy+Clone>
{
    pub forward: Walker<V>,
    pub backward: Walker<V>,
}

impl<V: Copy+Clone> BidirectionalWalker<V>
{

    pub fn new() -> BidirectionalWalker<V> {
        BidirectionalWalker {
            forward: Walker::Vec(vec![]),
            backward: Walker::Vec(vec![]),
        }
    }

    pub fn forward(self, combiner: Walker<V>) -> BidirectionalWalker<V> {
        BidirectionalWalker {
            forward: combiner,
            backward: self.backward,
        }
    }

    pub fn forward_values(self, values: Vec<V>) -> BidirectionalWalker<V> {
        BidirectionalWalker {
            forward: Walker::vec(values),
            backward: self.backward,
        }
    }

    pub fn forward_with<FP>(self,
                            anchor: V,
                            transform: FP)
                            ->  BidirectionalWalker<V>
        where FP: Fn(V) -> V + 'static
    {
        BidirectionalWalker {
            forward: Walker::generator(anchor, transform),
            backward: self.backward,
        }
    }

    pub fn backward(self, combiner: Walker<V>) -> BidirectionalWalker<V> {
        BidirectionalWalker {
            forward: self.forward,
            backward: combiner,
        }
    }

    pub fn backward_values(self,
                           values: Vec<V>)
                           -> BidirectionalWalker<V> {
        BidirectionalWalker {
            forward: self.forward,
            backward: Walker::vec(values),
        }
    }

    pub fn backward_with<BP>
        (self,
         anchor: V,
         transform: BP)
         -> BidirectionalWalker<V>
        where BP: Fn(V) -> V + 'static 
    {
        BidirectionalWalker {
            forward: self.forward,
            backward: Walker::generator(anchor, transform),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use Moment;
    use Interval;
    use period::*;
    use chrono::offset::local::Local;
    use chrono::TimeZone;

    #[test]
    fn test_interval_iterator() {
        let now = Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11));
        let interval = Interval::starting_at(now, Grain::Second);
        let mut iterator = Walker::generator(interval, |prev| prev + PeriodComp::days(1));

        assert_eq!(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)),
                   iterator.next().unwrap().start);
        assert_eq!(Moment(Local.ymd(2017, 04, 26).and_hms(9, 10, 11)),
                   iterator.next().unwrap().start);
        assert_eq!(Moment(Local.ymd(2017, 04, 27).and_hms(9, 10, 11)),
                   iterator.next().unwrap().start);
        assert_eq!(Moment(Local.ymd(2017, 04, 28).and_hms(9, 10, 11)),
                   iterator.next().unwrap().start);
    }

    #[test]
    fn test_interval_bidirectional() {
        let anchor = Interval::starting_at(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)),
                                           Grain::Second);

        let mut bidirectional = BidirectionalWalker::new()
            .forward_with(anchor, |prev| prev + PeriodComp::days(1))
            .backward_with(anchor - PeriodComp::days(1),
                           |prev| prev - PeriodComp::days(1));

        assert_eq!(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)),
                   bidirectional.forward.next().unwrap().start);
        assert_eq!(Moment(Local.ymd(2017, 04, 26).and_hms(9, 10, 11)),
                   bidirectional.forward.next().unwrap().start);
        assert_eq!(Moment(Local.ymd(2017, 04, 24).and_hms(9, 10, 11)),
                   bidirectional.backward.next().unwrap().start);
        assert_eq!(Moment(Local.ymd(2017, 04, 23).and_hms(9, 10, 11)),
                   bidirectional.backward.next().unwrap().start);
    }

    #[test]
    fn test_interval_bidirectional_forward_values() {
        let values =
            vec![Interval::starting_at(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)),
                                       Grain::Second),
                 Interval::starting_at(Moment(Local.ymd(2017, 04, 26).and_hms(9, 10, 11)),
                                       Grain::Second)];

        let mut only_forward = BidirectionalWalker::new().forward_values(values);

        assert_eq!(None, only_forward.backward.next());
        assert_eq!(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)),
                   only_forward.forward.next().unwrap().start);
        assert_eq!(Moment(Local.ymd(2017, 04, 26).and_hms(9, 10, 11)),
                   only_forward.forward.next().unwrap().start);
        assert_eq!(None, only_forward.forward.next());
    }

    #[test]
    fn test_interval_bidirectional_forward_closure() {
        let anchor = Interval::starting_at(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)),
                                           Grain::Second);

        let mut only_forward =
            BidirectionalWalker::new().forward_with(anchor, |prev| prev + PeriodComp::days(1));

        assert_eq!(None, only_forward.backward.next());
        assert_eq!(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)),
                   only_forward.forward.next().unwrap().start);
        assert_eq!(Moment(Local.ymd(2017, 04, 26).and_hms(9, 10, 11)),
                   only_forward.forward.next().unwrap().start);
    }

    #[test]
    fn test_interval_bidirectional_backward_values() {
        let values =
            vec![Interval::starting_at(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)),
                                       Grain::Second),
                 Interval::starting_at(Moment(Local.ymd(2017, 04, 24).and_hms(9, 10, 11)),
                                       Grain::Second)];

        let mut only_backward = BidirectionalWalker::new().backward_values(values);

        assert_eq!(None, only_backward.forward.next());
        assert_eq!(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)),
                   only_backward.backward.next().unwrap().start);
        assert_eq!(Moment(Local.ymd(2017, 04, 24).and_hms(9, 10, 11)),
                   only_backward.backward.next().unwrap().start);
        assert_eq!(None, only_backward.backward.next());
    }

    #[test]
    fn test_interval_bidirectional_backward_closure() {
        let anchor = Interval::starting_at(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)),
                                           Grain::Second);

        let mut only_backward =
            BidirectionalWalker::new().backward_with(anchor, |prev| prev - PeriodComp::days(1));

        assert_eq!(None, only_backward.forward.next());
        assert_eq!(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)),
                   only_backward.backward.next().unwrap().start);
        assert_eq!(Moment(Local.ymd(2017, 04, 24).and_hms(9, 10, 11)),
                   only_backward.backward.next().unwrap().start);
    }
}

