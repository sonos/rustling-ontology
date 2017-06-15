use rustling::*;
use values::dimension::*;
use values::dimension::Precision::*;
use values::helpers;
use regex::Regex;

pub fn rules_numbers(b:&mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1("integer (numeric)",
        b.reg(r#"(\d{1,18})"#)?,
        |text_match| {
            let value: i64 = text_match.group(1).parse()?;
            IntegerValue::new(value)
    });
    b.rule_1("integer with thousands separator ,",
        b.reg(r#"(\d{1,3}(,\d\d\d){1,5})"#)?,
        |text_match| {
            let reformatted_string = text_match.group(1).replace(",", "");
            let value: i64 = reformatted_string.parse()?;
            IntegerValue::new(value)
        }
    );
    b.rule_1("integer 0",
        b.reg(r#"영|공|빵"#)?,
        |_| IntegerValue::new(0)
    );

    b.rule_1("half - 반",
        b.reg(r#"반"#)?,
        |_| FloatValue::new(0.5)
    );
    b.rule_1("few 몇",
        b.reg(r#"몇"#)?,
        |_|  Ok(IntegerValue {
                value: 3,
                precision: Approximate,
                .. IntegerValue::default()
            })
    );
    b.rule_1("integer - TYPE 1",
        b.reg(r#"[일|이|삼|사|오|육|칠|팔|구|십|백|천|만|억|조]+"#)?,
        |text_match| {
            fn map_number(s: char) -> i64 {
                match s {
                    '일' => 1, 
                    '이' => 2, 
                    '삼' => 3, 
                    '사' => 4, 
                    '오' => 5, 
                    '육' => 6, 
                    '칠' => 7, 
                    '팔' => 8, 
                    '구' => 9, 
                    '천' => 1, 
                    '백' => 1, 
                    '십' => 1,
                    _ => 0,
                }
            }

            fn get_number(s: &str) -> RuleResult<i64> {
                let regex = Regex::new(r#"(.*천)?(.*백)?(.*십)?(.*)?"#)?;
                let groups = helpers::find_regex_group(&regex, s)?
                    .into_iter()
                    .nth(0)
                    .ok_or_else(|| format!("Regex {:?} has no match for {:?}", regex, s))?
                    .groups;
                let number = 1000 * groups.get(1).and_then(|g| *g)
                                          .and_then(|g| g.chars().nth(0))
                                          .map(|g| map_number(g))
                                          .unwrap_or(0)
                            + 100 * groups.get(2).and_then(|g| *g)
                                          .and_then(|g| g.chars().nth(0))
                                          .map(|g| map_number(g))
                                          .unwrap_or(0)
                            + 10 * groups.get(3).and_then(|g| *g)
                                          .and_then(|g| g.chars().nth(0))
                                          .map(|g| map_number(g))
                                          .unwrap_or(0)
                            + groups.get(4).and_then(|g| *g)
                                          .and_then(|g| g.chars().nth(0))
                                          .map(|g| map_number(g))
                                          .unwrap_or(0);
                Ok(number)
            }

            let regex = Regex::new(r#"(.*조)?(.*억)?(.*만)?(.*)?"#)?;
            let groups = helpers::find_regex_group(&regex, text_match.group(0))?
                    .into_iter()
                    .nth(0)
                    .ok_or_else(|| format!("Regex {:?} has no match for {:?}", regex, text_match.group(0)))?
                    .groups;

            let value = 1000000000000 * groups.get(1).and_then(|g| *g)
                                              .map(|g| get_number(g))
                                              .unwrap_or(Ok(0))?
                        + 100000000 * groups.get(2).and_then(|g| *g)
                                            .map(|g| get_number(g))
                                            .unwrap_or(Ok(0))?
                        + 10000 * groups.get(3).and_then(|g| *g)
                                        .map(|g| if g == "만" { Ok(1) } else { get_number(g)})
                                        .unwrap_or(Ok(0))?;

            IntegerValue::new(value)
        }
    );
    b.rule_1("integer (1..10) - TYPE 2",
        b.reg(r#"(하나|둘|셋|넷|다섯|여섯|일곱|여덟|아홉)"#)?,
        |text_match| {
            let value = match text_match.group(1).as_ref() {
                     "하나" => 1,
                     "둘" => 2,
                     "셋" => 3,
                     "넷" => 4,
                     "다섯" => 5,
                     "여섯" => 6,
                     "일곱" => 7,
                     "여덟" => 8,
                     "아홉" => 9,
                     _ => panic!("Unknow match"),
                 };
            IntegerValue::new(value)
        }
    );
    b.rule_1("integer (1..4) - for ordinals",
        b.reg(r#"(한|첫|두|세|네)"#)?,
        |text_match| {
            let value = match text_match.group(1).as_ref() {
                "한" => 1,
                "첫" => 1, 
                "두" => 2,
                "세" => 3,
                "네" => 4,
                _ => panic!("Unknow match"),
            };
            IntegerValue::new(value)
        }
    );

    b.rule_1("integer (20..90) - TYPE 2 and ordinals",
        b.reg(r#"(열|스물|서른|마흔|쉰|예순|일흔|여든|아흔)"#)?,
        |text_match| {
            let value = match text_match.group(1).as_ref() {
                "열"   => 10, 
                "스물" => 20, 
                "서른" => 30, 
                "마흔" => 40, 
                "쉰"   => 50,
                "예순" => 60, 
                "일흔" => 70, 
                "여든" => 80, 
                "아흔" => 90,
                _ => panic!("Unknow match"),
            };
            IntegerValue::new(value)
        }
    );

    b.rule_2("integer (21..99) - TYPE 2",
        integer_check!(20, 90, |integer: &IntegerValue| integer.value % 10 == 0),
        integer_check!(1, 9),
        |a, b| IntegerValue::new(a.value().value + b.value().value)
    );

    b.rule_1("decimal number",
        b.reg(r#"(\d*\.\d+)"#)?,
        |text_match| FloatValue::new(text_match.group(1).parse()?)
    );

    b.rule_2("number dot number - 삼점사",
        number_check!(|number: &NumberValue| !number.prefixed()),
        b.reg(r#"(점|쩜)([일|이|삼|사|오|육|칠|팔|구|영]+)"#)?,
        |a, text_match| {
            fn number_mapping(c: char) -> char {
                match c {
                    '일' => '1', 
                    '이' => '2', 
                    '삼' => '3',
                    '사' => '4', 
                    '오' => '5',
                    '육' => '6',
                    '칠' => '7',
                    '팔' => '8',
                    '구' => '9', 
                    '영' => '0',
                     _   => panic!("Unknow match"),
                }
            }
            let mut number: String = "0.".into();
            number.push_str(&text_match.group(2).chars().map(number_mapping).collect::<String>());
            IntegerValue::new(1)
        }
    );

    b.rule_1("decimal with thousands separator",
        b.reg(r#"(\d+(,\d\d\d)+\.\d+)"#)?,
        |text_match| FloatValue::new(text_match.group(1).replace(",", "").parse()?)
    );
    b.rule_2("numbers prefix with -, 마이너스, or 마이나스",
        b.reg(r#"-|마이너스\s?|마이나스\s?"#)?,
        number_check!(|number: &NumberValue| !number.prefixed()),
        |_, a| -> RuleResult<NumberValue> {
            Ok(match a.value().clone() {
                   NumberValue::Integer(integer) => {
                       IntegerValue {
                               value: integer.value * -1,
                               prefixed: true,
                               ..integer
                           }
                           .into()
                   }
                   NumberValue::Float(float) => {
                       FloatValue {
                               value: float.value * -1.0,
                               prefixed: true,
                               ..float
                           }
                           .into()
                   }
            })
        }
    );
    b.rule_2("ordinals (첫번째)",
        integer_check!(),
        b.reg(r#"번째|째|째번"#)?,
        |a, _| Ok(OrdinalValue { value: a.value().value })
    );
    b.rule_3("fraction",
        number_check!(|number: &NumberValue| !number.prefixed()),
        b.reg(r#"분(의|에)"#)?,
        number_check!(|number: &NumberValue| !number.suffixed()),
        |a, _, b| FloatValue::new(a.value().value() / b.value().value())
    );
    b.rule_3("fraction",
        number_check!(|number: &NumberValue| !number.prefixed()),
        b.reg(r#"/"#)?,
        number_check!(|number: &NumberValue| !number.suffixed()),
        |a, _, b| FloatValue::new(a.value().value() / b.value().value())
    );
    Ok(())
}
