use std::f32;
use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::helpers;

pub fn rules_numbers(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {

    b.rule_2("intersect numbers",
             number_check!(|number: &NumberValue| number.grain().unwrap_or(0) > 1),
             number_check!(),
             |a, b| helpers::compose_numbers(&a.value(), &b.value())
    );


    b.rule_3("intersect numbers",
             number_check!(|number: &NumberValue| number.grain().unwrap_or(0) > 1),
             b.reg(r#"e"#)?,
             number_check!(),
             |a, _,b| helpers::compose_numbers(&a.value(), &b.value())
    );

    b.rule_1_terminal("numbers (0..9)",
                      b.reg(r#"(zero|uma?|dois|duas|tr[eéê]s|quatro|cinco|s[eé]is|meia|sete|oito|nove)"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "zero" => 0,
                              "um" => 1,
                              "uma" => 1,
                              "dois" => 2,
                              "duas" => 2,
                              "tres" => 3,
                              "trés" => 3,
                              "três" => 3,
                              "quatro" => 4,
                              "cinco" => 5,
                              "seis" => 6,
                              "séis" => 6,
                              "meia" => 6,
                              "sete" => 7,
                              "oito" => 8,
                              "nove" => 9,
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          IntegerValue::new(value)
                      }
    );

    b.rule_1_terminal("numbers (10..19)",
                      b.reg(r#"(dezesseis|dezasseis|dezessete|dezoito|dezenove|dezanove|dez|onze|doze|treze|quatorze|catorze|quinze)"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "dez" => 10,
                              "onze" => 11,
                              "doze" => 12,
                              "treze" => 13,
                              "quatorze" => 14,
                              "catorze" => 14,
                              "quinze" => 15,
                              "dezesseis" => 16,
                              "dezasseis" => 16,
                              "dezessete" => 17,
                              "dezoito" => 18,
                              "dezenove" => 19,
                              "dezanove" => 19,
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          IntegerValue::new(value)
                      }
    );

    b.rule_1_terminal("numbers (20..90)",
                 b.reg(r#"(vinte|trinta|quarenta|cinquenta|sessenta|setenta|oitenta|noventa)"#)?,
                 |text_match| {
                     let value = match text_match.group(1).as_ref() {
                         "vinte" => 20,
                         "trinta" => 30,
                         "quarenta" => 40,
                         "cinquenta" => 50,
                         "sessenta" => 60,
                         "setenta" => 70,
                         "oitenta" => 80,
                         "noventa" => 90,
                         _ => return Err(RuleError::Invalid.into()),
                     };
                     IntegerValue::new(value)
                 }
        );

    b.rule_3("numbers (21...99)",
                 integer_check_by_range!(20, 90, |integer: &IntegerValue| integer.value % 10 == 0),
                 b.reg(r#"e"#)?,
                 integer_check_by_range!(1, 9),
                 |x, _, y| IntegerValue::new(x.value().value + y.value().value)
    );

    b.rule_1_terminal("cem",
                      b.reg(r#"cem"#)?,
                      |_| IntegerValue::new_with_grain(100,2)
    );

    b.rule_3("numbers (101...199)",
                 b.reg(r#"cento"#)?,
                 b.reg(r#"e"#)?,
                 integer_check_by_range!(1, 99),
                 |_, _, y| IntegerValue::new_with_grain(100 + y.value().value, 2)
    );

    b.rule_1_terminal("numbers (200..900)",
             b.reg(r#"(duzent|trezent|quatrocent|quinhent|seiscent|setecent|oitocent|novecent)(?:[oa]s)"#)?,
             |text_match| {
                 let value = match text_match.group(1).as_ref() {
                     "duzent" => 200,
                     "trezent" => 300,
                     "quatrocent" => 400,
                     "quinhent" => 500,
                     "seiscent" => 600,
                     "setecent" => 700,
                     "oitocent" => 800,
                     "novecent" => 900,
                     _ => return Err(RuleError::Invalid.into()),
                 };
                 IntegerValue::new_with_grain(value,2)
             }
    );

    b.rule_1_terminal("thousand",
        b.reg(r#"mil"#)?,
        |_| IntegerValue::new_with_grain(1000, 3)
    );

    b.rule_2("thousands",
        integer_check_by_range!(1, 999),
        b.reg(r#"mil"#)?,
        |a, _| {
            Ok(IntegerValue {
                   value: a.value().value * 1000,
                   grain: Some(3),
                   ..IntegerValue::default()
               })
    });

    b.rule_2("one million",
        integer_check! (|integer: &IntegerValue| integer.value == 1),
        b.reg(r#"milhão"#)?,
        |_,_| IntegerValue::new_with_grain(1000000, 6)
    );

    b.rule_2("millions",
        integer_check_by_range!(2, 999),
        b.reg(r#"milhões"#)?,
        |a, _| {
            Ok(IntegerValue {
                   value: a.value().value * 1000000,
                   grain: Some(6),
                   ..IntegerValue::default()
               })
    });

    b.rule_2("one billion",
        integer_check! (|integer: &IntegerValue| integer.value == 1),
        b.reg(r#"bilhão"#)?,
        |_,_| IntegerValue::new_with_grain(1000000000, 9)
    );

    b.rule_2("billions",
        integer_check_by_range!(2, 999),
        b.reg(r#"bilhões"#)?,
        |a, _| {
            Ok(IntegerValue {
                   value: a.value().value * 1000000000,
                   grain: Some(9),
                   ..IntegerValue::default()
               })
    });

//    b.rule_3("numbers (1,000,000...999,999,999)",
//                 integer_check_by_range!(1000000, 999000000),
//                 b.reg(r#"e?"#)?,
//                 integer_check_by_range!(1, 999999),
//                 |a, _, c| IntegerValue::new_with_grain(a.value().value + c.value().value,3)
//    );
//    b.rule_3("numbers (1,000...999,999)",
//                 integer_check_by_range!(1000, 999000),
//                 b.reg(r#"e?"#)?,
//                 integer_check_by_range!(1, 999),
//                 |a, _, c| IntegerValue::new_with_grain(a.value().value + c.value().value,3)
//    );
//    b.rule_3("numbers (200...999)",
//                integer_check_by_range!(200, 900, |integer: &IntegerValue| integer.value % 100 == 0),
//                 b.reg(r#"e"#)?,
//                 integer_check_by_range!(1, 99),
//                 |x, _, y| IntegerValue::new_with_grain(x.value().value + y.value().value,2)
//    );

    b.rule_1_terminal("some",
                      b.reg(r#"algumas|alguns"#)?,
                      |_| IntegerValue::new_with_grain(3, 1)
    );
    b.rule_1_terminal("several",
                      b.reg(r#"v[àa]rios"#)?,
                      |_| IntegerValue::new_with_grain(4, 1)
    );
    b.rule_1_terminal("integer (numeric)",
                      b.reg(r#"(\d{1,18})"#)?,
                      |text_match| IntegerValue::new(text_match.group(0).parse()?));
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
             b.reg(r#"v[íi]rgula"#)?,
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
             b.reg(r#"v[íi]rgula"#)?,
             b.reg(r#"(?:(?:zero )*(?:zero))"#)?,
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
             b.reg(r#"-|menos"#)?,
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
    b.rule_1_terminal("ordinals (primeiro..9)",
                      b.reg(r#"(primeir|segund|terceir|quart|quint|sext|s[eéè]tim|oitav|non)(?:[oa]s?)?"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "primeir" => 1,
                              "segund" => 2,
                              "terceir" => 3,
                              "quart" => 4,
                              "quint" => 5,
                              "sext" => 6,
                              "sétim" => 7,
                              "sètim" => 7,
                              "setim" => 7,
                              "oitav" => 8,
                              "non" => 9,
                              _ => return Err(RuleError::Invalid.into())
                          };
                          Ok(OrdinalValue::new(value))
                      }
    );
    b.rule_1_terminal("ordinals (10..90)",
                          b.reg(r#"(d[eéè]cim|vig[eéè]sim|trig[eéè]sim|quadrag[eéè]sim|quinquag[eéè]sim|sexag[eéè]sim|septuag[eéè]sim|setuag[eéè]sim|octog[eéè]sim|nonag[eéè]sim)(?:[oa]s?)?"#)?,
                          |text_match| {
                              let value = match text_match.group(1).as_ref() {
                                  "décim" => 10,
                                  "dècim" => 10,
                                  "decim" => 10,
                                  "vigésim" => 20,
                                  "vigèsim" => 20,
                                  "vigesim" => 20,
                                  "trigésim" => 30,
                                  "trigèsim" => 30,
                                  "trigesim" => 30,
                                  "quadragésim" => 40,
                                  "quadragèsim" => 40,
                                  "quadragesim" => 40,
                                  "quinquagésim" => 50,
                                  "quinquagesim" => 50,
                                  "quinquagèsim" => 50,
                                  "sexagésim" => 60,
                                  "sexagèsim" => 60,
                                  "sexagesim" => 60,
                                  "septuagésim" => 70,
                                  "septuagèsim" => 70,
                                  "septuagesim" => 70,
                                  "setuagèsim" => 70,
                                  "setuagesim" => 70,
                                  "setuagésim" => 70,
                                  "octogésim" => 80,
                                  "octogèsim" => 80,
                                  "octogesim" => 80,
                                  "nonagésim" => 90,
                                  "nonagèsim" => 90,
                                  "nonagesim" => 90,
                                  _ => return Err(RuleError::Invalid.into())
                              };
                              Ok(OrdinalValue::new(value))
                          }
    );

    b.rule_2("ordinals (11..99)",
        ordinal_check_by_range!(10, 90),
        ordinal_check_by_range!(1, 9),
        |a, b| {
            Ok(OrdinalValue::new(a.value().value + b.value().value))
        }
    );

    b.rule_1_terminal("ordinals (100..900)",
                              b.reg(r#"(cent[eéè]sim|ducent[eéè]sim|trecent[eéè]sim|tricent[eéè]sim|quadrin?gent[eéè]sim|quingent[eéè]sim|sexcent[eéè]sim|seiscent[eéè]sim|setingent[eéè]sim|septigent[eéè]sim|septingent[eéè]sim|octingent[eéè]sim|octigent[eéè]sim|nongent[eéè]sim|noningent[eéè]sim)(?:[oa]s?)?"#)?,
                              |text_match| {
                                  let value = match text_match.group(1).as_ref() {
                                      "centésim" => 100,
                                      "centèsim" => 100,
                                      "centesim" => 100,
                                      "ducentésim" => 200,
                                      "ducentèsim" => 200,
                                      "ducentesim" => 200,
                                      "trecentésim" => 300,
                                      "trecentèsim" => 300,
                                      "trecentesim" => 300,
                                      "tricentésim" => 300,
                                      "tricentèsim" => 300,
                                      "tricentesim" => 300,
                                      "quadrigentésim" => 400,
                                      "quadrigentèsim" => 400,
                                      "quadrigentesim" => 400,
                                      "quadringentésim" => 400,
                                      "quadringentèsim" => 400,
                                      "quadringentesim" => 400,
                                      "quingentésim" => 500,
                                      "quingentesim" => 500,
                                      "quingentèsim" => 500,
                                      "sexcentésim" => 600,
                                      "sexcentèsim" => 600,
                                      "sexcentesim" => 600,
                                      "seiscentésim" => 600,
                                      "seiscentèsim" => 600,
                                      "seiscentesim" => 600,
                                      "setingentésim" => 700,
                                      "setingentèsim" => 700,
                                      "setingentesim" => 700,
                                      "septingentèsim" => 700,
                                      "septingentesim" => 700,
                                      "septingentésim" => 700,
                                      "septigentésim" => 700,
                                      "septigentèsim" => 700,
                                      "septigentesim" => 700,
                                      "octingentésim" => 800,
                                      "octingentèsim" => 800,
                                      "octingentesim" => 800,
                                      "octigentèsim" => 800,
                                      "octigentésim" => 800,
                                      "octigentesim" => 800,
                                      "nongentésim" => 900,
                                      "nongentèsim" => 900,
                                      "nongentesim" => 900,
                                      "noningentésim" => 900,
                                      "noningentèsim" => 900,
                                      "noningentesim" => 900,
                                      _ => return Err(RuleError::Invalid.into())
                                  };
                                  Ok(OrdinalValue::new(value))
                              }
    );

    b.rule_2("ordinals (101..999)",
        ordinal_check_by_range!(100, 900),
        ordinal_check_by_range!(1, 99),
        |a, b| {
            Ok(OrdinalValue::new(a.value().value + b.value().value))
        }
    );

    b.rule_1_terminal("ordinal thousand",
                      b.reg(r#"(mil[eéè]sim)(?:[oa]s?)?"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "milésim" => 1000,
                              "milèsim" => 1000,
                              "milesim" => 1000,
                              _ => return Err(RuleError::Invalid.into())
                          };
                          Ok(OrdinalValue::new_with_grain(value,3))
                      }
    );

    b.rule_2("ordinal thousands",
        integer_check_by_range!(1, 999),
        ordinal_check! (|ordinal: &OrdinalValue| ordinal.value == 1000),
        |a, b| {
            Ok(OrdinalValue::new(a.value().value * b.value().value))
        }
    );

    b.rule_2("ordinal thousands + number",
        ordinal_check_by_range!(1000,999000),
        ordinal_check_by_range!(1, 999),
        |a, b| {
            Ok(OrdinalValue::new(a.value().value + b.value().value))
        }
    );

    b.rule_1_terminal("ordinal (digits)",
                      b.reg(r#"0*(\d+)[ºªoa]"#)?,
                      |text_match| {
                          let value: i64 = text_match.group(1).parse()?;
                          Ok(OrdinalValue::new(value))
                      }
    );
    Ok(())
}
