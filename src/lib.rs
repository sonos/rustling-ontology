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
//!     use rustling_ontology::dimension::IntegerValue;
//!     use rustling_ontology::AttemptTo;
//!
//!     let parser = rustling_ontology::build_parser(rustling_ontology::Lang::EN).unwrap();
//!     let result = parser.parse("twenty-one").unwrap();
//!
//!     assert_eq!(result.len(), 1);
//!     let int:i64 = result[0].value.attempt_to().unwrap();
//!     assert_eq!(21, int);
//!
//!     let int:IntegerValue = result[0].value.attempt_to().unwrap();
//!     assert_eq!(21, int.value);
//! }
//! ```
extern crate rmp_serde;
extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate rustling;
extern crate rustling_ontology_rules;
extern crate rustling_ontology_training as training;

pub use rustling::{AttemptTo, ParsedNode, ParserMatch, Range, Value, RustlingError, RustlingResult, Sym};
pub use rustling_ontology_rules::Lang;
pub use rustling_ontology_rules::dimension;
pub use rustling_ontology_rules::dimension::{Dimension, DimensionKind};

mod parser;

#[derive(Clone,PartialEq,Debug)]
pub enum Output {
    Number(dimension::NumberValue),
    AmountOfMoney(dimension::AmountOfMoneyValue),
    Ordinal(dimension::OrdinalValue),
    Temperature(dimension::TemperatureValue),
    MoneyUnit(dimension::MoneyUnitValue),
    Time(usize),
    Duration(dimension::DurationValue),
}

pub type RawParser = rustling::Parser<dimension::Dimension, parser::Feat, parser::FeatureExtractor>;
/// Main class to be use at runtime.
pub struct Parser(RawParser);

impl Parser {
    pub fn parse(&self, input: &str) -> RustlingResult<Vec<ParserMatch<Output>>> {
        Ok(self.0
               .parse(input)?
               .into_iter()
               .map(|pm| {
                        ParserMatch {
                            value: Output::Time(0),
                            range: pm.range,
                            probalog: pm.probalog,
                        }
                    })
               .collect())
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
                let rules = rules::$lang::rules_numbers()?;
                let exs = ::training::$lang::examples_numbers();
                let model = ::rustling::train::train(&rules, exs, ::parser::FeatureExtractor())?;
                Ok(Parser(::rustling::Parser::new(rules, model, ::parser::FeatureExtractor())))
            }

            pub fn build_raw_parser() -> RustlingResult<::RawParser> {
                let rules = rules::$lang::rules_numbers()?;
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
