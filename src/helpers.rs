use rustling::*;
use dimension::*;
use moment::*;
use std::rc::Rc;

pub fn compose_numbers(a: &NumberValue, b: &NumberValue) -> RuleResult<NumberValue> {
    let grain = a.grain().unwrap_or(0) as u32;
    if 10u64.pow(grain) as f32 > b.value() {
        match (a, b) {
            (&NumberValue::Integer(ref lhs), &NumberValue::Integer(ref rhs)) => {
                Ok(NumberValue::Integer(IntegerValue::new(lhs.value + rhs.value)?))
            }
            _ => Ok(NumberValue::Float(FloatValue::new(a.value() + b.value())?)),
        }
    } else {
        Err(RuleErrorKind::Invalid.into())
    }
}

pub fn compose_money(a: &AmountOfMoneyValue,
                     b: &AmountOfMoneyValue)
                     -> RuleResult<AmountOfMoneyValue> {
    let amount = a.value + b.value / 100.0;
    Ok(AmountOfMoneyValue {
           value: amount,
           unit: a.unit,
           ..AmountOfMoneyValue::default()
       })
}

pub fn compose_money_number(a: &AmountOfMoneyValue,
                            b: &NumberValue)
                            -> RuleResult<AmountOfMoneyValue> {
    let amount = a.value + b.value() / 100.0;
    Ok(AmountOfMoneyValue {
           value: amount,
           unit: a.unit,
           ..AmountOfMoneyValue::default()
       })
}

#[derive(Clone)]
pub struct TimeValue {
    pub constraint: Rc<IntervalConstraint>,
    pub form: Form,
    pub direction: Option<Direction>,
}

#[derive(Debug, Clone)]
pub enum Form {
    Month(u32),
    Empty,
}

#[derive(Debug, Clone)]
pub enum Direction {
    After,
    Before,
}

impl TimeValue {
    pub fn constraint<Constraint: IntervalConstraint + 'static>(constraint: Constraint) -> TimeValue {
        TimeValue {
            constraint: Rc::new(constraint),
            form: Form::Empty,
            direction: None,
        }
    }

    pub fn form(self, form: Form) -> TimeValue {
        TimeValue {
            form: form,
            .. self 
        }
    }

    pub fn direction(self, direction: Option<Direction>) -> TimeValue {
        TimeValue {
            direction: direction,
            .. self
        }
    }

    pub fn intersect(self, other: TimeValue) -> TimeValue {
        //Intersection::new(self.constraint, other.constraint);
        unimplemented!();
        //TimeValue
        //    ::constraint(*self.constraint.intersect(*other.constraint))
            //.direction(self.direction.or(other.direction))
    }
}

pub fn year(y: i32) -> RuleResult<TimeValue> {
    Ok(TimeValue::constraint(Year(y)))
}

pub fn month(m: u32) -> RuleResult<TimeValue> {
    if 1 <= m && m <= 12 {
        Ok(TimeValue::constraint(Month(m)).form(Form::Month(m)))
    } else {
        unimplemented!()
        //Err(RustlingErrorKind::ProductionRuleError(format!("{} is not a valid month number", m)))
    }
}

pub fn day_of_month(dom: u32) -> RuleResult<TimeValue> {
    if 1 <= dom && dom <= 31 {
        Ok(TimeValue::constraint(DayOfMonth(dom)).form(Form::Empty))
    } else {
        unimplemented!()
        //Err(RustlingErrorKind::ProductionRuleError(format!("{} is not a valid day of month number", dom)))
    }
}

pub fn month_day(month: u32, day: u32) -> RuleResult<TimeValue> {
    unimplemented!();
}
