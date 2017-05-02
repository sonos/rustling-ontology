use std::iter;
use std::rc::Rc;
use Interval;

pub trait CloneableIterator {
    fn dup(&self) -> Box<CloneableIterator>;

    fn next(&mut self) -> Option<Interval> {
        unimplemented!();
    }

    fn cloneable_map<F>(self, f: F) -> CMap<Self, F>
        where F: Fn(Interval) -> Interval, Self: Sized
    {
        CMap {
            iter: self,
            f: Rc::new(f),
        }
    }

    fn cloneable_filter<F>(self, f: F) -> CFilter<Self, F>
        where F: Fn(&Interval) -> bool + 'static, Self: Sized
    {
        CFilter {
            iter: self,
            f: Rc::new(f),
        }
    }

    fn cloneable_take_while<F>(self, f: F) -> CTakeWhile<Self, F>
        where F: Fn(&Interval) -> bool + 'static, Self: Sized
    {
        CTakeWhile {
            iter: self,
            flag: false,
            predicate: Rc::new(f),
        }
    }

    fn cloneable_skip_while<F>(self, f: F) -> CSkipWhile<Self, F>
        where F: Fn(&Interval) -> bool + 'static, Self: Sized
    {
        CSkipWhile {
            iter: self,
            flag: false,
            predicate: Rc::new(f),
        }
    }

}


/// ------ CMAP ----

pub struct CMap<Inner, F>
    where Inner: CloneableIterator,
          F: Fn(Interval) -> Interval + 'static
{
    iter: Inner,
    f: Rc<F>,
}


impl<Inner, F> CloneableIterator for CMap<Inner, F>
    where Inner: CloneableIterator + Clone,
          F: Fn(Interval) -> Interval
{
    fn dup(&self) -> Box<CloneableIterator> {
        unimplemented!();
        //Box::new(
        //CMap {
        //    iter: self.iter.clone(),
        //    f: self.f.clone(),
        //})
    }

    fn next(&mut self) -> Option<Interval> {
        if let Some(it) = self.iter.next() {
            Some((self.f)(it))
        } else {
            None
        }
    }
}


impl<Inner, F> Clone for CMap<Inner, F>
    where Inner: CloneableIterator + Clone,
          F: Fn(Interval) -> Interval
{
    fn clone(&self) -> CMap<Inner, F> {
        CMap {
            iter: self.iter.clone(),
            f: self.f.clone(),
        }
    }
}


/// ------ CFilter ----

pub struct CFilter<Iter, F>
    where Iter: CloneableIterator + 'static,
          F: Fn(&Interval) -> bool + 'static
{
    iter: Iter,
    f: Rc<F>,
}

impl<Inner, F> CloneableIterator for CFilter<Inner, F>
    where Inner: CloneableIterator + Clone + 'static,
          F: Fn(&Interval) -> bool + 'static
{
    fn dup(&self) -> Box<CloneableIterator> {
        Box::new(
        CFilter {
            iter: self.iter.clone(),
            f: self.f.clone(),
        })
    }

    #[inline]
    fn next(&mut self) -> Option<Interval> {
        while let Some(it) = self.iter.next() {
            if (self.f)(&it) {
                return Some(it);
            }
        }
        None
    }
}

impl<Inner, F> Clone for CFilter<Inner, F>
    where Inner: CloneableIterator + Clone,
          F: Fn(&Interval) -> bool
{
    fn clone(&self) -> CFilter<Inner, F> {
        CFilter {
            iter: self.iter.clone(),
            f: self.f.clone(),
        }
    }
}

/// ------ CTakeWhile ----

pub struct CTakeWhile<I, P> {
    iter: I,
    flag: bool,
    predicate: Rc<P>,
}

impl<Inner, F> CloneableIterator for CTakeWhile<Inner, F>
    where Inner: CloneableIterator + Clone + 'static,
          F: Fn(&Interval) -> bool + 'static
{
    fn dup(&self) -> Box<CloneableIterator> {
        Box::new(
        CTakeWhile {
            iter: self.iter.clone(),
            flag: self.flag,
            predicate: self.predicate.clone(),
        })
    }

    #[inline]
    fn next(&mut self) -> Option<Interval> {
        if self.flag {
            None
        } else {
            self.iter.next().and_then(|x| {
                if (self.predicate)(&x) {
                    Some(x)
                } else {
                    self.flag = true;
                    None
                }
            })
        }
    }
}

impl<Inner, F> Clone for CTakeWhile<Inner, F>
    where Inner: CloneableIterator + Clone,
          F: Fn(&Interval) -> bool
{
    fn clone(&self) -> CTakeWhile<Inner, F> {
        CTakeWhile {
            iter: self.iter.clone(),
            flag: self.flag,
            predicate: self.predicate.clone(),
        }
    }
}

/// ------ CSkipWhile ----

pub struct CSkipWhile<I, P> {
    iter: I,
    flag: bool,
    predicate: Rc<P>,
}

impl<Inner, F> CloneableIterator for CSkipWhile<Inner, F>
    where Inner: CloneableIterator + Clone + 'static,
          F: Fn(Interval) -> bool + 'static
{
    fn dup(&self) -> Box<CloneableIterator> {
        Box::new(
        CSkipWhile {
            iter: self.iter.clone(),
            flag: self.flag,
            predicate: self.predicate.clone(),
        })
    }

    #[inline]
    fn next(&mut self) -> Option<Interval> {
        unimplemented!();
        //for x in self.iter.as_ref() {
        //    if self.flag || !(self.predicate)(x) {
        //        self.flag = true;
        //        return Some(x);
        //    }
        //}
        None
    }
}

impl<Inner, F> Clone for CSkipWhile<Inner, F>
    where Inner: CloneableIterator + Clone,
          F: Fn(&Interval) -> bool
{
    fn clone(&self) -> CSkipWhile<Inner, F> {
        CSkipWhile {
            iter: self.iter.clone(),
            flag: self.flag,
            predicate: self.predicate.clone(),
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use super::CloneableIterator;

    #[derive(Clone)]
    struct Wrapper<CI: Iterator<Item=Interval> + Clone>(CI);
    impl<CI> CloneableIterator for Wrapper<CI> where CI: Iterator<Item=Interval> + Clone + 'static {
        fn dup(&self) -> Box<CloneableIterator> {
            Box::new(self.clone())
        }

        fn next(&mut self) -> Option<Interval> {
            self.0.next()
        }
    }

    #[test]
    fn test_cloneable_map() {
        let ints = Wrapper(vec![1usize, 2, 3].into_iter());
        let other_ints = ints.dup().cloneable_map(|i| i + 2);
        let other_other_ints = other_ints.dup().cloneable_map(|i| i + 2);
        //assert_eq!(vec![1usize, 2, 3], ints.collect::<Vec<_>>());
        //assert_eq!(vec![3, 4, 5], other_ints.collect::<Vec<_>>());
        //assert_eq!(vec![5, 6, 7], other_other_ints.collect::<Vec<_>>());
    }

    #[test]
    fn test_cloneable_filter() {
        let ints = Wrapper(vec![1usize, 2, 3, 4, 5, 6, 7].into_iter());
        let even = ints.dup().cloneable_filter(|i| i % 2 == 0);
        let even_mul3 = even.dup().cloneable_filter(|i| i % 3 == 0);
        //assert_eq!(vec![2, 4, 6], even.collect::<Vec<_>>());
        //assert_eq!(vec![6], even_mul3.collect::<Vec<_>>());
    }

    #[test]
    fn test_cloneable_take_while() {
        let ints = Wrapper(vec![1usize, 2, 3, 4, 5, 3, 2, 1].into_iter());
        let tw = ints.cloneable_take_while(|&i| i <= 3);
        //assert_eq!(vec![1, 2, 3], tw.collect::<Vec<_>>());
    }

    #[test]
    fn test_cloneable_skip_while() {
        let ints = Wrapper(vec![1usize, 2, 3, 4, 5, 3, 2, 1].into_iter());
        let tw = ints.cloneable_skip_while(|&i| i <= 3);
        //assert_eq!(vec![4, 5, 3, 2, 1], tw.collect::<Vec<_>>());
    }
}
*/
