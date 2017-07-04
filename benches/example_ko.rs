use rustling_ontology::*;
use rustling_ontology::AttemptInto;
use bencher::Bencher;

pub fn train_parser_ko(bench: &mut Bencher) {
    bench.iter(|| train_parser(Lang::KO).unwrap());
}

pub fn load_parser_ko(bench: &mut Bencher) {
    bench.iter(|| build_parser(Lang::KO).unwrap());
}

pub fn parse_small_number_ko(bench: &mut Bencher) {
    let parser = build_raw_parser(Lang::KO).unwrap();
    let number = "팔십이";
    let result = parser.parse(number, true).unwrap();
    let int: i64 = result[0].clone().value.attempt_into().unwrap();
    assert_eq!(82, int);

    bench.iter(|| parser.parse(number, true));
}

pub fn parse_big_number_ko(bench: &mut Bencher) {
    let parser = build_raw_parser(Lang::KO).unwrap();
    let number = "백오십이만 천 팔십이";
    let result = parser.parse(number, true).unwrap();
    let int: i64 = result[0].clone().value.attempt_into().unwrap();
    assert_eq!(1521082, int);

    bench.iter(|| parser.parse(number, true));
}

pub fn parse_book_restaurant_ko(bench: &mut Bencher) {
    let parser = build_raw_parser(Lang::KO).unwrap();
    let number = "식당에 네명 예약";
    let result = parser.parse(number, true).unwrap();
    //println!("{:?}", result);
    let int: i64 = result[0].clone().value.attempt_into().unwrap();
    assert_eq!(4, int);

    bench.iter(|| parser.parse(number, true));
}

pub fn parse_batch_sentence_ko(bench: &mut Bencher) {
   let input = vec![
        "정오에 날씨가 맑아요",
        "2036년 2월 11일에 햇볕이 좋은 날이 예상돼요?",
        "9주 뒤에 우리 동료가 있는 곳 근처에 날씨가 따뜻해?",
        "11년 후에 일기 예보는",
        "9년 반 뒤에 날씨가 어때",
        "한시간 십팔분 1초 뒤에 흐린지 알려줘",
        "1월 26일에 날씨가 더워",
        "지금보다 16초 후에 눈이 내릴까",
        "2037년 10월 24일에 뉴저지 카르마 근처에 날씨가 맑아요",
        "9개월 전에 오비드 근처에 날씨가 어땠어",
        "오전 5시에 미국 레딩 턴 쇼어에서 날씨가 어땠어",
        "21시간 19분 뒤에 날씨가 추울질 거예요?",
        "저녁 식사 때 온도가 올라갈 것이 예상돼요?",
        "1초 이전 날씨가 어땠어",
        "2031년 8월 21일에 더울 거야",
        "작년의 날씨는",
        "자정에 너의 집 근처 기상은 어땠어",
        "일출에 두울까요",
        "18시간 1분 3초 후의 기상은",
        "올 가을에 프랑스 파리에서 날씨가 습해요",
        "내년 기상은 어때",
        "3월 16일에 바람이 부를 것이 예상돼요?",
        "카라 집 근처에 비가 내릴 것이 예상돼요?",
        "올해 레이의 대학교 근처에 날씨가 습해요",
        "15분 있다가 날씨가 따뜻할까",
        "12월 25일에 날씨가 추운지 알려줘",
        "아침에 너의 이모의 호텔 근처에 날씨가 맑아요",
        "1분 이전 기상은",
        "8월 10일에 더월요",
        "오후 6시 20분에 비가 내릴까요",
        "화요일에서 날씨가 습해요",
        "작년 와이오밍 주 근처에 기상은 어땠어",
        "아침 식사 때 동티모르 주변에 날씨는",
   ];
   let parser = build_parser(Lang::KO).unwrap();
   let decoder = ParsingContext::default();
   bench.iter(|| {
        for i in input.iter() {
            parser.parse(&*i, &decoder, true);
        }
   });

}

pub fn parse_complex_train_sentence_ko(bench: &mut Bencher) {
    let parser = build_raw_parser(Lang::KO).unwrap();
    let sent = "보르도에서 출발하고 스트라스부르크에서 도착해서 5월 12일 금요일 오전 10시32 분부터 6월 7일 수요일 오후 6시 22분까지 가는 기차 표를 알려줘".to_lowercase();
    bench.iter(|| parser.parse(&*sent, true));
}

pub fn parse_complex_train_sentence_end_to_end_ko(bench: &mut Bencher) {
    let parser = build_parser(Lang::KO).unwrap();
    let decoder = ParsingContext::default();
    let sent = "보르도에서 출발하고 스트라스부르크에서 도착해서 5월 12일 금요일 오전 10시32 분부터 6월 7일 수요일 오후 6시 22분까지 가는 기차 표를 알려줘".to_lowercase();
    bench.iter(|| parser.parse(&*sent, &decoder, true));
}

pub fn time_resolve_complex_train_sentence_ko(bench: &mut Bencher) {
    let decoder = ParsingContext::default();
    let parser = build_raw_parser(Lang::KO).unwrap();
    let sent = "보르도에서 출발하고 스트라스부르크에서 도착해서 5월 12일 금요일 오전 10시32 분부터 6월 7일 수요일 오후 6시 22분까지 가는 기차 표를 알려줘".to_lowercase();
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