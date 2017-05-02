use std::iter;
use std::rc::Rc;

pub trait CloneableIterator: Clone + Iterator {
    fn clonable_map<B, F>(self, f: F) -> CMap<B, Self, F>
        where F: Fn(Self::Item) -> B
    {
        CMap {
            iter: self,
            f: Rc::new(f),
        }
    }

    fn clonable_filter<F>(self, f: F) -> CFilter<Self, F>
        where F: Fn(&Self::Item) -> bool
    {
        CFilter {
            iter: self,
            f: Rc::new(f),
        }
    }

    fn clonable_take_while<F>(self, f: F) -> CTakeWhile<Self, F>
        where F: Fn(&Self::Item) -> bool
    {
        CTakeWhile {
            iter: self,
            flag: false,
            predicate: Rc::new(f),
        }
    }

    fn clonable_skip_while<F>(self, f: F) -> CSkipWhile<Self, F>
        where F: Fn(&Self::Item) -> bool
    {
        CSkipWhile {
            iter: self,
            flag: false,
            predicate: Rc::new(f),
        }
    }
}

/// ------ CMAP ----

pub struct CMap<B, Inner, F>
    where Inner: CloneableIterator,
          F: Fn(Inner::Item) -> B
{
    iter: Inner,
    f: Rc<F>,
}

impl<B, Inner, F> iter::Iterator for CMap<B, Inner, F>
    where Inner: CloneableIterator,
          F: Fn(Inner::Item) -> B
{
    type Item = B;
    #[inline]
    fn next(&mut self) -> Option<B> {
        if let Some(it) = self.iter.next() {
            Some((self.f)(it))
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn fold<Acc, G>(self, init: Acc, mut g: G) -> Acc
        where G: FnMut(Acc, Self::Item) -> Acc
    {
        let f = self.f;
        self.iter.fold(init, move |acc, elt| g(acc, f(elt)))
    }
}

impl<B, Inner, F> Clone for CMap<B, Inner, F>
    where Inner: CloneableIterator,
          F: Fn(Inner::Item) -> B
{
    fn clone(&self) -> Self {
        CMap {
            iter: self.iter.clone(),
            f: self.f.clone(),
        }
    }
}

impl<B, Inner, F> CloneableIterator for CMap<B, Inner, F>
    where Inner: CloneableIterator,
          F: Fn(Inner::Item) -> B
{
}

/// ------ CFilter ----

pub struct CFilter<Iter, F>
    where Iter: CloneableIterator,
          F: Fn(&Iter::Item) -> bool
{
    iter: Iter,
    f: Rc<F>,
}

impl<Iter, F> iter::Iterator for CFilter<Iter, F>
    where Iter: CloneableIterator,
          F: Fn(&Iter::Item) -> bool
{
    type Item = Iter::Item;

    #[inline]
    fn next(&mut self) -> Option<Iter::Item> {
        while let Some(it) = self.iter.next() {
            if (self.f)(&it) {
                return Some(it);
            }
        }
        None
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.size_hint();
        (0, upper) // can't know a lower bound, due to the predicate
    }
}

impl<Iter, F> Clone for CFilter<Iter, F>
    where Iter: CloneableIterator,
          F: Fn(&Iter::Item) -> bool
{
    fn clone(&self) -> Self {
        CFilter {
            iter: self.iter.clone(),
            f: self.f.clone(),
        }
    }
}

impl<Iter, F> CloneableIterator for CFilter<Iter, F>
    where Iter: CloneableIterator,
          F: Fn(&Iter::Item) -> bool
{
}

/// ------ CTakeWhile ----

pub struct CTakeWhile<I, P> {
    iter: I,
    flag: bool,
    predicate: Rc<P>,
}

impl<Inner, F> iter::Iterator for CTakeWhile<Inner, F>
    where Inner: CloneableIterator,
          F: Fn(&Inner::Item) -> bool
{
    type Item = Inner::Item;

    #[inline]
    fn next(&mut self) -> Option<Inner::Item> {
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

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.size_hint();
        (0, upper) // can't know a lower bound, due to the predicate
    }
}

impl<Inner, F> Clone for CTakeWhile<Inner, F>
    where Inner: CloneableIterator,
          F: Fn(&Inner::Item) -> bool
{
    fn clone(&self) -> Self {
        CTakeWhile {
            iter: self.iter.clone(),
            flag: self.flag,
            predicate: self.predicate.clone(),
        }
    }
}

impl<Inner, F> CloneableIterator for CTakeWhile<Inner, F>
    where Inner: CloneableIterator,
          F: Fn(&Inner::Item) -> bool
{
}

/// ------ CSkipWhile ----

pub struct CSkipWhile<I, P> {
    iter: I,
    flag: bool,
    predicate: Rc<P>,
}

impl<Inner, F> iter::Iterator for CSkipWhile<Inner, F>
    where Inner: CloneableIterator,
          F: Fn(&Inner::Item) -> bool
{
    type Item = Inner::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        for x in self.iter.by_ref() {
            if self.flag || !(self.predicate)(&x) {
                self.flag = true;
                return Some(x);
            }
        }
        None
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.size_hint();
        (0, upper) // can't know a lower bound, due to the predicate
    }
}

impl<Inner, F> Clone for CSkipWhile<Inner, F>
    where Inner: CloneableIterator,
          F: Fn(&Inner::Item) -> bool
{
    fn clone(&self) -> Self {
        CSkipWhile {
            iter: self.iter.clone(),
            flag: self.flag,
            predicate: self.predicate.clone(),
        }
    }
}

impl<Inner, F> CloneableIterator for CSkipWhile<Inner, F>
    where Inner: CloneableIterator,
          F: Fn(&Inner::Item) -> bool
{
}

#[cfg(test)]
mod tests {
    use super::CloneableIterator;

    #[derive(Clone)]
    struct Wrapper<CI: Iterator + Clone>(CI);
    impl<CI> CloneableIterator for Wrapper<CI> where CI: Iterator + Clone {}
    impl<CI> Iterator for Wrapper<CI>
        where CI: Iterator + Clone
    {
        type Item = CI::Item;
        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }
    }

    #[test]
    fn test_cloneable_map() {
        let ints = Wrapper(vec![1usize, 2, 3].into_iter());
        let other_ints = ints.clone().clonable_map(|i| i + 2);
        let other_other_ints = other_ints.clone().clonable_map(|i| i + 2);
        assert_eq!(vec![1usize, 2, 3], ints.collect::<Vec<_>>());
        assert_eq!(vec![3, 4, 5], other_ints.collect::<Vec<_>>());
        assert_eq!(vec![5, 6, 7], other_other_ints.collect::<Vec<_>>());
    }

    #[test]
    fn test_cloneable_filter() {
        let ints = Wrapper(vec![1usize, 2, 3, 4, 5, 6, 7].into_iter());
        let even = ints.clone().clonable_filter(|i| i % 2 == 0);
        let even_mul3 = even.clone().clonable_filter(|i| i % 3 == 0);
        assert_eq!(vec![2, 4, 6], even.collect::<Vec<_>>());
        assert_eq!(vec![6], even_mul3.collect::<Vec<_>>());
    }

    #[test]
    fn test_cloneable_take_while() {
        let ints = Wrapper(vec![1usize, 2, 3, 4, 5, 3, 2, 1].into_iter());
        let tw = ints.clonable_take_while(|&i| i <= 3);
        assert_eq!(vec![1, 2, 3], tw.collect::<Vec<_>>());
    }

    #[test]
    fn test_cloneable_skip_while() {
        let ints = Wrapper(vec![1usize, 2, 3, 4, 5, 3, 2, 1].into_iter());
        let tw = ints.clonable_skip_while(|&i| i <= 3);
        assert_eq!(vec![4, 5, 3, 2, 1], tw.collect::<Vec<_>>());
    }
}
