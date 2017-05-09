use moment::*;
use dimension::*;

#[derive(Clone,PartialEq,Debug)]
pub enum Output {
    Integer(i64),
    Float(f32),
    Time(Interval),
    String(String),
}

variant_converters!(Output, Integer, i64);

#[derive(Default)]
pub struct ParsingContext {
    moment: Context,
}

impl ParsingContext {
    pub fn resolve(&self, dim: &Dimension) -> Option<Output> {
        match dim {
            &Dimension::Time(ref tv) => {
                let mut walker = tv.constraint
                    .to_walker(&self.moment.reference, &self.moment);
                walker
                    .forward
                    .next()
                    .or_else(|| walker.backward.next())
                    .map(Output::Time)
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