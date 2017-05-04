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
        Lang::EN => build_parser_en(),
        Lang::FR => build_parser_fr(),
        Lang::ES => build_parser_es(),
    }
}

fn build_parser_en() -> RustlingResult<Parser> {
    let rules = rules::en::rules_numbers()?;
    let exs = training::en::examples_numbers();
    let model = rustling::train::train(&rules, exs, parser::FeatureExtractor())?;
    Ok(rustling::Parser::new(rules, model, parser::FeatureExtractor()))
}

fn build_parser_fr() -> RustlingResult<Parser> {
    let rules = rules::fr::rules_numbers()?;
    let exs = training::fr::examples_numbers();
    let model = rustling::train::train(&rules, exs, parser::FeatureExtractor())?;
    Ok(rustling::Parser::new(rules, model, parser::FeatureExtractor()))
}

fn build_parser_es() -> RustlingResult<Parser> {
    let rules = rules::es::rules_numbers()?;
    let exs = training::es::examples_numbers();
    let model = rustling::train::train(&rules, exs, parser::FeatureExtractor())?;
    Ok(rustling::Parser::new(rules, model, parser::FeatureExtractor()))
}
