use moment::*;
use dimension::*;

#[derive(Clone,PartialEq,Debug)]
pub enum Output {
    Integer(i64),
    Float(f32),
    Time(TimeOutput),
    String(String),
}

#[derive(Clone,PartialEq,Debug)]
pub struct TimeOutput(pub Interval, pub bool);

variant_converters!(Output, Integer, i64);
variant_converters!(Output, Time, TimeOutput);

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
                        if tv.form.not_immediate().unwrap_or(false) && h.start <= self.ctx.reference.start {
                            walker.forward.next()
                        } else {
                            Some(h)
                        }
                    })
                    .or_else(|| walker.backward.next())
                    .map(|v| Output::Time(TimeOutput(v, tv.latent)))

            }
            &Dimension::Number(ref number) => {
                match number {
                    &NumberValue::Integer(ref v) => Some(Output::Integer(v.value)),
                    &NumberValue::Float(ref v) => Some(Output::Float(v.value)),
                }
            }
            _ => None,
        }
    }
}