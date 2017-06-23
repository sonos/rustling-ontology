use moment::*;
use dimension::*;

#[derive(Clone,PartialEq,Debug)]
pub enum Output {
    Integer(IntegerOutput),
    Float(FloatOutput),
    Ordinal(OrdinalOutput),
    Time(TimeOutput),
    TimeInterval(TimeIntervalOutput),
    AmountOfMoney(AmountOfMoneyOutput),
    Temperature(TemperatureOutput),
    Duration(DurationOutput),
}

#[derive(Clone,Copy,PartialEq,Debug)]
pub struct IntegerOutput(pub i64);

#[derive(Clone,Copy,PartialEq,Debug)]
pub struct FloatOutput(pub f32);

#[derive(Clone,Copy,PartialEq,Debug)]
pub struct OrdinalOutput(pub i64);

#[derive(Clone,Copy,PartialEq,Debug)]
pub struct TimeOutput {
    pub moment: Moment<Local>, 
    pub grain: Grain, 
    pub precision: Precision,
}

#[derive(Clone,Copy,PartialEq,Debug)]
pub enum TimeIntervalOutput {
    After(TimeOutput),
    Before(TimeOutput),
    Between(Moment<Local>, Moment<Local>, Precision)
}

#[derive(Clone,Copy,PartialEq,Debug)]
pub struct AmountOfMoneyOutput {
    pub value: f32, 
    pub precision: Precision, 
    pub unit: Option<&'static str>,
}

#[derive(Clone,Copy,PartialEq,Debug)]
pub struct TemperatureOutput {
    pub value: f32, 
    pub unit: Option<&'static str>,
}

#[derive(Clone,PartialEq,Debug)]
pub struct DurationOutput {
    pub period: Period, 
    pub precision: Precision,
}

variant_converters!(Output, Integer, IntegerOutput);
variant_converters!(Output, Float, FloatOutput);
variant_converters!(Output, Ordinal, OrdinalOutput);
variant_converters!(Output, Time, TimeOutput);
variant_converters!(Output, TimeInterval, TimeIntervalOutput);
variant_converters!(Output, AmountOfMoney, AmountOfMoneyOutput);
variant_converters!(Output, Temperature, TemperatureOutput);
variant_converters!(Output, Duration, DurationOutput);

#[derive(Default, Debug, Copy, Clone)]
pub struct ParsingContext {
    ctx: Context<Local>,
}

impl ParsingContext {
    pub fn new(now: Interval<Local>) -> ParsingContext {
        ParsingContext {
           ctx: Context::for_reference(now) 
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
                            Output::TimeInterval(TimeIntervalOutput::Between(interval.start, end, tv.precision))
                        } else {
                            let output = TimeOutput {
                                    moment: interval.start,
                                    grain: interval.grain,
                                    precision: tv.precision,
                                } ;
                            if let Some(Direction::After) = tv.direction {
                                Output::TimeInterval(TimeIntervalOutput::After(output))
                            } else if let Some(Direction::Before) = tv.direction {
                                Output::TimeInterval(TimeIntervalOutput::Before(output))
                            } else {
                                Output::Time(output)
                            }
                        }
                    })
            }
            &Dimension::Number(ref number) => {
                match number {
                    &NumberValue::Integer(ref v) => Some(Output::Integer(IntegerOutput(v.value))),
                    &NumberValue::Float(ref v) => Some(Output::Float(FloatOutput(v.value))),
                }
            }
            &Dimension::Ordinal(ref ordinal) => Some(Output::Ordinal(OrdinalOutput(ordinal.value))),
            &Dimension::AmountOfMoney(ref aom) => Some(Output::AmountOfMoney(AmountOfMoneyOutput {
                value: aom.value,
                precision: aom.precision,
                unit: aom.unit,
            })),
            &Dimension::Temperature(ref temp) if !temp.latent => Some(Output::Temperature(TemperatureOutput {
                value: temp.value,
                unit: temp.unit,
            })),
            &Dimension::Duration(ref duration) => Some(Output::Duration(DurationOutput {
                period: duration.period.clone(),
                precision: duration.precision,
            })),
            _ => None,
        }
    }
}