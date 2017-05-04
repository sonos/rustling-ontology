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
//!     use rustling_ontology::IntegerValue;
//!     use rustling::AttemptTo;
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
extern crate bincode;
extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate rustling;
extern crate rustling_ontology_rules as rules;
extern crate rustling_ontology_training as training;

pub use rustling::{AttemptTo, ParserMatch, Range, Value, RustlingError, RustlingResult};
pub use rules::Lang;
pub use rules::dimension::{Dimension, DimensionKind, IntegerValue, NumberValue, FloatValue, OrdinalValue,
                    TemperatureValue, AmountOfMoneyValue, MoneyUnitValue};

mod parser;

/// Main class to be use at runtime.
pub type Parser = rustling::Parser<Dimension, parser::Feat, parser::FeatureExtractor>;

/// Obtain a parser for a given language.
pub fn build_parser(lang: Lang) -> RustlingResult<Parser> {
    match lang {
        Lang::EN => en::build_parser(),
        Lang::FR => fr::build_parser(),
        Lang::ES => es::build_parser(),
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
            pub fn train_parser() -> ::RustlingResult<::Parser> {
                let rules = ::rules::$lang::rules_numbers()?;
                let exs = ::training::$lang::examples_numbers();
                let model = ::rustling::train::train(&rules, exs, ::parser::FeatureExtractor())?;
                Ok(::rustling::Parser::new(rules, model, ::parser::FeatureExtractor()))
            }

            pub fn build_parser() -> ::RustlingResult<::Parser> {
                let rules = ::rules::$lang::rules_numbers()?;
                let model = ::bincode::deserialize(include_bytes!(concat!(env!("OUT_DIR"), "/", stringify!($lang), ".bc"))).map_err(|e| format!("{:?}", e))?;
                Ok(::rustling::Parser::new(rules, model, ::parser::FeatureExtractor()))
            }
        }
    }
}

lang!(en);
lang!(es);
lang!(fr);
