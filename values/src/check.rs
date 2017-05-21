use rustling::{AttemptFrom, Check, ParsedNode};
use moment::{Grain, Interval, Moment, Local};
use dimension::*;
use output::*;

macro_rules! check_finance {
    ($value:expr) => (check_finance($value, None, Precision::Exact));
    ($value:expr, $unit:expr) => (check_finance($value, $unit, Precision::Exact));
    ($value:expr, $unit:expr, $precision:expr) => (check_finance($value, $unit, $precision));
}

#[derive(Debug)]
pub struct CheckInteger {
    pub value: i64,
}

impl Check<Dimension> for CheckInteger {
    fn check(&self, pn: &ParsedNode<Dimension>) -> bool {
        IntegerValue::attempt_from(pn.value.clone())
            .map(|v| v.value == self.value)
            .unwrap_or(false)
    }
}

pub fn check_integer(v: i64) -> CheckInteger {
    CheckInteger { value: v }
}

#[derive(Debug)]
pub struct CheckOrdinal {
    pub value: i64,
}

impl Check<Dimension> for CheckOrdinal {
    fn check(&self, pn: &ParsedNode<Dimension>) -> bool {
        OrdinalValue::attempt_from(pn.value.clone())
            .map(|v| v.value == self.value)
            .unwrap_or(false)
    }
}

pub fn check_ordinal(v: i64) -> CheckOrdinal {
    CheckOrdinal { value: v }
}

#[derive(Debug)]
pub struct CheckFloat {
    pub value: f32,
}

impl Check<Dimension> for CheckFloat {
    fn check(&self, pn: &ParsedNode<Dimension>) -> bool {
        FloatValue::attempt_from(pn.value.clone())
            .map(|v| v.value == self.value)
            .unwrap_or(false)
    }
}

pub fn check_float(v: f32) -> CheckFloat {
    CheckFloat { value: v }
}

#[derive(Debug)]
pub struct CheckMoment {
    pub direction: Option<Direction>,
    pub precision: Precision,
    pub interval: Interval<Local>,
    pub context: ParsingContext,
}

impl Check<Dimension> for CheckMoment {
    fn check(&self, pn: &ParsedNode<Dimension>) -> bool {
        match self.direction {
            None => {
                self.context.resolve(&pn.value)
                    .and_then(|v| TimeOutput::attempt_from(v))
                    .map(|v| {
                        let check_value = v.moment == self.interval.start && v.grain == self.interval.grain;
                        let check_precision = v.precision == self.precision;
                        check_value && check_precision
                    })
                    .unwrap_or(false)
            },
            Some(Direction::After) => {
                self.context.resolve(&pn.value)
                    .and_then(|v| TimeIntervalOutput::attempt_from(v))
                    .map(|v| {
                        if let TimeIntervalOutput::After(m) = v {
                            m == self.interval.start
                        } else {
                            true
                        }
                    })
                    .unwrap_or(false)
            },
            Some(Direction::Before) => {
                self.context.resolve(&pn.value)
                    .and_then(|v| TimeIntervalOutput::attempt_from(v))
                    .map(|v| {
                        if let TimeIntervalOutput::Before(m) = v {
                            m == self.interval.start
                        } else {
                            true
                        }
                    })
                    .unwrap_or(false)
            }

        }
    }
}

pub fn check_moment(context: ParsingContext, moment: Moment<Local>, grain: Grain, precision: Precision, direction: Option<Direction>)
                      -> CheckMoment {
    CheckMoment {
        direction: direction,
        precision: precision,
        interval: Interval::starting_at(moment, grain),
        context: context
    }
}
#[derive(Debug)]
pub struct CheckMomentSpan {
    pub interval: Interval<Local>,
    pub context: ParsingContext,
}

impl Check<Dimension> for CheckMomentSpan {
    fn check(&self, pn: &ParsedNode<Dimension>) -> bool {
        self.context.resolve(&pn.value)
            .and_then(|v| TimeIntervalOutput::attempt_from(v))
            .map(|v| {
                if let TimeIntervalOutput::Between(s, e) = v {
                    s == self.interval.start && Some(e) == self.interval.end
                } else {
                    false
                }
            })
            .unwrap_or(false)
    }
}

pub fn check_moment_span(context: ParsingContext, start: Moment<Local>, end: Moment<Local>, grain: Grain)
                      -> CheckMomentSpan {
    CheckMomentSpan { interval: Interval::new(start, Some(end), grain), context: context }
}

#[derive(Debug)]
pub struct CheckFinance {
    pub value: f32,
    pub unit: Option<&'static str>,
    pub precision: Precision,
}

impl Check<Dimension> for CheckFinance {
    fn check(&self, pn: &ParsedNode<Dimension>) -> bool {
        AmountOfMoneyValue::attempt_from(pn.value.clone())
            .map(|v| v.value == self.value && v.precision == self.precision && v.unit == self.unit)
            .unwrap_or(false)
    }
}

pub fn check_finance(value: f32, unit: Option<&'static str>, precision: Precision) -> CheckFinance {
    CheckFinance {
        value: value,
        precision: precision,
        unit: unit,
    }
}
