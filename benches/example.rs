#[macro_use]
extern crate bencher;
extern crate rustling_ontology;

use rustling_ontology::*;
use rustling_ontology::AttemptTo;
use bencher::Bencher;

fn train_parser_en(bench: &mut Bencher) {
    bench.iter(|| train_parser(Lang::EN).unwrap());
}

fn load_parser_en(bench: &mut Bencher) {
    bench.iter(|| build_parser(Lang::EN).unwrap());
}

fn parse_small_number_en(bench: &mut Bencher) {
    let parser = build_raw_parser(Lang::EN).unwrap();
    let number = "eighty-two";
    let result = parser.parse(number).unwrap();
    let int: i64 = result[0].value.attempt_to().unwrap();
    assert_eq!(82, int);

    bench.iter(|| parser.parse(number));
}

fn parse_big_number_en(bench: &mut Bencher) {
    let parser = build_raw_parser(Lang::EN).unwrap();
    let number = "one million five hundred twenty-one thousand eighty-two";
    let result = parser.parse(number).unwrap();
    println!("result: {:?}", result);
    let int: i64 = result[0].value.attempt_to().unwrap();
    assert_eq!(1521082, int);

    bench.iter(|| parser.parse(number));
}

fn parse_book_restaurant(bench: &mut Bencher) {
    let parser = build_raw_parser(Lang::EN).unwrap();
    let number = "book a restaurant for four people";
    let result = parser.parse(number).unwrap();
    let int: i64 = result[0].value.attempt_to().unwrap();
    assert_eq!(4, int);

    bench.iter(|| parser.parse(number));
}

fn parse_complex_train_sentence(bench: &mut Bencher) {
    let parser = build_raw_parser(Lang::EN).unwrap();
    let sent = "I want a return train ticket from Bordeaux to Strasbourg, friday the 12th of May, 10:32 am to wednesday the 7th of june, 6:22 pm".to_lowercase();
    bench.iter(|| parser.parse(&*sent));
}

fn time_resolve_complex_train_sentence(bench: &mut Bencher) {
    let decoder = ParsingContext::default();
    let parser = build_raw_parser(Lang::EN).unwrap();
    let sent = "I want a return train ticket from Bordeaux to Strasbourg, friday the 12th of May, 10:32 am to wednesday the 7th of june, 6:22 pm".to_lowercase();
    for it in parser.parse(&*sent).unwrap() {
        println!("resolve: {:?}", it);
    }
    let resolve = parser
        .parse(&*sent)
        .unwrap()
        .into_iter()
        .rev()
        .filter(|r| decoder.resolve(&r.value).is_some())
        .max_by_key(|r| r.range.1 - r.range.0)
        .unwrap();
    bench.iter(|| decoder.resolve(&resolve.value));
}

benchmark_group!(benches,
                 load_parser_en,
                 train_parser_en,
                 parse_small_number_en,
                 parse_big_number_en,
                 parse_book_restaurant,
                 parse_complex_train_sentence,
                 time_resolve_complex_train_sentence);
benchmark_main!(benches);
