extern crate rustling;
extern crate rustling_ontology_rules;
extern crate rustling_ontology_moment;

use rustling::*;
use rustling_ontology_moment::*;
pub use rustling_ontology_rules::dimension::*;
pub use rustling_ontology_rules::output::*;
pub use rustling_ontology_rules::output::ParsingContext;

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


macro_rules! lang {
    ($lang:ident, $lang_test:ident, [$($example:ident),*]) => {
        pub fn $lang() -> Vec<::rustling::train::Example<Dimension>> {
            let mut v = vec![];
            $( $lang::$example(&mut v); )*
            v
        }
        #[cfg(test)]
        mod $lang_test {
            use super::*;
            fn assert_examples(rules: &RuleSet<Dimension>, examples: Vec<Example<Dimension>>) {
                for ex in examples.iter() {
                    let stash = rules.apply_all(&ex.text.to_lowercase()).unwrap();
                    let correct_results = stash
                                .into_iter()
                                .filter(|candidate| candidate.root_node.range == ::Range(0, ex.text.len()) && ex.predicate.check(&candidate))
                                .collect::<Vec<_>>();
                    assert!(!correct_results.is_empty(), ex.text);
                }
            }
            #[test]
            fn test_examples() {
                let rules = ::rustling_ontology_rules::$lang().unwrap();
                let examples = $lang();
                assert_examples(&rules, examples);
            }
        }
    }
}

lang!(en, en_test, [examples_numbers, examples_time]);
lang!(fr, fr_test, [examples_numbers, examples_time]);
lang!(es, es_test, [examples_numbers, examples_time]);

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
    pub direction: Option<Direction>,
    pub precision: Precision,
    pub interval: Interval,
    pub context: ParsingContext,
}

impl Check<Dimension> for CheckMoment {
    fn check(&self, pn: &ParsedNode<Dimension>) -> bool {
        self.context.resolve(&pn.value)
            .and_then(|v| TimeOutput::attempt_from(v))
            .map(|v| {
                let check_value = v.moment == self.interval.start && v.grain == self.interval.grain;
                let check_direction = v.direction == self.direction;
                let check_precision = v.precision == self.precision;
                check_value && check_direction && check_precision
            })
            .unwrap_or(false)
    }
}

pub fn check_moment(context: ParsingContext, moment: Moment, grain: Grain, precision: Precision, direction: Option<Direction>)
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
    pub interval: Interval,
    pub context: ParsingContext,
}

impl Check<Dimension> for CheckMomentSpan {
    fn check(&self, pn: &ParsedNode<Dimension>) -> bool {
        self.context.resolve(&pn.value)
            .and_then(|v| TimeIntervalOutput::attempt_from(v))
            .map(|v| v.start == self.interval.start && Some(v.end) == self.interval.end)
            .unwrap_or(false)
    }
}

pub fn check_moment_span(context: ParsingContext, start: Moment, end: Moment, grain: Grain)
                      -> CheckMomentSpan {
    CheckMomentSpan { interval: Interval::new(start, Some(end), grain), context: context }
}
