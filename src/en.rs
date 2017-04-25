use rustling::*;
use dimension::*;
use dimension::Precision::*;
use helpers;
use examples::*;


pub fn rules_finance() -> DucklingResult<RuleSet<Dimension>> {
    Ok(RuleSet(vec![
        rule! {
            "intersect (X cents)",
            (
                amount_of_money_check!(),
                amount_of_money_check!(|money: &AmountOfMoneyValue| money.unit == Some("cent"))
            ),
            |a, b| Ok(*a.value())
        }
    ]))
}

pub fn rules_temperature() -> DucklingResult<RuleSet<Dimension>> {
    Ok(RuleSet(vec![
        rule! { 
            "number as temp", 
            (number_check!()), 
            |a| Ok(TemperatureValue { value: a.value().value(), unit: None, latent: true}) 
        },
        rule! {
            "<latent temp> degrees",
            (temperature_check!(), regex!(r#"(deg(ree?)?s?\.?)|°"#)),
            |a, _| Ok(TemperatureValue { value: a.value().value, unit: Some("degree"), latent: false})
        },
        rule! {
            "<temp> Celcius",
            (temperature_check!(), regex!(r#"c(el[cs]?(ius)?)?\.?"#)),
            |a, _| Ok(TemperatureValue { value: a.value().value, unit: Some("celsius"), latent: false})
        },
        rule! {
            "<temp> Fahrenheit",
            (temperature_check!(), regex!(r#"f(ah?rh?eh?n(h?eit)?)?\.?"#)),
            |a, _| Ok(TemperatureValue { value: a.value().value, unit: Some("fahrenheit"), latent: false})
        },
    
    ]))
}

pub fn rules_numbers() -> DucklingResult<RuleSet<Dimension>> {
    Ok(RuleSet(vec![
        rule! {
            "intersect (with and)",
            (
                number_check!(|number: &NumberValue| number.grain().unwrap_or(0) > 1),
                regex!(r#"and"#),
                number_check!()
            ),
            |a, _, b| helpers::compose_numbers(&a.value(), &b.value())
        },
        rule! {
            "intersect",
            (
                number_check!(|number: &NumberValue| number.grain().unwrap_or(0) > 1),
                number_check!()
            ),
            |a, b| helpers::compose_numbers(&a.value(), &b.value())
        },
        rule! { 
            "integer (0..19)", 
            (regex!(r#"(none|zilch|naught|nought|nil|zero|one|two|three|fourteen|four|five|sixteen|six|seventeen|seven|eighteen|eight|nineteen|nine|eleven|twelve|thirteen|fifteen)"#)),
            |text_match| {
                let value = match text_match.group(1).as_ref()  {
                    "none" => 0, 
                    "zilch" => 0, 
                    "naught" => 0, 
                    "nought" => 0, 
                    "nil" => 0, 
                    "zero" => 0,
                    "one" => 1, 
                    "two" => 2, 
                    "three" => 3, 
                    "four" => 4, 
                    "five" => 5,
                    "six" => 6, 
                    "seven" => 7, 
                    "eight" => 8,
                    "nine" => 9, 
                    "ten" => 10, 
                    "eleven" => 11,
                    "twelve" => 12,
                    "thirteen" => 13,
                    "fourteen" => 14,
                    "fifteen" => 15,
                    "sixteen" => 16,
                    "seventeen" => 17, 
                    "eighteen" => 18, 
                    "nineteen" => 19,
                    _ => panic!("Unknow match"),
                };
                IntegerValue::new_with_grain(value, 1) 
            }
        },
        rule! { "ten", (regex!(r#"ten"#)), |_| IntegerValue::new_with_grain(10, 1) },
        rule! { "single", (regex!(r#"single"#)), |_| IntegerValue::new_with_grain(1, 1) },
        rule! { "a pair", (regex!(r#"a pair(?: of)?"#)), |_| IntegerValue::new_with_grain(2, 1) },
        rule! { "dozen", (regex!(r#"dozen"#)), |_| IntegerValue::new_with_grain(12, 1) },
        rule! { "hundred", (regex!(r#"hundreds?"#)), |_| IntegerValue::new_with_grain(100, 2) },
        rule! { "thousand", (regex!(r#"thousands?"#)), |_| IntegerValue::new_with_grain(1000, 3) },
        rule! { "million", (regex!(r#"millions?"#)), |_| IntegerValue::new_with_grain(1000000, 6) },
        rule! { "couple", (regex!(r#"(a )?couple( of)?"#)), |_| IntegerValue::new_with_grain(2, 1) },
        rule! { "few", (regex!(r#"(a )?few"#)), 
            |_| Ok(IntegerValue { value: 3, grain:Some(1), precision: Approximate, .. IntegerValue::default() })
        },
        rule! {
            "integer (20..90)",
            (regex!(r#"(twenty|thirty|fou?rty|fifty|sixty|seventy|eighty|ninety)"#)),
            |text_match| {
                let value = match text_match.group(1).as_ref()  {
                    "twenty"  => 20,
                    "thirty"  => 30,
                    "fourty"  => 40,
                    "forty"   => 40,
                    "fifty"   => 50,
                    "sixty"   => 60,
                    "seventy" => 70,
                    "eighty"  => 80,
                    "ninety"  => 90,
                    _ => panic!("Unknow match"),
                };
                IntegerValue::new_with_grain(value, 1) }
            },
        rule! {
            "integer 21..99",
            (
                integer_check!(10, 90, |integer: &IntegerValue| integer.value % 10 == 0),
                integer_check!(1, 9)
            ),
            |a, b| IntegerValue::new(a.value().value + b.value().value)
        },
        rule! {
            "integer (numeric)",
            (regex!(r#"(\d{1,18})"#)),
            |text_match| IntegerValue::new(text_match.group(0).parse()?)
        },
        rule! {
            "integer with thousands separator ,",
            (regex!(r#"(\d{1,3}(,\d\d\d){1,5})"#)),
            |text_match| {
                let reformatted_string = text_match.group(1).replace(",", "");
                let value: i64 = reformatted_string.parse()?;
                IntegerValue::new(value)
            }
        },
        rule! {
            "special composition for missing hundreds like in one twenty two",
            (
                integer_check!(1, 9),
                integer_check!(10, 99)
            ),
            |a, b| {
                let value = a.value().value * 100 + b.value().value;
                IntegerValue::new_with_grain(value, 1) }
            },
        rule! {
            "number dozen",
            (
                integer_check!(1, 10),
                integer_filter!(|integer: &IntegerValue| integer.group)
            ),
            |a, b| Ok(IntegerValue { value: a.value().value * b.value().value, grain:b.value().grain, ..IntegerValue::default() })
        },
        rule! {
             "number hundreds",
             (
                 integer_check!(1, 99),
                 integer_check!(100, 100)
             ),
             |a, b| Ok(IntegerValue { value: a.value().value * b.value().value, grain:b.value().grain, ..IntegerValue::default() })
        },
        rule! {
             "number thousands",
             (
                 integer_check!(1, 999),
                 integer_check!(1000, 1000)
             ),
             |a, b| Ok(IntegerValue { value: a.value().value * b.value().value, grain:b.value().grain, ..IntegerValue::default() })
        },
        rule! {
             "number millions",
             (
                 integer_check!(1, 99),
                 integer_check!(1000000, 1000000)
             ),
             |a, b| Ok(IntegerValue { value: a.value().value * b.value().value, grain:b.value().grain, ..IntegerValue::default() })
        },
        rule! {
             "decimal number",
             (regex!(r#"(\d*\.\d+)"#)),
             |text_match| {
                 let value: f32 = text_match.group(0).parse()?;
                 Ok(FloatValue { value: value, .. FloatValue::default() })
             }
        },
        rule! {
             "number dot number",
             (
                 number_check!(|number: &NumberValue| !number.prefixed()),
                 regex!(r#"dot|point"#),
                 number_check!(|number: &NumberValue| !number.suffixed())
             ),
             |a, _, b| Ok(FloatValue { value: b.value().value() * 0.1 + a.value().value(), .. FloatValue::default() })
        },
        rule! {
             "decimal with thousands separator",
             (regex!(r#"(\d+(,\d\d\d)+\.\d+)"#)),
             |text_match| {
                 let reformatted_string = text_match.group(1).replace(",", "");
                 let value: f32 = reformatted_string.parse()?;
                 Ok(FloatValue { value: value, .. FloatValue::default() })
             }
        },
        rule! {
            "numbers prefix with -, negative or minus",
            (
                regex!(r#"-|minus\s?|negative\s?"#),
                number_check!(|number: &NumberValue| !number.prefixed())
            ),
            |_, a| -> RuleResult<NumberValue> {
                Ok(match a.value().clone() { // checked
                    NumberValue::Integer(integer) => IntegerValue {
                                                        value: integer.value * -1,
                                                        prefixed: true,
                                                        .. integer
                                                    }.into(),
                    NumberValue::Float(float) => FloatValue {
                                                        value: float.value * -1.0,
                                                        prefixed: true,
                                                        .. float
                                                    }.into(),
                })
            }
        },
        rule! {
            "numbers suffixes (K, M, G)",
            (
                number_check!(|number: &NumberValue| !number.suffixed()),
                regex_neg_lh!(r#"([kmg])"#, r#"^[\W\$€]"#)
            ),
            |a, text_match| -> RuleResult<NumberValue> {
                let multiplier = match text_match.group(0).as_ref() {
                    "k" => 1000,
                    "m" => 1000000,
                    "g" => 1000000000,
                    _ => panic!("Unknown match"),
                };
                Ok(match a.value().clone() { // checked
                    NumberValue::Integer(integer) => IntegerValue {
                                                        value: integer.value * multiplier,
                                                        suffixed: true,
                                                        .. integer
                                                    }.into(),
                    NumberValue::Float(float) => {
                        let product = float.value * (multiplier as f32);
                        if product.floor() == product {
                            IntegerValue { value: product as i64, suffixed: true, ..IntegerValue::default() }.into()
                        } else {
                            FloatValue {
                                                        value: product,
                                                        suffixed: true,
                                                        .. float
                                                    }.into()
                        }
                    }
                })
            }
        },
        rule! {
             "ordinals (first..31st)",
            (regex!(r#"(first|second|third|fourth|fifth|sixth|seventh|eighth|ninth|tenth|eleventh|twelfth|thirteenth|fourteenth|fifteenth|sixteenth|seventeenth|eighteenth|nineteenth|twentieth|twenty-first|twenty-second|twenty-third|twenty-fourth|twenty-fifth|twenty-sixth|twenty-seventh|twenty-eighth|twenty-ninth|thirtieth|thirty-first)"#)),
             |text_match| {
                 let value = match text_match.group(1).as_ref() {
                     "first" => 1,
                     "second" => 2,
                     "third" => 3,
                     "fourth" => 4,
                     "fifth" => 5,
                     "sixth" => 6,
                     "seventh" => 7,
                     "eighth" => 8,
                     "ninth" => 9,
                     "tenth" => 10,
                     "eleventh" => 11,
                     "twelfth" => 12,
                     "thirteenth" => 13,
                     "fourteenth" => 14,
                     "fifteenth" => 15,
                     "sixteenth" => 16,
                     "seventeenth" => 17,
                     "eighteenth" => 18,
                     "nineteenth" => 19,
                     "twentieth" => 20,
                     "twenty-first" => 21,
                     "twenty-second" => 22,
                     "twenty-third" => 23,
                     "twenty-fourth" => 24,
                     "twenty-fifth" => 25,
                     "twenty-sixth" => 26,
                     "twenty-seventh" => 27,
                     "twenty-eighth" => 28,
                     "twenty-ninth" => 29,
                     "thirtieth" => 30,
                     "thirty-first" => 31,
                     _ => panic!("Unknow match"),
                 };
                 Ok(OrdinalValue { value: value })
             }
        },
        rule! {
            "<number> <ordinal>",
             (
                 integer_check!(10, 90, |integer: &IntegerValue| integer.value % 10 == 0),
                 ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 9)
             ),
             |integer, ordinal| Ok(OrdinalValue { value: integer.value().value + ordinal.value().value })
        },
        rule! {
            "ordinal (digits)",
            (regex!(r#"0*(\d+) ?(st|nd|rd|th)"#)),
            |text_match| {
                let value: i64 = text_match.group(1).parse()?;
                Ok(OrdinalValue { value: value })
            }
        },
        rule! {
            "the <ordinal>",
            (
                regex!(r#"the"#),
                ordinal_check!()
            ),
            |_, ordinal| Ok(*ordinal.value())
        }
    ]))
}

pub fn examples_numbers() -> Vec<::rustling::train::Example<Dimension>> {
    let mut v = vec![];
    example!(v, check_integer(0), "0", "naught", "nought", "zero", "nil");
    example!(v, check_integer(1), "1", "one", "single");
    example!(v, check_integer(2), "2", "two", "a pair");
    example!(v, check_integer(33), "33", "thirty three", "0033");
    example!(v, check_integer(14), "14", "fourteen");
    example!(v, check_integer(16), "16", "sixteen");
    example!(v, check_integer(17), "17", "seventeen");
    example!(v, check_integer(18), "18", "eighteen");
    example!(v, check_float(1.1), "1.1", "1.10", "01.10");
    example!(v, check_float(0.77), "0.77", ".77");
    example!(v,
             check_integer(100000),
             "100,000",
             "100000",
             "100K",
             "100k");
    example!(v,
             check_integer(3000000),
             "3M",
             "3000K",
             "3000000",
             "3,000,000");
    example!(v,
             check_integer(1200000),
             "1,200,000",
             "1200000",
             "1.2M",
             "1200K",
             ".0012G");
    example!(v,
             check_integer(-1200000),
             "- 1,200,000",
             "-1200000",
             "minus 1,200,000",
             "negative 1200000",
             "-1.2M",
             "-1200K",
             "-.0012G");
    example!(v, check_integer(5000), "5 thousand", "five thousand");
    example!(v, check_integer(122), "one twenty two");
    example!(v, check_integer(200000), "two hundred thousand");
    example!(v, check_integer(21011), "twenty-one thousand eleven");
    example!(v,
             check_integer(721012),
             "seven hundred twenty-one thousand twelve",
             "seven hundred twenty-one thousand and twelve");
    example!(v,
             check_integer(31256721),
             "thirty-one million two hundred fifty-six thousand seven hundred twenty-one");
    example!(v, check_ordinal(4), "the 4th", "4th", "fourth");
    example!(v, check_ordinal(3), "the 3rd", "3rd", "third");
    example!(v, check_ordinal(2), "the 2nd", "2nd", "second");
    example!(v, check_ordinal(21), "the twenty first");
    v
}
