#[macro_use]
extern crate bencher;
extern crate rustling_ontology;

mod example_en;
mod example_ko;

use example_en::*;
use example_ko::*;

benchmark_group!(benches,
                 load_parser_en,
                 train_parser_en,
                 parse_small_number_en,
                 parse_big_number_en,
                 parse_book_restaurant_en,
                 parse_complex_train_sentence_en,
                 parse_batch_sentence_en,
                 parse_complex_train_sentence_end_to_end_en,
                 //time_resolve_complex_train_sentence_en,
                 load_parser_ko,
                 train_parser_ko,
                 parse_small_number_ko,
                 parse_big_number_ko,
                 //parse_book_restaurant_ko,
                 parse_complex_train_sentence_ko,
                 parse_batch_sentence_ko,
                 parse_complex_train_sentence_end_to_end_ko,
                 //time_resolve_complex_train_sentence_ko
                 );
benchmark_main!(benches);
