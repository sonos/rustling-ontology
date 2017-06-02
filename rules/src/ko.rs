use rustling::*;
use values::dimension::*;
use values::dimension::Precision::*;

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
    //b.rule_1("integer - TYPE 1",
    //    b.reg(r#"[일|이|삼|사|오|육|칠|팔|구|십|백|천|만|억|조]+"#)?,
    //    |text_match| {
    //        unimplemented!()
    //    }
    //);
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

    // b.rule_1("number dot number - 삼점사",
    //     number_check!(|number: &NumberValue| !number.prefixed()),
    //     b.reg(r#"(점|쩜)([일|이|삼|사|오|육|칠|팔|구|영]+)"#)?,
    //     |a, text_match| {
    //         fn number_mapping(c: char) -> char {
    //             match c {
    //                 "일" => "1", 
    //                 "이" => "2", 
    //                 "삼" => "3",
    //                 "사" => "4", 
    //                 "오" => "5",
    //                 "육" => "6",
    //                 "칠" => "7",
    //                 "팔" => "8",
    //                 "구" => "9", 
    //                 "영" => "0",
    //             }
    //         }
    //         text_match.group(2).chars().map(number_mapping).collect::<String>();
    //         IntegerValue::new(1)
    //     }
    // );

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






    Ok(())
}
