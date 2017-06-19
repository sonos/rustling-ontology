use rustling::*;
use values::dimension::*;
use values::dimension::Precision::*;
use values::helpers;

pub fn rules_numbers(b:&mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("intersect",
        number_check!(|number: &NumberValue| number.grain().unwrap_or(0) > 1),
        number_check!(),
        |a, b| helpers::compose_numbers(&a.value(), &b.value()));
    b.rule_3("numbers und",
        integer_check!(1, 9),
        b.reg(r#"und"#)?,
        integer_check!(10, 90, |integer: &IntegerValue| integer.value % 10 == 0),
        |a, _, b| IntegerValue::new(a.value().value + b.value().value)
    );
    b.rule_1("integer (0..19)",
        b.reg(r#"(keine?|keine?s|keiner|keinen|null|nichts|eins?(er)?|zwei|dreizehn|drei|vierzehn|vier|funf|sechzehn|sechs|siebzehn|sieben|achtzehn|acht|neunzehn|neun|elf|zwolf|fufzehn)"#)?,
        |text_match| {
            let value = match text_match.group(1).as_ref() {
                "kein" => 0, 
                "keine" => 0, 
                "keins" => 0, 
                "keines" => 0, 
                "keiner" => 0, 
                "keinen" => 0, 
                "null" => 0, 
                "nichts" => 0,
                "ein" => 1, 
                "eins" => 1, 
                "eine" => 1, 
                "einer" => 1, 
                "zwei" => 2, 
                "drei" => 3, 
                "vier" => 4, 
                "funf" => 5, 
                "sechs" => 6,
                "sieben" => 7, 
                "acht" => 8, 
                "neun" => 9, 
                "zehn" => 10, 
                "elf" => 11, 
                "zwolf" => 12, 
                "dreizehn" => 13, 
                "vierzehn" => 14,
                "funfzehn" => 15, 
                "sechzehn" => 16, 
                "siebzehn" => 17, 
                "achtzehn" => 18, 
                "neunzehn" => 19,
                _ => panic!("Unknown match {:?}", text_match.group(1)),
            };
            IntegerValue::new(value)
        }
    );
    b.rule_1("ten",
        b.reg(r#"zehn"#)?,
        |_| IntegerValue::new_with_grain(10, 1)
    );
    b.rule_1("dozen",
        b.reg(r#"dutzend"#)?,
        |_| Ok(IntegerValue {
                value: 12,
                grain: Some(1),
                group: true,
                ..IntegerValue::default()
            })
    );
    b.rule_1("hundred",
        b.reg(r#"hunderte?"#)?,
        |_| IntegerValue::new_with_grain(1000, 3)
    );
    b.rule_1("thousand",
        b.reg(r#"tausende?"#)?,
        |_| IntegerValue::new_with_grain(1000, 3)
    );
    b.rule_1("million",
        b.reg(r#"million(en)?"#)?,
        |_| IntegerValue::new_with_grain(1000000, 6)
    );
    b.rule_1("couple",
        b.reg(r#"(ein )?paar"#)?,
        |_| IntegerValue::new(2)
    );
    b.rule_1("few",
        b.reg(r#"mehrere"#)?,
        |_| Ok(IntegerValue {
            value: 3,
            grain: Some(1),
            precision: Approximate,
            ..IntegerValue::default()
        })
    );
    b.rule_1("integer (20..90)",
        b.reg(r#"(zwanzig|dreissig|vierzig|funfzig|sechzig|siebzig|achtzig|neunzig)"#)?,
        |text_match| {
            let value = match text_match.group(1).as_ref() {
                "zwanzig" => 20, 
                "dreissig" => 30, 
                "vierzig" => 40, 
                "funfzig" => 50, 
                "sechzig" => 60,
                "siebzig" => 70, 
                "achtzig" => 80, 
                "neunzig" => 90,
                _ => panic!("Unknown match {:?} with a text match: {:?}", text_match.group(1), text_match),
            };
            IntegerValue::new_with_grain(value, 1)
        }
    );
    b.rule_1("integer ([2-9][1-9])",
        b.reg(r#"(ein|zwei|drei|vier|funf|sechs|sieben|acht|neun)und(zwanzig|dreissig|vierzig|funfzig|sechzig|siebzig|achtzig|neunzig)"#)?,
        |text_match| {
            let digit = match text_match.group(1).as_ref() {
                "ein" => 1, 
                "zwei" => 2, 
                "drei" => 3, 
                "vier" => 4, 
                "funf" => 5,
                "sechs" => 6, 
                "sieben" => 7, 
                "acht" => 8, 
                "neun" => 9,
                _ => panic!("Unknown match {:?} with a text match: {:?}", text_match.group(1), text_match),
            };
            let tens_digit = match text_match.group(2).as_ref() {
                "zwanzig"  => 20, 
                "dreissig" => 30, 
                "vierzig"  => 40, 
                "funfzig"  => 50,
                "sechzig"  => 60, 
                "siebzig"  => 70, 
                "achtzig"  => 80, 
                "neunzig"  => 90,
                 _ => panic!("Unknown match {:?} with a text match: {:?}", text_match.group(2), text_match),

            };
            IntegerValue::new(digit + tens_digit)
        }
    );
    b.rule_1("integer (numeric)",
        b.reg(r#"(\d{1,18})"#)?,
        |text_match| IntegerValue::new(text_match.group(1).parse()?)
    );
    b.rule_1("integer with thousands separator .",
        b.reg(r#"(\d{1,3}(\.\d\d\d){1,5})"#)?,
        |text_match| IntegerValue::new(text_match.group(1).replace(".", "").parse()?)
    );
    
    b.rule_2("number hundreds",
        integer_check!(1, 99),
        integer_check!(100, 100),
        |a, b| Ok(IntegerValue {
            value: a.value().value * b.value().value,
            grain: b.value().grain,
            ..IntegerValue::default()
        })
    );

    b.rule_2("number millions",
        integer_check!(1, 99),
        integer_check!(1000, 1000),
        |a, b| Ok(IntegerValue {
            value: a.value().value * b.value().value,
            grain: b.value().grain,
            ..IntegerValue::default()
        })
    );
    b.rule_1("decimal number",
        b.reg(r#"(\d*,\d+)"#)?,
        |text_match| FloatValue::new(text_match.group(1).replace(",", ".").parse()?)
    );
    b.rule_3("number dot number",
        number_check!(|number: &NumberValue| !number.prefixed()),
        b.reg(r#"(?i)komma"#)?,
        number_check!(|number: &NumberValue| !number.suffixed()),
        |a, _, b| FloatValue::new(b.value().value() * 0.1 + a.value().value())
    );
    

    Ok(())
}
