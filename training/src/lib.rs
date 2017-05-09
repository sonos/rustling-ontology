extern crate rustling;
extern crate rustling_ontology_rules;
extern crate rustling_ontology_moment;

use rustling::*;
use rustling_ontology_moment::*;
pub use rustling_ontology_rules::dimension::*;

macro_rules! example {
    ($v:expr, $check:expr, $($ex:expr),*) => {
        $( $v.push($crate::rustling::Example::new($ex, Box::new($check))); )*
    };
}

#[macro_use]
mod macros;
pub mod en;
pub mod es;
pub mod fr;

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
    pub interval: Interval,
}

impl Check<Dimension> for CheckMoment {
    fn check(&self, _: &ParsedNode<Dimension>) -> bool {
        unimplemented!()
    }
}

pub fn check_moment(moment: Moment, grain: Grain)
                      -> CheckMoment {
    CheckMoment { interval: Interval::starting_at(moment, grain) }
}

#[derive(Debug)]
pub struct CheckMomentSpan {
    pub interval: Interval,
}

impl Check<Dimension> for CheckMomentSpan {
    fn check(&self, _: &ParsedNode<Dimension>) -> bool {
        unimplemented!()
    }
}

pub fn check_moment_span(start: Moment, end: Moment, grain: Grain)
                      -> CheckMomentSpan {
    CheckMomentSpan { interval: Interval::new(start, Some(end), grain) }
}
