use rustling::*;
use values::dimension::*;
use values::dimension::Precision::*;
use values::helpers;

pub fn rules_temperature(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1("number as temp",
             number_check!(),
             |a| {
                 Ok(TemperatureValue {
                     value: a.value().value(),
                     unit: None,
                     latent: true,
                 })
             });


    b.rule_2("<latent temp> degrees",
             temperature_check!(),
             b.reg(r#"度|°"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("degree"),
                     latent: false,
                 })
             });


    b.rule_2("<temp> Celcius",
             temperature_check!(),
             b.reg(r#"(摄|攝)氏(°|度)|(°)C"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("celsius"),
                     latent: false,
                 })
             });


    b.rule_3("Celcius <temp>",
             b.reg(r#"(摄|攝)氏"#)?,
             temperature_check!(),
             b.reg(r#"度|°"#)?,
             |_, b, _| {
                 Ok(TemperatureValue {
                     value: b.value().value,
                     unit: Some("celsius"),
                     latent: false,
                 })
             }
    );


    b.rule_2("<temp> Fahrenheit",
             temperature_check!(),
             b.reg(r#"(华|華)氏(°|度)|(°)F"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("fahrenheit"),
                     latent: false,
                 })
             });


    b.rule_3("Fahrenheit <temp>",
             b.reg(r#"(华|華)氏"#)?,
             temperature_check!(),
             b.reg(r#"度|°"#)?,
             |_, b, _| {
                 Ok(TemperatureValue {
                     value: b.value().value,
                     unit: Some("fahrenheit"),
                     latent: false,
                 })
             }
    );


    Ok(())
}


pub fn rules_numbers(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("integer (0..10)",
                      b.reg(r#"(〇|零|一|二|两|兩|三|四|五|六|七|八|九|十)(?:个|個)?"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "〇" => 0,
                              "零" => 0,
                              "一" => 1,
                              "二" => 2,
                              "两" => 2,
                              "兩" => 2,
                              "三" => 3,
                              "四" => 4,
                              "五" => 5,
                              "六" => 6,
                              "七" => 7,
                              "八" => 8,
                              "九" => 9,
                              "十" => 10,
                              _ => return Err(RuleErrorKind::Invalid.into())
                          };
                          IntegerValue::new_with_grain(value, 1)
                      });


    b.rule_1_terminal(
        "integer (numeric)",
        b.reg(r#"(\d{1,18})"#)?,
        |text_match| IntegerValue::new(text_match.group(0).parse()?));


    b.rule_1("decimal number", b.reg(r#"(\d*\.\d+)"#)?, |text_match| {
        let value: f32 = text_match.group(0).parse()?;
        Ok(FloatValue {
            value: value,
            ..FloatValue::default()
        })
    });


    b.rule_2("numbers prefix with -, negative or minus",
             b.reg(r#"-|负\s?|負\s?"#)?,
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


    b.rule_1_terminal("decimal with thousands separator",
                      b.reg(r#"(\d+(,\d\d\d)+\.\d+)"#)?,
                      |text_match| {
                          let reformatted_string = text_match.group(1).replace(",", "");
                          let value: f32 = reformatted_string.parse()?;
                          Ok(FloatValue {
                              value: value,
                              ..FloatValue::default()
                          })
                      });


    b.rule_2("<number>个",
             number_check!(),
             b.reg(r#"个"#)?,
             |number, _| Ok(number.value().clone()));


    b.rule_2("integer (20..90)",
             integer_check!(2, 9),
             b.reg(r#"十"#)?,
             |a, _| {
                 Ok(IntegerValue {
                     value: a.value().value * 10,
                     ..a.value().clone()
                 })
             });


    b.rule_2("numbers suffixes (K, M, G)",
             number_check!(|number: &NumberValue| !number.suffixed()),
             b.reg_neg_lh(r#"([kmg])"#, r#"^[^\W\$€元¥(?:人民币)]"#)?,
             |a, text_match| -> RuleResult<NumberValue> {
                 let multiplier = match text_match.group(0).as_ref() {
                     "k" => 1000,
                     "m" => 1000000,
                     "g" => 1000000000,
                     _ => return Err(RuleErrorKind::Invalid.into()),
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


    b.rule_2("integer 21..99",
             integer_check!(10, 90, |integer: &IntegerValue| integer.value % 10 == 0),
             integer_check!(1, 9),
             |a, b| IntegerValue::new(a.value().value + b.value().value));


    b.rule_2("integer (11..19)",
             b.reg(r#"十"#)?,
             integer_check!(1, 9),
             |_, b| IntegerValue::new(10 + b.value().value));


    b.rule_1("integer with thousands separator, ",
             b.reg(r#"(\d{1,3}(,\d\d\d){1,5})"#)?,
             |text_match| {
                 let reformatted_string = text_match.group(1).replace(",", "");
                 let value: i64 = reformatted_string.parse()?;
                 Ok(IntegerValue {
                     value: value,
                     ..IntegerValue::default()
                 })
             });


    Ok(())
}
