use std::ops;
use vec_map::VecMap;

enum_from_primitive! {
    #[derive(Debug,PartialEq,Copy,Clone,Eq,Ord,PartialOrd, Hash)]
    pub enum Grain {
        Year = 0,
        Quarter = 1,
        Month = 2,
        Week = 3,
        Day = 4,
        Hour = 5,
        Minute = 6,
        Second = 7,
    }
}

impl Grain {
    pub fn next(&self) -> Grain {
        match self {
            &Grain::Year => Grain::Month,
            &Grain::Quarter => Grain::Month,
            &Grain::Month => Grain::Day,
            &Grain::Week => Grain::Day,
            &Grain::Day => Grain::Hour,
            &Grain::Hour => Grain::Minute,
            &Grain::Minute => Grain::Second,
            &Grain::Second => Grain::Second,
        }
    }
}

impl Grain {
    pub fn all() -> Vec<Grain> {
        vec![
            Grain::Year,
            Grain::Quarter,
            Grain::Month,
            Grain::Week,
            Grain::Day,
            Grain::Hour,
            Grain::Minute,
            Grain::Second,
        ]
    }
}


#[derive(Debug, Clone, Eq, Default)]
pub struct Period(pub VecMap<i64>);

impl PartialEq for Period {
    fn eq(&self, other: &Period) -> bool {
        for grain in Grain::all() {
            let zero = 0;
            let lhs_comp = self.0.get(grain as usize).unwrap_or(&zero);
            let rhs_comp = other.0.get(grain as usize).unwrap_or(&zero);
            if lhs_comp != rhs_comp {
                return false
            }
        }
        return true
    }
}

impl Period {
    pub fn finer_grain(&self) -> Option<Grain> {
        use enum_primitive::FromPrimitive;
        self.0
            .iter()
            .max_by_key(|&(g, _)| g)
            .and_then(|(g, _)| Grain::from_usize(g))
    }

    pub fn comps(&self) -> Vec<PeriodComp> {
        use enum_primitive::FromPrimitive;
        self.0.iter()
            .filter_map(|(g, q)|  {
                if let Some(grain) = Grain::from_usize(g) {
                    Some(PeriodComp::new(grain, *q))
                } else {
                    None
                }
            }).collect()
    }
}

impl From<PeriodComp> for Period {
    fn from(pc: PeriodComp) -> Period {
        Period::default() + pc
    }
}

impl ops::AddAssign<PeriodComp> for Period {
    fn add_assign(&mut self, rhs: PeriodComp) {
        *self.0.entry(rhs.grain as usize).or_insert(0) += rhs.quantity
    }
}

impl<'a> ops::AddAssign<&'a PeriodComp> for Period {
    fn add_assign(&mut self, rhs: &'a PeriodComp) {
        *self.0.entry(rhs.grain as usize).or_insert(0) += rhs.quantity
    }
}

impl ops::Add<PeriodComp> for Period {
    type Output = Period;
    fn add(mut self, pc: PeriodComp) -> Period {
        self += pc;
        self
    }
}

impl<'a> ops::Add<&'a PeriodComp> for Period {
    type Output = Period;
    fn add(mut self, pc: &'a PeriodComp) -> Period {
        self += pc;
        self
    }
}

impl ops::Add<Period> for Period {
    type Output = Period;
    fn add(self, p: Period) -> Period {
        let mut result = Period::default();
        for i in 0..8 {
            if !self.0.get(i).is_none() || !p.0.get(i).is_none() {
                result
                    .0
                    .insert(i, *self.0.get(i).unwrap_or(&0) + *p.0.get(i).unwrap_or(&0));
            }
        }
        result
    }
}

impl<'a> ops::Add<&'a Period> for Period {
    type Output = Period;
    fn add(self, p: &'a Period) -> Period {
        let mut result = self;
        for i in 0..8 {
            if !p.0.get(i).is_none() {
                *result.0.entry(i).or_insert(0) += *p.0.get(i).unwrap_or(&0);
            }
        }
        result
    }
}

impl<'a, 'b> ops::Add<&'a Period> for &'b Period {
    type Output = Period;
    fn add(self, p: &'a Period) -> Period {
        let mut result = Period::default();
        for i in 0..8 {
            if !self.0.get(i).is_none() || !p.0.get(i).is_none() {
                result
                    .0
                    .insert(i, *self.0.get(i).unwrap_or(&0) + *p.0.get(i).unwrap_or(&0));
            }
        }
        result
    }
}

impl<'a> ops::Add<Period> for &'a Period {
    type Output = Period;
    fn add(self, p: Period) -> Period {
        p + self
    }
}

impl ops::Neg for Period {
    type Output = Period;

    fn neg(self) -> Period {
        Period(self.0.iter().map(|(k, v)| (k, -*v)).collect())
    }
}

impl<'a> ops::Neg for &'a Period {
    type Output = Period;

    fn neg(self) -> Period {
        Period(self.0.iter().map(|(k, v)| (k, -*v)).collect())
    }
}


#[derive(Debug,PartialEq,Copy,Clone,Eq)]
pub struct PeriodComp {
    pub grain: Grain,
    pub quantity: i64,
}

impl PeriodComp {
    pub fn new(grain: Grain, quantity: i64) -> PeriodComp {
        PeriodComp {
            grain: grain,
            quantity: quantity,
        }
    }

    pub fn years(n: i64) -> PeriodComp {
        PeriodComp {
            grain: Grain::Year,
            quantity: n,
        }
    }
    pub fn quarters(n: i64) -> PeriodComp {
        PeriodComp {
            grain: Grain::Quarter,
            quantity: n,
        }
    }
    pub fn months(n: i64) -> PeriodComp {
        PeriodComp {
            grain: Grain::Month,
            quantity: n,
        }
    }
    pub fn weeks(n: i64) -> PeriodComp {
        PeriodComp {
            grain: Grain::Week,
            quantity: n,
        }
    }
    pub fn days(n: i64) -> PeriodComp {
        PeriodComp {
            grain: Grain::Day,
            quantity: n,
        }
    }
    pub fn hours(n: i64) -> PeriodComp {
        PeriodComp {
            grain: Grain::Hour,
            quantity: n,
        }
    }
    pub fn minutes(n: i64) -> PeriodComp {
        PeriodComp {
            grain: Grain::Minute,
            quantity: n,
        }
    }
    pub fn seconds(n: i64) -> PeriodComp {
        PeriodComp {
            grain: Grain::Second,
            quantity: n,
        }
    }
}

impl ops::Neg for PeriodComp {
    type Output = PeriodComp;

    fn neg(self) -> PeriodComp {
        PeriodComp {
            quantity: -self.quantity,
            ..self
        }
    }
}

impl<'a> ops::Neg for &'a PeriodComp {
    type Output = PeriodComp;
    fn neg(self) -> PeriodComp {
        PeriodComp {
            quantity: -self.quantity,
            grain: self.grain,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn period_comp_add_to_period() {
        assert_eq!(Some(&1),
                   (Period::default() + PeriodComp::years(1))
                       .0
                       .get(Grain::Year as usize));
        assert_eq!(Some(&1),
                   (Period::default() + PeriodComp::days(1))
                       .0
                       .get(Grain::Day as usize));
        assert_eq!(None,
                   (Period::default() + PeriodComp::days(1))
                       .0
                       .get(Grain::Year as usize));
    }

    #[test]
    fn period_comp_add_assign_to_period() {
        let mut period = Period::default();
        period += PeriodComp::years(1);
        assert_eq!(Some(&1), period.0.get(Grain::Year as usize));
    }

    #[test]
    fn period_add_to_period() {
        let mut a = Period::default();
        a.0.insert(Grain::Year as usize, 1);

        let mut b = Period::default();
        b.0.insert(Grain::Day as usize, 2);

        let mut c = Period::default();
        c.0.insert(Grain::Day as usize, 3);
        c.0.insert(Grain::Year as usize, 5);
        c.0.insert(Grain::Month as usize, 1);

        assert_eq!(Some(&2), (&a + &b).0.get(Grain::Day as usize));
        assert_eq!(Some(&1), (a.clone() + &b).0.get(Grain::Year as usize));
        assert_eq!(Some(&6), (&a + c.clone()).0.get(Grain::Year as usize));
        assert_eq!(Some(&5), (&b + c.clone()).0.get(Grain::Day as usize));
        assert_eq!(Some(&1), (&b + &c + &a).0.get(Grain::Month as usize));
    }

    #[test]
    fn neg_period() {
        let mut a = Period::default();
        a.0.insert(Grain::Year as usize, 1);
        assert_eq!(Some(&-1), (-&a).0.get(Grain::Year as usize));
        assert_eq!(Some(&-1), (-a).0.get(Grain::Year as usize));
    }

    #[test]
    fn neg_period_comp() {
        assert_eq!(-1, (-&PeriodComp::years(1)).quantity);
        assert_eq!(Grain::Year, (-&PeriodComp::years(1)).grain);
        assert_eq!(-1, (-PeriodComp::years(1)).quantity);
        assert_eq!(Grain::Year, (-PeriodComp::years(1)).grain);
    }

    #[test]
    fn finer_grain() {
        let mut a = Period::default();
        a.0.insert(Grain::Year as usize, 1);
        a.0.insert(Grain::Month as usize, 3);
        assert_eq!(a.finer_grain(), Some(Grain::Month));

        a.0.insert(Grain::Hour as usize, 4);
        assert_eq!(a.finer_grain(), Some(Grain::Hour));
    }
}
