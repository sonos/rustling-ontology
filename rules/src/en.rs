use rustling::*;
use dimension::*;
use dimension::Precision::*;
use helpers;

#[allow(dead_code)]
pub fn rules_finance() -> RustlingResult<RuleSet<Dimension>> {
    let b = RuleSetBuilder::default();
    b.rule_2("intersect (X cents)",
             amount_of_money_check!(),
             amount_of_money_check!(|money: &AmountOfMoneyValue| money.unit == Some("cent")),
             |a, b| helpers::compose_money(a.value(), b.value()));
    b.rule_3("intersect (and X cents)",
             amount_of_money_check!(),
             b.reg(r#"and"#)?,
             amount_of_money_check!(|money: &AmountOfMoneyValue| money.unit == Some("cent")),
             |a, _, b| helpers::compose_money(&a.value(), &b.value()));
    b.rule_2("intersect",
             amount_of_money_check!(),
             number_check!(),
             |a, b| helpers::compose_money_number(&a.value(), &b.value()));
    b.rule_3("intersect (and number)",
             amount_of_money_check!(),
             b.reg(r#"and"#)?,
             number_check!(),
             |a, _, b| helpers::compose_money_number(&a.value(), &b.value()));
    b.rule_1("$",
             b.reg(r#"\$|dollars?"#)?,
             |_| Ok(MoneyUnitValue { unit: Some("$") }));
    b.rule_1("€",
             b.reg(r#"€|([e€]uro?s?)"#)?,
             |_| Ok(MoneyUnitValue { unit: Some("€") }));
    b.rule_1("£",
             b.reg(r#"£|pounds?"#)?,
             |_| Ok(MoneyUnitValue { unit: Some("£") }));
    b.rule_1("USD",
             b.reg(r#"us[d\$]"#)?,
             |_| Ok(MoneyUnitValue { unit: Some("USD") }));
    b.rule_1("GBP",
             b.reg(r#"gbp"#)?,
             |_| Ok(MoneyUnitValue { unit: Some("GBP") }));
    b.rule_1("PTS",
             b.reg(r#"pta?s?"#)?,
             |_| Ok(MoneyUnitValue { unit: Some("PTS") }));
    b.rule_1("cent",
             b.reg(r#"cents?|penn(y|ies)|c|¢"#)?,
             |_| Ok(MoneyUnitValue { unit: Some("cent") }));
    b.rule_1("INR",
             b.reg(r#""#)?,
             |_| Ok(MoneyUnitValue { unit: Some("INR") }));
    b.rule_1("unnamed currency",
             b.reg(r#"(buck|balle|pouloute)s?"#)?,
             |_| Ok(MoneyUnitValue { unit: None }));
    b.rule_2("<unit> <amount>", money_unit!(), number_check!(), |a, b| {
        Ok(AmountOfMoneyValue {
               value: b.value().value(),
               unit: a.value().unit,
               ..AmountOfMoneyValue::default()
           })
    });
    b.rule_2("<amount> <unit>", number_check!(), money_unit!(), |a, b| {
        Ok(AmountOfMoneyValue {
               value: a.value().value(),
               unit: b.value().unit,
               ..AmountOfMoneyValue::default()
           })
    });
    b.rule_2("about <amount-of-money>",
             b.reg(r#"about|approx(\.|imately)?|close to|near( to)?|around|almost"#)?,
             amount_of_money_check!(),
             |_, a| {
                 Ok(AmountOfMoneyValue {
                        precision: Approximate,
                        ..a.value().clone()
                    })
             });
    b.rule_2("exactly <amount-of-money>",
             b.reg(r#"exactly|precisely"#)?,
             amount_of_money_check!(),
             |_, a| {
                 Ok(AmountOfMoneyValue {
                        precision: Exact,
                        ..a.value().clone()
                    })
             });
    Ok(b.build())
}

#[allow(dead_code)]
pub fn rules_temperature() -> RustlingResult<RuleSet<Dimension>> {
    let b = RuleSetBuilder::default();
    b.rule_1("number as temp", number_check!(), |a| {
        Ok(TemperatureValue {
               value: a.value().value(),
               unit: None,
               latent: true,
           })
    });
    b.rule_2("<latent temp> degrees",
             temperature_check!(),
             b.reg(r#"(deg(ree?)?s?\.?)|°"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                        value: a.value().value,
                        unit: Some("degree"),
                        latent: false,
                    })
             });
    b.rule_2("<temp> Celcius",
             temperature_check!(),
             b.reg(r#"c(el[cs]?(ius)?)?\.?"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                        value: a.value().value,
                        unit: Some("celsius"),
                        latent: false,
                    })
             });
    b.rule_2("<temp> Fahrenheit",
             temperature_check!(),
             b.reg(r#"f(ah?rh?eh?n(h?eit)?)?\.?"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                        value: a.value().value,
                        unit: Some("fahrenheit"),
                        latent: false,
                    })
             });
    Ok(b.build())
}

pub fn rules_numbers() -> RustlingResult<RuleSet<Dimension>> {
    let b = RuleSetBuilder::default();
    b.rule_3("intersect (with and)",
             number_check!(|number: &NumberValue| number.grain().unwrap_or(0) > 1),
             b.reg(r#"and"#)?,
             number_check!(),
             |a, _, b| helpers::compose_numbers(&a.value(), &b.value()));
    b.rule_2("intersect",
             number_check!(|number: &NumberValue| number.grain().unwrap_or(0) > 1),
             number_check!(),
             |a, b| helpers::compose_numbers(&a.value(), &b.value()));
    b.rule_1(
            "integer (0..19)", 
            b.reg(r#"(none|zilch|naught|nought|nil|zero|one|two|three|fourteen|four|five|sixteen|six|seventeen|seven|eighteen|eight|nineteen|nine|eleven|twelve|thirteen|fifteen)"#)?,
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
            });
    b.rule_1("ten",
             b.reg(r#"ten"#)?,
             |_| IntegerValue::new_with_grain(10, 1));
    b.rule_1("single",
             b.reg(r#"single"#)?,
             |_| IntegerValue::new_with_grain(1, 1));
    b.rule_1("a pair",
             b.reg(r#"a pair(?: of)?"#)?,
             |_| IntegerValue::new_with_grain(2, 1));
    b.rule_1("dozen",
             b.reg(r#"dozen"#)?,
             |_| IntegerValue::new_with_grain(12, 1));
    b.rule_1("hundred",
             b.reg(r#"hundreds?"#)?,
             |_| IntegerValue::new_with_grain(100, 2));
    b.rule_1("thousand",
             b.reg(r#"thousands?"#)?,
             |_| IntegerValue::new_with_grain(1000, 3));
    b.rule_1("million",
             b.reg(r#"millions?"#)?,
             |_| IntegerValue::new_with_grain(1000000, 6));
    b.rule_1("couple",
             b.reg(r#"(a )?couple( of)?"#)?,
             |_| IntegerValue::new_with_grain(2, 1));
    b.rule_1("few", b.reg(r#"(a )?few"#)?, |_| {
        Ok(IntegerValue {
               value: 3,
               grain: Some(1),
               precision: Approximate,
               ..IntegerValue::default()
           })
    });
    b.rule_1("integer (20..90)",
             b.reg(r#"(twenty|thirty|fou?rty|fifty|sixty|seventy|eighty|ninety)"#)?,
             |text_match| {
        let value = match text_match.group(1).as_ref() {
            "twenty" => 20,
            "thirty" => 30,
            "fourty" => 40,
            "forty" => 40,
            "fifty" => 50,
            "sixty" => 60,
            "seventy" => 70,
            "eighty" => 80,
            "ninety" => 90,
            _ => panic!("Unknow match"),
        };
        IntegerValue::new_with_grain(value, 1)
    });
    b.rule_2("integer 21..99",
             integer_check!(10, 90, |integer: &IntegerValue| integer.value % 10 == 0),
             integer_check!(1, 9),
             |a, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_1("integer (numeric)",
             b.reg(r#"(\d{1,18})"#)?,
             |text_match| IntegerValue::new(text_match.group(0).parse()?));
    b.rule_1("integer with thousands separator ,",
             b.reg(r#"(\d{1,3}(,\d\d\d){1,5})"#)?,
             |text_match| {
                 let reformatted_string = text_match.group(1).replace(",", "");
                 let value: i64 = reformatted_string.parse()?;
                 IntegerValue::new(value)
             });
    b.rule_2("special composition for missing hundreds like in one twenty two",
             integer_check!(1, 9),
             integer_check!(10, 99),
             |a, b| {
                 let value = a.value().value * 100 + b.value().value;
                 IntegerValue::new_with_grain(value, 1)
             });
    b.rule_2("number dozen",
             integer_check!(1, 10),
             integer_filter!(|integer: &IntegerValue| integer.group),
             |a, b| {
                 Ok(IntegerValue {
                        value: a.value().value * b.value().value,
                        grain: b.value().grain,
                        ..IntegerValue::default()
                    })
             });
    b.rule_2("number hundreds",
             integer_check!(1, 99),
             integer_check!(100, 100),
             |a, b| {
                 Ok(IntegerValue {
                        value: a.value().value * b.value().value,
                        grain: b.value().grain,
                        ..IntegerValue::default()
                    })
             });
    b.rule_2("number thousands",
             integer_check!(1, 999),
             integer_check!(1000, 1000),
             |a, b| {
                 Ok(IntegerValue {
                        value: a.value().value * b.value().value,
                        grain: b.value().grain,
                        ..IntegerValue::default()
                    })
             });
    b.rule_2("number millions",
             integer_check!(1, 99),
             integer_check!(1000000, 1000000),
             |a, b| {
                 Ok(IntegerValue {
                        value: a.value().value * b.value().value,
                        grain: b.value().grain,
                        ..IntegerValue::default()
                    })
             });
    b.rule_1("decimal number", b.reg(r#"(\d*\.\d+)"#)?, |text_match| {
        let value: f32 = text_match.group(0).parse()?;
        Ok(FloatValue {
               value: value,
               ..FloatValue::default()
           })
    });
    b.rule_3("number dot number",
             number_check!(|number: &NumberValue| !number.prefixed()),
             b.reg(r#"dot|point"#)?,
             number_check!(|number: &NumberValue| !number.suffixed()),
             |a, _, b| {
                 Ok(FloatValue {
                        value: b.value().value() * 0.1 + a.value().value(),
                        ..FloatValue::default()
                    })
             });
    b.rule_1("decimal with thousands separator",
             b.reg(r#"(\d+(,\d\d\d)+\.\d+)"#)?,
             |text_match| {
                 let reformatted_string = text_match.group(1).replace(",", "");
                 let value: f32 = reformatted_string.parse()?;
                 Ok(FloatValue {
                        value: value,
                        ..FloatValue::default()
                    })
             });
    b.rule_2("numbers prefix with -, negative or minus",
             b.reg(r#"-|minus\s?|negative\s?"#)?,
             number_check!(|number: &NumberValue| !number.prefixed()),
             |_, a| -> RuleResult<NumberValue> {
        Ok(match a.value().clone() { // checked
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
    });
    b.rule_2("numbers suffixes (K, M, G)",
             number_check!(|number: &NumberValue| !number.suffixed()),
             b.reg_neg_lh(r#"([kmg])"#, r#"^[\W\$€]"#)?,
             |a, text_match| -> RuleResult<NumberValue> {
        let multiplier = match text_match.group(0).as_ref() {
            "k" => 1000,
            "m" => 1000000,
            "g" => 1000000000,
            _ => panic!("Unknown match"),
        };
        Ok(match a.value().clone() { // checked
               NumberValue::Integer(integer) => {
                   IntegerValue {
                           value: integer.value * multiplier,
                           suffixed: true,
                           ..integer
                       }
                       .into()
               }
               NumberValue::Float(float) => {
            let product = float.value * (multiplier as f32);
            if product.floor() == product {
                IntegerValue {
                        value: product as i64,
                        suffixed: true,
                        ..IntegerValue::default()
                    }
                    .into()
            } else {
                FloatValue {
                        value: product,
                        suffixed: true,
                        ..float
                    }
                    .into()
            }
        }
           })
    });
    b.rule_1(
             "ordinals (first..31st)",
            b.reg(r#"(first|second|third|fourth|fifth|sixth|seventh|eighth|ninth|tenth|eleventh|twelfth|thirteenth|fourteenth|fifteenth|sixteenth|seventeenth|eighteenth|nineteenth|twentieth|twenty-first|twenty-second|twenty-third|twenty-fourth|twenty-fifth|twenty-sixth|twenty-seventh|twenty-eighth|twenty-ninth|thirtieth|thirty-first)"#)?,
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
             });
    b.rule_2("<number> <ordinal>",
             integer_check!(10, 90, |integer: &IntegerValue| integer.value % 10 == 0),
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 9),
             |integer, ordinal| {
                 Ok(OrdinalValue { value: integer.value().value + ordinal.value().value })
             });
    b.rule_1("ordinal (digits)",
             b.reg(r#"0*(\d+) ?(st|nd|rd|th)"#)?,
             |text_match| {
                 let value: i64 = text_match.group(1).parse()?;
                 Ok(OrdinalValue { value: value })
             });
    b.rule_2("the <ordinal>",
             b.reg(r#"the"#)?,
             ordinal_check!(),
             |_, ordinal| Ok(*ordinal.value()));
    Ok(b.build())
}

