use output::*;
use dimension::*;
use rustling::Value;
use moment::*;

pub trait ParsingContext<V: Value> {
    type O;
    fn resolve(&self, value: &V) -> Option<Self::O>;
}

pub struct IdentityContext<V: Value+Clone> {
    _phantom: ::std::marker::PhantomData<V>
}

impl<V: Value+Clone> IdentityContext<V> {
    pub fn new() -> IdentityContext<V> {
        IdentityContext {
            _phantom: ::std::marker::PhantomData,
        }
    }
}

impl<V: Value+Clone> ParsingContext<V> for IdentityContext<V> {
    type O = V;
    fn resolve(&self, value: &V) -> Option<V> {
        Some(value.clone())
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct ResolverContext {
    ctx: Context<Local>,
}

impl ResolverContext {
    pub fn new(now: Interval<Local>) -> ResolverContext {
        ResolverContext {
           ctx: Context::for_reference(now) 
        }
    }
}

impl ParsingContext<Dimension> for ResolverContext {
    type O = Output;
    fn resolve(&self, dim: &Dimension) -> Option<Output> {
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
                            Output::TimeInterval(
                                TimeIntervalOutput::Between {
                                    start: interval.start, 
                                    end: end, 
                                    precision: tv.precision,
                                    latent: tv.latent,
                                }
                            )
                        } else {
                            let output = TimeOutput {
                                    moment: interval.start,
                                    grain: interval.grain,
                                    precision: tv.precision,
                                    latent: tv.latent,
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
                latent: temp.latent,
            })),
            &Dimension::Duration(ref duration) => Some(Output::Duration(DurationOutput {
                period: duration.period.clone(),
                precision: duration.precision,
            })),
            _ => None,
        }
    }
}

