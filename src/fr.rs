use rustling::*;
use dimension::*;
use examples;

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
             b.reg(r#"(deg(r[éeè])?s?\.?)|°"#)?,
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
             b.reg(r#"f(ah?reh?n(h?eit)?)?\.?"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                        value: a.value().value,
                        unit: Some("fahrenheit"),
                        latent: false,
                    })
             });
    b.rule_2("<latent temp> en dessous de zero",
             temperature_check!(),
             b.reg(r#"en dessous de (0|z[ée]ro)"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                        value: -1.0 * a.value().value,
                        latent: false,
                        ..*a.value()
                    })
             });
    Ok(b.build())
}

pub fn rules_numbers() -> RustlingResult<RuleSet<Dimension>> {
    let b = RuleSetBuilder::default();
    b.rule_1(
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
                        _ => panic!("Unknow match"),
                    };
                    IntegerValue::new(value) 
            });
    b.rule_1("number (20..60)",
             b.reg(r#"(vingt|trente|quarante|cinquante|soixante)"#)?,
             |text_match| {
        let value = match text_match.group(1).as_ref() {
            "vingt" => 20,
            "trente" => 30,
            "quarante" => 40,
            "cinquante" => 50,
            "soixante" => 60,
            _ => panic!("Unknow match"),
        };
        IntegerValue::new(value)
    });
    b.rule_2("number (17..19)",
             integer_check!(10, 10),
             integer_check!(7, 9),
             |_, b| IntegerValue::new(b.value().value + 10));
    b.rule_2("number 80", //
             b.reg(r#"quatre"#)?,
             b.reg(r#"vingts?"#)?,
             |_, _| IntegerValue::new(80));
    b.rule_3("numbers 21 31 41 51",
             integer_check!(20, 50, |integer: &IntegerValue| integer.value % 10 == 0),
             b.reg(r#"et"#)?,
             integer_check!(1, 1),
             |a, _, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_2("numbers 22..29 32..39 .. 52..59",
             integer_check!(20, 50, |integer: &IntegerValue| integer.value % 10 == 0),
             integer_check!(2, 9),
             |a, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_3("numbers 61 71",
             integer_check!(60, 60),
             b.reg(r#"-?et-?"#)?,
             integer_check!(1,
                            11,
                            |integer: &IntegerValue| integer.value == 1 || integer.value == 11),
             |a, _, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_2("numbers 81 91",
             integer_check!(80, 80),
             integer_check!(1,
                            11,
                            |integer: &IntegerValue| integer.value == 1 || integer.value == 11),
             |a, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_2("numbers 62..69 .. 92..99",
             integer_check!(60,
                            80,
                            |integer: &IntegerValue| integer.value == 60 || integer.value == 80),
             integer_check!(2, 19),
             |a, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_1("integer (numeric)", b.reg(r#"(\d{1,18})"#)?, |text_match| {
        let value: i64 = text_match.group(1).parse()?;
        IntegerValue::new(value)
    });
    b.rule_1("integer with thousands separator .",
             b.reg(r#"(\d{1,3}(\.\d\d\d){1,5})"#)?,
             |text_match| {
                 let reformatted_string = text_match.group(1).replace(".", "");
                 let value: i64 = reformatted_string.parse()?;
                 IntegerValue::new(value)
             });
    b.rule_1("decimal number", b.reg(r#"(\d*,\d+)"#)?, |text_match| {
        let reformatted_string = text_match.group(1).replace(",", ".");
        let value: f32 = reformatted_string.parse()?;
        FloatValue::new(value)
    });
    b.rule_1("decimal with thousands separator",
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
            "ordinals (premier..seizieme)",
            b.reg(r#"(premi(?:ere?|ère)|(?:deux|trois|quatr|cinqu|six|sept|huit|neuv|dix|onz|douz|treiz|quatorz|quinz|seiz)i[eè]me)"#)?,
            |text_match| {
                let value = match text_match.group(1).as_ref() {
                    "premier" => 1,
                    "premiere" => 1,
                    "première" => 1, 
                    "deuxieme" => 2,
                    "troisieme" => 3,
                    "quatrieme" => 4, 
                    "cinquieme" => 5, 
                    "sixieme" => 6,
                    "septieme" => 7, 
                    "huitieme" => 8, 
                    "neuvieme" => 9,
                    "dixieme" => 10,
                    "onzieme" => 11,
                    "douzieme" => 12,
                    "treizieme" => 13,
                    "quatorzieme" => 14,
                    "quinzieme" => 15,
                    "seizieme" => 16,
                    "deuxième" => 2,
                    "troisième" => 3,
                    "quatrième" => 4, 
                    "cinquième" => 5, 
                    "sixième" => 6, 
                    "septième" => 7,
                    "huitième" => 8, 
                    "neuvième" => 9, 
                    "dixième" => 10, 
                    "onzième" => 11, 
                    "douzième" => 12, 
                    "treizième" => 13,
                    "quatorzième" => 14,
                    "quinzième" => 15,
                    "seizième" => 16,
                     _ => panic!("Unknow match"),
                 };
                 Ok(OrdinalValue { value: value })
            });
    b.rule_1("ordinal (digits)",
             b.reg(r#"0*(\d+) ?(ere?|ère|ème|eme|ieme|ième)"#)?,
             |text_match| {
                 let value: i64 = text_match.group(1).parse()?;
                 Ok(OrdinalValue { value: value })
             });
    b.rule_2("le <ordinal>",
             b.reg(r#"le"#)?,
             ordinal_check!(),
             |_, a| Ok(*a.value()));
    Ok(b.build())
}

pub fn examples_numbers() -> Vec<::rustling::train::Example<Dimension>> {
    let mut v = vec![];
    example!(v, examples::check_integer(1), "1", "un", "une");
    example!(v, examples::check_integer(11), "onze");
    example!(v, examples::check_integer(17), "dix sept", "dix-sept");
    example!(v, examples::check_integer(21), "vingt et un", "vingt-et-un");
    example!(v, examples::check_integer(23), "vingt trois", "vingt-trois");
    example!(v, examples::check_integer(70), "soixante dix");
    example!(v, examples::check_integer(78), "soixante dix huit");
    example!(v, examples::check_integer(73), "soixante treize");
    example!(v, examples::check_integer(80), "quatre vingt");
    example!(v, examples::check_integer(81), "quatre vingt un");
    example!(v, examples::check_integer(90), "quatre vingt dix");
    example!(v, examples::check_integer(91), "quatre vingt onze");
    example!(v, examples::check_integer(99), "quatre vingt dix neuf");
    example!(v,
             examples::check_integer(33),
             "33",
             "trente trois",
             "trente-trois",
             "trente 3");
    example!(v,
             examples::check_integer(100000),
             "100.000",
             "100000",
             "100K",
             "100k");
    example!(v,
             examples::check_integer(3000000),
             "3M",
             "3000K",
             "3000000",
             "3.000.000");
    example!(v,
             examples::check_integer(1200000),
             "1.200.000",
             "1200000",
             "1,2M",
             "1200K",
             ",0012G");
    example!(v,
             examples::check_integer(-1200000),
             "- 1.200.000",
             "-1200000",
             "moins 1200000",
             "-1,2M",
             "-1200K",
             "-,0012G");
    example!(v, examples::check_ordinal(1), "1er", "1ere", "le 1er");
    example!(v,
             examples::check_ordinal(3),
             "3ieme",
             "le 3ieme",
             "3eme",
             "3ème",
             "troisième",
             "troisieme");
    v
}
