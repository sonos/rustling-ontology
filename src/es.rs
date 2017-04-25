use rustling::*;
use dimension::*;
use dimension::Precision::*;
use helpers;
use examples::*;

#[allow(dead_code)]
pub fn rules_temperature() -> RustlingResult<RuleSet<Dimension>> {
    Ok(RuleSet(vec![
        rule! { 
            "number as temp", 
            (number_check!()), 
            |a| Ok(TemperatureValue { value: a.value().value(), unit: None, latent: true}) 
        },
        rule! {
            "<latent temp> temp",
            (temperature_check!(), regex!(r#"(grados?)|°"#)),
            |a, _| Ok(TemperatureValue { value: a.value().value, unit: Some("degree"), latent: false})
        },
        rule! {
            "<temp> Celcius",
            (temperature_check!(), regex!(r#"(cent(i|í)grados?|c(el[cs]?(ius)?)?\.?)"#)),
            |a, _| Ok(TemperatureValue { value: a.value().value, unit: Some("celsius"), latent: false})
        },
        rule! {
            "<temp> Fahrenheit",
            (temperature_check!(), regex!(r#"f(ah?reh?n(h?eit)?)?\.?"#)),
            |a, _| Ok(TemperatureValue { value: a.value().value, unit: Some("fahrenheit"), latent: false})
        },
        rule! {
            "<latent temp> temp bajo cero",
            (temperature_check!(), regex!(r#"((grados?)|°)?( bajo cero)"#)),
            |a, _| Ok(TemperatureValue { value: -1.0 * a.value().value, latent: false, .. *a.value()})
        }
    
    ]))
}

pub fn rules_numbers() -> RustlingResult<RuleSet<Dimension>> {
    Ok(RuleSet(vec![
        rule! {
            "number (0..15)",
            (regex!(r#"((?:c|z)ero|un(?:o|a)?|dos|tr(?:é|e)s|cuatro|cinco|s(?:e|é)is|siete|ocho|nueve|die(?:z|s)|once|doce|trece|catorce|quince)"#)),
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
                     _ => panic!("Unknow match"),
                };
                IntegerValue::new(value)
             }
        },
        rule! {
            "number (20..90)",
            (regex!(r#"(veinte|treinta|cuarenta|cincuenta|sesenta|setenta|ochenta|noventa)"#)),
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
                    _ => panic!("Unknow match"),
                };
                IntegerValue::new(value)
            }
        },
        rule! {
            "number (16..19)",
            (
                integer_check!(0, 10),
                regex!(r#"y"#),
                integer_check!(6, 9)
            ),
            |_, _, a| IntegerValue::new(a.value().value + 10)
        },
        rule! {
            "number (21..29 31..39 41..49 51..59 61..69 71..79 81..89 91..99)",
            (
                integer_check!(20, 90, |integer: &IntegerValue| integer.value % 10 == 0),
                regex!(r#"y"#),
                integer_check!(1, 9)
            ),
            |a, _, b| IntegerValue::new(a.value().value + b.value().value)
        },
        rule! {
            "number (16..19 21..29)",
            (regex!(r#"(die(?:c|s)is(?:é|e)is|diecisiete|dieciocho|diecinueve|veintiun(?:o|a)|veintidos|veintitr(?:é|e)s|veinticuatro|veinticinco|veintis(?:é|e)is|veintisiete|veintiocho|veintinueve)"#)),
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
                    "veintidos" => 22,
                    "veintitres" => 23,
                    "veintitrés" => 23,
                    "veinticuatro" => 24,
                    "veinticinco" => 25,
                    "veintiseis" => 26,
                    "veintiséis" => 26, 
                    "veintisiete" => 27,
                    "veintiocho" => 28, 
                    "veintinueve" => 29,
                    _ => panic!("Unknown match")
                };
                IntegerValue::new(value)
            }
        },
        rule! {
            "number 100..1000",
            (regex!(r#"(cien(?:to)?s?|doscientos|trescientos|cuatrocientos|quinientos|seiscientos|setecientos|ochocientos|novecientos|mil)"#)),
            |text_match| {
                let value = match text_match.group(1).as_ref() {
                    "cien" => 100,
                    "cientos" => 100,
                    "ciento" => 100,
                    "doscientos" => 200,
                    "trescientos" => 300,
                    "cuatrocientos" => 400,
                    "quinientos" => 500, 
                    "seiscientos" => 600, 
                    "setecientos" => 700,
                    "ochocientos" => 800,
                    "novecientos" => 900,
                    "mil" => 1000,
                    _ => panic!("Unknown match")
                };
                IntegerValue::new(value)
            }
        },
        rule! {
            "numbers 200..999",
            (
                integer_check!(2, 9),
                integer_check!(100, 100),
                integer_check!(0, 99)
            ),
            |a, b, c| IntegerValue::new(a.value().value * b.value().value + c.value().value)
        },
        rule! {
            "integer (numeric)",
            (regex!(r#"(\d{1,18})"#)),
            |text_match| IntegerValue::new(text_match.group(0).parse()?)
        },
        rule! {
            "integer with thousands separator .",
            (regex!(r#"(\d{1,3}(\.\d\d\d){1,5})"#)),
            |text_match| {
                let reformatted_string = text_match.group(1).replace(".", "");
                let value: i64 = reformatted_string.parse()?;
                IntegerValue::new(value)
            }
        },
        rule! {
            "decimal number",
            (regex!(r#"(\d*,\d+)"#)),
            |text_match| {
                let reformatted_string = text_match.group(1).replace(",", ".");
                let value: f32 = reformatted_string.parse()?;
                FloatValue::new(value)
            }
        },
        rule! {
             "number dot number",
             (
                 number_check!(|number: &NumberValue| !number.prefixed()),
                 regex!(r#"punto"#),
                 number_check!(|number: &NumberValue| !number.suffixed())
             ),
             |a, _, b| Ok(FloatValue { value: b.value().value() * 0.1 + a.value().value(), .. FloatValue::default() })
        },
        rule! {
            "decimal with thousands separator",
            (regex!(r#"(\d+(\.\d\d\d)+,\d+)"#)),
            |text_match| {
                let reformatted_string = text_match.group(1).replace(".", "").replace(",", ".");
                let value: f32 = reformatted_string.parse()?;
                FloatValue::new(value)
            }
        },
        rule! {
            "numbers prefix with -, negative or minus",
            (
                regex!(r#"-|menos"#),
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
            "ordinals (primero..10)",
            (regex!(r#"(primer|tercer(os?|as?)?|(primer|segund|cuart|quint|sext|s[eé]ptim|octav|noven|d[eé]cim)(os?|as?))"#)),
            |text_match| {
                let value = match text_match.group(1).as_ref() {
                    "primer" => 1,
                    "primero" => 1,
                    "segundo" => 2,
                    "tercero" => 3,
                    "tercer" => 3,
                    "cuarto" => 4,
                    "quinto" => 5,
                    "sexto" => 6,
                    "séptimo" => 7,
                    "septimo" => 7,
                    "octavo" => 8,
                    "noveno" => 9,
                    "décimo" => 10,
                    "decimo" => 10,
                    "primeros" => 1, 
                    "segundos" => 2, 
                    "terceros" => 3, 
                    "cuartos" => 4, 
                    "quintos" => 5, 
                    "sextos" => 6,
                    "séptimos" => 7, 
                    "septimos" => 7, 
                    "octavos" => 8, 
                    "novenos" => 9, 
                    "décimos" => 10, 
                    "decimos" => 10,
                    "primera" => 1, 
                    "segunda" => 2, 
                    "tercera" => 3, 
                    "cuarta" => 4, 
                    "quinta" => 5, 
                    "sexta" => 6,
                    "séptima" => 7, 
                    "septima" => 7, 
                    "octava" => 8, 
                    "novena" => 9, 
                    "décima" => 10,
                    "decima" => 10,
                    "primeras" => 1, 
                    "segundas" => 2, 
                    "terceras" => 3, 
                    "cuartas" => 4, 
                    "quintas" => 5, 
                    "sextas" => 6,
                    "séptimas" => 7, 
                    "septimas" => 7, 
                    "octavas" => 8, 
                    "novenas" => 9, 
                    "décimas" => 10, 
                    "decimas" => 10,
                    _ => panic!("Unknown match")
                };
                Ok(OrdinalValue { value: value})
            }
        }
    ]))
}

pub fn examples_numbers() -> Vec<::rustling::train::Example<Dimension>> {
    let mut v = vec![];
    example!(v, check_integer(1), "1", "uno", "una");
    example!(v, check_integer(11), "once");
    example!(v, check_integer(16), "dieciséis", "dieciseis", "diesiseis", "diez y seis");
    example!(v, check_integer(21), "veintiuno", "veinte y uno");
    example!(v, check_integer(23), "veintitrés", "veinte y tres");
    example!(v, check_integer(70), "setenta");
    example!(v, check_integer(78), "setenta y ocho");
    example!(v, check_integer(80), "ochenta");
    example!(v, check_integer(33), "33", "treinta y tres", "treinta y 3");
    example!(v, check_float(1.1), "1,1", "1,10", "01,10");
    example!(v, check_float(0.77), "0,77", ",77");
    example!(v, check_integer(100000), "100.000", "100000", "100K", "100k");
    example!(v, check_integer(300), "trescientos");
    example!(v, check_integer(243), "243");
    example!(v, check_integer(3000000), "3M", "3000K", "3000000", "3.000.000");
    example!(v, check_integer(1200000), "1.200.000", "1200000", "1,2M", "1200K", ",0012G");
    example!(v, check_integer(-1200000), "- 1.200.000", "-1200000", "menos 1.200.000", "-1,2M", "-1200K", "-,0012G");
    example!(v, check_float(1.5), "1 punto cinco", "una punto cinco", "1,5");
    v
}