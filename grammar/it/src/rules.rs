use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::helpers;
use rustling_ontology_moment::{Weekday, Grain, PeriodComp};

pub fn rules_percentage(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("<number> per cent",
        number_check!(),
        b.reg(r"(?:%|percento|per cento)")?,
        |number, _| Ok(PercentageValue(number.value().value()))
    );
    Ok(())
}

pub fn rules_finance(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("intersect (X cents)",
             amount_of_money_check!(|money: &AmountOfMoneyValue| money.unit != Some("cent")),
             amount_of_money_check!(|money: &AmountOfMoneyValue| money.unit == Some("cent")),
             |a, b| helpers::compose_money(a.value(), b.value()));
    b.rule_3("intersect (and X cents)",
             amount_of_money_check!(|money: &AmountOfMoneyValue| money.unit != Some("cent")),
             b.reg(r#"y"#)?,
             amount_of_money_check!(|money: &AmountOfMoneyValue| money.unit == Some("cent")),
             |a, _, b| helpers::compose_money(&a.value(), &b.value()));
    b.rule_2("intersect",
             amount_of_money_check!(|money: &AmountOfMoneyValue| money.unit != Some("cent")),
             number_check!(),
             |a, b| helpers::compose_money_number(&a.value(), &b.value()));
    b.rule_1_terminal("$",
        b.reg(r#"\$|dollar[oi]"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("$") })
    );
    b.rule_1_terminal("EUR",
        b.reg(r#"€|[e€]uro?s?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("EUR") })
    );
    b.rule_1_terminal("£",
        b.reg(r#"sterlin[ae]|£"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("£") })
    );
    b.rule_1_terminal("USD",
        b.reg(r#"us[d\$]|dollar[oi]? american(o|i)"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("USD") })
    );
    b.rule_1_terminal("Bitcoin",
        b.reg(r#"bitcoins?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("฿") })
    );
    b.rule_1_terminal("GBP",
        b.reg(r#"gbp|sterlin[ae] "#)?,
        |_| Ok(MoneyUnitValue { unit: Some("GBP") })
    );
    b.rule_1_terminal("INR",
                      b.reg(r#"rupi[ae]"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("INR") })
    );
    b.rule_1_terminal("cent",
                      b.reg(r#"cent(?:esim[oi]|s)?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("cent") })
    );
    b.rule_2("<amount> <unit>",
             number_check!(),
             money_unit!(),
             |a, b| {
                 Ok(AmountOfMoneyValue {
                     value: a.value().value(),
                     unit: b.value().unit,
                     ..AmountOfMoneyValue::default()
                 })
             });

    b.rule_2("<unit> <amount>",
             money_unit!(),
             number_check!(),
             |a, b| {
                 Ok(AmountOfMoneyValue {
                     value: b.value().value(),
                     unit: a.value().unit,
                     ..AmountOfMoneyValue::default()
                 })
             });
    Ok(())
}

pub fn rules_duration(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("second (unit-of-duration)",
                      b.reg(r#"sec(?:ond[oi])?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Second))
    );
    b.rule_1_terminal("minute (unit-of-duration)",
                      b.reg(r#"min(?:ut[oi])?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Minute))
    );
    b.rule_1_terminal("hour (unit-of-duration)",
                      b.reg(r#"or[ae]"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Hour))
    );
    b.rule_1_terminal("day (unit-of-duration)",
                      b.reg(r#"giorn[oi]"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Day))
    );
    b.rule_1_terminal("week (unit-of-duration)",
                      b.reg(r#"settiman[ae]"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Week))
    );
    b.rule_1_terminal("month (unit-of-duration)",
                      b.reg(r#"mes(?:e|i)"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Month))
    );
    b.rule_1_terminal("year (unit-of-duration)",
                      b.reg(r#"ann[oi]"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Year))
    );
    b.rule_2("<integer> <unit-of-duration>",
             integer_check_by_range!(0),
             unit_of_duration_check!(),
             |integer, uod| Ok(DurationValue::new(PeriodComp::new(uod.value().grain, integer.value().value).into()))
    );
    b.rule_2("en <duration>",
             b.reg(r#"en"#)?,
             duration_check!(),
             |_, duration| duration.value().in_present()
    );
    b.rule_2("fa <duration>",
             b.reg(r#"fa"#)?,
             duration_check!(),
             |_, duration| duration.value().ago()
    );
    b.rule_2("during <duration>",
             b.reg(r#"(?:durante|per)"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().prefixed())
    );
    Ok(())
}

pub fn rules_cycle(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("minuti (cycle)",
                      b.reg(r#"minut[oi]"#)?,
                      |_| CycleValue::new(Grain::Minute)
    );
    Ok(())
}

pub fn rules_time(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("intersect",
             time_check!(|time: &TimeValue| !time.latent),
             time_check!(|time: &TimeValue| !time.latent),
             |a, b| a.value().intersect(b.value())
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"lun(?:ed[íi]|\.)?"#)?,
                      |_| helpers::day_of_week(Weekday::Mon)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"mar(?:ted[íi]|\.)?"#)?,
                      |_| helpers::day_of_week(Weekday::Tue)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"mer(?:coled[íi]|\.)?"#)?,
                      |_| helpers::day_of_week(Weekday::Wed)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"gio(?:ved[íi]|v?\.)?"#)?,
                      |_| helpers::day_of_week(Weekday::Thu)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"ven(?:erd[íi]|\.)?"#)?,
                      |_| helpers::day_of_week(Weekday::Fri)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"sab(?:at[oi]|\.)?"#)?,
                      |_| helpers::day_of_week(Weekday::Sat)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"dom(?:enic(?:a|he)|\.)?"#)?,
                      |_| helpers::day_of_week(Weekday::Sun)
    );
    b.rule_1_terminal("right now",
                      b.reg(r#"ora|adesso|oggi|subito"#)?,
                      |_| helpers::cycle_nth(Grain::Second, 0)
    );
    b.rule_1_terminal("tomorrow",
                      b.reg(r#"domani"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 1)
    );
    b.rule_1_terminal("yesterday",
                      b.reg(r#"ieri"#)?,
                      |_| helpers::cycle_nth(Grain::Day, -1)
    );
    b.rule_1_terminal("the day after tomorrow",
                      b.reg(r#"dopodomani"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 2)
    );
    b.rule_1_terminal("tonight",
                      b.reg(r#"stanotte"#)?,
                      |_| {
                          let period = helpers::hour(18, false)?.span_to(&helpers::hour(0, false)?, false)?;
                          Ok(helpers::cycle_nth(Grain::Day, 0)?
                              .intersect(&period)?
                              .form(Form::PartOfDay(PartOfDayForm::Night)))
                      }
    );
    b.rule_2("alle <time-of-day>",
             b.reg(r#"a(?:ll(?:e|'))?"#)?,
             time_check!(form!(Form::TimeOfDay(_))),
             |_, tod| Ok(tod.value().clone().not_latent())
    );
    b.rule_1_terminal("hh(:|.|h)mm (time-of-day)",
                      b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:\.]([0-5]\d)"#)?,
                      |text_match| helpers::hour_minute(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?,
                          true
                      )
    );
    b.rule_1_terminal("hh:mm:ss",
                      b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:.]([0-5]\d)[:.]([0-5]\d)"#)?,
                      |text_match| helpers::hour_minute_second(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(3).parse()?,
                          true
                      )
    );
    b.rule_1_terminal("morning",
                      b.reg(r#"mattin(?:a|o)"#)?,
                      |_| Ok(helpers::hour(4, false)?.span_to(&helpers::hour(12, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Morning))
                          .latent())
    );
    b.rule_1_terminal("afternoon",
                      b.reg(r#"pomeriggio"#)?,
                      |_| Ok(helpers::hour(12, false)?.span_to(&helpers::hour(19, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Afternoon))
                          .latent())
    );
    b.rule_1_terminal("noon",
                      b.reg(r#"mezzogiorno"#)?,
                      |_| Ok(helpers::hour(12, false)?.span_to(&helpers::hour(17, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Afternoon))
                          .latent())
    );
    b.rule_1_terminal("evening",
                      b.reg(r#"sera"#)?,
                      |_| Ok(helpers::hour(18, false)?.span_to(&helpers::hour(0, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Evening))
                          .latent())
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
    b.rule_2("<latent temp> temp",
             temperature_check!(),
             b.reg(r#"(?:grad[oi]?)|°"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("degree"),
                     latent: false,
                 })
             });
    b.rule_2("<temp> Celsius",
             temperature_check!(),
             b.reg(r#"centigrad[oi]|celsius"#)?,
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
    b.rule_2("<latent temp> temp bajo cero",
             temperature_check!(),
             b.reg(r#"(?:(?:grad[oi]?)|°)?(?: sotto (lo) zero)"#)?,
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
    b.rule_1_terminal("number (0..19)",
                      b.reg(r#"(zero|un[oa']?|due|tre|quattro|cinque|sei|sette|otto|nove|dieci|(?:undici|dodici|tredici|quattordici|quindici|sedici)|(?:dici(?:assette|otto|annove)))"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "zero" => 0,
                              "un" => 1,
                              "un'" => 1,
                              "uno" => 1,
                              "una" => 1,
                              "due" => 2,
                              "tre" => 3,
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
                      b.reg(r#"(cento?|duecento|trecento|quattrocento|cinquecento|seicento|settecento|ottocento|novecento|mil)"#)?,
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
                              "mil" => 1000,
                              _ => return Err(RuleError::Invalid.into())
                          };
                          IntegerValue::new(value)
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
             b.reg(r#"punto"#)?,
             number_check!(|number: &NumberValue| !number.suffixed()),
             |a, _, b| {
                 Ok(FloatValue {
                     value: b.value().value() * 0.1 + a.value().value(),
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
    b.rule_1_terminal("ordinals (primo..10)",
                      b.reg(r#"((?:prim|second|terz|quart|quint|sest|settim|ottav|non|decim)[oiae])"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "primo" => 1,
                              "secondo" => 2,
                              "terzo" => 3,
                              "quarto" => 3,
                              "quinto" => 4,
                              "sesto" => 5,
                              "settimo" => 7,
                              "ottavo" => 8,
                              "nono" => 9,
                              "decimo" => 10,
                              "primi" => 1,
                              "secondi" => 2,
                              "terzi" => 3,
                              "quarti" => 3,
                              "quinti" => 4,
                              "sesti" => 5,
                              "settimi" => 7,
                              "ottavi" => 8,
                              "noni" => 9,
                              "decimi" => 10,
                              "prima" => 1,
                              "seconda" => 2,
                              "terza" => 3,
                              "quarta" => 3,
                              "quinta" => 4,
                              "sesta" => 5,
                              "settima" => 7,
                              "ottava" => 8,
                              "nona" => 9,
                              "decima" => 10,
                              "prime" => 1,
                              "seconde" => 2,
                              "terze" => 3,
                              "quarte" => 3,
                              "quinte" => 4,
                              "seste" => 5,
                              "settime" => 7,
                              "ottave" => 8,
                              "none" => 9,
                              "decime" => 10,
                              _ => return Err(RuleError::Invalid.into())
                          };
                          Ok(OrdinalValue::new(value))
                      });
    Ok(())
}
