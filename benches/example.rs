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
use bencher::Bencher;

#[derive(Debug, Deserialize)]
struct BenchInput {
   language: String,
   small_number: String,
   big_number: String,
   book_restaurant: String,
   batch_sentences: Vec<String>,
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

fn parsing_tagger<'a>(kinds: &'a [DimensionKind], context: &'a IdentityContext<Dimension>) -> CandidateTagger<'a, IdentityContext<Dimension>> {
    CandidateTagger {
        order: kinds,
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

fn parse_small_number(bench: &mut Bencher) {
    let input = parse_bench_input();

    let parser = build_raw_parser(input.rustling_lang()).unwrap();
    let number = input.small_number;
    let context = IdentityContext::new();
    let dims = DimensionKind::all();
    let tagger = parsing_tagger(&dims, &context);
    //let result = parser.parse(&number, true).unwrap();
    //let int: i64 = result[0].clone().value.attempt_into().unwrap();
    //assert_eq!(82, int);

    bench.iter(|| parser.parse(&number, &tagger));
}

fn parse_big_number(bench: &mut Bencher) {
    let input = parse_bench_input();

    let parser = build_raw_parser(input.rustling_lang()).unwrap();
    let number = input.big_number;
    let context = IdentityContext::new();
    let dims = DimensionKind::all();
    let tagger = parsing_tagger(&dims, &context);
    //let result = parser.parse(&number, true).unwrap();
    //let int: i64 = result[0].clone().value.attempt_into().unwrap();
    //assert_eq!(1521082, int);

    bench.iter(|| parser.parse(&number, &tagger));
}

fn parse_book_restaurant(bench: &mut Bencher) {
    let input = parse_bench_input();

    let parser = build_raw_parser(input.rustling_lang()).unwrap();
    let sentence = input.book_restaurant;
    let context = IdentityContext::new();
    let dims = DimensionKind::all();
    let tagger = parsing_tagger(&dims, &context);
    //let result = parser.parse(&sentence, true).unwrap();
    //println!("{:?}", result);
    //let int: i64 = result[0].clone().value.attempt_into().unwrap();
    //assert_eq!(4, int);

    bench.iter(|| parser.parse(&sentence, &tagger));
}

fn parse_batch_sentence(bench: &mut Bencher) {
    let input = parse_bench_input();

    let parser = build_parser(input.rustling_lang()).unwrap();
    let sentences = input.batch_sentences;
    let decoder = ResolverContext::default();
    bench.iter(|| {
        for i in sentences.iter() {
            let _ = parser.parse(&*i, &decoder);
        }
    });
}

fn parse_complex_train_sentence(bench: &mut Bencher) {
    let input = parse_bench_input();

    let parser = build_raw_parser(input.rustling_lang()).unwrap();
    let sent = input.complex_sentence.to_lowercase();
    let context = IdentityContext::new();
    let dims = DimensionKind::all();
    let tagger = parsing_tagger(&dims, &context);
    bench.iter(|| parser.parse(&*sent, &tagger));
}

fn parse_complex_train_sentence_end_to_end(bench: &mut Bencher) {
    let input = parse_bench_input();

    let parser = build_parser(input.rustling_lang()).unwrap();
    let decoder = ResolverContext::default();
    let sent = input.complex_sentence.to_lowercase();
    bench.iter(|| parser.parse(&*sent, &decoder));
}

fn time_resolve_complex_train_sentence(bench: &mut Bencher) {
    let input = parse_bench_input();

    let parser = build_raw_parser(input.rustling_lang()).unwrap();
    let decoder = ResolverContext::default();
    let context = IdentityContext::new();
    let dims = DimensionKind::all();
    let tagger = parsing_tagger(&dims, &context);
    let sent = input.complex_sentence.to_lowercase();
    /*
    for it in parser.parse(&*sent).unwrap() {
        println!("resolve: {:?}", it);
    }
    */
    let resolve = parser
        .parse(&*sent, &tagger)
        .unwrap()
        .into_iter()
        .rev()
        .filter(|r| decoder.resolve(r.value.as_ref().unwrap()).is_some())
        .max_by_key(|r| r.byte_range.1 - r.byte_range.0)
        .unwrap();
    bench.iter(|| decoder.resolve(resolve.value.as_ref().unwrap()));
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

benchmark_group!(benches,
                 parser_training,
                 parser_loading,
                 parse_small_number,
                 parse_big_number,
                 parse_book_restaurant,
                 parse_complex_train_sentence,
                 parse_batch_sentence,
                 parse_complex_train_sentence_end_to_end,
                 /*time_resolve_complex_train_sentence*/);

benchmark_main!(benches);
