use std::{ fmt, result };

use rustling::*;

rustling_value! { Dimension
    Number(NumberValue),
    Ordinal(OrdinalValue),
    Temperature(TemperatureValue),
}

impl Value for Dimension {
    fn same_dimension_as(&self, other: &Self) -> bool {
        match (self, other) {
            (&Dimension::Number(_), &Dimension::Number(_))
                | (&Dimension::Ordinal(_), &Dimension::Ordinal(_))
                | (&Dimension::Temperature(_), &Dimension::Temperature(_)) => true,
            _ => false
        }
    }
}

impl fmt::Display for Dimension {
    fn fmt(&self, fmt:&mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self {
            &Dimension::Number(ref number) => {
                match number {
                    &NumberValue::Integer(ref v) => write!(fmt, "Number: {}", v.value),
                    &NumberValue::Float(ref v) => write!(fmt, "Number: {}", v.value)
                }
            },
            &Dimension::Ordinal(_) => write!(fmt, "Ordinal"),
            &Dimension::Temperature(_) => write!(fmt, "Temperature"),
        }
    }
}

#[derive(Debug,PartialEq,Copy,Clone)]
pub struct OrdinalValue {
    pub value: i64,
}

#[derive(Debug,PartialEq,Copy,Clone)]
pub enum Precision {
    Approximate,
    Exact,
}

impl Default for Precision {
    fn default() -> Precision {
        Precision::Exact
    }
}

#[derive(Debug,PartialEq,Clone,Default)]
pub struct IntegerValue {
    pub value: i64,
    pub grain: Option<u8>,
    pub group: bool,
    pub prefixed: bool,
    pub suffixed: bool,
    pub precision: Precision,
}

impl IntegerValue {
    pub fn new(value:i64) -> RuleResult<IntegerValue> {
        Ok(IntegerValue { value: value, grain: None, .. IntegerValue::default() })
    }
    pub fn new_with_grain(value:i64, grain:u8) -> RuleResult<IntegerValue> {
        Ok(IntegerValue { value: value, grain: Some(grain), .. IntegerValue::default() })
    }
}

impl From<IntegerValue> for Dimension {
    fn from(v: IntegerValue) -> Dimension {
        Dimension::Number(NumberValue::Integer(v))
    }
}

impl From<FloatValue> for Dimension {
    fn from(v: FloatValue) -> Dimension {
        Dimension::Number(NumberValue::Float(v))
    }
}

impl From<IntegerValue> for NumberValue {
    fn from(v: IntegerValue) -> NumberValue {
        NumberValue::Integer(v)
    }
}

impl AttemptFrom<Dimension> for IntegerValue {
    fn attempt_from(v: Dimension) -> Option<IntegerValue> {
        if let Dimension::Number(value) = v {
            if let NumberValue::Integer(integer) = value {
                Some(integer)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl AttemptFrom<Dimension> for FloatValue {
    fn attempt_from(v: Dimension) -> Option<FloatValue> {
        if let Dimension::Number(value) = v {
            if let NumberValue::Float(float) = value {
                Some(float)
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Debug,PartialEq,Clone,Default)]
pub struct FloatValue {
    pub value: f32,
    pub prefixed: bool,
    pub suffixed: bool,
    pub precision: Precision,
}

impl FloatValue {
    pub fn new(value: f32) -> RuleResult<FloatValue> {
        Ok(FloatValue {value: value, .. FloatValue::default()})
    }
}

impl From<FloatValue> for NumberValue {
    fn from(v: FloatValue) -> NumberValue {
        NumberValue::Float(v)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum NumberValue {
    Float(FloatValue),
    Integer(IntegerValue),
}

impl NumberValue {
    pub fn prefixed(&self) -> bool {
        match self {
            &NumberValue::Float(ref v) => v.prefixed,
            &NumberValue::Integer(ref v) => v.prefixed,
        }
    }

    pub fn suffixed(&self) -> bool {
        match self {
            &NumberValue::Float(ref v) => v.suffixed,
            &NumberValue::Integer(ref v) => v.suffixed,
        }
    }

    pub fn value(&self) -> f32 {
        match self {
            &NumberValue::Float(ref v) => v.value,
            &NumberValue::Integer(ref v) => v.value as f32,
        }
    }

    pub fn grain(&self) -> Option<u8> {
        match self {
            &NumberValue::Float(_) => None,
            &NumberValue::Integer(ref v) => v.grain
        }
    }
}

#[derive(Debug,PartialEq,Clone)]
pub struct TemperatureValue {
    pub value: f32,
    pub unit: Option<&'static str>,
    pub latent: bool
}
