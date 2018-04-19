use json_utils::*;
use moment;
use rustling_ontology::{ResolverContext, Interval, build_parser, Lang};
use serde_json;

use std::env;
use std::path;

pub fn run_json_test<P: AsRef<path::Path>>(lang: Lang, path: P) {
    let file = ::std::fs::File::open(path).unwrap();
    let utterances: Vec<Utterance> = serde_json::from_reader(&file).unwrap();
    let utterances: Vec<Utterance> = utterances.into_iter().filter(|it| it.in_grammar).collect();
    let parser = build_parser(lang).unwrap();
    for utterance in utterances {
        let context = ResolverContext::new(Interval::starting_at(utterance.context, moment::Grain::Second));
        let entities = parser.parse(utterance.phrase.as_str(), &context).unwrap();
        assert_eq!(entities.len(), 1, "Only one match was exepcted for this sentence: {:?}", utterance.phrase.as_str());

        let entity = entities.first();
        match (entity, utterance.value) {
            (Some(entity), Some(expected_value)) => {
                assert_eq!(entity.byte_range.len(), utterance.phrase.len(), "Expected full match for this sentence: {:?} found: {:?}", utterance.phrase.as_str(), entity);
                let value: SlotValue = entity.value.clone().into();
                assert_eq!(value, expected_value, "Sentence: {:?}, Found: {:?} expected: {:?}", utterance.phrase.as_str(), entities, expected_value);
            }
            (None, None) => {},
            (entity, utterance_value) => {
                assert!(false, "Found: {:?} expected: {:?}", entity, utterance_value);
            }
        }

    }
}

pub fn build_resources_path(lang: &str, file_name: &str) -> path::PathBuf {
    let path = env::var("SNIPS_RUSTLING_COVERAGE_RESOURCES")
        .map_err(|_| "SNIPS_RUSTLING_COVERAGE_RESOURCES env var not defined")
        .unwrap();
    path::PathBuf::from(path)
            .join(lang)
            .join(file_name)
}