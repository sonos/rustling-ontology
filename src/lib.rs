//! General purpose ontology based on rustling.
//!
//! Contains detectors for various entities, like numbers, temperatures, dates
//! in french, english, ...
//!
//! ```
//! extern crate rustling;
//! extern crate rustling_ontology;
//!
//! fn main() {
//!     use rustling_ontology::*;
//!
//!     let ctx = ParsingContext::default();
//!     let parser = build_parser(rustling_ontology::Lang::EN).unwrap();
//!     let result = parser.parse("twenty-one", &ctx).unwrap();
//!
//!     let int:i64 = result[0].value.attempt_to().unwrap();
//!     assert_eq!(21, int);
//! }
//! ```
extern crate rmp_serde;
extern crate serde;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate rustling;
extern crate rustling_ontology_rules;
extern crate rustling_ontology_moment;
extern crate rustling_ontology_training as training;

pub use rustling::{AttemptTo, ParsedNode, ParserMatch, Range, Value, RustlingError,
                   RustlingResult, Sym};
pub use rustling_ontology_rules::Lang;
pub use rustling_ontology_rules::dimension;
pub use rustling_ontology_rules::dimension::{Dimension, DimensionKind, NumberValue};
pub use rustling_ontology_moment::Interval;

mod parser;

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
    moment: rustling_ontology_moment::interval_constraints::Context,
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

// Rustling raw parser. Don't use directly
#[doc(hidden)]
pub type RawParser = rustling::Parser<dimension::Dimension, parser::Feat, parser::FeatureExtractor>;

/// Main class to be use at runtime.
pub struct Parser(RawParser);

impl Parser {
    fn translate_values(&self,
                        input: Vec<ParserMatch<Dimension>>,
                        context: &ParsingContext)
                        -> Vec<ParserMatch<Output>> {
        input
            .into_iter()
            .filter_map(|pm| {
                context
                    .resolve(&pm.value)
                    .map(|o| {
                             ParserMatch {
                                 value: o,
                                 range: pm.range,
                                 probalog: pm.probalog,
                             }
                         })
            })
            .collect()
    }

    pub fn parse_with_kind_order(&self,
                                 input: &str,
                                 context: &ParsingContext,
                                 order: &[DimensionKind])
                                 -> RustlingResult<Vec<ParserMatch<Output>>> {
        Ok(self.translate_values(self.0.parse_with_kind_order(input, order)?, context))
    }

    pub fn parse(&self,
                 input: &str,
                 context: &ParsingContext)
                 -> RustlingResult<Vec<ParserMatch<Output>>> {
        Ok(self.translate_values(self.0.parse(input)?, context))
    }
}

/// Obtain a parser for a given language.
pub fn build_parser(lang: Lang) -> RustlingResult<Parser> {
    match lang {
        Lang::EN => en::build_parser(),
        Lang::FR => fr::build_parser(),
        Lang::ES => es::build_parser(),
    }
}

/// Obtain a parser for a given language.
pub fn build_raw_parser(lang: Lang) -> RustlingResult<RawParser> {
    match lang {
        Lang::EN => en::build_raw_parser(),
        Lang::FR => fr::build_raw_parser(),
        Lang::ES => es::build_raw_parser(),
    }
}

pub fn train_parser(lang: Lang) -> RustlingResult<Parser> {
    match lang {
        Lang::EN => en::train_parser(),
        Lang::FR => fr::train_parser(),
        Lang::ES => es::train_parser(),
    }
}

macro_rules! lang {
    ($lang:ident) => {
        mod $lang {
            use rustling_ontology_rules as rules;
            use super::*;

            pub fn train_parser() -> RustlingResult<Parser> {
                let rules = rules::$lang()?;
                let exs = ::training::$lang::examples_numbers();
                let model = ::rustling::train::train(&rules, exs, ::parser::FeatureExtractor())?;
                Ok(Parser(::rustling::Parser::new(rules, model, ::parser::FeatureExtractor())))
            }

            pub fn build_raw_parser() -> RustlingResult<::RawParser> {
                let rules = rules::$lang()?;
                let model = ::rmp_serde::decode::from_read(&include_bytes!(concat!(env!("OUT_DIR"), "/", stringify!($lang), ".rmp"))[..]).map_err(|e| format!("{:?}", e))?;
                Ok(::RawParser::new(rules, model, ::parser::FeatureExtractor()))
            }

            pub fn build_parser() -> RustlingResult<::Parser> {
                build_raw_parser().map(::Parser)
            }
        }
    }
}

lang!(en);
lang!(es);
lang!(fr);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_long_number_en() {
        let ctx = ParsingContext::default();
        let parser = build_parser(Lang::EN).unwrap();
        let number = "one million five hundred twenty-one thousand eighty-two";
        let result = parser.parse_with_kind_order(number, &ctx,  &[DimensionKind::Number]).unwrap();
        let int: i64 = result[0].value.attempt_to().unwrap();
        assert_eq!(1521082, int);
    }

    #[test]
    #[ignore]
    fn time_resolve_complex_train_sentence() {
        let parser = build_raw_parser(Lang::EN).unwrap();
        //        let sent = "I want a return train ticket from Bordeaux to Strasbourg, friday the 12th of May, 10:32 am to wednesday the 7th of june, 6:22 pm";
        let sent = "I want a return train ticket from Bordeaux to Strasbourg, friday the 12th of May, 10:32 am to wednesday the 7th of june, 6:22 pm".to_lowercase();
        let result = parser.candidates(&*sent, |_| Some(0)).unwrap();
        println!("{}", result.len());
        for r in &result {
            println!("{:?}", &sent[r.node.root_node.range.0..r.node.root_node.range.1]);
        }
        panic!();
    }
}
