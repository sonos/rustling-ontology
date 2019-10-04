use std::f32;
use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::helpers;
use rustling_ontology_values::dimension::Precision::*;

pub fn rules_numbers(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("intersect",
             number_check!(|number: &NumberValue| number.grain().unwrap_or(0) > 1),
             number_check!(),
             |a, b| helpers::compose_numbers(&a.value(), &b.value())
    );
    b.rule_1_terminal("number (0..15)",
                      b.reg(r#"(und[eé]cimo|[cz]ero|un[oa]?|dos|tr[ée]s|cuatro|cinco|s[eé]is|siete|ocho|nueve|die(?:z|s)|once|doce|trece|catorce|quince)"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "cero" => 0,
                              "zero" => 0,
                              "un" => 1,
                              "uno" => 1,
                              "una" => 1,
                              "dos" => 2,
                              "tres" => 3,
                              "trés" => 3,
                              "cuatro" => 4,
                              "cinco" => 5,
                              "seis" => 6,
                              "séis" => 6,
                              "siete" => 7,
                              "ocho" => 8,
                              "nueve" => 9,
                              "diez" => 10,
                              "dies" => 10,
                              "once" => 11,
                              "doce" => 12,
                              "trece" => 13,
                              "catorce" => 14,
                              "quince" => 15,
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          IntegerValue::new(value)
                      }
    );
    b.rule_3("number (16..19)",
             integer_check_by_range!(0, 10),
             b.reg(r#"y"#)?,
             integer_check_by_range!(6, 9),
             |_, _, a| IntegerValue::new(a.value().value + 10));
    b.rule_1_terminal("number (20..90)",
                      b.reg(r#"(veinte|treinta|(?:cuar|cincu|ses|set|och|nov)enta)"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "veinte" => 20,
                              "treinta" => 30,
                              "cuarenta" => 40,
                              "cincuenta" => 50,
                              "sesenta" => 60,
                              "setenta" => 70,
                              "ochenta" => 80,
                              "noventa" => 90,
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          IntegerValue::new(value)
                      });
    b.rule_3("number (31..39 41..49 51..59 61..69 71..79 81..89 91..99)",
             integer_check_by_range!(30, 90, |integer: &IntegerValue| integer.value % 10 == 0),
             b.reg(r#"y"#)?,
             integer_check_by_range!(1, 9),
             |a, _, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_1_terminal("number (16..19 21..29)",
                      b.reg(r#"(die[cs]i(?:s[eéè]is|siete|ocho|nueve)|veinti(?:(?:un[oa]|[ùuú]n)|d[oó]s|tr[eéè]s|cuatro|cinco|s[eéè]is|siete|ocho|nueve))"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "dieciseis" => 16,
                              "diesiseis" => 16,
                              "diesiséis" => 16,
                              "dieciséis" => 16,
                              "diecisiete" => 17,
                              "dieciocho" => 18,
                              "diecinueve" => 19,
                              "veintiuno" => 21,
                              "veintiuna" => 21,
                              "veintiùn" => 21,
                              "veintiun" => 21,
                              "veintiún" => 21,
                              "veintidos" => 22,
                              "veintidós" => 22,
                              "veintitres" => 23,
                              "veintitrés" => 23,
                              "veinticuatro" => 24,
                              "veinticinco" => 25,
                              "veintiseis" => 26,
                              "veintiséis" => 26,
                              "veintisiete" => 27,
                              "veintiocho" => 28,
                              "veintinueve" => 29,
                              _ => return Err(RuleError::Invalid.into())
                          };
                          IntegerValue::new(value)
                      });
    b.rule_1_terminal("hundred",
                      b.reg(r#"cien(?:t[oa]s?)?"#)?,
                      |_| IntegerValue::new_with_grain(100, 2)
    );
    b.rule_1_terminal("number 200..900 except 500",
                      b.reg(r#"(dos|tres|cuatro|seis|sete|ocho|nove)cien(?:t[oa]s?)?"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "dos" => 200,
                              "tres" => 300,
                              "cuatro" => 400,
                              "quinientos" => 500,
                              "seis" => 600,
                              "sete" => 700,
                              "ocho" => 800,
                              "nove" => 900,
                              _ => return Err(RuleError::Invalid.into())
                          };
                          IntegerValue::new(value)
                      });
    b.rule_1_terminal("500",
                      b.reg(r#"quinient[oa]s"#)?,
                          |_| IntegerValue::new(500)
    );
    b.rule_1_terminal("thousand",
                      b.reg(r#"mil|un millar"#)?,
                      |_| IntegerValue::new_with_grain(1000, 3)
    );
    b.rule_1_terminal("million",
                      b.reg(r#"(?:un )?mill[oóò]n(?:es)?"#)?,
                      |_| IntegerValue::new_with_grain(1000000, 6)
    );
    // Warning: 'billón' is a trillion in Es (cf. English scale)
    b.rule_1_terminal("billion",
                      b.reg(r#"mil mill[oóò]n(?:es)?"#)?,
                      |_| IntegerValue::new_with_grain(1000000000, 9)
    );
    // Could catch hundreds written unglued
    b.rule_2("number hundreds",
             integer_check_by_range!(1, 99),
             b.reg(r#"cient[oa]s?"#)?,
             |a, _| {
                 Ok(IntegerValue {
                     value: a.value().value * 100,
                     grain: Some(2),
                     ..IntegerValue::default()
                 })
             });
    // FIXME: Don't understand why this couldn't be caught by intersection
    b.rule_2("hundreds number",
             integer_check_by_range!(100, 900),
             integer_check_by_range!(1, 99),
             |a, b| {
                 Ok(IntegerValue {
                     value: a.value().value + b.value().value,
                     grain: Some(2),
                     ..IntegerValue::default()
                 })
             });
    b.rule_2("number thousands",
             integer_check_by_range!(1, 999),
             b.reg(r#"mil"#)?,
             |a, _| {
                 Ok(IntegerValue {
                     value: a.value().value * 1000,
                     grain: Some(3),
                     ..IntegerValue::default()
                 })
             });
    // Warning! Es uses long scale, i.e. 1 billion = 1000 million
    b.rule_2("number millions",
             integer_check_by_range!(1, 999999),
             b.reg(r#"mill[oóò]n(?:es)?"#)?,
             |a, _| {
                 Ok(IntegerValue {
                     value: a.value().value * 1000000,
                     grain: Some(6),
                     ..IntegerValue::default()
                 })
             });
    b.rule_2("number billions",
             integer_check_by_range!(1, 999),
             b.reg(r#"mil mill[oóò]n(?:es)?"#)?,
             |a, _| {
                 Ok(IntegerValue {
                     value: a.value().value * 1000000000,
                     grain: Some(9),
                     ..IntegerValue::default()
                 })
             });
    b.rule_1("few", b.reg(r#"un[oa]s"#)?, |_| {
        Ok(IntegerValue {
            value: 3,
            grain: Some(1),
            precision: Approximate,
            ..IntegerValue::default()
        })
    });
    b.rule_1_terminal("several",
                      b.reg(r#"vario[ao]s"#)?,
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
             b.reg(r#"punto|coma"#)?,
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
             b.reg(r#"punto|coma"#)?,
             b.reg(r#"(?:(?:[zc]ero )*(?:[zc]ero))"#)?,
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
                 Ok(match a.value().clone() {
                     // checked
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
    // TODO: Add approximate numbers/quantities
    b.rule_1_terminal("ordinals 1 and 3",
    b.reg(r#"(prim|terc)er"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "prim" => 1,
                              "terc" => 3,
                              _ => return Err(RuleError::Invalid.into())
                          };
                          Ok(OrdinalValue::new(value))
                      }
    );
    b.rule_1_terminal("ordinals (primero..10)",
                      b.reg(r#"(primer|segund|tercer|cuart|quint|sext|s[eéè]ptim|octav|noven|d[eéè]cim)(?:[oa]s?)?"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "primer" => 1,
                              "segund" => 2,
                              "tercer" => 3,
                              "cuart" => 4,
                              "quint" => 5,
                              "sext" => 6,
                              "séptim" => 7,
                              "sèptim" => 7,
                              "septim" => 7,
                              "octav" => 8,
                              "noven" => 9,
                              "décim" => 10,
                              "dècim" => 10,
                              "decim" => 10,
                              _ => return Err(RuleError::Invalid.into())
                          };
                          Ok(OrdinalValue::new(value))
                      }
    );
    b.rule_1_terminal("ordinals 11 and 12",
                      b.reg(r#"(un|duo)d[eé]cim[oa]s?"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "un" => 11,
                              "duo" => 12,
                              _ => return Err(RuleError::Invalid.into())
                          };
                          Ok(OrdinalValue::new(value))
                      }
    );
    b.rule_1_terminal("ordinals 20 and 30",
                      b.reg(r#"(vi|tri)g[eé]simo"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "vi" => 20,
                              "tri" => 30,
                              _ => return Err(RuleError::Invalid.into())
                          };
                          Ok(OrdinalValue::new(value))
                      }
    );
    b.rule_1_terminal("ordinals 11-19",
                      b.reg(r#"d[eé]cimo?s? ?(primer|segund|tercer|cuart|quint|sext|s[eéè]ptim|octav|noven)(?:[oa]s?)?"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "primer" => 11,
                              "segund" => 12,
                              "tercer" => 13,
                              "cuart" => 14,
                              "quint" => 15,
                              "sext" => 16,
                              "séptim" => 17,
                              "sèptim" => 17,
                              "septim" => 17,
                              "octav" => 18,
                              "noven" => 19,
                              _ => return Err(RuleError::Invalid.into())
                          };
                          Ok(OrdinalValue::new(value))
                      }
    );
    b.rule_1_terminal("ordinal (digits)",
                      b.reg(r#"0*(\d+)(?:[ºªoa]|\.er)"#)?,
                      |text_match| {
                          let value: i64 = text_match.group(1).parse()?;
                          Ok(OrdinalValue::new(value))
                      }
    );
    Ok(())
}
