use std::f64;
use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::helpers;

pub fn rules_numbers(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("intersect",
             number_check!(|number: &NumberValue| number.grain().unwrap_or(0) > 1),
             number_check!(),
             |a, b| helpers::compose_numbers(&a.value(), &b.value()));
    b.rule_3("intersect with and",
             number_check!(|number: &NumberValue| number.grain().unwrap_or(0) > 1),
             b.reg(r#"e"#)?,
             number_check!(),
             |a, _, b| helpers::compose_numbers(&a.value(), &b.value()));
    // Keep the order of patterns as is, otherwise 'undici' is caught with 'un'
    b.rule_1_terminal("number (0..19)",
                      b.reg(r#"(dici(?:assette|otto|annove)|(?:un|do|tre|quattor|quin|se)dici|zero|un[oa']?|due|tr[eé]|quattro|cinque|sei|sette|otto|nove|dieci)"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "zero" => 0,
                              "un" => 1,
                              "un'" => 1,
                              "uno" => 1,
                              "una" => 1,
                              "due" => 2,
                              "tre" => 3,
                              "tré" => 3,
                              "quattro" => 4,
                              "cinque" => 5,
                              "sei" => 6,
                              "sette" => 7,
                              "otto" => 8,
                              "nove" => 9,
                              "dieci" => 10,
                              "undici" => 11,
                              "dodici" => 12,
                              "tredici" => 13,
                              "quattordici" => 14,
                              "quindici" => 15,
                              "sedici" => 16,
                              "diciassette" => 17,
                              "diciotto" => 18,
                              "diciannove" => 19,
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          IntegerValue::new(value)
                      }
    );
    b.rule_1_terminal("number (20..90)",
                      b.reg(r#"(venti|trenta|(?:(?:quar|cinqu|sess|sett|ott|nov)anta))"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "venti" => 20,
                              "trenta" => 30,
                              "quaranta" => 40,
                              "cinquanta" => 50,
                              "sessanta" => 60,
                              "settanta" => 70,
                              "ottanta" => 80,
                              "novanta" => 90,
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          IntegerValue::new(value)
                      });
    b.rule_2("number (21..29 31..39 41..49 51..59 61..69 71..79 81..89 91..99)",
             b.reg(r#"(venti?|trenta?|(?:(?:quar|cinqu|sess|sett|ott|nov)anta?))"#)?,
             integer_check_by_range!(1, 9),
             |text_match, b| {
                 let value = match text_match.group(1).as_ref() {
                     "venti" => 20,
                     "trenta" => 30,
                     "quaranta" => 40,
                     "cinquanta" => 50,
                     "sessanta" => 60,
                     "settanta" => 70,
                     "ottanta" => 80,
                     "novanta" => 90,
                     "vent" => 20,
                     "trent" => 30,
                     "quarant" => 40,
                     "cinquant" => 50,
                     "sessant" => 60,
                     "settant" => 70,
                     "ottant" => 80,
                     "novant" => 90,
                     _ => return Err(RuleError::Invalid.into())
                 };
                 IntegerValue::new(value + b.value().value)
             });
    b.rule_1_terminal("number 100..1000",
                      b.reg(r#"(cento?|duecento|trecento|quattrocento|cinquecento|seicento|settecento|ottocento|novecento|mil(?:le|a))"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "cent" => 100,
                              "cento" => 100,
                              "duecento" => 200,
                              "trecento" => 300,
                              "quattrocento" => 400,
                              "cinquecento" => 500,
                              "seicento" => 600,
                              "settecento" => 700,
                              "ottocento" => 800,
                              "novecento" => 900,
                              "mille" => 1000,
                              "mila" => 1000,
                              _ => return Err(RuleError::Invalid.into())
                          };
                          IntegerValue::new_with_grain(value, 2)
                      });
    b.rule_2("numbers 100..199",
             integer_check_by_range!(100, 100),
             integer_check_by_range!(0, 99),
             |_, b| IntegerValue::new(b.value().value + 100));
    b.rule_3("numbers 200..999",
             integer_check_by_range!(2, 9),
             integer_check_by_range!(100, 100),
             integer_check_by_range!(0, 99),
             |a, b, c| IntegerValue::new(a.value().value * b.value().value + c.value().value));
    b.rule_1_terminal("hundred",
                      b.reg(r#"cento"#)?,
                      |_| IntegerValue::new_with_grain(100, 2)
    );
    b.rule_2("N hundreds",
             integer_check_by_range!(1, 99),
             b.reg(r#"cento"#)?,
             |a, _| {
                 Ok(IntegerValue {
                     value: a.value().value * 100,
                     grain: Some(2),
                     ..IntegerValue::default()
                 })
             });
    b.rule_1_terminal("thousand",
                      b.reg(r#"mil(?:le|a)"#)?,
                      |_| IntegerValue::new_with_grain(1000, 3)
    );
    b.rule_2("N thousands",
             integer_check_by_range!(1, 999),
             b.reg(r#"mil(?:le|a)"#)?,
             |a, _| {
                 Ok(IntegerValue {
                     value: a.value().value * 1000,
                     grain: Some(3),
                     ..IntegerValue::default()
                 })
             });
    b.rule_1_terminal("million",
                      b.reg(r#"milione?"#)?,
                      |_| IntegerValue::new_with_grain(1000000, 6)
    );
    b.rule_2("N millions",
             integer_check_by_range!(1, 999),
             b.reg(r#"milion[ei]?(?: e)?"#)?,
             |a, _| {
                 Ok(IntegerValue {
                     value: a.value().value * 1000000,
                     grain: Some(6),
                     ..IntegerValue::default()
                 })
             });
    b.rule_1_terminal("billion",
                      b.reg(r#"miliardo"#)?,
                      |_| IntegerValue::new_with_grain(1000000000, 9)
    );
    b.rule_2("N billions",
             integer_check_by_range!(1, 999),
             b.reg(r#"miliard[oi]"#)?,
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
                          let value: f64 = reformatted_string.parse()?;
                          FloatValue::new(value)
                      });
    b.rule_3("number dot number",
             number_check!(|number: &NumberValue| !number.prefixed()),
             b.reg(r#"punto|virgola"#)?,
             number_check!(|number: &NumberValue| !number.suffixed()),
             |a, _, b| {
                 let power = b.value().value().to_string().chars().count();
                 let coeff = 10.0_f64.powf(-1.0 * power as f64);
                 Ok(FloatValue {
                     value: b.value().value() * coeff + a.value().value(),
                     ..FloatValue::default()
                 })
             });

    b.rule_4("number dot zero ... number",
             number_check!(|number: &NumberValue| !number.prefixed()),
             b.reg(r#"punto|virgola"#)?,
             b.reg(r#"(?:(?:zero )*(?:zero))"#)?,
             number_check!(|number: &NumberValue| !number.suffixed()),
             |a, _, zeros, b| {
                 let power = zeros.group(0).split_whitespace().count() + b.value().value().to_string().chars().count();
                 let coeff = 10.0_f64.powf(-1.0 * power as f64);
                 Ok(FloatValue {
                     value: b.value().value() * coeff + a.value().value(),
                     ..FloatValue::default()
                 })
             });

    b.rule_1_terminal("decimal with thousands separator",
                      b.reg(r#"(\d+(\.\d\d\d)+,\d+)"#)?,
                      |text_match| {
                          let reformatted_string = text_match.group(1).replace(".", "").replace(",", ".");
                          let value: f64 = reformatted_string.parse()?;
                          FloatValue::new(value)
                      });
    b.rule_2("numbers prefix with -, negative or minus",
             b.reg(r#"-|meno"#)?,
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
                             value: integer.value * multiplier,
                             suffixed: true,
                             ..integer
                         }
                             .into()
                     }
                     NumberValue::Float(float) => {
                         let product = float.value * (multiplier as f64);
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


    // Ordinals
    b.rule_1_terminal("ordinals (1-2-3 abbrev)",
                      b.reg(r#"(?:il |la )?(1|2|3)[oa°]"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "1" => 1,
                              "2" => 2,
                              "3" => 3,
                              _ => return Err(RuleError::Invalid.into())
                          };
                          Ok(OrdinalValue::new(value))
                      });
    b.rule_1_terminal("ordinals (primo..10)",
                      b.reg(r#"((?:il |la )?1[oa°]|zeresim|prim|second|terz|quart|quint|sest|settim|ottav|non|decim)[oiae]"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "zeresim" => 0,
                              "prim" => 1,
                              "second" => 2,
                              "terz" => 3,
                              "quart" => 4,
                              "quint" => 5,
                              "sest" => 6,
                              "settim" => 7,
                              "ottav" => 8,
                              "non" => 9,
                              "decim" => 10,
                              _ => return Err(RuleError::Invalid.into())
                          };
                          Ok(OrdinalValue::new(value))
                      });
    Ok(())
}
