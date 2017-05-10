use rustling::*;
use dimension::*;
use helpers;
use moment::{Weekday, Grain, PeriodComp};

pub fn rules_duration(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1("seconde (unit-of-duration)",
        b.reg(r#"sec(onde)?s?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Second))
    );
    b.rule_1("minute (unit-of-duration)",
        b.reg(r#"min(ute)?s?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Minute))
    );
    b.rule_1("heure (unit-of-duration)",
        b.reg(r#"h(eure)?s?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Hour))
    );
    b.rule_1("jour (unit-of-duration)",
        b.reg(r#"jour(n[ée]e?)?s?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Day))
    );
    b.rule_1("semaine (unit-of-duration)",
        b.reg(r#"semaines?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Week))
    );
    b.rule_1("mois (unit-of-duration)",
        b.reg(r#"mois?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Month))
    );
    b.rule_1("année (unit-of-duration)",
        b.reg(r#"an(n[ée]e?)?s?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Year))
    );
    b.rule_1("un quart heure",
        b.reg(r#"(1/4\s?h(eure)?|(un|1) quart d'heure)"#)?,
        |_| Ok(DurationValue::new(PeriodComp::minutes(15).into()))
    );
    b.rule_1("une demi heure",
        b.reg(r#"(?:1/2\s?h(?:eure)?|(?:1|une) demi(?:e)?(?:\s|-)heure)"#)?,
        |_| Ok(DurationValue::new(PeriodComp::minutes(30).into()))
    );
    b.rule_1("trois quarts d'heure",
        b.reg(r#"(?:3/4\s?h(?:eure)?|(?:3|trois) quart(?:s)? d'heure)"#)?,
        |_| Ok(DurationValue::new(PeriodComp::minutes(45).into()))
    );
    b.rule_2("<integer> <unit-of-duration>",
        integer_check!(0),
        unit_of_duration_check!(),
        |integer, unit| Ok(DurationValue::new(PeriodComp::new(unit.value().grain, integer.value().value).into()))
    );
    b.rule_2("une <unit-of-duration>",
        b.reg(r#"une|la|le?"#)?,
        unit_of_duration_check!(),
        |_, unit| Ok(DurationValue::new(PeriodComp::new(unit.value().grain, 0).into()))
    );
    b.rule_2("dans <duration>",
        b.reg(r#"dans"#)?,
        duration_check!(),
        |_, duration| duration.value().in_present()
    );
    b.rule_2("environ <duration>",
        b.reg(r#"environ"#)?,
        duration_check!(),
        |_, duration| duration.value().in_present()
    );
    b.rule_2("il y a <duration>",
        b.reg(r#"il y a"#)?,
        duration_check!(),
        |_, duration| duration.value().ago()
    );
    b.rule_3("<duration> apres <time>",
        duration_check!(),
        b.reg(r#"apr[eè]s"#)?,
        time_check!(),
        |duration, _ , time| duration.value().after(time.value())
    );
    b.rule_3("<duration> apres <time>",
        duration_check!(),
        b.reg(r#"apr[eè]s"#)?,
        time_check!(),
        |duration, _ , time| duration.value().after(time.value())
    );
    b.rule_3("<duration> avant <time>",
        duration_check!(),
        b.reg(r#"avant"#)?,
        time_check!(),
        |duration, _, time| duration.value().before(time.value())
    );
    Ok(())
}

pub fn rules_cycle(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1("seconde (cycle)",
        b.reg(r#"secondes?"#)?,
        |_| CycleValue::new(Grain::Second)
    );
    b.rule_1("minute (cycle)",
        b.reg(r#"minutes?"#)?,
        |_| CycleValue::new(Grain::Minute)
    );
    b.rule_1("heure (cycle)",
        b.reg(r#"heures?"#)?,
        |_| CycleValue::new(Grain::Hour)
    );
    b.rule_1("jour (cycle)",
        b.reg(r#"jour(n[ée]e?)?s?"#)?,
        |_| CycleValue::new(Grain::Day)
    );
    b.rule_1("semaine (cycle)",
        b.reg(r#"semaines?"#)?,
        |_| CycleValue::new(Grain::Week)
    );
    b.rule_1("mois (cycle)",
        b.reg(r#"mois"#)?,
        |_| CycleValue::new(Grain::Month)
    );
    b.rule_1("année (cycle)",
        b.reg(r#"an(n[ée]e?)?s?"#)?,
        |_| CycleValue::new(Grain::Year)
    );
    b.rule_2("ce|dans le <cycle>",
        b.reg(r#"(?:cet?t?e?s?)|(?:dans l[ae']? ?)"#)?,
        cycle_check!(),
        |_, cycle| helpers::cycle_nth(cycle.value().grain, 0)
    );
    b.rule_3("le <cycle> dernier",
        b.reg(r#"l[ae']? ?"#)?,
        cycle_check!(),
        b.reg(r#"derni[èe]re?|pass[ée]e?"#)?,
        |_, cycle, _| helpers::cycle_nth(cycle.value().grain, -1)
    );
    b.rule_3("le <cycle> prochain|suivant|d'après",
        b.reg(r#"l[ae']? ?|une? ?"#)?,
        cycle_check!(),
        b.reg(r#"prochaine?|suivante?|qui suit|(?:d'? ?)?apr[eèé]s"#)?,
        |_, cycle, _| helpers::cycle_nth(cycle.value().grain, 1)
    );
    b.rule_2("<cycle> dernier",
        cycle_check!(),
        b.reg(r#"derni[èe]re?|pass[ée]e?|pr[eé]c[eé]dente?|(?:d')? ?avant"#)?,
        |cycle, _| helpers::cycle_nth(cycle.value().grain, -1)
    );
    b.rule_2("<cycle> prochain|suivant|d'après",
        cycle_check!(),
        b.reg(r#"prochaine?|suivante?|qui suit|(?:d')? ?apr[eèé]s"#)?,
        |cycle, _| helpers::cycle_nth(cycle.value().grain, 1)
    );
    b.rule_3("n <cycle> avant",
        integer_check!(2, 9999),
        cycle_check!(),
        b.reg(r#"(?:d')? ?avant|plus t[oô]t"#)?,
        |integer, cycle, _| helpers::cycle_nth(cycle.value().grain, -1 * integer.value().value)
    );
    b.rule_3("n <cycle> après",
        integer_check!(2, 9999),
        cycle_check!(),
        b.reg(r#"(?:d')? ?apr[eèé]s|qui sui(?:t|ves?)|plus tard"#)?,
        |integer, cycle, _| helpers::cycle_nth(cycle.value().grain, integer.value().value)
    );
    b.rule_4("le <cycle> après|suivant <time>",
        b.reg(r#"l[ea']? ?"#)?,
        cycle_check!(),
        b.reg(r#"suivante?|apr[eèé]s"#)?,
        time_check!(),
        |_, cycle, _, time| helpers::cycle_nth_after(cycle.value().grain, 1, time.value())
    );
    b.rule_4("le <cycle> avant|précédent <time>",
        b.reg(r#"l[ea']? ?"#)?,
        cycle_check!(),
        b.reg(r#"avant|pr[ée]c[ée]dent"#)?,
        time_check!(),
        |_, cycle, _, time| helpers::cycle_nth_after(cycle.value().grain, -1, time.value())
    );
    b.rule_3("n derniers <cycle>",
        integer_check!(2, 9999),
        b.reg(r#"derni.re?s?"#)?,
        cycle_check!(),
        |integer,  _, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    b.rule_3("n prochains <cycle>",
        integer_check!(2, 9999),
        b.reg(r#"prochaine?s?|suivante?s?|apr[eèé]s"#)?,
        cycle_check!(),
        |integer, _, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    b.rule_3("n <cycle> passes|precedents",
        integer_check!(2, 9999),
        cycle_check!(),
        b.reg(r#"pass[eèé][eèé]?s?|pr[eé]c[eé]dente?s?|(?:d')? ?avant|plus t[oô]t"#)?,
        |integer, cycle, _| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    b.rule_3("n <cycle> suivants",
        integer_check!(2, 9999),
        cycle_check!(),
        b.reg(r#"prochaine?s?|suivante?s?|apr[eèé]s|qui sui(?:t|ves?)|plus tard"#)?,
        |integer, cycle, _| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    b.rule_4("<ordinal> <cycle> de <time>",
        ordinal_check!(),
        cycle_check!(),
        b.reg(r#"d['eu]|en"#)?,
        time_check!(),
        |ordinal, cycle, _, time| helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value, time.value())
    );
    b.rule_5("le <ordinal> <cycle> de <time>",
        b.reg(r#"l[ea]"#)?,
        ordinal_check!(),
        cycle_check!(),
        b.reg(r#"d['eu]|en"#)?,
        time_check!(),
        |_, ordinal, cycle, _, time| helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value, time.value())
    );
    b.rule_4("le <cycle> de <time>",
        b.reg(r#"l[ea]"#)?,
        cycle_check!(),
        b.reg(r#"d['eu]|en"#)?,
        time_check!(),
        |_, cycle, _, time| helpers::cycle_nth_after_not_immediate(cycle.value().grain, 0, time.value())
    );
    b.rule_2("le lendemain du <time>",
        b.reg(r#"(?:le|au)? ?lendemain du"#)?,
        time_check!(),
        |_, time| helpers::cycle_nth_after_not_immediate(Grain::Day, 1, time.value())
    );
    b.rule_2("le veille du <time>",
        b.reg(r#"(la )?veille du"#)?,
        time_check!(),
        |_, time| helpers::cycle_nth_after_not_immediate(Grain::Day, -1, time.value())
    );
    Ok(())
}

pub fn rules_temperature(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1("number as temp", number_check!(), |a| {
        Ok(TemperatureValue {
               value: a.value().value(),
               unit: None,
               latent: true,
           })
    });
    b.rule_2("<latent temp> degrees",
             temperature_check!(),
             b.reg(r#"(?:deg(?:r[éeè])?s?\.?)|°"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                        value: a.value().value,
                        unit: Some("degree"),
                        latent: false,
                    })
             });
    b.rule_2("<temp> Celcius",
             temperature_check!(),
             b.reg(r#"c(?:el[cs]?(?:ius)?)?\.?"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                        value: a.value().value,
                        unit: Some("celsius"),
                        latent: false,
                    })
             });
    b.rule_2("<temp> Fahrenheit",
             temperature_check!(),
             b.reg(r#"f(?:ah?reh?n(?:h?eit)?)?\.?"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                        value: a.value().value,
                        unit: Some("fahrenheit"),
                        latent: false,
                    })
             });
    b.rule_2("<latent temp> en dessous de zero",
             temperature_check!(),
             b.reg(r#"en dessous de (?:0|z[ée]ro)"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                        value: -1.0 * a.value().value,
                        latent: false,
                        ..*a.value()
                    })
             });
    Ok(())
}

pub fn rules_numbers(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
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
    Ok(())
}

pub fn rules_finance(_b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    Ok(())
}

pub fn rules_time(_b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    Ok(())
}
