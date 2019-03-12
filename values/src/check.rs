use rustling::{AttemptFrom, Check, ParsedNode};
use moment::{Grain, Interval, Moment, Local, Period};
use dimension::*;
use output::*;
use context::{ParsingContext, ResolverContext};

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
pub struct CheckDuration {
    pub period: Period,
    pub precision: Precision
}

impl Check<Dimension> for CheckDuration {
    fn check(&self, pn: &ParsedNode<Dimension>) -> bool {
        DurationValue::attempt_from(pn.value.clone())
            .map(|v| v.precision == self.precision && v.period == self.period)
            .unwrap_or(false)
    }
}

pub fn check_duration(period: Period, precision: Precision) -> CheckDuration {
    CheckDuration { period, precision }
}


#[derive(Debug)]
pub struct CheckMoment {
    pub direction: Option<Direction>,
    pub precision: Precision,
    pub interval: Interval<Local>,
    pub context: ResolverContext,
}

impl Check<Dimension> for CheckMoment {
    fn check(&self, pn: &ParsedNode<Dimension>) -> bool {
        match self.direction {
            None => {
                self.context.resolve(&pn.value)
                    .and_then(|v| DatetimeOutput::attempt_from(v))
                    .map(|v| {
                        let check_value = v.moment == self.interval.start && v.grain == self.interval.grain;
                        let check_precision = v.precision == self.precision;
                        check_value && check_precision
                    })
                    .unwrap_or(false)
            }
            Some(Direction::After) => {
                self.context.resolve(&pn.value)
                    .and_then(|v| DatetimeIntervalOutput::attempt_from(v))
                    .map(|v| {
                        if let DatetimeIntervalOutput::After(m) = v {
                            let check_value = m.moment == self.interval.start && m.grain == self.interval.grain;
                            let check_precision = m.precision == self.precision;
                            check_value && check_precision
                        } else {
                            false
                        }
                    })
                    .unwrap_or(false)
            }
            Some(Direction::Before) => {
                self.context.resolve(&pn.value)
                    .and_then(|v| DatetimeIntervalOutput::attempt_from(v))
                    .map(|v| {
                        if let DatetimeIntervalOutput::Before(m) = v {
                            let check_value = m.moment == self.interval.start && m.grain == self.interval.grain;
                            let check_precision = m.precision == self.precision;
                            check_value && check_precision
                        } else {
                            false
                        }
                    })
                    .unwrap_or(false)
            }
        }
    }
}

pub fn check_moment(context: ResolverContext, moment: Moment<Local>, grain: Grain, precision: Precision, direction: Option<Direction>)
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
    pub precision: Precision,
    pub context: ResolverContext,
}

impl Check<Dimension> for CheckMomentSpan {
    fn check(&self, pn: &ParsedNode<Dimension>) -> bool {
        self.context.resolve(&pn.value)
            .and_then(|v| DatetimeIntervalOutput::attempt_from(v))
            .map(|v| {
                if let DatetimeIntervalOutput::Between { start, end, precision, .. } = v {
                    start == self.interval.start && Some(end) == self.interval.end && precision == self.precision
                } else {
                    false
                }
            })
            .unwrap_or(false)
    }
}

pub fn check_moment_span(context: ResolverContext, precision: Precision, start: Moment<Local>, end: Moment<Local>, grain: Grain)
                         -> CheckMomentSpan {
    CheckMomentSpan { interval: Interval::new(start, Some(end), grain), precision, context }
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

#[derive(Debug)]
pub struct CheckPercentage {
    pub value: f32,
}

impl Check<Dimension> for CheckPercentage {
    fn check(&self, pn: &ParsedNode<Dimension>) -> bool {
        PercentageValue::attempt_from(pn.value.clone())
            .map(|v| v.0 == self.value)
            .unwrap_or(false)
    }
}

pub fn check_percentage(value: f32) -> CheckPercentage {
    CheckPercentage {
        value: value,
    }
}

#[derive(Debug)]
pub struct CheckTemperature {
    pub value: f32,
    pub unit: Option<&'static str>,
}

impl Check<Dimension> for CheckTemperature {
    fn check(&self, pn: &ParsedNode<Dimension>) -> bool {
        TemperatureValue::attempt_from(pn.value.clone())
            .map(|v| v.value == self.value && v.unit == self.unit)
            .unwrap_or(false)
    }
}

pub fn check_temperature(value: f32, unit: Option<&'static str>) -> CheckTemperature {
    CheckTemperature {
        value: value,
        unit: unit,
    }
}

