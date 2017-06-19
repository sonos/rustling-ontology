use rustling::*;
use values::dimension::*;
use values::dimension::Precision::*;
use values::helpers;
use moment::{Grain, PeriodComp};


pub fn rules_duration(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1("second (unit-of-duration)",
        b.reg(r#"sek(?:unde)?nb?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Second))
    );
    b.rule_1("minute (unit-of-duration)",
        b.reg(r#"min(?:ute)?n?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Minute))
    );
    b.rule_1("hour (unit-of-duration)",
        b.reg(r#"stunden?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Hour))
    );
    b.rule_1("day (unit-of-duration)",
        b.reg(r#"tage?n?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Day))
    );
    b.rule_1("week (unit-of-duration)",
        b.reg(r#"wochen?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Week))
    );
    b.rule_1("month (unit-of-duration)",
        b.reg(r#"monate?n?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Month))
    );
    b.rule_1("year (unit-of-duration)",
        b.reg(r#"jahre?n?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Year))
    );
    b.rule_1("half an hour",
        b.reg(r#"(?:1/2\s?|(?:einer )halbe?n? )stunde"#)?,
        |_| Ok(DurationValue::new(PeriodComp::minutes(30).into()))
    );
    b.rule_1("fortnight",
        b.reg(r#"(?:a|one)? fortnight"#)?,
        |_| Ok(DurationValue::new(PeriodComp::days(14).into()))
    );
    b.rule_2("a <duration>",
        b.reg(r#"(?:in )?eine?(?:r|n)?"#)?,
        duration_check!(),
        |_, duration| duration.value().in_present() 
    );
    b.rule_2("<integer> <unit-of-duration>",
        integer_check!(0),
        unit_of_duration_check!(),
        |integer, uod| Ok(DurationValue::new(PeriodComp::new(uod.value().grain, integer.value().value).into()))
    );
    b.rule_2("number.number hours",
        b.reg(r#"(\d+)\.(\d+)"#)?,
        b.reg(r#"stunden?"#)?,
        |text_match, _| Ok(DurationValue::new(
                    PeriodComp::new(
                        Grain::Minute, 
                        helpers::decimal_hour_in_minute(text_match.group(1), text_match.group(2))?
                    ).into()
                ))
    );
    b.rule_2("<integer> and an half hours",
        integer_check!(0),
        b.reg(r#"ein ?halb stunden?"#)?,
        |integer, _| Ok(DurationValue::new(PeriodComp::minutes(integer.value().value * 60 + 30).into()))
    );
    b.rule_2("a <unit-of-duration>",
        b.reg(r#"eine?(?:r|n)?"#)?,
        unit_of_duration_check!(),
        |_, uod| Ok(DurationValue::new(PeriodComp::new(uod.value().grain, 1).into()))
    );
    // TODO check this rule
    b.rule_2("in <duration>",
        b.reg(r#"in"#)?,
        duration_check!(),
        |_, duration| duration.value().in_present()
    );
    b.rule_2("after <duration>",
        b.reg(r#"nach"#)?,
        duration_check!(),
        |_, duration| duration.value().in_present()
    );
    b.rule_2("<duration> from now",
        duration_check!(),
        b.reg(r#"ab (heute|jetzt)"#)?,
        |duration, _| duration.value().in_present()
    );
    b.rule_2("<duration> ago",
        b.reg(r#"vor"#)?,
        duration_check!(),
        |_, duration| duration.value().ago()
    );
    b.rule_2("<duration> hence",
        duration_check!(),
        b.reg(r#"hence"#)?,
        |duration, _| duration.value().in_present()
    );
    b.rule_3("<duration> after <time>",
        duration_check!(),
        b.reg(r#"nach"#)?,
        time_check!(),
        |duration, _, time| duration.value().after(time.value())
    );
    b.rule_3("<duration> before <time>",
        duration_check!(),
        b.reg(r#"vor"#)?,
        time_check!(),
        |duration, _, time| duration.value().before(time.value())
    );
    b.rule_2("about <duration>",
        b.reg(r#"ungefahr|zirka"#)?,
        duration_check!(),
        |_, duration| Ok(duration.value().clone().precision(Approximate))
    );
    Ok(())
}

pub fn rules_cycle(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1("second (cycle)",
        b.reg(r#"sekunden?"#)?,
        |_| CycleValue::new(Grain::Second)
    );
    b.rule_1("minute (cycle)",
        b.reg(r#"minuten?"#)?,
        |_| CycleValue::new(Grain::Minute)
    );
    b.rule_1("hour (cycle)",
        b.reg(r#"stunden?"#)?,
        |_| CycleValue::new(Grain::Hour)
    );
    b.rule_1("day (cycle)",
        b.reg(r#"tage?n?"#)?,
        |_| CycleValue::new(Grain::Day)
    );
    b.rule_1("week (cycle)",
        b.reg(r#"wochen?"#)?,
        |_| CycleValue::new(Grain::Week)
    );
    b.rule_1("month (cycle)",
        b.reg(r#"monate?n?"#)?,
        |_| CycleValue::new(Grain::Month)
    );
    b.rule_1("quarter (cycle)",
        b.reg(r#"quartale?"#)?,
        |_| CycleValue::new(Grain::Quarter)
    );
    b.rule_1("year (cycle)",
        b.reg(r#"jahre?n?"#)?,
        |_| CycleValue::new(Grain::Year)
    );
    b.rule_2("this <cycle>",
        b.reg(r#"diese(?:r|n|s)?|kommende(?:r|n|s)?"#)?,
        cycle_check!(),
        |_, cycle| helpers::cycle_nth(cycle.value().grain, 0)
    );
    b.rule_2("last <cycle>",
        b.reg(r#"letzte(?:r|n|s)?|vergangene(?:r|n|s)?"#)?,
        cycle_check!(),
        |_, cycle| helpers::cycle_nth(cycle.value().grain, -1)
    );
    b.rule_2("next <cycle>",
        b.reg(r#"nachste(?:r|n|s)?|kommende(?:r|n|s)?"#)?,
        cycle_check!(),
        |_, cycle| helpers::cycle_nth(cycle.value().grain, 1)
    );
    b.rule_4("the <cycle> after <time>",
        b.reg(r#"der"#)?,
        cycle_check!(),
        b.reg(r#"nach"#)?,
        time_check!(),
        |_, cycle, _, time| helpers::cycle_nth_after(cycle.value().grain, 1, time.value())
    );
    b.rule_4("the <cycle> before <time>",
        b.reg(r#"der"#)?,
        cycle_check!(),
        b.reg(r#"vor"#)?,
        time_check!(),
        |_, cycle, _, time| helpers::cycle_nth_after(cycle.value().grain, -1, time.value())
    );
    b.rule_3("last n <cycle>",
        b.reg(r#"letzten?|vergangenen?"#)?,
        integer_check!(1, 9999),
        cycle_check!(),
        |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    b.rule_3("next n <cycle>",
        b.reg(r#"nachsten?|kommenden?"#)?,
        integer_check!(1, 9999),
        cycle_check!(),
        |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    b.rule_4("<ordinal> <cycle> of <time>",
        ordinal_check!(),
        cycle_check!(),
        b.reg(r#"im|in|von"#)?,
        time_check!(),
        |ordinal, cycle, _, time| helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value - 1, time.value())
    );
    b.rule_5("the <ordinal> <cycle> of <time>",
        b.reg(r#"der|die|das"#)?,
        ordinal_check!(),
        cycle_check!(),
        b.reg(r#"im|in|von"#)?,
        time_check!(),
        |_, ordinal, cycle, _, time| helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value - 1, time.value())
    );
    b.rule_4("<ordinal> <cycle> after <time>",
        ordinal_check!(),
        cycle_check!(),
        b.reg(r#"nach"#)?,
        time_check!(),
        |ordinal, cycle, _, time| helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value - 1, time.value())
    );
    b.rule_2("<ordinal> quarter",
        ordinal_check!(),
        cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Quarter),
        |ordinal, _| helpers::cycle_nth_after(Grain::Quarter, ordinal.value().value - 1, &helpers::cycle_nth(Grain::Year, 0)?)
    );
    b.rule_3("<ordinal> quarter <year>",
        ordinal_check!(),
        cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Quarter),
        time_check!(),
        |ordinal, _, time| helpers::cycle_nth_after(Grain::Quarter, ordinal.value().value - 1, time.value())
    );
    Ok(())
}

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
        b.reg(r#"million(?:en)?"#)?,
        |_| IntegerValue::new_with_grain(1000000, 6)
    );
    b.rule_1("couple",
        b.reg(r#"(?:ein )?paar"#)?,
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
        b.reg(r#"komma"#)?,
        number_check!(|number: &NumberValue| !number.suffixed()),
        |a, _, b| FloatValue::new(b.value().value() * 0.1 + a.value().value())
    );
    b.rule_1("decimal with thousands separator",
        b.reg(r#"(\d+(\.\d\d\d)+\,\d+)"#)?,
        |text_match| FloatValue::new(text_match.group(1).replace(",", "").replace(",", ".").parse()?)
    );
    b.rule_2("numbers prefix with -, negative or minus",
        b.reg(r#"-|minus|negativ"#)?,
        number_check!(|number: &NumberValue| !number.prefixed()),
        |_, a| -> RuleResult<NumberValue> {
            Ok(match a.value().clone() {
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
        b.reg_neg_lh(r#"([kmg])"#, r#"^[^\W\$€]"#)?,
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
    b.rule_1("ordinals (first..19th)",
        b.reg(r#"(erste(r|s)?|zweite(r|s)|dritte(r|s)|vierte(r|s)|fuenfte(r|s)|sechste(r|s)|siebte(r|s)|achte(r|s)|neunte(r|s)|zehnte(r|s)|elfter|zwolfter|dreizenter|vierzehnter|funfzehnter|sechzenter|siebzehnter|achtzehnter|neunzehnter)"#)?,
        |text_match| {
            let value = match text_match.group(1).as_ref() {
                "erste"       => 1, 
                "erster"      => 1, 
                "erstes"      => 1,
                "zweite"      => 2, 
                "zweiter"     => 2, 
                "zweites"     => 2,
                "dritte"      => 3, 
                "dritter"     => 3, 
                "drittes"     => 3,
                "vierte"      => 4, 
                "vierter"     => 4, 
                "viertes"     => 4,
                "funfte"      => 5, 
                "funfter"     => 5, 
                "funftes"     => 5,
                "sechste"     => 6, 
                "sechster"    => 6, 
                "sechstes"    => 6,
                "siebte"      => 7, 
                "siebter"     => 7, 
                "siebtes"     => 7,
                "achte"       => 8, 
                "achter"      => 8, 
                "achtes"      => 8,
                "neunte"      => 9, 
                "neunter"     => 9, 
                "neuntes"     => 9,
                "zehnte"      => 10, 
                "zehnter"     => 10, 
                "zehntes"     => 10,
                "elfter"      => 11, 
                "zwolfter"    => 12, 
                "dreizehnter" => 13,
                "vierzehnter" => 14, 
                "funfzehnter" => 15, 
                "sechzehnter" => 16,
                "siebzehnter" => 17, 
                "achtzehnter" => 18, 
                "neunzehnter" => 19,
                _ => panic!("Unknown match {:?} with a text match: {:?}", text_match.group(1), text_match),
            };
            Ok(OrdinalValue { value: value })
        }
    );
    b.rule_1("ordinal (digits)",
        b.reg(r#"0*(\d+)(\.| ?(te(n|r|s)?)|(ste(n|r|s)?))"#)?,
        |text_match| Ok(OrdinalValue { value: text_match.group(1).parse()? })
    );
    Ok(())
}
