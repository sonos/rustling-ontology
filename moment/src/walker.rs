use std::rc::Rc;

#[derive(Clone)]
pub enum Walker<V:Copy+Clone> {
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
    TakeWhile {
        inner: Box<Walker<V>>,
        flag: bool,
        predicate: Rc<Fn(&V) -> bool>,
    },
    SkipWhile {
        inner: Box<Walker<V>>,
        flag: bool,
        predicate: Rc<Fn(&V) -> bool>,
    }
}

impl<V: Copy+Clone> Walker<V> {
    pub fn vec(mut vec:Vec<V>) -> Walker<V> {
        vec.reverse();
        Walker::Vec(vec)
    }
    pub fn generator<F>(anchor: V, transform:F) -> Walker<V>
        where F:Fn(V) -> V + 'static
    {
        Walker::Generator {
            current: anchor, transform: Rc::new(transform)
        }
    }
    pub fn map<F>(&self, transform:F) -> Walker<V>
        where F:Fn(V) -> V + 'static
    {
        Walker::Map {
            inner: Box::new(self.clone()), transform: Rc::new(transform)
        }
    }
    pub fn filter<F>(&self, predicate:F) -> Walker<V>
        where F:Fn(&V) -> bool + 'static
    {
        Walker::Filter {
            inner: Box::new(self.clone()), predicate: Rc::new(predicate)
        }
    }
    pub fn take_while<F>(&self, predicate:F) -> Walker<V>
        where F:Fn(&V) -> bool + 'static
    {
        Walker::TakeWhile {
            inner: Box::new(self.clone()), flag:false, predicate: Rc::new(predicate)
        }
    }
    pub fn skip_while<F>(&self, predicate:F) -> Walker<V>
        where F:Fn(&V) -> bool + 'static
    {
        Walker::SkipWhile {
            inner: Box::new(self.clone()), flag: false, predicate: Rc::new(predicate)
        }
    }
    pub fn next(&mut self) -> Option<V> {
        match self {
            &mut Walker::Vec(ref mut vec) => {
                vec.pop()
            }
            &mut Walker::Generator { ref mut current, ref transform } => {
                let result = *current;
                *current = transform(*current);
                Some(result)
            }
            &mut Walker::Map { ref mut inner, ref transform } => {
                inner.next().map(|it| transform(it))
            }
            &mut Walker::Filter { ref mut inner, ref predicate } => {
                while let Some(it) = inner.next() {
                    if predicate(&it) {
                        return Some(it)
                    }
                }
                None
            }
            &mut Walker::TakeWhile { ref mut inner, ref mut flag, ref predicate } => {
                if *flag {
                    None
                } else {
                    inner.next().and_then(|x| {
                        if predicate(&x) {
                            Some(x)
                        } else {
                            *flag = true;
                            None
                        }
                    })
                }
            }
            &mut Walker::SkipWhile { ref mut inner, ref mut flag, ref predicate } => {
                while let Some(x) = inner.next() {
                    if *flag || !predicate(&x) {
                        *flag = true;
                        return Some(x)
                    }
                }
                None
            }
        }
    }
}

pub struct WalkerIter<V: Copy+Clone>(Walker<V>);

impl<V:Copy+Clone> Iterator for WalkerIter<V> {
    type Item = V;
    fn next(&mut self) -> Option<V> {
        self.0.next()
    }
}

impl<V: Copy+Clone> ::std::iter::IntoIterator for Walker<V> {
    type Item = V;
    type IntoIter = WalkerIter<V>;
    fn into_iter(self) -> Self::IntoIter {
        WalkerIter(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let ints = Walker::vec(vec![1usize, 2, 3]);
        let other_ints = ints.map(|i| i + 2);
        let other_other_ints = other_ints.map(|i| i + 2);
        assert_eq!(vec![1usize, 2, 3], ints.into_iter().collect::<Vec<_>>());
        assert_eq!(vec![3, 4, 5], other_ints.into_iter().collect::<Vec<_>>());
        assert_eq!(vec![5, 6, 7], other_other_ints.into_iter().collect::<Vec<_>>());
    }

    #[test]
    fn test_filter() {
        let ints = Walker::vec(vec![1usize, 2, 3, 4, 5, 6, 7]);
        let even = ints.filter(|i| i % 2 == 0);
        let even_mul3 = even.filter(|i| i % 3 == 0);
        assert_eq!(vec![2, 4, 6], even.into_iter().collect::<Vec<_>>());
        assert_eq!(vec![6], even_mul3.into_iter().collect::<Vec<_>>());
    }

    #[test]
    fn test_cloneable_take_while() {
        let ints = Walker::vec(vec![1usize, 2, 3, 4, 5, 3, 2, 1]);
        let tw = ints.take_while(|&i| i <= 3);
        assert_eq!(vec![1, 2, 3], tw.into_iter().collect::<Vec<_>>());
    }

    #[test]
    fn test_cloneable_skip_while() {
        let ints = Walker::vec(vec![1usize, 2, 3, 4, 5, 3, 2, 1]);
        let tw = ints.skip_while(|&i| i <= 3);
        assert_eq!(vec![4, 5, 3, 2, 1], tw.into_iter().collect::<Vec<_>>());
    }

}
