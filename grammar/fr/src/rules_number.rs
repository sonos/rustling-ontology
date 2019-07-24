use std::f32;
use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::helpers;

pub fn rules_numbers(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("intersect",
             number_check!(|number: &NumberValue| number.grain().unwrap_or(0) > 1),
             number_check!(),
             |a, b| helpers::compose_numbers(&a.value(), &b.value()));
    b.rule_1_terminal(
        "number (0..16)",
        b.reg(r#"(z[eé]ro|une?|deux|trois|quatre|cinq|six|sept|huit|neuf|dix|onze|douze|treize|quatorze|quinze|seize)"#)?,
        |text_match| {
            let value = match text_match.group(1).as_ref()  {
                "zéro" => 0,
                "zero" => 0,
                "un" => 1,
                "une" => 1,
                "deux" => 2,
                "trois" => 3,
                "quatre" => 4,
                "cinq" => 5,
                "six" => 6,
                "sept" => 7,
                "huit" => 8,
                "neuf" => 9,
                "dix" => 10,
                "onze" => 11,
                "douze" => 12,
                "treize" => 13,
                "quatorze" => 14,
                "quinze" => 15,
                "seize" => 16,
                _ => return Err(RuleError::Invalid.into()),
            };
            IntegerValue::new(value)
        });
    b.rule_1_terminal("quelques",
                      b.reg(r#"quelques"#)?,
                      |_| IntegerValue::new_with_grain(3, 1)
    );
    b.rule_1_terminal("number (20..60)",
                      b.reg(r#"(vingt|trente|quarante|cinquante|soixante)"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "vingt" => 20,
                              "trente" => 30,
                              "quarante" => 40,
                              "cinquante" => 50,
                              "soixante" => 60,
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          IntegerValue::new(value)
                      });
    b.rule_2("number (17..19)",
             integer_check_by_range!(10, 10),
             integer_check_by_range!(7, 9),
             |_, b| IntegerValue::new(b.value().value + 10));
    b.rule_3("number (17..19)",
             integer_check_by_range!(10, 10),
             b.reg(r"-")?,
             integer_check_by_range!(7, 9),
             |_, _, b| IntegerValue::new(b.value().value + 10));
    b.rule_2_terminal("number 80",
             b.reg(r#"quatre"#)?,
             b.reg(r#"vingts?"#)?,
             |_, _| IntegerValue::new(80));
    b.rule_3_terminal("number 80",
             b.reg(r#"quatre"#)?,
             b.reg(r"-")?,
             b.reg(r#"vingts?"#)?,
             |_, _, _| IntegerValue::new(80));
    b.rule_3("numbers 21 31 41 51",
             integer_check_by_range!(20, 50, |integer: &IntegerValue| integer.value % 10 == 0),
             b.reg(r#"-?et-?"#)?,
             integer_check_by_range!(1, 1),
             |a, _, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_2("numbers 22..29 32..39 .. 52..59",
             integer_check_by_range!(20, 50, |integer: &IntegerValue| integer.value % 10 == 0),
             integer_check_by_range!(2, 9),
             |a, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_3("numbers 22..29 32..39 .. 52..59",
             integer_check_by_range!(20, 50, |integer: &IntegerValue| integer.value % 10 == 0),
             b.reg(r"-")?,
             integer_check_by_range!(2, 9),
             |a, _, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_3("numbers 61 71",
             integer_check_by_range!(60, 60),
             b.reg(r#"-?et-?"#)?,
             integer_check_by_range!(1,
                            11,
                            |integer: &IntegerValue| integer.value == 1 || integer.value == 11),
             |a, _, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_2("numbers 81 91",
             integer_check_by_range!(80, 80),
             integer_check_by_range!(1,
                            11,
                            |integer: &IntegerValue| integer.value == 1 || integer.value == 11),
             |a, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_3("numbers 81 91",
             integer_check_by_range!(80, 80),
             b.reg(r#"-"#)?,
             integer_check_by_range!(1,
                            11,
                            |integer: &IntegerValue| integer.value == 1 || integer.value == 11),
             |a, _, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_2("numbers 62..69 .. 92..99",
             integer_check_by_range!(60,
                            80,
                            |integer: &IntegerValue| integer.value == 60 || integer.value == 80),
             integer_check_by_range!(2, 19),
             |a, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_3("numbers 62..69 .. 92..99",
             integer_check_by_range!(60,
                            80,
                            |integer: &IntegerValue| integer.value == 60 || integer.value == 80),
             b.reg(r"-")?,
             integer_check_by_range!(2, 19),
             |a, _, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_1_terminal("hundred",
        b.reg(r#"cents?"#)?,
        |_| IntegerValue::new_with_grain(100, 2)
    );
    b.rule_1_terminal("thousand",
        b.reg(r#"milles?"#)?,
        |_| IntegerValue::new_with_grain(1000, 3)
    );
    b.rule_1_terminal("million",
        b.reg(r#"millions?"#)?,
        |_| IntegerValue::new_with_grain(1000000, 6)
    );
    b.rule_1_terminal("billion",
        b.reg(r#"milliards?"#)?,
        |_| IntegerValue::new_with_grain(1000000000, 9)
    );
    b.rule_2("number hundreds",
        integer_check_by_range!(1, 99),
        b.reg(r#"cents?"#)?,
        |a, _| {
            Ok(IntegerValue {
                   value: a.value().value * 100,
                   grain: Some(2),
                   ..IntegerValue::default()
               })
        });
    b.rule_2("number thousands",
        integer_check_by_range!(1, 999),
        b.reg(r#"milles?"#)?,
        |a, _| {
            Ok(IntegerValue {
                   value: a.value().value * 1000,
                   grain: Some(3),
                   ..IntegerValue::default()
               })
    });
    b.rule_2("number millions",
        integer_check_by_range!(1, 999),
        b.reg(r#"millions?"#)?,
        |a, _| {
            Ok(IntegerValue {
                   value: a.value().value * 1000000,
                   grain: Some(6),
                   ..IntegerValue::default()
               })
    });
    b.rule_2("number billions",
        integer_check_by_range!(1, 999),
        b.reg(r#"milliards?"#)?,
        |a, _| {
            Ok(IntegerValue {
                   value: a.value().value * 1000000000,
                   grain: Some(9),
                   ..IntegerValue::default()
               })
    });
    b.rule_1_terminal("integer (numeric)",
        b.reg(r#"(\d{1,18})"#)?,
        |text_match| {
            let value: i64 = text_match.group(1).parse()?;
            IntegerValue::new(value)
    });
    b.rule_1_terminal("integer with thousands separator .",
             b.reg(r#"(\d{1,3}(\.\d\d\d){1,5})"#)?,
             |text_match| {
                 let reformatted_string = text_match.group(1).replace(".", "");
                 let value: i64 = reformatted_string.parse()?;
                 IntegerValue::new(value)
             });
    b.rule_1_terminal("decimal number",
        b.reg(r#"(\d*,\d+)"#)?,
        |text_match| {
            let reformatted_string = text_match.group(1).replace(",", ".");
            let value: f32 = reformatted_string.parse()?;
            FloatValue::new(value)
    });
    b.rule_3("number dot number",
        number_check!(|number: &NumberValue| !number.prefixed()),
        b.reg(r#"virgule|point"#)?,
        number_check!(|number: &NumberValue| !number.suffixed()),
        |a, _, b| {
            let power = b.value().value().to_string().chars().count();
            let coeff = 10.0_f32.powf(-1.0 * power as f32);
            Ok(FloatValue {
                value: b.value().value() * coeff + a.value().value(),
                ..FloatValue::default()
            })
    });
    b.rule_4("number dot zero ... number",
         number_check!(|number: &NumberValue| !number.prefixed()),
         b.reg(r#"virgule|point"#)?,
             b.reg(r#"(?:(?:z[eé]ro )*(?:z[eé]ro))"#)?,
         number_check!(|number: &NumberValue| !number.suffixed()),
         |a, _, zeros, b| {
             let power = zeros.group(0).split_whitespace().count() + b.value().value().to_string().chars().count();
             let coeff = 10.0_f32.powf(-1.0 * power as f32);
             Ok(FloatValue {
                 value: b.value().value() * coeff + a.value().value(),
                 ..FloatValue::default()
             })
    });
    b.rule_1_terminal("decimal with thousands separator",
             b.reg(r#"(\d+(\.\d\d\d)+,\d+)"#)?,
             |text_match| {
                 let reformatted_string = text_match.group(1).replace(".", "").replace(",", ".");
                 let value: f32 = reformatted_string.parse()?;
                 FloatValue::new(value)
             });
    b.rule_2("numbers prefix with -, negative or minus",
             b.reg(r#"-|moins"#)?,
             number_check!(|number: &NumberValue| !number.prefixed()),
             |_, a| -> RuleResult<NumberValue> {
                 Ok(match a.value().clone() {
                     // checked
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
    b.rule_2("numbers prefix with +, positive",
             b.reg(r#"\+"#)?,
             number_check!(|number: &NumberValue| !number.prefixed()),
             |_, a| -> RuleResult<NumberValue> {
                 Ok(match a.value().clone() {
                     // checked
                     NumberValue::Integer(integer) => {
                         IntegerValue {
                             prefixed: true,
                             ..integer
                         }
                             .into()
                     }
                     NumberValue::Float(float) => {
                         FloatValue {
                             prefixed: true,
                             ..float
                         }
                             .into()
                     }
                 })
             }
    );
    b.rule_2("numbers suffixes (K, M, G)",
             number_check!(|number: &NumberValue| !number.suffixed()),
             b.reg_neg_lh(r#"([kmg])"#, r#"^[\W\$€]"#)?,
             |a, text_match| -> RuleResult<NumberValue> {
                 let multiplier = match text_match.group(0).as_ref() {
                     "k" => 1000,
                     "m" => 1000000,
                     "g" => 1000000000,
                     _ => return Err(RuleError::Invalid.into()),
                 };
                 Ok(match a.value().clone() { // checked
                     NumberValue::Integer(integer) => {
                         IntegerValue {
                             prefixed: true,
                             ..integer
                         }
                             .into()
                     }
                     NumberValue::Float(float) => {
                         FloatValue {
                             prefixed: true,
                             ..float
                         }
                             .into()
                     }
                 })
             }
    );
    b.rule_2("numbers suffixes (K, M, G)",
             number_check!(|number: &NumberValue| !number.suffixed()),
             b.reg_neg_lh(r#"([kmg])"#, r#"^[\W\$€]"#)?,
             |a, text_match| -> RuleResult<NumberValue> {
        let multiplier = match text_match.group(0).as_ref() {
            "k" => 1000,
            "m" => 1000000,
            "g" => 1000000000,
            _ => return Err(RuleError::Invalid.into()),
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
    b.rule_1_terminal("(douzaine ... soixantaine)",
        b.reg(r#"(demi[ -]douz|diz|douz|quinz|vingt|trent|quarant|cinquant|soixant|cent)aines?"#)?,
        |text_match| {
            let value = match text_match.group(1).as_ref() {
                "demi douz" => 6,
                "demi-douz" => 6,
                "diz" => 10,
                "douz" => 12,
                "quinz" => 15,
                "vingt" => 20,
                "trent" => 30,
                "quarant" => 40,
                "cinquant" => 50,
                "soixant" => 60,
                "cent" => 100,
                _ => return Err(RuleError::Invalid.into()),
            };
            Ok(IntegerValue {
                value,
                group: true,
                .. IntegerValue::default()
            })
        }
    );
    b.rule_2("number dozen",
            integer_check_by_range!(1, 9),
            integer_check!(|integer: &IntegerValue| integer.group),
            |a, b| {
                 Ok(IntegerValue {
                     value: a.value().value * b.value().value,
                     grain: b.value().grain,
                     group: true,
                     ..IntegerValue::default()
                 })
    });
    b.rule_1_terminal("ordinal 0",
        b.reg(r#"z[eé]rot?i[eè]me"#)?,
        |_| {
            Ok(OrdinalValue::new(0))
        }
    );
    b.rule_1_terminal("ordinal 1",
        b.reg(r#"premi[eè]re?"#)?,
        |_| {
            Ok(OrdinalValue::new(1))
        }
    );
    b.rule_1_terminal("ordinal 2",
        b.reg(r#"seconde?|deuxi[eè]me"#)?,
        |_| {
            Ok(OrdinalValue::new(2))
        }
    );
    b.rule_1_terminal(
            "ordinals (premier..seizieme)",
            b.reg(r#"(trois|quatr|cinqu|six|sept|huit|neuv|dix|onz|douz|treiz|quatorz|quinz|seiz)i[eè]me"#)?,
            |text_match| {
                let value = match text_match.group(1).as_ref() {
                    "trois" => 3,
                    "quatr" => 4,
                    "cinqu" => 5,
                    "six" => 6,
                    "sept" => 7,
                    "huit" => 8,
                    "neuv" => 9,
                    "dix" => 10,
                    "onz" => 11,
                    "douz" => 12,
                    "treiz" => 13,
                    "quatorz" => 14,
                    "quinz" => 15,
                    "seiz" => 16,
                     _ => return Err(RuleError::Invalid.into()),
                 };
                 Ok(OrdinalValue::new(value))
            });
    b.rule_2("17ieme, 18ieme, 19ieme",
        b.reg(r#"dix-?"#)?,
        ordinal_check_by_range!(7, 9),
        |_, ordinal| {
            Ok(OrdinalValue::new(10 + ordinal.value().value))
        }
    );
    b.rule_1_terminal("20ieme, 30ieme, 40ieme, 50ieme, 60ieme",
        b.reg(r#"(vingt|trent|quarant|cinquant|soixant)i[èe]me"#)?,
        |text_match| {
            let value = match text_match.group(1).as_ref() {
                "vingt" => 20,
                "trent" => 30,
                "quarant" => 40,
                "cinquant" => 50,
                "soixant" => 60,
                _ => return Err(RuleError::Invalid.into()),
            };
            Ok(OrdinalValue::new(value))
        }
    );
    b.rule_1_terminal("80ieme",
        b.reg(r#"quatre[- ]vingts?i[èe]me"#)?,
        |_| {
            Ok(OrdinalValue::new(80))
        }
    );
    b.rule_2("22ieme...29ieme, 32ieme...39ieme, 42ieme...49ieme, 52ieme...59ieme",
        integer_check_by_range!(20, 50, |integer: &IntegerValue| integer.value % 10 == 0),
        ordinal_check_by_range!(2, 9),
        |integer, ordinal| {
            Ok(OrdinalValue::new(integer.value().value + ordinal.value().value))
        }
    );
    b.rule_3("22ieme...29ieme, 32ieme...39ieme, 42ieme...49ieme, 52ieme...59ieme",
        integer_check_by_range!(20, 50, |integer: &IntegerValue| integer.value % 10 == 0),
        b.reg(r"-")?,
        ordinal_check_by_range!(2, 9),
        |integer, _, ordinal| {
            Ok(OrdinalValue::new(integer.value().value + ordinal.value().value))
        }
    );
    b.rule_2("62ieme...70ieme, 72ieme...79ieme, 90ieme, 92ieme...99ieme",
        integer_check_by_range!(60, 80, |integer: &IntegerValue| integer.value == 60 || integer.value == 80),
        ordinal_check_by_range!(2, 19),
        |integer, ordinal| {
            Ok(OrdinalValue::new(integer.value().value + ordinal.value().value))
        }
    );
    b.rule_3("62ieme...70ieme, 72ieme...79ieme, 90ieme, 92ieme...99ieme",
        integer_check_by_range!(60, 80, |integer: &IntegerValue| integer.value == 60 || integer.value == 80),
        b.reg(r"-")?,
        ordinal_check_by_range!(2, 19),
        |integer, _, ordinal| {
            Ok(OrdinalValue::new(integer.value().value + ordinal.value().value))
        }
    );
    b.rule_2("21, 31, 41, 51, 61",
        integer_check_by_range!(20, 60, |integer: &IntegerValue| integer.value % 10 == 0),
        b.reg(r#"(?:et |-)uni[èe]me"#)?,
        |integer, _| {
            Ok(OrdinalValue::new(integer.value().value + 1))
        }
    );
    b.rule_2("81",
        integer_check_by_range!(80, 80),
        b.reg(r#"(?:et )?uni[èe]me"#)?,
        |integer, _| {
            Ok(OrdinalValue::new(integer.value().value + 1))
        }
    );
    b.rule_2("71, 91",
        integer_check_by_range!(60, 60),
        b.reg(r#"et onzi[eè]me"#)?,
        |integer, _| {
            Ok(OrdinalValue::new(integer.value().value + 11))
        }
    );
    b.rule_2("<number> et demi",
        integer_check_by_range!(0, 99),
        b.reg(r#"et demie?"#)?,
        |integer, _| {
            FloatValue::new(integer.value().value as f32 + 0.5)
        }
    );
    b.rule_1_terminal("70, 80, 90 (Belgium and Switzerland)",
        b.reg(r#"(sept|huit|non)ante"#)?,
        |text_match| {
            let value = match text_match.group(1).as_ref() {
                "sept" => 70,
                "huit" => 80,
                "non" => 90,
                _ => return Err(RuleError::Invalid.into()),
            };
            IntegerValue::new(value)
        }
    );
    b.rule_1_terminal("71, 81, 91 (Belgium and Switzerland)",
        b.reg(r#"(sept|huit|non)ante et une?"#)?,
        |text_match| {
            let value = match text_match.group(1).as_ref() {
                "sept" => 71,
                "huit" => 81,
                "non" => 91,
                _ => return Err(RuleError::Invalid.into()),
            };
            IntegerValue::new(value)
        }
    );

    b.rule_2("72..79, 82..89, 92..99, (Belgium and Switzerland)",
        b.reg(r#"(sept|huit|non)ante"#)?,
        integer_check_by_range!(2, 9),
        |text_match, integer| {
            let value = match text_match.group(1).as_ref() {
                "sept" => 70,
                "huit" => 80,
                "non" => 90,
                _ => return Err(RuleError::Invalid.into()),
            };
            IntegerValue::new(value + integer.value().value)
        }
    );
    b.rule_1_terminal("ordinal (100, 1_000, 1_000_000)",
        b.reg(r#"(cent|mill|million|milliard)i[èe]me"#)?,
        |text_match| {
            let (value, grain) = match text_match.group(1).as_ref() {
                "cent" => (100, 2),
                "mill" => (1_000, 3),
                "million" => (1_000_000, 6),
                "milliard" => (1_000_000_000, 9),
                _ => return Err(RuleError::Invalid.into()),
            };
            Ok(OrdinalValue::new_with_grain(value, grain))
        }
    );

    b.rule_2("ordinal (200..900, 2_000..9_000, 2_000_000..9_000_000_000)",
        integer_check_by_range!(2, 999),
        b.reg(r#"(cent|mill|million|milliard)i[èe]me"#)?,
        |integer, text_match| {
            let (value, grain) = match text_match.group(1).as_ref() {
                "cent" => (100, 2),
                "mill" => (1_000, 3),
                "million" => (1_000_000, 6),
                "milliard" => (1_000_000_000, 9),
                _ => return Err(RuleError::Invalid.into()),
            };
            Ok(OrdinalValue::new_with_grain(integer.value().value * value, grain))
        }
    );

    b.rule_2("ordinal (1_1_000..9_999_999_000)",
        integer_check_by_range!(1000, 99_999_999_000),
        ordinal_check!(|ordinal: &OrdinalValue| {
            let grain = ordinal.grain.unwrap_or(0);
            grain == 2 || grain % 3 == 0
        }),
        |integer, ordinal| {
            let grain = ordinal.value().grain.unwrap_or(0);
            let next_grain = (grain / 3) * 3 + 3;
            if integer.value().value % 10i64.pow(next_grain as u32) != 0 { return Err(RuleError::Invalid.into()); }
            Ok(OrdinalValue::new(integer.value().value + ordinal.value().value))
        }
    );

    b.rule_2("ordinal (102...9_999_999)",
        integer_check!(|integer: &IntegerValue| integer.value >= 100 || integer.value % 100 == 0),
        ordinal_check_by_range!(2, 99),
        |integer, ordinal| {
            Ok(OrdinalValue::new(integer.value().value + ordinal.value().value))
        }
    );
    b.rule_2("ordinal (101, 201, 301, ...)",
        integer_check!(|integer: &IntegerValue| integer.value >= 100 || integer.value % 100 == 0),
        b.reg(r#"(?:et |-)?uni[èe]me"#)?,
        |integer, _| {
            Ok(OrdinalValue::new(integer.value().value + 1))
        }
    );
    b.rule_1_terminal("ordinal (digits)",
             b.reg(r#"0*(\d+) ?(ere?|ère|ème|eme|ieme|ième)"#)?,
             |text_match| {
                 let value: i64 = text_match.group(1).parse()?;
                 Ok(OrdinalValue::new(value))
    });
    b.rule_2("le <ordinal>",
             b.reg(r#"l[ea]"#)?,
             ordinal_check!(),
             |_, a| Ok((*a.value()).prefixed())
    );
    Ok(())
}
