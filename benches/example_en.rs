use rustling_ontology::*;
use rustling_ontology::AttemptInto;
use bencher::Bencher;

pub fn train_parser_en(bench: &mut Bencher) {
    bench.iter(|| train_parser(Lang::EN).unwrap());
}

pub fn load_parser_en(bench: &mut Bencher) {
    bench.iter(|| build_parser(Lang::EN).unwrap());
}

pub fn parse_small_number_en(bench: &mut Bencher) {
    let parser = build_raw_parser(Lang::EN).unwrap();
    let number = "eighty-two";
    let result = parser.parse(number, true).unwrap();
    let int: i64 = result[0].clone().value.attempt_into().unwrap();
    assert_eq!(82, int);

    bench.iter(|| parser.parse(number, true));
}

pub fn parse_big_number_en(bench: &mut Bencher) {
    let parser = build_raw_parser(Lang::EN).unwrap();
    let number = "one million five hundred twenty-one thousand eighty-two";
    let result = parser.parse(number, true).unwrap();
    let int: i64 = result[0].clone().value.attempt_into().unwrap();
    assert_eq!(1521082, int);

    bench.iter(|| parser.parse(number, true));
}

pub fn parse_book_restaurant_en(bench: &mut Bencher) {
    let parser = build_raw_parser(Lang::EN).unwrap();
    let number = "book a restaurant for four people";
    let result = parser.parse(number, true).unwrap();
    //println!("{:?}", result);
    let int: i64 = result[0].clone().value.attempt_into().unwrap();
    assert_eq!(4, int);

    bench.iter(|| parser.parse(number, true));
}

pub fn parse_batch_sentence_en(bench: &mut Bencher) {
   let input = vec![
        "will it be sunny for Midday",
        "should we expect the sun to shine  on February 11, 2036",
        "will it be warm near our colleague's' current location 9 weeks  from now",
        "how is the meteo in eleven years  please",
        "what are the weather conditions like in nine years and a half",
        "is it going to be cloudy one hour eighteen minutes and 1 second from now pleasewill it be hot  on January 26",
        "will it snow 16 seconds from now pleaseis it going to be sunny around Calmar, NJ on October the 24th, 2037",
        "what was the weather condition close from Ovid nine  months ago what were the weather conditions in the area of Redington Shores, Chera dynasty at five a.m.",
        "is it going to be cold in twenty one hours and 19 minutes",
        "should we expect the temperature to rise for supper",
        "what were the weather conditions one second ago pleaseis it going to be hot  on Aug. 21, 2031",
        "how was the meteo last year how were the weather conditions close to your place for midnight please",
        "is it going to be hot for sunrise",
        "how will the weather conditions be  in 18 hours 1 minute and 3 seconds",
        "will it be humid in Pataha, Palestyna  this autumn",
        "how will the weather condition be like next year",
        "should we expect wind  on march the 16th",
        "should I expect rain around kara's' house  on Nov. 7",
        "is it gonna be humid around leigh's' college this year",
        "is it going to be warm fifteen minutes from now is it going to be cold  on december the twenty-fifth",
        "is it going to be sunny next to your aunt's' hotel at morning please",
        "what were the weather conditions like 1 minute ago is it going to be hot  on august the 10th",
        "is it going to rain for twenty nine past eighteen",
        "will it be humid Tues.",
        "how were the weather conditions close to WY last year how is the weather condition in the area of Timor-Leste at breakfast"
   ];
   let parser = build_parser(Lang::EN).unwrap();
   let decoder = ParsingContext::default();
   bench.iter(|| {
        for i in input.iter() {
            parser.parse(&*i, &decoder, true);
        }
   });

}

pub fn parse_complex_train_sentence_en(bench: &mut Bencher) {
    let parser = build_raw_parser(Lang::EN).unwrap();
    let sent = "I want a return train ticket from Bordeaux to Strasbourg, friday the 12th of May, 10:32 am to wednesday the 7th of june, 6:22 pm".to_lowercase();
    bench.iter(|| parser.parse(&*sent, true));
}

pub fn parse_complex_train_sentence_end_to_end_en(bench: &mut Bencher) {
    let parser = build_parser(Lang::EN).unwrap();
    let decoder = ParsingContext::default();
    let sent = "I want a return train ticket from Bordeaux to Strasbourg, friday the 12th of May, 10:32 am to wednesday the 7th of june, 6:22 pm".to_lowercase();
    bench.iter(|| parser.parse(&*sent, &decoder, true));
}

pub fn time_resolve_complex_train_sentence_en(bench: &mut Bencher) {
    let decoder = ParsingContext::default();
    let parser = build_raw_parser(Lang::EN).unwrap();
    let sent = "I want a return train ticket from Bordeaux to Strasbourg, friday the 12th of May, 10:32 am to wednesday the 7th of june, 6:22 pm".to_lowercase();
    /*
    for it in parser.parse(&*sent).unwrap() {
        println!("resolve: {:?}", it);
    }
    */
    let resolve = parser
        .parse(&*sent, true)
        .unwrap()
        .into_iter()
        .rev()
        .filter(|r| decoder.resolve(&r.value).is_some())
        .max_by_key(|r| r.byte_range.1 - r.byte_range.0)
        .unwrap();
    bench.iter(|| decoder.resolve(&resolve.value));
}