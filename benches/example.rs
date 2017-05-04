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
    let parser = build_parser(Lang::EN).unwrap();
    let number = "eighty-two";
    let result = parser.parse(number).unwrap();
    let int: i64 = result[0].value.attempt_to().unwrap();
    assert_eq!(82, int);

    bench.iter(|| parser.parse(number));
}

fn parse_big_number_en(bench: &mut Bencher) {
    let parser = build_parser(Lang::EN).unwrap();
    let number = "one million five hundred twenty-one thousand eighty-two";
    let result = parser.parse(number).unwrap();
    let int: i64 = result[0].value.attempt_to().unwrap();
    assert_eq!(1521082, int);

    bench.iter(|| parser.parse(number));
}

fn parse_book_restaurant(bench: &mut Bencher) {
    let parser = build_parser(Lang::EN).unwrap();
    let number = "book a restaurant for four people";
    let result = parser.parse(number).unwrap();
    let int: i64 = result[0].value.attempt_to().unwrap();
    assert_eq!(4, int);

    bench.iter(|| parser.parse(number));
}

benchmark_group!(benches,
                 load_parser_en,
                 train_parser_en,
                 parse_small_number_en,
                 parse_big_number_en,
                 parse_book_restaurant);
benchmark_main!(benches);
