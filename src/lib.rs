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
//!     let ctx = ResolverContext::default();
//!     let parser = build_parser(rustling_ontology::Lang::EN).unwrap();
//!     let result = parser.parse("twenty-one", &ctx).unwrap();
//!
//!     let int: output::IntegerOutput= result[0].value.clone().attempt_into().unwrap();
//!     assert_eq!(21, int.0);
//! }
//! ```
extern crate rmp_serde;
extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate rustling_ontology_moment;
extern crate rustling;
extern crate rustling_ontology_rules;
extern crate rustling_ontology_values;
extern crate rustling_ontology_training as training;

pub use rustling::{AttemptInto, ParsedNode, ParserMatch, Range, Value, Sym, ParsingAnalysis};
pub use rustling::errors::*;
pub use rustling_ontology_rules::{Lang, dims};
pub use rustling_ontology_values::dimension;
pub use rustling_ontology_values::output;
pub use rustling_ontology_values::output::{Output, OutputKind};
pub use rustling_ontology_values::{ResolverContext, IdentityContext, ParsingContext};
pub use rustling_ontology_moment::Interval;
pub use rustling_ontology_moment::Grain;

mod parser;
mod tagger;

pub use tagger::CandidateTagger;

// Rustling raw parser. Don't use directly
#[doc(hidden)]
pub type RawParser = rustling::Parser<dimension::Dimension, parser::Feat, parser::FeatureExtractor>;

/// Main class to be use at runtime.
pub struct Parser(RawParser);

impl Parser {
    pub fn parse_with_kind_order(&self,
                                 input: &str,
                                 context: &ResolverContext,
                                 order: &[OutputKind])
                                 -> RustlingResult<Vec<ParserMatch<Output>>> {
        let tagger = CandidateTagger {
            order: order,
            context: context,
            resolve_all_candidates: false,
        };
        Ok(self.0.parse(input, &tagger)?
            .into_iter()
            .filter_map(|m| {
                if let Some(v) = m.value {
                    Some(ParserMatch {
                        byte_range: m.byte_range,
                        char_range: m.char_range,
                        parsing_tree_height: m.parsing_tree_height,
                        parsing_tree_num_nodes: m.parsing_tree_num_nodes,
                        value: v,
                        probalog: m.probalog,
                        latent: m.latent,
                    })
                } else {
                    None
                }
            })
            .collect())
    }

    pub fn parse(&self,
                 input: &str,
                 context: &ResolverContext)
                 -> RustlingResult<Vec<ParserMatch<Output>>> {
        let all_output = OutputKind::all();
        self.parse_with_kind_order(input, context, &all_output)
    }

    pub fn analyse_with_kind_order(&self,
                                    examples: Vec<&str>,
                                    context: &ResolverContext,
                                    order:  &[OutputKind]) -> RustlingResult<ParsingAnalysis> {
        let tagger = CandidateTagger {
            order: order,
            context: context,
            resolve_all_candidates: false,
        };
        self.0.analyse(examples, &tagger)
    }

    pub fn analyse(&self, examples: Vec<&str>, context: &ResolverContext) -> RustlingResult<ParsingAnalysis> {
        let all_kind = OutputKind::all();
        self.analyse_with_kind_order(examples, &context, &all_kind)
    }

    pub fn num_rules(&self) -> usize {
        self.0.num_rules()
    }

    pub fn num_text_patterns(&self) -> usize {
        self.0.num_text_patterns()
    }
}

/// Obtain a parser for a given language.
pub fn build_parser(lang: Lang) -> RustlingResult<Parser> {
    match lang {
        Lang::DE => de::build_parser(),
        Lang::EN => en::build_parser(),
        Lang::FR => fr::build_parser(),
        Lang::ES => es::build_parser(),
        Lang::KO => ko::build_parser(),
        Lang::ZH => zh::build_parser(),
    }
}

/// Obtain a parser for a given language.
pub fn build_raw_parser(lang: Lang) -> RustlingResult<RawParser> {
    match lang {
        Lang::DE => de::build_raw_parser(),
        Lang::EN => en::build_raw_parser(),
        Lang::FR => fr::build_raw_parser(),
        Lang::ES => es::build_raw_parser(),
        Lang::KO => ko::build_raw_parser(),
        Lang::ZH => zh::build_raw_parser(),
    }
}

pub fn train_parser(lang: Lang) -> RustlingResult<Parser> {
    match lang {
        Lang::DE => de::train_parser(),
        Lang::EN => en::train_parser(),
        Lang::FR => fr::train_parser(),
        Lang::ES => es::train_parser(),
        Lang::KO => ko::train_parser(),
        Lang::ZH => zh::train_parser(),
    }
}

macro_rules! lang {
    ($lang:ident, $config:ident) => {
        mod $lang {
            use rustling_ontology_rules as rules;
            use super::*;

            pub fn train_parser() -> RustlingResult<Parser> {
                let rules = rules::$config::rule_set()?;
                let exs = ::training::$lang();
                let model = ::rustling::train::train(&rules, exs, ::parser::FeatureExtractor())?;
                Ok(Parser(::rustling::Parser::new(rules, model, ::parser::FeatureExtractor())))
            }

            pub fn build_raw_parser() -> RustlingResult<::RawParser> {
                let rules = rules::$config::rule_set()?;
                let model = ::rmp_serde::decode::from_read(&include_bytes!(concat!(env!("OUT_DIR"), "/", stringify!($lang), ".rmp"))[..]).map_err(|e| format!("{:?}", e))?;
                Ok(::RawParser::new(rules, model, ::parser::FeatureExtractor()))
            }

            pub fn build_parser() -> RustlingResult<::Parser> {
                build_raw_parser().map(::Parser)
            }
        }
    }
}

lang!(de, de_config);
lang!(en, en_config);
lang!(es, es_config);
lang!(fr, fr_config);
lang!(ko, ko_config);
lang!(zh, zh_config);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_long_number_en() {
        let ctx = ResolverContext::default();
        let parser = build_parser(Lang::EN).unwrap();
        let number = "one million five hundred twenty-one thousand eighty-two";
        let result = parser.parse_with_kind_order(number, &ctx,  &[OutputKind::Number]).unwrap();
        let int: output::IntegerOutput = result[0].value.clone().attempt_into().unwrap();
        assert_eq!(1521082, int.0);
    }

    #[test]
    #[ignore]
    fn time_resolve_complex_train_sentence() {
        let parser = build_raw_parser(Lang::EN).unwrap();
        //        let sent = "I want a return train ticket from Bordeaux to Strasbourg, friday the 12th of May, 10:32 am to wednesday the 7th of june, 6:22 pm";
        let sent = "I want a return train ticket from Bordeaux to Strasbourg, friday the 12th of May, 10:32 am to wednesday the 7th of june, 6:22 pm".to_lowercase();
        let tagger = CandidateTagger {
            order: &OutputKind::all(),
            context: &ResolverContext::default(),
            resolve_all_candidates: false,
        };
        let result = parser.candidates(&*sent, &tagger).unwrap();
        println!("{}", result.len());
        for r in &result {
            println!("{:?}", &sent[r.node.root_node.byte_range.0..r.node.root_node.byte_range.1]);
        }
        panic!();
    }
}
