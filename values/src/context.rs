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
            &Dimension::Datetime(ref datetime_value) => {
                let mut walker = datetime_value.constraint
                    .to_walker(&self.ctx.reference, &self.ctx);
                walker.forward
                    .next()
                    .and_then(|h| {
                        if datetime_value.form.not_immediate().unwrap_or(false) && h.intersect(self.ctx.reference).is_some() {
                            walker.forward.next()
                        } else {
                            Some(h)
                        }
                    })
                    .or_else(|| walker.backward.next())
                    .map(|interval| {

                        if let Some(bounded_direction) = datetime_value.direction {

                            let datetime_kind = match datetime_value.datetime_kind {
                                DatetimeKind::Date |
                                DatetimeKind::DatePeriod => DatetimeKind::DatePeriod,
                                DatetimeKind::Time |
                                DatetimeKind::TimePeriod => DatetimeKind::TimePeriod,
                                DatetimeKind::DatetimeComplement |
                                DatetimeKind::Datetime |
                                DatetimeKind::Empty => DatetimeKind::Datetime,
                            };

                            let anchor = match bounded_direction.bound {
                                Bound::Start => interval.start,
                                Bound::End { only_interval } if only_interval => interval.end.unwrap_or(interval.start),
                                Bound::End { .. } => interval.end_moment(),
                            };

                            let datetime_output_value = DatetimeOutput {
                                moment: anchor,
                                grain: interval.grain,
                                precision: datetime_value.precision,
                                latent: datetime_value.latent,
                                datetime_kind: datetime_kind,
                            };

                            match bounded_direction.direction {
                                Direction::After => {
                                    let datetime_interval_output_value = DatetimeIntervalOutput {
                                        interval_kind: DatetimeIntervalKind::After(datetime_output_value),
                                        datetime_kind: datetime_kind,
                                    };
                                    Output::DatetimeInterval(datetime_interval_output_value)
                                },
                                Direction::Before => {
                                    let datetime_interval_output_value = DatetimeIntervalOutput {
                                        interval_kind: DatetimeIntervalKind::Before(datetime_output_value),
                                        datetime_kind: datetime_kind,
                                    };
                                    Output::DatetimeInterval(datetime_interval_output_value)
                                },
                            }
                        } else if let Some(end) = interval.end {
                            let datetime_kind = match datetime_value.datetime_kind {
                                DatetimeKind::Date |
                                DatetimeKind::DatePeriod => DatetimeKind::DatePeriod,
                                DatetimeKind::Time |
                                DatetimeKind::TimePeriod => DatetimeKind::TimePeriod,
                                DatetimeKind::DatetimeComplement |
                                DatetimeKind::Datetime |
                                DatetimeKind::Empty => DatetimeKind::Datetime,
                            };
                            let datetime_interval_output_value = DatetimeIntervalOutput {
                                interval_kind: DatetimeIntervalKind::Between {
                                    start: interval.start,
                                    end: end,
                                    precision: datetime_value.precision,
                                    latent: datetime_value.latent,
                                },
                                datetime_kind: datetime_kind,
                            };
                            Output::DatetimeInterval(datetime_interval_output_value)
                        } else {
                            let datetime_output_value = DatetimeOutput {
                                    moment: interval.start,
                                    grain: interval.grain,
                                    precision: datetime_value.precision,
                                    latent: datetime_value.latent,
                                    datetime_kind: datetime_value.datetime_kind,
                            };
                            Output::Datetime(datetime_output_value)
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

