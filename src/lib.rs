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

extern crate rustling;
extern crate rustling_ontology_moment;
extern crate rustling_ontology_grammar as grammar;
extern crate rustling_ontology_values;

pub use rustling::{AttemptInto, ParsedNode, ParserMatch, Range, Value, Sym, ParsingAnalysis};
pub use rustling::RustlingResult;
pub use grammar::{Lang, dims};
pub use rustling_ontology_values::dimension;
pub use rustling_ontology_values::output;
pub use rustling_ontology_values::output::{Output, OutputKind};
pub use rustling_ontology_values::{ResolverContext, IdentityContext, ParsingContext};
pub use rustling_ontology_moment::{Interval, Moment, Local, TimeZone};
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
    build_raw_parser(lang).map(::Parser)
}



/// Obtain a parser for a given language.
pub fn build_raw_parser(lang: Lang) -> RustlingResult<RawParser> {
    let rules = grammar::rules(lang)?;
    let model = match lang {
        Lang::DE => { ::rmp_serde::decode::from_read(&include_bytes!(concat!(env!("OUT_DIR"), "/de.rmp"))[..]) },
        Lang::EN => { ::rmp_serde::decode::from_read(&include_bytes!(concat!(env!("OUT_DIR"), "/en.rmp"))[..]) },
        Lang::ES => { ::rmp_serde::decode::from_read(&include_bytes!(concat!(env!("OUT_DIR"), "/es.rmp"))[..]) },
        Lang::IT => { ::rmp_serde::decode::from_read(&include_bytes!(concat!(env!("OUT_DIR"), "/it.rmp"))[..]) },
        Lang::FR => { ::rmp_serde::decode::from_read(&include_bytes!(concat!(env!("OUT_DIR"), "/fr.rmp"))[..]) },
        Lang::PT => { ::rmp_serde::decode::from_read(&include_bytes!(concat!(env!("OUT_DIR"), "/pt.rmp"))[..]) },
        Lang::JA => { ::rmp_serde::decode::from_read(&include_bytes!(concat!(env!("OUT_DIR"), "/ja.rmp"))[..]) },
        Lang::KO => { ::rmp_serde::decode::from_read(&include_bytes!(concat!(env!("OUT_DIR"), "/ko.rmp"))[..]) },
        Lang::ZH => { ::rmp_serde::decode::from_read(&include_bytes!(concat!(env!("OUT_DIR"), "/zh.rmp"))[..]) },
    }?;
    Ok(::RawParser::new(rules, model, ::parser::FeatureExtractor()))
}


pub fn train_parser(lang: Lang) -> RustlingResult<Parser> {
    let rules = grammar::rules(lang)?;
    let examples = grammar::examples(lang);
    let model = ::rustling::train::train(&rules, examples, ::parser::FeatureExtractor())?;
    Ok(Parser(::rustling::Parser::new(rules, model, ::parser::FeatureExtractor())))
}

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
