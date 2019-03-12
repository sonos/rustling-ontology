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

    pub fn from_secs(secs: i64) -> ResolverContext {
        let anchor = Interval::starting_at(Moment(Local.timestamp(secs, 0)), Grain::Second);
        ResolverContext::new(anchor)
    }

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
            &Dimension::Datetime(ref dtv) => {
                let mut walker = dtv.constraint
                    .to_walker(&self.ctx.reference, &self.ctx);
                walker.forward
                    .next()
                    .and_then(|h| {
                        if dtv.form.not_immediate().unwrap_or(false) && h.intersect(self.ctx.reference).is_some() {
                            walker.forward.next()
                        } else {
                            Some(h)
                        }
                    })
                    .or_else(|| walker.backward.next())
                    .map(|interval| {
                        if let Some(bounded_direction) = dtv.direction {
                            let anchor = match bounded_direction.bound {
                                Bound::Start => interval.start,
                                Bound::End { only_interval } if only_interval => interval.end.unwrap_or(interval.start),
                                Bound::End { .. } => interval.end_moment(),
                            };
                            
                            let output = DatetimeOutput {
                                moment: anchor,
                                grain: interval.grain,
                                precision: dtv.precision,
                                latent: dtv.latent,
                            };
                            
                            match bounded_direction.direction {
                                Direction::After => Output::DatetimeInterval(DatetimeIntervalOutput::After(output)),
                                Direction::Before => Output::DatetimeInterval(DatetimeIntervalOutput::Before(output)),
                            }
                        } else if let Some(end) = interval.end {
                            Output::DatetimeInterval(
                                    DatetimeIntervalOutput::Between {
                                        start: interval.start, 
                                        end: end, 
                                        precision: dtv.precision,
                                        latent: dtv.latent,
                                    }
                                )
                        } else {
                            let output = DatetimeOutput {
                                    moment: interval.start,
                                    grain: interval.grain,
                                    precision: dtv.precision,
                                    latent: dtv.latent,
                            };
                            Output::Datetime(output)
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
            &Dimension::Temperature(ref temp) => Some(Output::Temperature(TemperatureOutput {
                value: temp.value,
                unit: temp.unit,
                latent: temp.latent,
            })),
            &Dimension::Duration(ref duration) => Some(Output::Duration(DurationOutput {
                period: duration.period.clone(),
                precision: duration.precision,
            })),
            &Dimension::Percentage(ref percentage) => Some(Output::Percentage(PercentageOutput(percentage.0))),
            _ => None,
        }
    }
}

