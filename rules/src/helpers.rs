use rustling::*;
use dimension::*;
use moment::*;

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
    pub constraint: RcConstraint,
    pub form: Form,
    pub direction: Option<Direction>,
}

#[derive(Debug, Clone)]
pub enum Form {
    Month(u32),
    TimeOfDay(Option<TimeOfDayForm>),
    Empty,
}

#[derive(Debug, Clone)]
pub struct TimeOfDayForm {
    full_hour: u32, 
    is_12_clock: bool,
}

impl Form {
    fn time_of_day(full_hour: u32, is_12_clock: bool) -> Form {
        Form::TimeOfDay(Some(TimeOfDayForm { full_hour: full_hour, is_12_clock: is_12_clock }))
    }
}

#[derive(Debug, Clone)]
pub enum Direction {
    After,
    Before,
}

impl TimeValue {
    pub fn constraint(constraint: RcConstraint) -> TimeValue {
        TimeValue {
            constraint: constraint,
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
        self.constraint.intersect(other.constraint);
        //Intersection::new(self.constraint, other.constraint);
        unimplemented!();
        //TimeValue
        //    ::constraint(*self.constraint.intersect(*other.constraint))
            //.direction(self.direction.or(other.direction))
    }
}

pub fn year(y: i32) -> RuleResult<TimeValue> {
    Ok(TimeValue::constraint(Year::new(y)))
}

pub fn month(m: u32) -> RuleResult<TimeValue> {
    if 1 <= m && m <= 12 {
        Ok(TimeValue::constraint(Month::new(m)).form(Form::Month(m)))
    } else {
        unimplemented!()
        //Err(RustlingErrorKind::ProductionRuleError(format!("{} is not a valid month number", m)))
    }
}

pub fn day_of_month(dom: u32) -> RuleResult<TimeValue> {
    if 1 <= dom && dom <= 31 {
        Ok(TimeValue::constraint(DayOfMonth::new(dom)).form(Form::Empty))
    } else {
        unimplemented!()
        //Err(RustlingErrorKind::ProductionRuleError(format!("{} is not a valid day of month number", dom)))
    }
}

pub fn month_day(m: u32, d: u32) -> RuleResult<TimeValue> {
    Ok(month(m)?.intersect(day_of_month(d)?))
}

pub fn hour(h: u32, is_12_clock: bool) -> RuleResult<TimeValue> {
    if is_12_clock {
        Ok(TimeValue::constraint(Hour::clock_12(h)).form(Form::time_of_day(h, true)))
    } else {
        Ok(TimeValue::constraint(Hour::clock_24(h)).form(Form::time_of_day(h, true)))
    }
}

pub fn minute(m: u32) -> RuleResult<TimeValue> {
    Ok(TimeValue::constraint(Minute::new(m)))
}

pub fn second(s: u32) -> RuleResult<TimeValue> {
    Ok(TimeValue::constraint(Second::new(s)))
}

pub fn hour_minute(h: u32, m: u32, is_12_clock: bool) -> RuleResult<TimeValue> {
    Ok(hour(h, is_12_clock)?.intersect(minute(m)?).form(Form::TimeOfDay(None)))
}

pub fn hour_minute_second_clock_12(h: u32, m: u32, s: u32, is_12_clock: bool) -> RuleResult<TimeValue> {
    Ok(hour_minute(h, m, is_12_clock)?.intersect(second(s)?).form(Form::TimeOfDay(None)))
}
