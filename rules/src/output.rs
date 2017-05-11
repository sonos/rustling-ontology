use moment::*;
use dimension::*;

#[derive(Clone,PartialEq,Debug)]
pub enum Output {
    Integer(i64),
    Float(f32),
    Ordinal(i64),
    Time { moment: Moment, grain: Grain, direction: Option<Direction> },
    TimeInterval { start: Moment, end: Moment },
    AmountOfMoney { value: f32, precision: Precision, unit: Option<&'static str>},
    Temperature { value: f32, unit: Option<&'static str> },
    Duration { period: Period, precision: Precision },
}

variant_converters!(Output, Integer, i64);

#[derive(Default, Debug, Copy, Clone)]
pub struct ParsingContext {
    ctx: Context,
}

impl ParsingContext {
    pub fn new(now: Interval, years_span: u32) -> ParsingContext {
        ParsingContext {
           ctx: Context::new(now,
                     now - PeriodComp::years(years_span as i64),
                     now + PeriodComp::years(years_span as i64)) 
        }
    }
}

impl ParsingContext {
    pub fn resolve(&self, dim: &Dimension) -> Option<Output> {
        match dim {
            &Dimension::Time(ref tv) => {
                let mut walker = tv.constraint
                    .to_walker(&self.ctx.reference, &self.ctx);
                walker.forward
                    .next()
                    .and_then(|h| {
                        if tv.form.not_immediate().unwrap_or(false) && h.intersect(self.ctx.reference).is_some() {
                            walker.forward.next()
                        } else {
                            Some(h)
                        }
                    })
                    .or_else(|| walker.backward.next())
                    .map(|interval| {
                        if let Some(end) = interval.end {
                            Output::TimeInterval {
                                start: interval.start,
                                end: end,
                            }
                        } else {
                            Output::Time {
                                moment: interval.start,
                                grain: interval.grain,
                                direction: tv.direction
                            } 
                        }
                    })
            }
            &Dimension::Number(ref number) => {
                match number {
                    &NumberValue::Integer(ref v) => Some(Output::Integer(v.value)),
                    &NumberValue::Float(ref v) => Some(Output::Float(v.value)),
                }
            }
            &Dimension::Ordinal(ref ordinal) => Some(Output::Ordinal(ordinal.value)),
            &Dimension::AmountOfMoney(ref aom) => Some(Output::AmountOfMoney {
                value: aom.value,
                precision: aom.precision,
                unit: aom.unit,
            }),
            &Dimension::Temperature(ref temp) => Some(Output::Temperature {
                value: temp.value,
                unit: temp.unit,
            }),
            &Dimension::Duration(ref duration) => Some(Output::Duration {
                period: duration.period.clone(),
                precision: duration.precision,
            }),
            _ => None,
        }
    }
}