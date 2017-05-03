use std::rc::Rc;

#[derive(Clone)]
pub enum Walker<V: Copy + Clone> {
    Vec(Vec<V>),
    Generator {
        current: V,
        transform: Rc<Fn(V) -> V>,
    },
    Map {
        inner: Box<Walker<V>>,
        transform: Rc<Fn(V) -> V>,
    },
    Filter {
        inner: Box<Walker<V>>,
        predicate: Rc<Fn(&V) -> bool>,
    },
    FilterMap {
        inner: Box<Walker<V>>,
        transform: Rc<Fn(V) -> Option<V>>,
    },
    FlatMap {
        inner: Box<Walker<V>>,
        transform: Rc<Fn(V) -> Walker<V>>,
        current: Option<Box<Walker<V>>>,
    },
    TakeWhile {
        inner: Box<Walker<V>>,
        flag: bool,
        predicate: Rc<Fn(&V) -> bool>,
    },
    SkipWhile {
        inner: Box<Walker<V>>,
        flag: bool,
        predicate: Rc<Fn(&V) -> bool>,
    },
    Skip { inner: Box<Walker<V>>, n: usize },
    Take { inner: Box<Walker<V>>, n: usize },
}

impl<V: Copy + Clone> Walker<V> {
    pub fn vec(mut vec: Vec<V>) -> Walker<V> {
        vec.reverse();
        Walker::Vec(vec)
    }

    pub fn generator<F>(anchor: V, transform: F) -> Walker<V>
        where F: Fn(V) -> V + 'static
    {
        Walker::Generator {
            current: anchor,
            transform: Rc::new(transform),
        }
    }

    pub fn map<F>(&self, transform: F) -> Walker<V>
        where F: Fn(V) -> V + 'static
    {
        Walker::Map {
            inner: Box::new(self.clone()),
            transform: Rc::new(transform),
        }
    }

    pub fn filter<F>(&self, predicate: F) -> Walker<V>
        where F: Fn(&V) -> bool + 'static
    {
        Walker::Filter {
            inner: Box::new(self.clone()),
            predicate: Rc::new(predicate),
        }
    }

    pub fn filter_map<F>(&self, transform: F) -> Walker<V>
        where F: Fn(V) -> Option<V> + 'static
    {
        Walker::FilterMap {
            inner: Box::new(self.clone()),
            transform: Rc::new(transform),
        }
    }

    pub fn flat_map<F>(&self, transform: F) -> Walker<V>
        where F: Fn(V) -> Walker<V> + 'static
    {
        Walker::FlatMap {
            inner: Box::new(self.clone()),
            transform: Rc::new(transform),
            current: None,
        }
    }

    pub fn take_while<F>(&self, predicate: F) -> Walker<V>
        where F: Fn(&V) -> bool + 'static
    {
        Walker::TakeWhile {
            inner: Box::new(self.clone()),
            flag: false,
            predicate: Rc::new(predicate),
        }
    }

    pub fn skip_while<F>(&self, predicate: F) -> Walker<V>
        where F: Fn(&V) -> bool + 'static
    {
        Walker::SkipWhile {
            inner: Box::new(self.clone()),
            flag: false,
            predicate: Rc::new(predicate),
        }
    }

    pub fn skip(&self, n: usize) -> Walker<V> {
        Walker::Skip {
            inner: Box::new(self.clone()),
            n: n,
        }
    }

    pub fn take(&self, n: usize) -> Walker<V> {
        Walker::Take {
            inner: Box::new(self.clone()),
            n: n,
        }
    }

    pub fn next(&mut self) -> Option<V> {
        match self {
            &mut Walker::Vec(ref mut vec) => vec.pop(),
            &mut Walker::Generator {
                     ref mut current,
                     ref transform,
                 } => {
                let result = *current;
                *current = transform(*current);
                Some(result)
            }
            &mut Walker::Map {
                     ref mut inner,
                     ref transform,
                 } => inner.next().map(|it| transform(it)),
            &mut Walker::Filter {
                     ref mut inner,
                     ref predicate,
                 } => {
                while let Some(it) = inner.next() {
                    if predicate(&it) {
                        return Some(it);
                    }
                }
                None
            }
            &mut Walker::FilterMap {
                     ref mut inner,
                     ref transform,
                 } => {
                while let Some(it) = inner.next() {
                    if let Some(it) = transform(it) {
                        return Some(it);
                    }
                }
                None
            }
            &mut Walker::FlatMap {
                     ref mut inner,
                     ref transform,
                     ref mut current,
                 } => {
                while let Some(walker) =
                    current
                        .take()
                        .or_else(|| inner.next().map(|i| Box::new(transform(i)))) {
                    *current = Some(walker);
                    if let Some(item) = current.as_mut().unwrap().next() {
                        return Some(item);
                    } else {
                        *current = None
                    }
                }
                None
            }
            &mut Walker::TakeWhile {
                     ref mut inner,
                     ref mut flag,
                     ref predicate,
                 } => {
                if *flag {
                    None
                } else {
                    inner
                        .next()
                        .and_then(|x| if predicate(&x) {
                                      Some(x)
                                  } else {
                                      *flag = true;
                                      None
                                  })
                }
            }
            &mut Walker::SkipWhile {
                     ref mut inner,
                     ref mut flag,
                     ref predicate,
                 } => {
                while let Some(x) = inner.next() {
                    if *flag || !predicate(&x) {
                        *flag = true;
                        return Some(x);
                    }
                }
                None
            }
            &mut Walker::Skip {
                     ref mut inner,
                     ref mut n,
                 } => {
                if *n == 0 {
                    inner.next()
                } else {
                    let mut counter = *n;
                    *n = 0;
                    while let Some(x) = inner.next() {
                        if counter == 0 {
                            return Some(x);
                        };
                        counter -= 1;
                    }
                    None
                }
            }
            &mut Walker::Take {
                     ref mut inner,
                     ref mut n,
                 } => {
                if *n == 0 {
                    None
                } else {
                    *n -= 1;
                    inner.next()
                }
            }
        }
    }
}

pub struct WalkerIter<V: Copy + Clone>(Walker<V>);

impl<V: Copy + Clone> Iterator for WalkerIter<V> {
    type Item = V;
    fn next(&mut self) -> Option<V> {
        self.0.next()
    }
}

impl<V: Copy + Clone> ::std::iter::IntoIterator for Walker<V> {
    type Item = V;
    type IntoIter = WalkerIter<V>;
    fn into_iter(self) -> Self::IntoIter {
        WalkerIter(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! w {
        ($($e:expr),*) => { Walker::vec(vec!($($e),*)) }
    }

    #[test]
    fn test_map() {
        let ints = w![1usize, 2, 3];
        let other_ints = ints.map(|i| i + 2);
        let other_other_ints = other_ints.map(|i| i + 2);
        assert_eq!(vec![1usize, 2, 3], ints.into_iter().collect::<Vec<_>>());
        assert_eq!(vec![3, 4, 5], other_ints.into_iter().collect::<Vec<_>>());
        assert_eq!(vec![5, 6, 7],
                   other_other_ints.into_iter().collect::<Vec<_>>());
    }

    #[test]
    fn test_filter() {
        let ints = w![1usize, 2, 3, 4, 5, 6, 7];
        let even = ints.filter(|i| i % 2 == 0);
        let even_mul3 = even.filter(|i| i % 3 == 0);
        assert_eq!(vec![2, 4, 6], even.into_iter().collect::<Vec<_>>());
        assert_eq!(vec![6], even_mul3.into_iter().collect::<Vec<_>>());
    }

    #[test]
    fn test_cloneable_take_while() {
        let ints = w![1usize, 2, 3, 4, 5, 3, 2, 1];
        let tw = ints.take_while(|&i| i <= 3);
        assert_eq!(vec![1, 2, 3], tw.into_iter().collect::<Vec<_>>());
    }

    #[test]
    fn test_cloneable_skip_while() {
        let ints = w![1usize, 2, 3, 4, 5, 3, 2, 1];
        let tw = ints.skip_while(|&i| i <= 3);
        assert_eq!(vec![4, 5, 3, 2, 1], tw.into_iter().collect::<Vec<_>>());
    }

    #[test]
    fn test_skip() {
        let ints = w![1usize, 2, 3, 4, 5, 3, 2, 1];
        let tw = ints.skip(5);
        assert_eq!(vec![3, 2, 1], tw.into_iter().collect::<Vec<_>>());
    }

    #[test]
    fn test_take() {
        let ints = w![1usize, 2, 3, 4, 5, 3, 2, 1];
        let take = ints.take(5);
        assert_eq!(vec![1, 2, 3, 4, 5], take.into_iter().collect::<Vec<_>>());
    }

    #[test]
    fn test_filter_map() {
        let ints = w![1usize, 2, 3, 4, 5, 3, 2, 1];
        let filter_map = ints.filter_map(|i| if i % 2 == 0 { Some(3 * i) } else { None });
        assert_eq!(vec![6, 12, 6], filter_map.into_iter().collect::<Vec<_>>());
    }

    #[test]
    fn test_flat_map() {
        fn f(i: usize) -> Walker<usize> {
            Walker::<usize>::vec(vec![1; i])
        }
        assert_eq!(vec![0;0],
                   w![0usize].flat_map(f).into_iter().collect::<Vec<_>>());
        assert_eq!(vec![1],
                   w![1usize].flat_map(f).into_iter().collect::<Vec<_>>());
        assert_eq!(vec![1],
                   w![1usize, 0]
                       .flat_map(f)
                       .into_iter()
                       .collect::<Vec<_>>());
        assert_eq!(vec![1],
                   w![0usize, 1]
                       .flat_map(f)
                       .into_iter()
                       .collect::<Vec<_>>());
        assert_eq!(vec![1, 1, 1],
                   w![1usize, 2]
                       .flat_map(f)
                       .into_iter()
                       .collect::<Vec<_>>());
        assert_eq!(vec![1, 1, 1],
                   w![1usize, 0, 2]
                       .flat_map(f)
                       .into_iter()
                       .collect::<Vec<_>>());
        assert_eq!(vec![1, 1, 1],
                   w![1usize, 0, 2, 0]
                       .flat_map(f)
                       .into_iter()
                       .collect::<Vec<_>>());
        assert_eq!(vec![1, 1, 1, 1],
                   w![1usize, 0, 2, 0, 1]
                       .flat_map(f)
                       .into_iter()
                       .collect::<Vec<_>>());
    }

}
