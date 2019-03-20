#[macro_use]
extern crate bencher;
extern crate rustling_ontology;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::env;
use std::fs;
use std::path;
use std::str::FromStr;

use rustling_ontology::*;
use rustling_ontology::dimension::Dimension;
use bencher::Bencher;

#[derive(Debug, Deserialize)]
struct BenchInput {
   language: String,
   small_numbers: Option<Vec<String>>,
   big_numbers: Option<Vec<String>>,
   intent_sentences: Vec<String>,
   complex_sentence: String,
}

impl BenchInput {
    fn rustling_lang(&self) -> Lang {
        Lang::from_str(&self.language).unwrap()
    }
}

fn parse_bench_input() -> BenchInput {
    let path = env::var("SNIPS_RUSTLING_BENCH_INPUT")
        .map_err(|_| "SNIPS_RUSTLING_BENCH_INPUT env var not defined")
        .unwrap();
    let file = fs::File::open(file_path(&path)).unwrap();

    serde_json::from_reader(file).unwrap()
}

fn file_path(file_name: &str) -> path::PathBuf {
    if env::var("DINGHY").is_ok() {
        env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("test_data/data")
            .join(file_name)
    } else {
        path::PathBuf::from("data").join(file_name)
    }
}

fn parsing_tagger<'a>(kinds: &'a [OutputKind], context: &'a IdentityContext<Dimension>) -> CandidateTagger<'a, IdentityContext<Dimension>> {
    CandidateTagger {
        output_kind_filter: kinds,
        context: context,
        resolve_all_candidates: false,
    }
}

fn parser_training(bench: &mut Bencher) {
    let input = parse_bench_input();

    bench.iter(|| train_parser(input.rustling_lang()).unwrap());
}

fn parser_loading(bench: &mut Bencher) {
    let input = parse_bench_input();

    bench.iter(|| build_parser(input.rustling_lang()).unwrap());
}

fn parse_small_numbers(bench: &mut Bencher) {
    let input = parse_bench_input();

    let parser = build_parser(input.rustling_lang()).unwrap();
    let context = ResolverContext::default();
    let sentences = input.small_numbers.unwrap_or(vec![]);
    bench.iter(|| {
        for i in sentences.iter() {
            let _ = parser.parse(&*i, &context);
        }
    });
}

fn parse_big_numbers(bench: &mut Bencher) {
    let input = parse_bench_input();
    let parser = build_parser(input.rustling_lang()).unwrap();
    let context = ResolverContext::default();
    let sentences = input.big_numbers.unwrap_or(vec![]);
    bench.iter(|| {
        for i in sentences.iter() {
            let _ = parser.parse(&*i, &context);
        }
    });
}

fn parse_intent_sentences(bench: &mut Bencher) {
    let input = parse_bench_input();
    let parser = build_parser(input.rustling_lang()).unwrap();
    let sentences = input.intent_sentences;
    let context = ResolverContext::default();
    bench.iter(|| {
        for i in sentences.iter() {
            let _ = parser.parse(&*i, &context);
        }
    });
}

fn parse_complex_time_sentence(bench: &mut Bencher) {
    let input = parse_bench_input();
    let parser = build_parser(input.rustling_lang()).unwrap();
    let context = ResolverContext::default();
    let sentence = input.complex_sentence;
    bench.iter(|| parser.parse(&sentence, &context));
}

benchmark_group!(benches,
                 parse_small_numbers,
                 parse_big_numbers,
                 parse_intent_sentences,
                 parse_complex_time_sentence
                 );

benchmark_main!(benches);
