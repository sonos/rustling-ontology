use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::dimension::Precision::*;
use rustling_ontology_values::helpers;
use rustling_ontology_moment::{Grain, PeriodComp, Weekday, Period};

fn german_article_regex() -> &'static str {
    r#"(?:i[nm]s?|zu[rm]?|beim?|um|w[äa]h?rend|f[uü]r) ?(?:de(?:r|m|s|n)|die|das)?"#
}

fn german_article_before_cycle() -> &'static str {
    r#"(?:i[nm]s?|a[nm]) (?:de(?:r|m|s|n)|die|das)|(?:i[nm]s?|a[nm])|(?:de(?:r|m|s|n)|die|das)"#
}

pub fn rules_percentage(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("<number> per cent",
        number_check!(),
        b.reg(r"(?:%|prozente?s?|vom hundert)")?,
        |number, _| Ok(PercentageValue(number.value().value()))
    );
    Ok(())
}

pub fn rules_finance(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("intersect (X cents)",
             amount_of_money_check!(),
             amount_of_money_check!(|money: &AmountOfMoneyValue| money.unit == Some("cent")),
             |a, b| helpers::compose_money(a.value(), b.value())
    );
    b.rule_1_terminal("$",
                      b.reg(r#"\$|dollar[sn]?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("$") })
    );
    b.rule_1_terminal("USD",
                      b.reg(r#"us[d\$]|us[ -]dollar[sn]?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("USD") })
    );
    b.rule_1_terminal("AUD",
                      b.reg(r#"au[d\$]|australische[rnm]? dollar[sn]?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("AUD") })
    );
    b.rule_1_terminal("CAD",
                      b.reg(r#"cad|can\$|kanadische[rnm]? dollar[sn]?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("CAD") })
    );
    b.rule_1_terminal("HKD",
                      b.reg(r#"hk[d\$]|hong ?kong dollar[sn]?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("HKD") })
    );
    b.rule_1_terminal("EUR",
                      b.reg(r#"euros?|eur|€"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("EUR") })
    );
    b.rule_1_terminal("£",
                      b.reg(r#"£|pfund(?:e?s)?|pfd\."#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("£") })
    );
    b.rule_1_terminal("GBP",
                      b.reg(r#"gbp|britische[rnms]? pfund(?:e?s)?|pfund(?:e?s)? sterling"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("GBP") })
    );
    b.rule_1_terminal("CHF",
                      b.reg(r#"chf|(?:schweizer )?frankens?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("CHF") })
    );
    b.rule_1_terminal("KR",
                      b.reg(r#"kronen?|kr"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("KR") })
    );
    b.rule_1_terminal("DKK",
                      b.reg(r#"dkk|d[äa]nische[nr]? kronen?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("DKK") })
    );
    b.rule_1_terminal("NOK",
                      b.reg(r#"nok|norwegische[nr]? kronen?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("NOK") })
    );
    b.rule_1_terminal("SEK",
                      b.reg(r#"sek|schwedische[nr]? kronen?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("SEK") })
    );
    b.rule_1_terminal("RUB",
                      b.reg(r#"₽|(?:russische[rnm]? )?rubel[sn]?|rub"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("RUB") })
    );
    b.rule_1_terminal("INR",
                      b.reg(r#"inr|₹|(?:indische[rn]? )?rupien?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("INR") })
    );
    b.rule_1_terminal("JPY",
                      b.reg(r#"jpy|yens?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("JPY") })
    );
    b.rule_1_terminal("CNY",
                      b.reg(r#"cny|(?:chinesische[rnm]? )?yuans?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("CNY") })
    );
    b.rule_1_terminal("¥",
                      b.reg(r#"¥"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("¥") })
    );
     b.rule_1_terminal("KRW",
                      b.reg(r#"krw|₩|(?:s[üu]dkoreanische[rnm]? )?wons?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("KRW") })
    );
    b.rule_1_terminal("BTC",
                      b.reg(r#"btc|฿|bitcoins?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("฿") })
    );
    b.rule_1_terminal("cent",
                      b.reg(r#"cents?|penn(?:y|ies)|pence|cts?|c|¢"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("cent") })
    );
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
    b.rule_2("<amount> <unit>",
             number_check!(),
             money_unit!(),
             |a, b| Ok(AmountOfMoneyValue {
                 value: a.value().value(),
                 unit: b.value().unit,
                 ..AmountOfMoneyValue::default()
             })
    );
    b.rule_2("about <amount-of-money>",
             b.reg(r#"zirka|circa|nahezu|beinahe|ungef[äa]hr|fast|ca\.?"#)?,
             amount_of_money_check!(),
             |_, a| {
                 Ok(AmountOfMoneyValue {
                     precision: Approximate,
                     ..a.value().clone()
                 })
             });
    b.rule_2("exactly <amount-of-money>",
             b.reg(r#"(?:haar|ganz |sehr )?genau|exakt|rund|gerade|pr[äa]zise"#)?,
             amount_of_money_check!(),
             |_, a| {
                 Ok(AmountOfMoneyValue {
                     precision: Exact,
                     ..a.value().clone()
                 })
             });
    Ok(())
}

pub fn rules_duration(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("second (unit-of-duration)",
                      b.reg(r#"sek(?:unden?|\.?)|s\.|sec"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Second))
    );
    b.rule_1_terminal("minute (unit-of-duration)",
                      b.reg(r#"min(?:uten?|\.?)"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Minute))
    );
    b.rule_1_terminal("hour (unit-of-duration)",
                      b.reg(r#"st(?:unden?|dn?\.?)"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Hour))
    );
    b.rule_1_terminal("day (unit-of-duration)",
                      b.reg(r#"tage?n?s?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Day))
    );
    b.rule_1_terminal("week (unit-of-duration)",
                      b.reg(r#"wochen?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Week))
    );
    b.rule_1_terminal("month (unit-of-duration)",
                      b.reg(r#"monate?n?s?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Month))
    );
    b.rule_1_terminal("year (unit-of-duration)",
                      b.reg(r#"jahre?n?s?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Year))
    );
    b.rule_2("few unit of duration",
             b.reg(r#"wenigen?"#)?,
             unit_of_duration_check!(),
             |_, uod| Ok(DurationValue::new(PeriodComp::new(uod.value().grain, 3).into()))
    );
    b.rule_1_terminal("1/4 hour",
                      b.reg(r#"(?:1/4\s?|(?:eine?r? )viertel)stunde"#)?,
                      |_| Ok(DurationValue::new(PeriodComp::minutes(15).into()))
    );
    b.rule_1_terminal("half an hour",
                      b.reg(r#"(?:1/2\s?|(?:eine?r? )halbe?n? )stunde"#)?,
                      |_| Ok(DurationValue::new(PeriodComp::minutes(30).into()))
    );
    b.rule_1_terminal("3/4 hour",
                      b.reg(r#"(?:3/4\s?|(?:eine?r? )dreiviertel)stunde"#)?,
                      |_| Ok(DurationValue::new(PeriodComp::minutes(45).into()))
    );
    b.rule_3("<integer> and a half <unit-of-duration>",
             integer_check_by_range!(0),
             b.reg(r#"einhalb"#)?,
             unit_of_duration_check!(),
             |integer, _, uod| {
                let half_period: Period = uod.value().grain.half_period().map(|a| a.into()).unwrap_or_else(|| Period::default());
                Ok(DurationValue::new(half_period + PeriodComp::new(uod.value().grain, integer.value().value)))
            }
    );
    b.rule_2("while <duration>",
             duration_check!(),
             b.reg(r#"lang"#)?,
             |duration, _| Ok(duration.value().clone())
    );
    b.rule_2("a <duration>",
             b.reg(r#"(?:in )?eine?(?:r|n)?"#)?,
             duration_check!(),
             |_, duration| duration.value().in_present()
    );
    b.rule_2("<integer> <unit-of-duration>",
             integer_check_by_range!(0),
             unit_of_duration_check!(),
             |integer, uod| Ok(DurationValue::new(PeriodComp::new(uod.value().grain, integer.value().value).into()))
    );
    b.rule_2("number.number hours",
             b.reg(r#"(\d+)\.(\d+)"#)?,
             unit_of_duration_check!(|uod: &UnitOfDurationValue| uod.grain == Grain::Hour),
             |text_match, _| Ok(DurationValue::new(
                 PeriodComp::new(
                     Grain::Minute,
                     helpers::decimal_hour_in_minute(text_match.group(1), text_match.group(2))?
                 ).into()
             ))
    );
    b.rule_2("1..12 and an half hour",
             b.reg(r#"(ein|zwei|drei|vier|f[üu]nf|sechs|sieben|acht|neun|zehn|elf|zw[öo]lf)einhalb"#)?,
             unit_of_duration_check!(|uod: &UnitOfDurationValue| uod.grain == Grain::Hour),
             |text_match, _| {
                 let value = match text_match.group(1).as_ref() {
                     "ein" => 1,
                     "zwei" => 2,
                     "drei" => 3,
                     "vier" => 4,
                     "funf" => 5,
                     "fünf" => 5,
                     "sechs" => 6,
                     "sieben" => 7,
                     "acht" => 8,
                     "neun" => 9,
                     "zehn" => 10,
                     "elf" => 11,
                     "zwolf" => 12,
                     "zwölf" => 12,
                     _ => return Err(RuleError::Invalid.into()),
                 };
                 Ok(DurationValue::new(PeriodComp::minutes(value * 60 + 30).into()))
             }
    );
    b.rule_2("half an hour",
             b.reg(r#"anderthalb"#)?,
             unit_of_duration_check!(|uod: &UnitOfDurationValue| uod.grain == Grain::Hour),
             |_, _| Ok(DurationValue::new(PeriodComp::minutes(90).into()))
    );
    b.rule_3("<integer> and an half hours",
             integer_check_by_range!(0),
             b.reg(r#"ein ?halb"#)?,
             unit_of_duration_check!(|uod: &UnitOfDurationValue| uod.grain == Grain::Hour),
             |integer, _, _| Ok(DurationValue::new(PeriodComp::minutes(integer.value().value * 60 + 30).into()))
    );
    b.rule_2("in <duration>",
             b.reg(r#"in"#)?,
             duration_check!(),
             |_, duration| duration.value().in_present()
    );
    b.rule_2("in next <duration>",
             b.reg(r#"in(?:\s(?:de(?:n|r|m)\s)?(?:n[äa]chste(?:n|r|m)|kommende(?:r|n|m)))"#)?,
             duration_check!(),
             |_, duration| {
                 let start = helpers::cycle_nth(Grain::Second, 0)?;
                 let end = duration.value().in_present()?;
                 start.span_to(&end, false)
             }
    );
    b.rule_2("after <duration>",
             b.reg(r#"nach"#)?,
             duration_check!(),
             |_, duration| duration.value().in_present()
    );
    b.rule_2("<duration> from now",
             duration_check!(),
             b.reg(r#"(?:ab|von) (?:heute|jetzt|sofort)(?: an)?"#)?,
             |duration, _| duration.value().in_present()
    );
    b.rule_2("<duration> from now",
             b.reg(r#"(?:ab|von) (?:heute|jetzt|sofort)(?: an)?"#)?,
             duration_check!(),
             |_, duration| duration.value().in_present()
    );
    b.rule_3("in <duration> from now",
             b.reg(r#"in"#)?,
             duration_check!(),
             b.reg(r#"(?:ab|von) (?:heute|jetzt|sofort)(?: an)?"#)?,
             |_, duration, _| duration.value().in_present()
    );
    b.rule_2("in <duration> from now",
             b.reg(r#"(?:ab|von) (?:heute|jetzt|sofort)(?: an)? in"#)?,
             duration_check!(),
             |_, duration| duration.value().in_present()
    );
    b.rule_3("for <duration> from now",
             b.reg(r#"f[uü]r"#)?,
             duration_check!(),
             b.reg(r#"(?:ab|von) (?:heute|jetzt|sofort)(?: an)?"#)?,
             |_, duration, _| {
                 let start = helpers::cycle_nth(Grain::Second, 0)?;
                 let end = duration.value().in_present()?;
                 start.span_to(&end, false)
             }
    );
    b.rule_2("for <duration> from now",
             b.reg(r#"(?:ab|von) (?:heute|jetzt|sofort)(?: an)? f[uü]r"#)?,
             duration_check!(),
             |_, duration| {
                 let start = helpers::cycle_nth(Grain::Second, 0)?;
                 let end = duration.value().in_present()?;
                 start.span_to(&end, false)
             }
    );
    b.rule_2("<duration> ago",
             b.reg(r#"vor"#)?,
             duration_check!(),
             |_, duration| duration.value().ago()
    );
    b.rule_2("<duration> ago",
             duration_check!(),
             b.reg(r#"fr[üu]her"#)?,
             |duration, _| duration.value().ago()
    );
    b.rule_3("<duration> after <time>",
             duration_check!(),
             b.reg(r#"nach"#)?,
             datetime_check!(),
             |duration, _, time| duration.value().after(time.value())
    );
    b.rule_3("<duration> before <time>",
             duration_check!(),
             b.reg(r#"vor"#)?,
             datetime_check!(),
             |duration, _, time| duration.value().before(time.value())
    );
    b.rule_2("about <duration>",
             b.reg(r#"ungef[äa]hr|zirka|circa|ca.|etwa|fast"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().precision(Approximate))
    );
    b.rule_2("exactly <duration>",
             b.reg(r#"genau|exakt"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().precision(Exact))
    );
    b.rule_3("<duration> and <duration>",
             duration_check!(|duration: &DurationValue| !duration.suffixed),
             b.reg(r#"und"#)?,
             duration_check!(|duration: &DurationValue| !duration.prefixed),
             |a, _, b| Ok(a.value() + b.value())
    );
    b.rule_2("<duration> <duration>",
             duration_check!(|duration: &DurationValue| !duration.suffixed),
             duration_check!(|duration: &DurationValue| !duration.prefixed),
             |a, b| Ok(a.value() + b.value())
    );
    Ok(())
}

pub fn rules_cycle(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("second (cycle)",
                      b.reg(r#"sekund(en|e)"#)?,
                      |text_match| {
                        match text_match.group(1).as_ref() {
                            "en" => CycleValue::new(Grain::Second)?.mark_as_plural(),
                            "e"  => CycleValue::new(Grain::Second),
                            _    => Err(RuleError::Invalid.into()) 
                        }
                    }
    );
    b.rule_1_terminal("minute (cycle)",
                      b.reg(r#"minut(en|e)"#)?,
                      |text_match| {
                        match text_match.group(1).as_ref() {
                            "en" => CycleValue::new(Grain::Minute)?.mark_as_plural(),
                            "e"  => CycleValue::new(Grain::Minute),
                            _    => Err(RuleError::Invalid.into()) 
                        }
                    }
    );
    b.rule_1_terminal("hour (cycle)",
                      b.reg(r#"stund(en|e)"#)?,
                      |text_match| {
                        match text_match.group(1).as_ref() {
                            "en" => CycleValue::new(Grain::Hour)?.mark_as_plural(),
                            "e"  => CycleValue::new(Grain::Hour),
                            _    => Err(RuleError::Invalid.into()) 
                        }
                    }
    );
    b.rule_1_terminal("day (cycle)",
                      b.reg(r#"ta(gen|ges|ge|gs|g)"#)?,
                      |text_match| {
                        match text_match.group(1).as_ref() {
                            "gen" | "ge" => CycleValue::new(Grain::Day)?.mark_as_plural(),
                            "gs"  | "ges" | "g" => CycleValue::new(Grain::Day),
                            _    => Err(RuleError::Invalid.into()) 
                        }
                    }
    );
    b.rule_1_terminal("week (cycle)",
                      b.reg(r#"woch(en|e)"#)?,
                      |text_match| {
                        match text_match.group(1).as_ref() {
                            "en" => CycleValue::new(Grain::Week)?.mark_as_plural(),
                            "e"  => CycleValue::new(Grain::Week),
                            _    => Err(RuleError::Invalid.into()) 
                        }
                    }
    );
    b.rule_1_terminal("month (cycle)",
                      b.reg(r#"mona(ten|tes|te|ts|t)"#)?,
                      |text_match| {
                        match text_match.group(1).as_ref() {
                            "ten" | "te" => CycleValue::new(Grain::Month)?.mark_as_plural(),
                            "ts"  | "tes" | "t" => CycleValue::new(Grain::Month),
                            _    => Err(RuleError::Invalid.into()) 
                        }
                    }
    );
    b.rule_1_terminal("quarter (cycle)",
                      b.reg(r#"quarta(len|les|le|ls|l)"#)?,
                      |text_match| {
                        match text_match.group(1).as_ref() {
                            "len" | "le" => CycleValue::new(Grain::Quarter)?.mark_as_plural(),
                            "ls" | "les" | "l" => CycleValue::new(Grain::Quarter),
                            _    => Err(RuleError::Invalid.into()) 
                        }
                    }
    );
    b.rule_1_terminal("year (cycle)",
                      b.reg(r#"jah(ren|res|re|rs|r)"#)?,
                      |text_match|{ 
                        match text_match.group(1).as_ref() {
                            "ren" | "re" => CycleValue::new(Grain::Year)?.mark_as_plural(),
                            "rs" | "res" | "r"  => CycleValue::new(Grain::Year),
                            _    => Err(RuleError::Invalid.into()) 
                        }
                    }
    );
    b.rule_2("this <cycle>",
             b.reg(r#"(?:in )?diese(?:r|n|s|m)?|de[sr]"#)?,
             cycle_check!(|cycle: &CycleValue| !cycle.is_plural),
             |_, cycle| helpers::cycle_nth(cycle.value().grain, 0)
    );
    b.rule_2("this <cycle>",
             b.reg(r#"(?:in )?diesen"#)?,
             cycle_check!(|cycle: &CycleValue| cycle.is_plural),
             |_, cycle| Ok(helpers::cycle_nth(cycle.value().grain, -1)?
                 .span_to(&helpers::cycle_nth(cycle.value().grain, 2)?, true)?
                 .precision(Approximate))
    );
    b.rule_2("last <cycle>",
             b.reg(r#"(?:die )?(?:letzte(?:r|n|s)?|vergangene(?:r|n|s)?|vor(?:her)?ige(?:r|n|s)?)"#)?,
              cycle_check!(|cycle: &CycleValue| !cycle.is_plural),
             |_, cycle| helpers::cycle_nth(cycle.value().grain, -1)
    );
    b.rule_2("last <cycle>",
             b.reg(r#"(?:die )?(?:letzten|vergangenen|vor(?:her)?igen)"#)?,
             cycle_check!(|cycle: &CycleValue| cycle.is_plural),
             |_, cycle| Ok(helpers::cycle_nth(cycle.value().grain, -3)?
                 .span_to(&helpers::cycle_nth(cycle.value().grain, -1)?, true)?
                 .precision(Approximate))
    );
    b.rule_2("before last <cycle>",
             b.reg(r#"(?:die )?(?:vorvergangene[rnm]?|vorletzte[srnm]?)"#)?,
             cycle_check!(|cycle: &CycleValue| !cycle.is_plural),
             |_, cycle| helpers::cycle_nth(cycle.value().grain, -2)
    );
    b.rule_2("before last <cycle>",
             b.reg(r#"(?:die )?(?:vorvergangenen|vorletzten)"#)?,
             cycle_check!(|cycle: &CycleValue| cycle.is_plural),
             |_, cycle| Ok(helpers::cycle_nth(cycle.value().grain, -5)?
                 .span_to(&helpers::cycle_nth(cycle.value().grain, -3)?, true)?
                 .precision(Approximate))
    );
    b.rule_2("next <cycle>",
             b.reg(r#"(?:die )?(?:n[äa]chste[rns]?|kommende[rns]?|folgende[rns]?)"#)?,
             cycle_check!(|cycle: &CycleValue| !cycle.is_plural),
             |_, cycle| helpers::cycle_nth(cycle.value().grain, 1)
    );
    b.rule_2("next <cycle>",
             b.reg(r#"(?:die )?(?:n[äa]chsten|kommenden|folgenden)"#)?,
             cycle_check!(|cycle: &CycleValue| cycle.is_plural),
             |_, cycle| Ok(helpers::cycle_nth(cycle.value().grain, 1)?
                 .span_to(&helpers::cycle_nth(cycle.value().grain, 3)?, true)?
                 .precision(Approximate))
    );
    b.rule_2("after next <cycle>",
             b.reg(r#"(?:die )?[üu]bern[äa]chste[rnms]?"#)?,
             cycle_check!(|cycle: &CycleValue| !cycle.is_plural),
             |_, cycle| helpers::cycle_nth(cycle.value().grain, 2)
    );
    b.rule_2("after next <cycle>",
             b.reg(r#"(?:die )?[üu]bern[äa]chsten"#)?,
             cycle_check!(|cycle: &CycleValue| cycle.is_plural),
             |_, cycle| Ok(helpers::cycle_nth(cycle.value().grain, 3)?
                 .span_to(&helpers::cycle_nth(cycle.value().grain, 5)?, true)?
                 .precision(Approximate))
    );
    b.rule_4("the <cycle> after <time>",
             b.reg(german_article_before_cycle())?,
             cycle_check!(),
             b.reg(r#"nach"#)?,
             datetime_check!(),
             |_, cycle, _, time| helpers::cycle_nth_after(cycle.value().grain, 1, time.value())
    );
    b.rule_4("the <cycle> before <time>",
             b.reg(german_article_before_cycle())?,
             cycle_check!(),
             b.reg(r#"vor"#)?,
             datetime_check!(),
             |_, cycle, _, time| helpers::cycle_nth_after(cycle.value().grain, -1, time.value())
    );
    b.rule_3("last n <cycle>",
             b.reg(r#"letzten?|vergangenen?"#)?,
             integer_check_by_range!(1, 9999),
             cycle_check!(),
             |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    b.rule_3("next n <cycle>",
             b.reg(r#"n[äa]chsten?|kommenden?"#)?,
             integer_check_by_range!(1, 9999),
             cycle_check!(),
             |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    b.rule_4("<ordinal> <cycle> of/nach <time>",
             ordinal_check!(),
             cycle_check!(),
             b.reg(r#"im|in(?: de[mr])?|von|nach|de[sr]"#)?,
             datetime_check!(),
             |ordinal, cycle, _, time| helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value - 1, time.value())
    );
    b.rule_4("<ordinal> <cycle> of/nach <time>",
             ordinal_check!(),
             cycle_check!(),
             b.reg(r#"de[sr]"#)?,
             cycle_check!(),
             |ordinal, a, _, b| helpers::cycle_nth_after_not_immediate(a.value().grain, ordinal.value().value - 1, &helpers::cycle_nth(b.value().grain, 0)?)
    );
    b.rule_3("<ordinal> <time> <cycle>",
             ordinal_check!(),
             datetime_check!(),
             cycle_check!(),
             |ordinal, time, cycle| helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value - 1, time.value())
    );
    // TODO wrong production rule
    // b.rule_3("next <month> <cycle>",
    //     b.reg(r#"n[äa]chsten?|kommenden?"#)?,
    //     datetime_check!(form!(Form::Month(_))),
    //     cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Week),
    //     |_, month, cycle| Ok(helpers::cycle(cycle.value().grain)?
    //             .intersect(month.value())?
    //             .latent())
    // );
    b.rule_2("<ordinal> quarter",
             ordinal_check!(),
             cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Quarter),
             |ordinal, _| helpers::cycle_nth_after(Grain::Quarter, ordinal.value().value - 1, &helpers::cycle_nth(Grain::Year, 0)?)
    );
    b.rule_3("<ordinal> quarter <year>",
             ordinal_check!(),
             cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Quarter),
             datetime_check!(),
             |ordinal, _, time| helpers::cycle_nth_after(Grain::Quarter, ordinal.value().value - 1, time.value())
    );
    Ok(())
}

pub fn rules_datetime(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("intersect <time>",
             datetime_check!(|time: &TimeValue| !time.latent && excluding_form!(Form::PartOfDay(_))(time)),
             datetime_check!(|time: &TimeValue| !time.latent && excluding_form!(Form::PartOfDay(_))(time)),
             |a, b| a.value().intersect(b.value())
    );
    b.rule_3("intersect by 'of', 'from', 's",
             datetime_check!(|time: &TimeValue| !time.latent),
             b.reg(r#"von|de(?:r|s|n|m)|im"#)?,
             datetime_check!(|time: &TimeValue| !time.latent),
             |a, _, b| a.value().intersect(b.value())
    );
    b.rule_3("intersect by ','",
             datetime_check!(|time: &TimeValue| !time.latent),
             b.reg(r#","#)?,
             datetime_check!(|time: &TimeValue| !time.latent),
             |a, _, b| a.value().intersect(b.value())
    );
    b.rule_2("during period of time",
             b.reg(r#"im (?:laufe|verlauf)"#)?,
             datetime_check!(),
             |_, time| Ok(time.value().clone())
    );
    b.rule_2("on a named-day",
             b.reg(r#"an einem|an dem"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             |_, time| Ok(time.value().clone())
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"montags?|mo\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Mon)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"die?nstags?|di\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Tue)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"mittwochs?|mi\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Wed)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"donn?erstags?|do\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Thu)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"freitage?s?|fr\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Fri)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"sonnabends?|samstags?|sa\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Sat)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"sonntags?|so\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Sun)
    );
    b.rule_2("the month <named-month>",
             b.reg(r#"de[srnm] monats"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, month| Ok(month.value().clone())
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"januars?|j[äa]nners?|j[äa]n\.?"#)?,
                      |_| helpers::month(1)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"februars?|feb\.?"#)?,
                      |_| helpers::month(2)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"m[äa]rz(?:es)?|m[äa]r\.?"#)?,
                      |_| helpers::month(3)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"aprils?|apr\.?"#)?,
                      |_| helpers::month(4)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"maie?s?"#)?,
                      |_| helpers::month(5)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"junis?|jun\.?"#)?,
                      |_| helpers::month(6)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"julis?|jul\.?"#)?,
                      |_| helpers::month(7)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"auguste?s?|aug\.?"#)?,
                      |_| helpers::month(8)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"septembers?|sept?\.?"#)?,
                      |_| helpers::month(9)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"oktobers?|okt\.?"#)?,
                      |_| helpers::month(10)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"novembers?|nov\.?"#)?,
                      |_| helpers::month(11)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"dezembers?|dez\.?"#)?,
                      |_| helpers::month(12)
    );
    b.rule_1_terminal("christmas",
                      b.reg(r#"christtag|weih?nacht(?:en|s(?:feier)?tag)?"#)?,
                      |_| Ok(helpers::month_day(12, 25)?.form(Form::Celebration))
    );
    b.rule_1_terminal("christmas days (24/12-26/12)",
                      b.reg(r#"weihnachtsfest"#)?,
                      |_| {
                        Ok(helpers::month_day(12, 24)?
                          .span_to(&helpers::month_day(12, 26)?, true)?
                          .form(Form::Celebration))
                      }
    );
    b.rule_1_terminal("christmas eve",
                      b.reg(r#"christnacht|(?:heilig(?:e[r|n])?|weihnachts) ?abend"#)?,
                      |_| Ok(helpers::month_day(12, 24)?.form(Form::Celebration))
    );
    b.rule_1_terminal("new year's eve",
                      b.reg(r#"silvester|neujahrsabend"#)?,
                      |_| Ok(helpers::month_day(12, 31)?.form(Form::Celebration))
    );
    b.rule_1_terminal("new year's day",
                      b.reg(r#"neujahr(?:s?tag)?"#)?,
                      |_| Ok(helpers::month_day(1, 1)?.form(Form::Celebration))
    );
    b.rule_1_terminal("Epiphanias",
                      b.reg(r#"heiligen? drei k[öo]nigen?"#)?,
                      |_| Ok(helpers::month_day(1, 6)?.form(Form::Celebration).too_ambiguous())
    );

    b.rule_1_terminal("Candlemess",
        b.reg(r#"lichtmess"#)?,
        |_| Ok(helpers::month_day(2, 2)?.form(Form::Celebration).too_ambiguous())
    );

    b.rule_1_terminal("rosenmontag (Shrove Monday)",
        b.reg(r#"rosenmontag"#)?,
        |_| Ok(helpers::cycle_nth_after(Grain::Day, -48, &helpers::easter()?)?.form(Form::Celebration)),
    );

    b.rule_1_terminal("fastnachtsdienstag (Shrove Tuesday)",
        b.reg(r#"fastnachtsdienstag"#)?,
        |_| Ok(helpers::cycle_nth_after(Grain::Day, -47, &helpers::easter()?)?.form(Form::Celebration)),
    );

    b.rule_1_terminal("aschermittwoch (Ash Wednesday)",
        b.reg(r#"aschermittwoch"#)?,
        |_| Ok(helpers::cycle_nth_after(Grain::Day, -46, &helpers::easter()?)?.form(Form::Celebration)),
    );

    b.rule_1_terminal("palmsonntag (Palm Sunday)",
        b.reg(r#"palmsonntag"#)?,
        |_| Ok(helpers::cycle_nth_after(Grain::Day, -7, &helpers::easter()?)?.form(Form::Celebration)),
    );
    b.rule_1_terminal("Holy Thursday",
        b.reg(r#"gr[üu]ndonnerstag"#)?,
        |_| Ok(helpers::cycle_nth_after(Grain::Day, -3, &helpers::easter()?)?
                .form(Form::Celebration))
    );
    b.rule_1_terminal("Good Friday",
        b.reg(r#"karfreitag"#)?,
        |_| Ok(helpers::cycle_nth_after(Grain::Day, -2, &helpers::easter()?)?
                .form(Form::Celebration))
    );
    b.rule_1_terminal("Lent",
        b.reg(r#"(?:in|w[aä]hrend) der fastenzeit"#)?,
        |_| Ok(helpers::cycle_nth_after(Grain::Day, -47, &helpers::easter()?)?
                          .span_to(&helpers::easter()?, false)?
                          .form(Form::Celebration))
    );

    b.rule_1_terminal("fasnet",
        b.reg(r#"fast?nacht|(?:in|w[aä]hrend) der fasnet"#)?,
        |_| Ok(helpers::month_day(11, 11)?
                          .span_to(&helpers::cycle_nth_after(Grain::Day, -47, &helpers::easter()?)?, false)?
                          .form(Form::Celebration))
    );
    b.rule_1_terminal("Easter",
        b.reg(r#"oster(?:n|sonntag)"#)?,
        |_| Ok(helpers::easter()?
                .form(Form::Celebration))
    );
    b.rule_1_terminal("Easter Monday",
        b.reg(r#"ostermontag"#)?,
        |_| Ok(helpers::cycle_nth_after(Grain::Day, 1, &helpers::easter()?)?
                .form(Form::Celebration))
    );
    b.rule_1_terminal("ascension",
        b.reg(r#"himmelfahrt|auffahrt"#)?,
        |_| Ok(helpers::cycle_nth_after(Grain::Day, 39, &helpers::easter()?)?
                .form(Form::Celebration))

    );
    b.rule_1_terminal("Pencost",
        b.reg(r#"pfingst(?:en|sonntag|feiertag(?:en)?)"#)?,
        |_| Ok(helpers::cycle_nth_after(Grain::Day, 49, &helpers::easter()?)?
                .form(Form::Celebration))
    );

    b.rule_1_terminal("Pencost Monday",
        b.reg(r#"pfingstmontag"#)?,
        |_| Ok(helpers::cycle_nth_after(Grain::Day, 50, &helpers::easter()?)?
                .form(Form::Celebration))
    );

    b.rule_1_terminal("valentine's day",
                      b.reg(r#"valentin'?stag"#)?,
                      |_| Ok(helpers::month_day(2, 14)?.form(Form::Celebration))
    );
    b.rule_1_terminal("labor day",
                      b.reg(r#"tag der arbeit"#)?,
                      |_| Ok(helpers::month_day(5, 1)?.form(Form::Celebration))
    );
    b.rule_1_terminal("Schweizer Bundesfeiertag",
                      b.reg(r#"schweiz(?:er)? (?:bundes)?feiertag|bundes feiertag"#)?,
                      |_| Ok(helpers::month_day(8, 1)?.form(Form::Celebration))
    );
    b.rule_1_terminal("Augsburg Celebration",
                      b.reg(r#"augsburger hohe[smn] friedensfest"#)?,
                      |_| Ok(helpers::month_day(8, 8)?.form(Form::Celebration))
    );
    b.rule_1_terminal("assumption day",
                      b.reg(r#"mari[äa] himmelfahrt(?:stag)?"#)?,
                      |_| Ok(helpers::month_day(8, 15)?.form(Form::Celebration))
    );
    b.rule_1_terminal("reformation day",
                      b.reg(r#"reformations(?:tag|fest)?"#)?,
                      |_| Ok(helpers::month_day(10, 31)?.form(Form::Celebration))
    );
    b.rule_1_terminal("All saint's day",
                      b.reg(r#"allerheiligen(?:tag)?"#)?,
                      |_| Ok(helpers::month_day(11, 1)?.form(Form::Celebration))
    );
    b.rule_1_terminal("Holy Joseph",
                      b.reg(r#"sankt josef"#)?,
                      |_| Ok(helpers::month_day(3, 19)?.form(Form::Celebration))
    );
    b.rule_1_terminal("Holy Florian",
                      b.reg(r#"sankt florian"#)?,
                      |_| Ok(helpers::month_day(5, 4)?.form(Form::Celebration))
    );
    b.rule_1_terminal("Holy Rupert",
                      b.reg(r#"sankt rupert"#)?,
                      |_| Ok(helpers::month_day(9, 24)?.form(Form::Celebration))
    );
    b.rule_1_terminal("German national celebration",
                      b.reg(r#"tag (?:der )?deutsc?hen? einheit"#)?,
                      |_| Ok(helpers::month_day(10, 3)?.form(Form::Celebration))
    );
    b.rule_1_terminal("Day of popular vote",
                      b.reg(r#"tag der volksabtimmun"#)?,
                      |_| Ok(helpers::month_day(10, 10)?.form(Form::Celebration))
    );
    b.rule_1_terminal("Austrian national celebration",
                      b.reg(r#"(?:[öo]sterreichischer? )?nationalfeiertag|national feiertag"#)?,
                      |_| Ok(helpers::month_day(10, 26)?.form(Form::Celebration))
    );
    b.rule_1_terminal("Armistice Celebration",
                      b.reg(r#"waffenstillstandserkl[äa]rung"#)?,
                      |_| Ok(helpers::month_day(11, 11)?.form(Form::Celebration))
    );
    b.rule_1_terminal("Holy Martin",
                      b.reg(r#"sankt martin|martinstag"#)?,
                      |_| Ok(helpers::month_day(11, 11)?.form(Form::Celebration))
    );
    b.rule_1_terminal("Holy Leopold",
                      b.reg(r#"sankt leopold"#)?,
                      |_| Ok(helpers::month_day(11, 15)?.form(Form::Celebration))
    );
    b.rule_1_terminal("Holy Joseph",
                      b.reg(r#"josefstag"#)?,
                      |_| Ok(helpers::month_day(3, 19)?.form(Form::Celebration))
    );
    b.rule_1_terminal("Switzerland national celebration",
                      b.reg(r#"an der bundesfeier"#)?,
                      |_| Ok(helpers::month_day(8, 1)?.form(Form::Celebration))
    );
    b.rule_1_terminal("Berchtoldstag",
        b.reg(r#"berchtoldstag"#)?,
        |_| Ok(helpers::month_day(1, 2)?.form(Form::Celebration))
    );
    b.rule_1_terminal("Immaculate conception",
                      b.reg(r#"mari[äa] empf[äa]ngnis"#)?,
                      |_| Ok(helpers::month_day(12, 8)?.form(Form::Celebration))
    );
    b.rule_1_terminal("Stephanie's day",
                      b.reg(r#"stefanitag"#)?,
                      |_| Ok(helpers::month_day(12, 26)?.form(Form::Celebration))
    );
    b.rule_1_terminal("Women's day",
                      b.reg(r#"(?:internationale[rnm] )?frauentag"#)?,
                      |_| Ok(helpers::month_day(3, 8)?.form(Form::Celebration))
    );

    b.rule_1("Father's Day",  // third Sunday of June
        b.reg(r#"vatt?er(?: ?tag)?|(?:herren|m[äa]nner)tag"#)?,
        |_| Ok(helpers::cycle_nth_after(Grain::Day, 39, &helpers::easter()?)?
                .form(Form::Celebration))
    );
    b.rule_1_terminal("Mother's Day",
                      b.reg(r#"mutt?ertag|mutt?er (?:tag)?"#)?,
                      |_| Ok(helpers::day_of_week(Weekday::Sun)?
                          .intersect(&helpers::month(5)?)?
                          .intersect(&helpers::cycle_nth_after(Grain::Week, 1, &helpers::month_day(5, 1)?)?)?
                          .form(Form::Celebration))
    );
    b.rule_1_terminal("halloween day",
                      b.reg(r#"hall?owe?en?"#)?,
                      |_| Ok(helpers::month_day(10, 31)?.form(Form::Celebration))
    );
    b.rule_1_terminal("Allerheiligen",
                      b.reg(r#"allerheiligen?|aller heiligen?"#)?,
                      |_| Ok(helpers::month_day(11, 1)?.form(Form::Celebration))
    );
    b.rule_1_terminal("Sunday of the dead (German protestant)",
        b.reg(r#"totensonntag"#)?,
        |_| Ok(helpers::day_of_week(Weekday::Sun)?
                          .intersect(&helpers::cycle_nth_after(Grain::Week, 3, &helpers::month_day(11, 1)?)?)?
                          .form(Form::Celebration))
    );

    b.rule_1_terminal("Nikolaus",
                      b.reg(r#"nikolaus(?: ?tag|abend)?|nikolo"#)?,
                      |_| Ok(helpers::month_day(12, 6)?.form(Form::Celebration))
    );

    b.rule_2("<ordinal> advent sunday",
        ordinal_check_by_range!(1, 4),
        b.reg(r#"advents?"#)?,
        |ordinal, _| {
            let christmas = helpers::month_day(12, 25)?;
            let offset = - (4 - ordinal.value().value + 1);
            Ok(helpers::cycle_nth_after(Grain::Week, offset, &christmas)?
                .intersect(&helpers::day_of_week(Weekday::Sun)?)?
                .form(Form::Celebration))
        }    
    );
    //Volkstrauertag

    b.rule_1_terminal("memorial day",
        b.reg(r#"volkstrauertag"#)?,
        |_| {
            let christmas = helpers::month_day(12, 25)?;
            let offset = -6;
            Ok(helpers::cycle_nth_after(Grain::Week, offset, &christmas)?
                .intersect(&helpers::day_of_week(Weekday::Sun)?)?
                .form(Form::Celebration))
        }    
    );

    b.rule_1_terminal("now",
                      b.reg(r#"(?:genau ?)?jetzt|aktuelle(?:r|n|s|m)?|gegenw[äa]rtige(?:r|n|s|m)?|(?:diesen|im|in diesem) (?:moment|augenblick)|nun|sofort|gerade (?:eben|jetzt)"#)?,
                      |_| helpers::cycle_nth(Grain::Second, 0)
    );
    b.rule_1_terminal("today",
                      b.reg(r#"heute?|(?:um diese |zu dieser |zur|der)zeit|um diesen zeitpunkt|zu diesem zeitpunkt|derzeitig|momentan"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 0)
    );
    b.rule_1_terminal("tomorrow",
                      b.reg(r#"morgen"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 1)
    );
    b.rule_1_terminal("after tomorrow",
                      b.reg(r#"[üu]bermorgen"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 2)
    );
    b.rule_1_terminal("after after tomorrow",
                      b.reg(r#"[üu]ber[üu]bermorgen"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 3)
    );
    b.rule_1_terminal("yesterday",
                      b.reg(r#"gestern"#)?,
                      |_| helpers::cycle_nth(Grain::Day, -1)
    );
    b.rule_1_terminal("before yesterday",
                      b.reg(r#"vorgestern"#)?,
                      |_| helpers::cycle_nth(Grain::Day, -2)
    );
    b.rule_1_terminal("before before yesterday",
                      b.reg(r#"vorvorgestern"#)?,
                      |_| helpers::cycle_nth(Grain::Day, -3)
    );


    b.rule_2("this <time>",
             b.reg(r#"diese[nrsm]?|(?:im )?laufende[nrs]"#)?,
             datetime_check!(excluding_too_ambiguous!()),
             |_, time| time.value().the_nth(0)
    );
    b.rule_2("next <time>",
             b.reg(r#"(?:de[rnms] |die |das )?(?:n[äa]chst|kommend)e[nsrm]?"#)?,
             datetime_check_exclude_too_ambiguous!(),
             |_, time| time.value().the_nth_not_immediate(0)
    );
    b.rule_2("last <time>",
             b.reg(r#"(?:de[rnms] |die |das )?(?:letzt|vor(?:her)?ig|vergangen)e[nsmr]?"#)?,
             datetime_check!(excluding_too_ambiguous!()),
             |_, time| time.value().the_nth(-1)
    );
    b.rule_2("before last <time>",
             b.reg(r#"(?:de[rnms] |die |das )?(?:vorvergangene|vorletzte)[srnm]?"#)?,
             datetime_check!(excluding_too_ambiguous!()),
             |_, time| time.value().the_nth(-2)
    );
    b.rule_2("after next <time>",
             b.reg(r#"(?:de[rnms] |die |das )?[üu]bern[äa]chste[rsnm]"#)?,
             datetime_check!(excluding_too_ambiguous!()),
             |_, time| time.value().the_nth_not_immediate(1)
    );
    b.rule_2("<time> after next",
             datetime_check!(excluding_too_ambiguous!()),
             b.reg(r#"nach de[mrn] n[äa]chsten"#)?,
             |time, _| time.value().the_nth_not_immediate(1)
    );
    b.rule_4("last <day-of-week> of <time>",
             b.reg(r#"(?:de[rnms] |die |das )?letzte[rns]?"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"um|im|in der"#)?,
             datetime_check!(excluding_too_ambiguous!()),
             |_, dow, _, time| dow.value().last_of(time.value())
    );
    b.rule_3(" <day-of-week> <ordinal> <month>",
             datetime_check!(form!(Form::DayOfWeek{..})),
             ordinal_check_by_range!(1, 31),
             datetime_check!(form!(Form::Month(_))),
             |dow, ordinal, month| helpers::month_day(month.value().form_month()?, ordinal.value().value as u32)?
                .intersect(dow.value())
    );
    b.rule_4(" <day-of-week> <ordinal> <month> <year>",
             datetime_check!(form!(Form::DayOfWeek{..})),
             ordinal_check_by_range!(1, 31),
             datetime_check!(form!(Form::Month(_))),
             datetime_check!(|time: &TimeValue| excluding_latent!()(time) && form!(Form::Year(_))(time)),
             |dow, ordinal, month, year| helpers::year(year.value().form_year()?)?.intersect(&helpers::month_day(month.value().form_month()?, ordinal.value().value as u32)?)?
                .intersect(dow.value())
    );
    b.rule_4("last <cycle> of <time>",
             b.reg(r#"letzte(?:r|n|s)?"#)?,
             cycle_check!(),
             b.reg(r#"um|im|in der|des"#)?,
             datetime_check!(),
             |_, cycle, _, time| cycle.value().last_of(time.value())
    );
    b.rule_4("last <cycle> of this <cycle>",
             b.reg(r#"letzte(?:r|n|s)?"#)?,
             cycle_check!(),
             b.reg(german_article_before_cycle())?,
             cycle_check!(),
             |_, a, _, b| a.value().last_of(&helpers::cycle_nth(b.value().grain, 0)?)
    );
    b.rule_3("last <cycle> of <time-cycle>",
             b.reg(r#"letzte(?:r|n|s)?"#)?,
             cycle_check!(),
             datetime_check!(form!(Form::Cycle(_))),
             |_, cycle, time_cycle| cycle.value().last_of(time_cycle.value())
    );
    b.rule_3("last <month> <cycle>",
             b.reg(r#"letzte(?:r|n|s)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Day || cycle.grain == Grain::Week),
             |_, month, cycle| cycle.value().last_of(month.value())
    );
    b.rule_5("the last <cycle> of <time>",
             b.reg(r#"de(?:r|s|n|m)"#)?,
             b.reg(r#"letzte(?:r|n|s)?"#)?,
             cycle_check!(),
             b.reg(r#"um|im|in der"#)?,
             datetime_check!(),
             |_, _, cycle, _, time| cycle.value().last_of(time.value())
    );
    b.rule_4("nth <time> of <time>",
             ordinal_check!(),
             datetime_check!(),
             b.reg(r#"im|in de(?:r|n)|de(?:s|n)"#)?,
             datetime_check!(),
             |ordinal, a, _, b| b.value()
                 .intersect(a.value())?
                 .the_nth(ordinal.value().value - 1)
    );
    b.rule_4("nth <time> after <time>",
             ordinal_check!(),
             datetime_check!(),
             b.reg(r#"nach"#)?,
             datetime_check!(),
             |ordinal, a, _, b| a.value().the_nth_after(ordinal.value().value - 1, b.value())
    );
    b.rule_2("in <month>",
             b.reg(r#"im"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, month| Ok(month.value().clone())
    );
    b.rule_2("in <year>",
             b.reg(r#"im(?: jahre?n?)|in"#)?,
             datetime_check!(form!(Form::Year(_))),
             |_, year| Ok(year.value().clone())
    );
    // das Jahr
    b.rule_1("year",
             integer_check_by_range!(1900, 2100),
             |integer| {
                 helpers::year(integer.value().value as i32)
             }
    );
    b.rule_2("year",
             b.reg(r#"(?:de[rnms]|das|die) jahre?s?n?"#)?,
             integer_check_by_range!(-1000, 2100),
             |_, integer| {
                 helpers::year(integer.value().value as i32)
             }
    );
    b.rule_1("year (latent)",
             integer_check_by_range!(-1000, 1899),
             |integer| {
                 Ok(helpers::year(integer.value().value as i32)?.latent())
             }
    );
    b.rule_1("year (latent)",
             integer_check_by_range!(2101, 2200),
             |integer| {
                 Ok(helpers::year(integer.value().value as i32)?.latent())
             }
    );
    b.rule_2("the <day-of-month> (ordinal)",
             b.reg(r#"de(?:r|n|m|s)"#)?,
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
             |_, ordinal| helpers::day_of_month(ordinal.value().value as u32)
    );

    b.rule_1("<day-of-month> (ordinal)",
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
             |ordinal| Ok(helpers::day_of_month(ordinal.value().value as u32)?.latent())
    );
    // TODO This rule seems to never happen (should be with an ordinal token)
    b.rule_2("the <day-of-month> (non ordinal)",
             b.reg(r#"de(?:r|n|m|s)"#)?,
             integer_check_by_range!(1, 31),
             |_, integer| Ok(helpers::day_of_month(integer.value().value as u32)?.latent())
    );
    b.rule_2("<named-month> <day-of-month> (ordinal)",
             datetime_check!(form!(Form::Month(_))),
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
             |time, ordinal| time.value().intersect(&helpers::day_of_month(ordinal.value().value as u32)?)
    );
    b.rule_3("<named-month> <day-of-month> (non ordinal) <time>",
             datetime_check!(form!(Form::Month(_))),
             integer_check_by_range!(1, 31),
             datetime_check!(),
             |month, integer, time| month.value()
                .intersect(&helpers::day_of_month(integer.value().value as u32)?)?
                .intersect(time.value())
    );
    b.rule_3("<time> <named-month> <day-of-month> (non ordinal)",
             datetime_check!(),
             datetime_check!(form!(Form::Month(_))),
             integer_check_by_range!(1, 31),
             |time, month, integer| month.value()
                .intersect(&helpers::day_of_month(integer.value().value as u32)?)?
                .intersect(time.value())
    );
    b.rule_3("<day-of-week> <named-month> <day-of-month> (non ordinal)",
             datetime_check!(form!(Form::DayOfWeek{..})),
             datetime_check!(form!(Form::Month(_))),
             integer_check_by_range!(1, 31),
             |dow, month, integer| month.value()
                .intersect(&helpers::day_of_month(integer.value().value as u32)?)?
                .intersect(dow.value())
    );
    b.rule_3("<day-of-month> (non ordinal) of <named-month>",
             integer_check_by_range!(1, 31),
             b.reg(r#"vom|von|im"#)?,
             datetime_check!(form!(Form::Month(_))),
             |integer, _, month| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_2("<day-of-month> (non ordinal) <named-month>",
             integer_check_by_range!(1, 31),
             datetime_check!(form!(Form::Month(_))),
             |integer, month| month.value()
                 .intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_4("the <day-of-month> (non ordinal) of <named-month>",
             b.reg(r#"de[rnms]"#)?,
             integer_check_by_range!(1, 31),
             b.reg(r#"vom|von|im"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, integer, _, month| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_3("the <day-of-month> (non ordinal) <named-month>",
             b.reg(r#"de[rnms]"#)?,
             integer_check_by_range!(1, 31),
             datetime_check!(form!(Form::Month(_))),
             |_, integer, month| month.value()
                 .intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_2("<day-of-month>(ordinal) <named-month>",
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
             datetime_check!(form!(Form::Month(_))),
             |ordinal, month| month.value()
                 .intersect(&helpers::day_of_month(ordinal.value().value as u32)?)
    );

    b.rule_3("<day-of-month>(ordinal) <named-month> year",
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
             datetime_check!(form!(Form::Month(_))),
             b.reg(r#"(\d{2,4})"#)?,
             |ordinal, month, text_match| month.value()
                 .intersect(&helpers::day_of_month(ordinal.value().value as u32)?)?
                 .intersect(&helpers::year(text_match.group(1).parse()?)?)
    );
    b.rule_2("the ides of <named-month>",
             b.reg(r#"die iden (des?)"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, month| {
                 let day_of_month = match month.value().form_month()? {
                     3 | 5 | 7 | 10 => 15,
                     _ => 13,
                 };
                 month.value().intersect(&helpers::day_of_month(day_of_month)?)
             }
    );
    b.rule_1("time-of-day (latent)",
             integer_check_by_range!(1, 23),
             |integer| Ok(helpers::hour(integer.value().value as u32, integer.value().value < 12)?.latent())
    );
    b.rule_1("midnight (latent)",
             integer_check_by_range!(24, 24),
             |_| Ok(helpers::hour(0, false)?.latent())
    );
    b.rule_1("midnight (latent)",
             integer_check_by_range!(0, 0),
             |_| Ok(helpers::hour(0, false)?.latent())
    );
    b.rule_2("<time-of-day> o'clock",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"uhr|h|u"#)?,
             |time, _| Ok(time.value().clone().not_latent())
    );
    b.rule_1_terminal("hh:mm",
                      b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:.]([0-5]\d)(?:(?i)uhr|h)?"#)?,
                      |text_match| helpers::hour_minute(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?,
                          false)
    );
    b.rule_2("<time-of-day> am|pm",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"([ap])\.?m\.?"#)?,
             |a, text_match| {
                 let day_period = if text_match.group(1) == "a" {
                     helpers::hour(0, false)?.span_to(&helpers::hour(12, false)?, false)?
                 } else {
                     helpers::hour(12, false)?.span_to(&helpers::hour(0, false)?, false)?
                 };
                 Ok(a.value().intersect(&day_period)?.form(a.value().form.clone()))
             }
    );
    b.rule_1_terminal("noon",
                      b.reg(r#"(?:am )?mittag|zwolf(?: uhr)?"#)?,
                      |_| helpers::hour(12, false)
    );
    b.rule_1_terminal("midnight|end of day",
                      b.reg(r#"mitternacht|tagesende|ende (?:des)? tag(?:es)?"#)?,
                      |_| helpers::hour(0, false)
    );
    b.rule_1_terminal("quarter (relative minutes)",
                      b.reg(r#"vie?rtel"#)?,
                      |_| Ok(RelativeMinuteValue(15))
    );
    b.rule_1_terminal("half (relative minutes)",
                      b.reg(r#"halbe?"#)?,
                      |_| Ok(RelativeMinuteValue(30))
    );
    b.rule_1_terminal("number (as relative minutes for minute=1)",
             b.reg(r#"eins"#)?,
             |_| Ok(RelativeMinuteValue(1))
    );
    b.rule_1("number (as relative minutes for minute>1)",
             integer_check_by_range!(2, 59),
             |integer| Ok(RelativeMinuteValue(integer.value().value as i32))
    );
    b.rule_2("number <minutes> (as relative minutes)",
             integer_check_by_range!(1, 59),
             b.reg(r#"minuten?"#)?,
             |a, _| Ok(RelativeMinuteValue(a.value().value as i32))
    );
    b.rule_3("<hour-of-day> <integer> (as relative minutes)",
             datetime_check!(|time: &TimeValue| !time.latent && form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))(time)),
             b.reg(r#"\s|und"#)?,
             relative_minute_check!(),
             |time, _, relative_minute| helpers::hour_relative_minute(
                 time.value().form_time_of_day()?.full_hour(),
                 relative_minute.value().0,
                 time.value().form_time_of_day()?.is_12_clock())
    );
    b.rule_3("relative minutes to|till|before <integer> (hour-of-day)",
             relative_minute_check!(),
             b.reg(r#"vor"#)?,
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))),
             |relative_minute, _, time| helpers::hour_relative_minute(
                 time.value().form_time_of_day()?.full_hour(),
                 -1 * relative_minute.value().0,
                 time.value().form_time_of_day()?.is_12_clock())
    );
    b.rule_3("relative minutes after|past <integer> (hour-of-day)",
             relative_minute_check!(),
             b.reg(r#"nach"#)?,
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))),
             |relative_minute, _, time| helpers::hour_relative_minute(
                 time.value().form_time_of_day()?.full_hour(),
                 relative_minute.value().0,
                 time.value().form_time_of_day()?.is_12_clock())
    );
    b.rule_2("viertel <integer> (german style hour-of-day)",
             b.reg(r#"vie?rtel"#)?,
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))),
             |_, time| helpers::hour_relative_minute(
                 time.value().form_time_of_day()?.full_hour(),
                 -45,
                 time.value().form_time_of_day()?.is_12_clock())
    );
    b.rule_2("half <integer> (german style hour-of-day)",
             b.reg(r#"halbe?"#)?,
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))),
             |_, time| helpers::hour_relative_minute(
                 time.value().form_time_of_day()?.full_hour(),
                 -30,
                 time.value().form_time_of_day()?.is_12_clock())
    );
    b.rule_2("dreiviertel <integer> (german style hour-of-day)",
             b.reg(r#"dreivie?rtel"#)?,
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))),
             |_, time| helpers::hour_relative_minute(
                 time.value().form_time_of_day()?.full_hour(),
                 -15,
                 time.value().form_time_of_day()?.is_12_clock())
    );
    // Written dates in numeric formats
    b.rule_1_terminal("yyyy-mm-dd - ISO",
                      b.reg(r#"(\d{4})[-/](0?[1-9]|1[0-2])[-/](3[01]|[12]\d|0?[1-9])"#)?,
                      |text_match| helpers::year_month_day(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(3).parse()?)
    );
    b.rule_1_terminal("dd/mm/yy or dd/mm/yyyy",
                      b.reg(r#"(0?[1-9]|[12]\d|3[01])[-\./](0?[1-9]|1[0-2])[-\./](\d{2,4})"#)?,
                      |text_match| helpers::year_month_day(
                          text_match.group(3).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(1).parse()?,
                      )
    );
    // Warning: this pattern matches for months: (1[0-2]|0?[1-9]) but not this one: (0?[1-9]|1[0-2])
    b.rule_1_terminal("dd/mm",
                      b.reg(r#"(0?[1-9]|[12]\d|3[01])[\./](1[0-2]|0?[1-9])\.?"#)?,
                      |text_match| helpers::month_day(
                          text_match.group(2).parse()?,
                          text_match.group(1).parse()?)
    );
    // End of Written dates in numeric formats
    b.rule_1_terminal("breakfast (latent)",
                      b.reg(r#"fr[üu]hst[üu]ck(?:szeit|spause|s)?"#)?,
                      |_| Ok(helpers::hour(6, false)?
                          .span_to(&helpers::hour(9, false)?, false)?
                          .latent()
                          .form(Form::Meal))
    );
    b.rule_1("lunch (latent)",
             b.reg(r#"mittag(?:szeit|pause|essen(?:szeit)?)"#)?,
             |_| Ok(helpers::hour(12, false)?
                 .span_to(&helpers::hour(14, false)?, false)?
                 .latent()
                 .form(Form::Meal))
    );

    b.rule_1_terminal("lunch",
                      b.reg(r#"mittags"#)?,
                      |_| Ok(helpers::hour(12, false)?
                          .span_to(&helpers::hour(14, false)?, false)?
                          .form(Form::Meal))
    );
    b.rule_1_terminal("coffee break",
        b.reg(r#"kaffee"#)?,
        |_| Ok(helpers::hour(15, false)?
                .span_to(&helpers::hour(17, false)?, false)?
                .too_ambiguous()
                .form(Form::Meal))
    );
    b.rule_1_terminal("dinner",
                      b.reg(r#"abendessen(?:szeit)?|abendbrot(?:zeit)?|vesper(?:zeit)?|brotzeit"#)?,
                      |_| Ok(helpers::hour(18, false)?
                          .span_to(&helpers::hour(20, false)?, false)?
                          .latent()
                          .form(Form::Meal))
    );
    b.rule_1_terminal("dawn",
                      b.reg(r#"tagesanbruch|morgengrauen"#)?,
                      |_| Ok(helpers::hour(4, false)?
                          .span_to(&helpers::hour(7, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Morning)).latent())
    );
    b.rule_1_terminal("very early morning",
                      b.reg(r#"fr[üu]h(?:en )?morgens?|am morgen(?: fruh)|fr[üu]he?"#)?,
                      |_| Ok(helpers::hour(4, false)?
                          .span_to(&helpers::hour(9, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Morning)).latent())
    );
    b.rule_1_terminal("very early morning",
                      b.reg(r#"morgens|am morgen"#)?,
                      |_| Ok(helpers::hour(4, false)?
                          .span_to(&helpers::hour(10, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Morning)).latent())
    );
    b.rule_1_terminal("early morning",
                      b.reg(r#"fr[üu]hen vormittag"#)?,
                      |_| Ok(helpers::hour(7, false)?
                          .span_to(&helpers::hour(9, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Morning)).latent())
    );
    b.rule_1_terminal("morning",
                      b.reg(r#"vormittag(?:s(?:zeit)?)?"#)?,
                      |_| Ok(helpers::hour(7, false)?
                          .span_to(&helpers::hour(12, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Morning)).latent())
    );
    b.rule_1_terminal("late morning",
                      b.reg(r#"am sp[äa]ten vor ?mittag"#)?,
                      |_| Ok(helpers::hour(10, false)?
                          .span_to(&helpers::hour(12, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Morning)))
    );
    b.rule_1_terminal("just before noon",
                      b.reg(r#"kurz vor ?mittag"#)?,
                      |_| Ok(helpers::hour(11, false)?
                          .span_to(&helpers::hour(12, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Morning)))
    );
    b.rule_1_terminal("just after noon",
                      b.reg(r#"kurz nach mittag"#)?,
                      |_| Ok(helpers::hour(12, false)?
                          .span_to(&helpers::hour(13, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
    );
    b.rule_1_terminal("after lunch",
                      b.reg(r#"nach dem mittagessen"#)?,
                      |_| Ok(helpers::hour(13, false)?
                          .span_to(&helpers::hour(14, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
    );
    b.rule_1_terminal("early afternoon (latent)",
                      b.reg(r#"fr[üu]hen nachmittags?(?:stunden?)?"#)?,
                      |_| Ok(helpers::hour(13, false)?
                          .span_to(&helpers::hour(16, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
    );
    b.rule_1_terminal("afternoon",
                      b.reg(r#"nach ?mittags?"#)?,
                      |_| Ok(helpers::hour(13, false)?
                          .span_to(&helpers::hour(19, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
    );
    b.rule_1_terminal("late afternoon (latent)",
                      b.reg(r#"sp[äa]t(?:e[nr] )?nachmittags?(?:stunden?)?"#)?,
                      |_| Ok(helpers::hour(16, false)?
                          .span_to(&helpers::hour(19, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
    );
    b.rule_1_terminal("early evening (latent)",
                      b.reg(r#"fr[üu]h am abend|abend fr[üu]h"#)?,
                      |_| Ok(helpers::hour(18, false)?
                          .span_to(&helpers::hour(21, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Evening)))
    );
    b.rule_1_terminal("early evening",
                      b.reg(r#"fr[üu]he[nr] abend(?:stunden?)?|abends fr[üu]h"#)?,
                      |_| Ok(helpers::hour(18, false)?
                          .span_to(&helpers::hour(21, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Evening)))
    );
    b.rule_1_terminal("evening (latent)",
                      b.reg(r#"abend(?:zeit)?"#)?,
                      |_| Ok(helpers::hour(18, false)?
                          .span_to(&helpers::hour(23, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Evening)))
    );
    b.rule_1_terminal("evening",
                      b.reg(r#"abends"#)?,
                      |_| Ok(helpers::hour(18, false)?
                          .span_to(&helpers::hour(23, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Evening)))
    );
    b.rule_1_terminal("late evening (latent)",
                      b.reg(r#"sp[äa]te[nr] abend(?:stunden?)?|abend sp[äa]t"#)?,
                      |_| Ok(helpers::hour(21, false)?
                          .span_to(&helpers::hour(23, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Evening)))
    );
    b.rule_1_terminal("late evening",
                      b.reg(r#"sp[äa]t(?: am )?abends?|abends sp[äa]t|sp[äa]t abends"#)?,
                      |_| Ok(helpers::hour(21, false)?
                          .span_to(&helpers::hour(23, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Evening)))
    );
    b.rule_1_terminal("early night (latent)",
                      b.reg(r#"fr[üu]he[nr]? nacht(?:stunden?)?"#)?,
                      |_| Ok(helpers::hour(21, false)?
                          .span_to(&helpers::hour(0, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Night)))
    );

    b.rule_1_terminal("early night",
                      b.reg(r#"fr[üu]h in der nacht|fr[üu]h nachts"#)?,
                      |_| Ok(helpers::hour(21, false)?
                          .span_to(&helpers::hour(0, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Night)))
    );
    b.rule_1_terminal("night (latent)",
                      b.reg(r#"nacht"#)?,
                      |_| Ok(helpers::hour(23, false)?
                          .span_to(&helpers::hour(5, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Night)))
    );
    b.rule_1_terminal("night",
                      b.reg(r#"nachts"#)?,
                      |_| Ok(helpers::hour(23, false)?
                          .span_to(&helpers::hour(5, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Night)))
    );
    b.rule_1_terminal("deep night",
                      b.reg(r#"tief(?: in der)? ?nachts?"#)?,
                      |_| Ok(helpers::hour(0, false)?
                          .span_to(&helpers::hour(3, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Night)))
    );
    b.rule_1_terminal("late night",
                      b.reg(r#"sp[äa]t(?: in der)? ?nachts?"#)?,
                      |_| Ok(helpers::hour(3, false)?
                          .span_to(&helpers::hour(5, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Night)))
    );

    b.rule_2("<article> <time>",
             b.reg(german_article_regex())?,
             datetime_check!(),
             |_, time| Ok(time.value().clone().not_latent())
    );

    b.rule_2("around <meal/celebration>",
             b.reg("um die")?,
             datetime_check!(|time: &TimeValue| (form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)) && !time.is_too_ambiguous()),
             |_, time| Ok(time.value().clone().precision(Precision::Approximate))
    );

    b.rule_2("<article> <time-of-day>",
             b.reg(r#"a[nm](?: de[rn])?"#)?,
             datetime_check!(excluding_form!(Form::TimeOfDay(_))),
             |_, time| Ok(time.value().clone().not_latent())
    );

    b.rule_2("this <part-of-day>",
             b.reg(r#"diese[snm]?|heute"#)?,
             datetime_check!(|time: &TimeValue| (form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)) && !time.is_too_ambiguous()),
             |_, time| Ok(helpers::cycle_nth(Grain::Day, 0)?
            .intersect(time.value())?
            .form(time.value().form.clone()))
    );
    b.rule_1_terminal("tonight",
        b.reg(r#"heute? (?:am)? abends?"#)?,
        |_| Ok(helpers::cycle_nth(Grain::Day, 0)?
            .intersect(&helpers::hour(18, false)?
                .span_to(&helpers::hour(0, false)?, false)?)?
            .form(Form::PartOfDay(PartOfDayForm::Night)))
    );
    b.rule_1_terminal("after work",
        b.reg(r#"nach (?:der)? arbeit|(?:am)? feier ?abend"#)?,
        |_| Ok(helpers::cycle_nth(Grain::Day, 0)?
            .intersect(&helpers::hour(17, false)?
                .span_to(&helpers::hour(21, false)?, false)?)?
            .form(Form::PartOfDay(PartOfDayForm::Evening)))
    );
    b.rule_2("<time> <part-of-day/meal>", // There are rules for <time-of-day> and <part-of-day>
        datetime_check!(|time: &TimeValue| excluding_form!(Form::Year(_))(time) && excluding_form!(Form::TimeOfDay(_))(time) && excluding_form!(Form::Month(_))(time)),
             datetime_check!(|time: &TimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
             |time, pod| time.value().intersect(pod.value())
    );

    b.rule_3("<time> <meal> <time>", // There are rules for <time-of-day> and <part-of-day>
        datetime_check!(|time: &TimeValue| excluding_form!(Form::Year(_))(time) && excluding_form!(Form::Month(_))(time)),
             datetime_check!(form!(Form::Meal)),
             datetime_check!(|time: &TimeValue| excluding_form!(Form::Year(_))(time) && excluding_form!(Form::Month(_))(time)),
             |a, pod, b| a.value().intersect(b.value())?.intersect(pod.value())
    );
    b.rule_3("<time> <part-of-day> <time-of-day>",
             datetime_check!(|time: &TimeValue| excluding_form!(Form::Year(_))(time) && excluding_form!(Form::Month(_))(time)),
             datetime_check!(form!(Form::PartOfDay(_))),
             datetime_check!(|time: &TimeValue| excluding_form!(Form::Year(_))(time) && excluding_form!(Form::Month(_))(time)),
             |a, pod, b| {
            let pod_form = pod.value().form_part_of_day()?;
            let period = match pod_form {
                PartOfDayForm::Morning => {
                    helpers::hour(1, false)?
                        .span_to(&helpers::hour(12, false)?, true)?
                },
                PartOfDayForm::Afternoon => {
                    helpers::hour(12, false)?
                        .span_to(&helpers::hour(20, false)?, true)?
                },
                PartOfDayForm::Evening => {
                    helpers::hour(17, false)?
                        .span_to(&helpers::hour(23, false)?, true)?
                },
                PartOfDayForm::Night => {
                    helpers::hour(18, false)?
                        .span_to(&helpers::hour(4, false)?, true)?
                },
                PartOfDayForm::None => pod.value().clone()
            };
            a.value().intersect(&period)?.intersect(b.value())
        }
    );

    b.rule_2("<time-of-day> <part-of-day>",
             datetime_check!(form!(Form::TimeOfDay(_))),
             datetime_check!(form!(Form::PartOfDay(_))),
             |tod, pod| {
            let pod_form = pod.value().form_part_of_day()?;
            let period = match pod_form {
                PartOfDayForm::Morning => {
                    helpers::hour(1, false)?
                        .span_to(&helpers::hour(12, false)?, true)?
                },
                PartOfDayForm::Afternoon => {
                    helpers::hour(12, false)?
                        .span_to(&helpers::hour(20, false)?, true)?
                },
                PartOfDayForm::Evening => {
                    helpers::hour(17, false)?
                        .span_to(&helpers::hour(23, false)?, true)?
                },
                PartOfDayForm::Night => {
                    helpers::hour(18, false)?
                        .span_to(&helpers::hour(4, false)?, true)?
                },
                PartOfDayForm::None => pod.value().clone()
            };
            Ok(period.intersect(tod.value())?.form(tod.value().form.clone()))
        }
    );


    b.rule_2("<part-of-day> <time-of-day>",
             datetime_check!(form!(Form::PartOfDay(_))),
             datetime_check!(form!(Form::TimeOfDay(_))),
             |pod, tod| {
            let pod_form = pod.value().form_part_of_day()?;
            let period = match pod_form {
                PartOfDayForm::Morning => {
                    helpers::hour(1, false)?
                        .span_to(&helpers::hour(12, false)?, true)?
                },
                PartOfDayForm::Afternoon => {
                    helpers::hour(12, false)?
                        .span_to(&helpers::hour(20, false)?, true)?
                },
                PartOfDayForm::Evening => {
                    helpers::hour(17, false)?
                        .span_to(&helpers::hour(23, false)?, true)?
                },
                PartOfDayForm::Night => {
                    helpers::hour(18, false)?
                        .span_to(&helpers::hour(4, false)?, true)?
                },
                PartOfDayForm::None => pod.value().clone()
            };
            Ok(period.intersect(tod.value())?.form(tod.value().form.clone()))
        }
    );


    // b.rule_2("<part-of-day/meal> <time>", // There are rules for <time-of-day> and <part-of-day>
    //     datetime_check!(|time: &TimeValue| excluding_form!(Form::Year(_))(time) && excluding_form!(Form::TimeOfDay(_))(time) && excluding_form!(Form::Month(_))(time)),
    //     datetime_check!(|time: &TimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
    //     |pod, time| time.value().intersect(pod.value())
    // );
    b.rule_3("<part-of-day/meal> of <time>", // There are rules for <time-of-day> and <part-of-day>
        datetime_check!(|time: &TimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
             b.reg(r#"de[sr]|vo[nm]|am"#)?,
             datetime_check!(|time: &TimeValue| excluding_form!(Form::Year(_))(time) &&  excluding_form!(Form::TimeOfDay(_))(time) && excluding_form!(Form::Month(_))(time)),
             |pod, _, time| time.value().intersect(pod.value())
    );
    b.rule_1_terminal("week-end",
        b.reg(r#"wochen ?enden?"#)?,
        |_| {
            let friday = helpers::day_of_week(Weekday::Fri)?
                .intersect(&helpers::hour(18, false)?)?;
            let monday = helpers::day_of_week(Weekday::Mon)?
                .intersect(&helpers::hour(0, false)?)?;
            friday.span_to(&monday, false)
        }
    );
    b.rule_1_terminal("season",
                      b.reg(r#"sommer(?:zeit|s)?"#)?,
                      |_| Ok(helpers::month_day(6, 21)?
                          .span_to(&helpers::month_day(9, 23)?, false)?
                          .form(Form::PartOfYear))
    );
    b.rule_1_terminal("Summer solstice",
        b.reg(r#"sommersonn(?:en)?wende"#)?,
        |_| Ok(helpers::month_day(6, 21)?.form(Form::Celebration).too_ambiguous())
    );
    b.rule_1_terminal("season",
                      b.reg(r#"herbst(?:zeit|s|es)?|sp[äa]tjahr(?:es)?"#)?,
                      |_| Ok(helpers::month_day(9, 23)?
                          .span_to(&helpers::month_day(12, 21)?, false)?
                          .form(Form::PartOfYear))
    );
    b.rule_1_terminal("season",
                      b.reg(r#"winter(?:zeit|s)?"#)?,
                      |_| Ok(helpers::month_day(12, 21)?
                          .span_to(&helpers::month_day(3, 20)?, false)?
                          .form(Form::PartOfYear))
    );
    b.rule_1_terminal("Winter solstice",
        b.reg(r#"wintersonnwende"#)?,
        |_| Ok(helpers::month_day(12, 21)?.form(Form::Celebration).too_ambiguous())
    );
    b.rule_1_terminal("season",
                      b.reg(r#"(?:fr[üu]hlings?|fr[üu]hjahr(?:es)?)(?:zeit)?"#)?,
                      |_| Ok(helpers::month_day(3, 20)?
                          .span_to(&helpers::month_day(6, 21)?, false)?
                          .form(Form::PartOfYear))
    );
    b.rule_2("im <part-of-year>",
             b.reg(r#"(?:(?:in )?(?:de[nrms]|die|das)|im|ins)"#)?,
             datetime_check!(form!(Form::PartOfYear)),
             |_, time| Ok(time.value().clone())
    );
    b.rule_2("<time-of-day> approximately",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:um )?zirka|ungef[äa]hr|etwa"#)?,
             |time, _| Ok(time.value().clone().not_latent().precision(Approximate))
    );
    b.rule_2("<time-of-day> exactly",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"genau|exakt|p[üu]nktlich|punkt(?: um)?"#)?,
             |time, _| Ok(time.value().clone().not_latent().precision(Exact))
    );
    b.rule_2("about <time-of-day>",
             b.reg(r#"(?:um )?zirka|ungef[äa]hr|etwa|gegen"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, time| Ok(time.value().clone().not_latent().precision(Approximate))
    );
    b.rule_2("exactly <time-of-day>",
             b.reg(r#"genau|exakt|p[üu]nktlich|punkt(?: um)?"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, time| Ok(time.value().clone().not_latent().precision(Exact))
    );
    b.rule_4("dd-dd (interval) <month>",
             b.reg(r#"([012]?\d|30|31)(?:ter|\.)?"#)?,
             b.reg(r#"\-|bis"#)?,
             b.reg(r#"([012]?\d|30|31)(?:ter|\.)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |d1, _, d2, month| {
                 let start = month.value()
                     .intersect(&helpers::day_of_month(d1.group(1).parse()?)?)?;
                 let end = month.value()
                     .intersect(&helpers::day_of_month(d2.group(1).parse()?)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_5("dd-dd (interval) <month>",
             b.reg(r#"vo[nm]|ab|nach"#)?,
             b.reg(r#"([012]?\d|30|31)(?:ter|\.)?"#)?,
             b.reg(r#"bis(?: zum?r?)?|auf|\-"#)?,
             b.reg(r#"([012]?\d|30|31)(?:ter|\.)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, d1, _, d2, month| {
                 let start = month.value()
                     .intersect(&helpers::day_of_month(d1.group(1).parse()?)?)?;
                 let end = month.value()
                     .intersect(&helpers::day_of_month(d2.group(1).parse()?)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_5("dd-dd (interval) <month>",
             b.reg(r#"vo[nm]|ab|nach"#)?,
             ordinal_check!(|ordinal: &OrdinalValue| ordinal.value >= 1 && ordinal.value <= 31),
             b.reg(r#"bis(?: zum?r?)?|auf|\-"#)?,
             ordinal_check!(|ordinal: &OrdinalValue| ordinal.value >= 1 && ordinal.value <= 31),
             datetime_check!(form!(Form::Month(_))),
             |_, d1, _, d2, month| {
                 let start = month.value()
                     .intersect(&helpers::day_of_month(d1.value().value as u32)?)?;
                 let end = month.value()
                     .intersect(&helpers::day_of_month(d2.value().value as u32)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_3("<datetime> - <datetime> (interval)",
             datetime_check!(|time: &TimeValue| !time.latent),
             b.reg(r#"\-|bis"#)?,
             datetime_check!(|time: &TimeValue| !time.latent),
             |start, _, end| {
                 start.value().span_to(end.value(), true)
             }
    );
    b.rule_4("between <datetime> and <datetime> (interval)",
             b.reg(r#"zwischen"#)?,
             datetime_check!(|time: &TimeValue| !time.latent && excluding_form!(Form::TimeOfDay(_))(time)),
             b.reg(r#"und"#)?,
             datetime_check!(|time: &TimeValue| !time.latent && excluding_form!(Form::TimeOfDay(_))(time)),
             |_, start, _, end| start.value().smart_span_to(end.value(), true)
    );
    b.rule_3("<time-of-day> - <time-of-day> (interval)",
             datetime_check!(|time: &TimeValue| !time.latent && form!(Form::TimeOfDay(_))(time)),
             b.reg(r#"\-|bis"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |start, _, end| start.value().smart_span_to(end.value(), false)
    );
    b.rule_4("from <time> to <time>",
             b.reg(r#"vo[nm]|ab|nach"#)?,
             datetime_check!(|time: &TimeValue| !time.latent),
             b.reg(r#"bis(?: zum?r?)?|auf"#)?,
             datetime_check!(|time: &TimeValue| !time.latent),
             |_, start, _, end| start.value().span_to(end.value(), false)
    );
    b.rule_4("from <time-of-day> - <time-of-day> (interval)",
             b.reg(r#"(?:vo[nm]|nach|ab|(?:fr[üu]h|sp[äa]t)estens(?: um| ab)?)"#)?,
             datetime_check!(|time: &TimeValue| form!(Form::TimeOfDay(_))(time)),
             b.reg(r#"(?:(?:noch|aber|jedoch)? vor)|\-|bis"#)?,
             datetime_check!(|time: &TimeValue| form!(Form::TimeOfDay(_))(time)),
             |_, start, _, end| {
                start.value().smart_span_to(end.value(), false)
            }
    );
    b.rule_4("between <time-of-day> and <time-of-day> (interval)",
             b.reg(r#"zwischen"#)?,
             datetime_check!(|time: &TimeValue| form!(Form::TimeOfDay(_))(time)),
             b.reg(r#"und"#)?,
             datetime_check!(|time: &TimeValue| form!(Form::TimeOfDay(_))(time)),
             |_, start, _, end| start.value().smart_span_to(end.value(), false)
    );
    b.rule_2("within <duration>",
             b.reg(r#"binnen|innerhalb(?: von)?"#)?,
             duration_check!(),
             |_, duration| helpers::cycle_nth(Grain::Second, 0)?
                 .span_to(&duration.value().in_present()?, false)
    );
//    b.rule_2("by the end of <time>",
//             b.reg(r#"bis zum ende"#)?,
//             datetime_check!(excluding_form!(Form::PartOfCycle(_))),
//             |_, time| Ok(time.value().clone().mark_before_end_all())
//    );
    b.rule_2("before <time>",
             b.reg(r#"vor(?: de[nmr]| )|bis(?:(?: zu[rm]?(?: den)?)| in d(?:en|ie|as))?"#)?,
             datetime_check!(excluding_form!(Form::PartOfForm(_))),
             |_, time| Ok(time.value().clone().mark_before_start())
    );

    b.rule_2("before <part-of-form> (specific cases)",
             b.reg(r#"vor(?: de[nmr]| )|bis(?:(?: zu[rm]?(?: den)?)| in d(?:en|ie|as))?"#)?,
             datetime_check!(form!(Form::PartOfForm(_))),
             |_, time| {
                let part_of_form = time.value().form_part_of_form()?;
                match part_of_form.position {
                    Position::Start => Ok(time.value().clone().mark_before_start()),
                    Position::Middle => Ok(time.value().clone().mark_before_start()),
                    Position::End => Ok(time.value().clone().mark_before_end_all()),
                }
              }
    );

    b.rule_2("until inclusive <time>",
             b.reg(r#"sp[äa]testens (?:ins|i[nm]|beim?|w[äa]h?rend|nach) ?(?:de(?:r|m|s|n)|die|das)?"#)?,
             datetime_check!(),
             |_, tod| Ok(tod.value().clone().mark_before_end_all().not_latent())
    );
    b.rule_2("until exclusive <time>",
             b.reg(r#"sp[äa]testens (?:zu[rm]?|vor) ?(?:de(?:r|m|s|n)|die|das)?"#)?,
             datetime_check!(),
             |_, tod| Ok(tod.value().clone().mark_before_start().not_latent())
    );
    b.rule_2("until exclusive <time-of-day>",
             b.reg(r#"kurz vor|sp[äa]testens"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, tod| Ok(tod.value().clone().mark_before_start().not_latent())
    );
    b.rule_2("until inclusive <time> 2",
             b.reg_neg_lh(r#"sp[äa]testens"#, r#"\s?zu[rm]?|vor"#)?,
             datetime_check!(),
             |_, tod| Ok(tod.value().clone().mark_before_end_all().not_latent())
    );
    b.rule_2("after <time>",
             b.reg(r#"nach(?: de[nmr])?"#)?,
             datetime_check!(excluding_form!(Form::TimeOfDay { .. })),
             |_, time| Ok(time.value().clone().mark_after_end_all())
    );
    b.rule_2("after <time-of-day>",
             b.reg(r#"(?:kurz )?nach"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, tod| Ok(tod.value().clone().mark_after_end().not_latent())
    );
    b.rule_1_terminal("start of week",
                      b.reg(r#"(?:de[rnms]|zu )?(anfang|beginn) der woche"#)?,
                      |_| {
                          let current_week = helpers::cycle_nth(Grain::Week, 0)?;
                          let start = current_week.intersect(&helpers::day_of_week(Weekday::Mon)?)?;
                          let end = current_week.intersect(&helpers::day_of_week(Weekday::Tue)?)?;
                          Ok(start.span_to(&end, true)?
                            .form(PartOfForm::start_of(Form::PartOfYear)))
                      }
    );
    b.rule_1_terminal("start of month",
                      b.reg(r#"(?:zu )?(anfang|beginn) des monate?s"#)?,
                      |_| {
                          let current_month = helpers::cycle_nth(Grain::Month, 0)?;
                          let start = current_month.intersect(&helpers::day_of_month(1)?)?;
                          let end = current_month.intersect(&helpers::day_of_month(10)?)?;
                          Ok(start.span_to(&end, true)?
                            .form(PartOfForm::start_of(Form::PartOfYear)))
                      }
    );
    b.rule_1_terminal("start of year",
                      b.reg(r#"jahres(?:anfang|beginn)|(?:anfang|beginn) des jahres"#)?,
                      |_| {
                          let current_year = helpers::cycle_nth(Grain::Year, 0)?;
                          let start = current_year.intersect(&helpers::month(1)?)?;
                          let end = current_year.intersect(&helpers::month(3)?)?;
                          Ok(start.span_to(&end, true)?
                                .form(PartOfForm::start_of(Form::PartOfYear)))
                      }
    );
    b.rule_2("middle of week",
             b.reg(r#"(?:in |im )?(?:der |die )?mitte der"#)?,
             datetime_check!(form!(Form::Cycle(Grain::Week))),
             |_, week| {
                 let start = week.value().intersect(&helpers::day_of_week(Weekday::Fri)?)?;
                 let end = week.value().intersect(&helpers::day_of_week(Weekday::Sun)?)?;
                 Ok(start.span_to(&end, true)?
                    .form(PartOfForm::middle_of(Form::PartOfWeek)))
             }
    );
    b.rule_1_terminal("end of week",
        b.reg(r#"ende der woche"#)?,
        |_| Ok(helpers::day_of_week(Weekday::Thu)
                    ?.span_to(&helpers::day_of_week(Weekday::Sun)?, false)?
                    .form(PartOfForm::end_of(Form::PartOfWeek)))
    );

    b.rule_1_terminal("end of month",
             b.reg(r#"(?:am )?ende (?:des|vom) monate?s?|monatsende"#)?,
        |_| {
            let month = helpers::cycle_nth(Grain::Month, 1)?;
            Ok(helpers::cycle_nth_after(Grain::Day, -10, &month)?
                .span_to(&month, false)?
                .latent()
                .form(PartOfForm::end_of(Form::PartOfMonth)))
        }
    );

    b.rule_1_terminal("end of year",
         b.reg(r#"jahr(?:es)?(?:ende|schluss)|ende (?:des|vom) jahr(?:e?s)?"#)?,
        |_| {
            let current_year = helpers::cycle_nth(Grain::Year, 0)?;
            let start = current_year.intersect(&helpers::month(10)?)?;
            let end = current_year.intersect(&helpers::month(12)?)?;
            Ok(start.span_to(&end, true)?
                .form(PartOfForm::end_of(Form::PartOfYear)))
        }
    );


    b.rule_2("since <time>",
             b.reg(r#"(?:ab|seit)(?: de[rm])?"#)?,
             datetime_check!(),
             |_, time| Ok(time.value().the_nth(-1)?.mark_after_start().not_latent())
    );

    b.rule_3("since <time>",
             b.reg(r#"vo[nm](?: de[rm])?"#)?,
             datetime_check!(),
             b.reg(r#"an"#)?,
             |_, time, _| Ok(time.value().the_nth(-1)?.mark_after_start().not_latent())
             //|_, time, _| Ok(time.value().clone().mark_after_start().not_latent())
    );
    Ok(())
}

pub fn rules_temperature(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("<temperature> plus",
             temperature_check!(|temp: &TemperatureValue| !temp.latent),
             b.reg(r#"plus"#)?,
             |temp, _| Ok(TemperatureValue {
                 value: temp.value().value,
                 unit: temp.value().unit,
                 latent: temp.value().latent,
             })
    );
    b.rule_2("<temperature> minus",
             temperature_check!(|temp: &TemperatureValue| !temp.latent),
             b.reg(r#"minus"#)?,
             |temp, _| Ok(TemperatureValue {
                 value: -1.0 * temp.value().value,
                 unit: temp.value().unit,
                 latent: temp.value().latent,
             })
    );
    b.rule_1("number as temp",
             number_check!(),
             |a| Ok(TemperatureValue {
                 value: a.value().value(),
                 unit: None,
                 latent: true,
             })
    );
    b.rule_2("below <temp>",
             b.reg(r#"minus"#)?,
             temperature_check!(),
             |_, temp| {
                 if temp.value().value >= 0.0 {
                     Ok(TemperatureValue {
                         value: -1.0 * temp.value().value,
                         unit: temp.value().unit,
                         latent: temp.value().latent,
                     })
                 } else {
                     Ok(temp.value().clone())
                 }
             }
    );
    b.rule_2("<temp> below",
             temperature_check!(),
             b.reg(r#"unter(?:m| de[mn])? (?:gefrierpunkt|null|0)"#)?,
             |temp, _| {
                 if temp.value().value >= 0.0 {
                     Ok(TemperatureValue {
                         value: -1.0 * temp.value().value,
                         unit: temp.value().unit,
                         latent: false,
                     })
                 } else {
                     Ok(temp.value().clone())
                 }
             }
    );
    b.rule_2("above <temp>",
             b.reg(r#"plus"#)?,
             temperature_check!(|temp: &TemperatureValue| !temp.latent),
             |_, temp| {
                 if temp.value().value <= 0.0 {
                     Ok(TemperatureValue {
                         value: -1.0 * temp.value().value,
                         unit: temp.value().unit,
                         latent: false,
                     })
                 } else {
                     Ok(temp.value().clone())
                 }
             }
    );
    b.rule_2("<temp> above",
             temperature_check!(),
             b.reg(r#"[üu]ber(?:m| de[mn])? (?:gefrierpunkt|null| 0)"#)?,
             |temp, _| {
                 if temp.value().value <= 0.0 {
                     Ok(TemperatureValue {
                         value: -1.0 * temp.value().value,
                         unit: temp.value().unit,
                         latent: false,
                     })
                 } else {
                     Ok(temp.value().clone())
                 }
             }
    );
    b.rule_2("<latent temp> degrees",
             temperature_check!(),
             b.reg(r#"grade?s?|°"#)?,
             |temp, _| Ok(TemperatureValue {
                 value: temp.value().value,
                 unit: Some("degree"),
                 latent: false,
             })
    );
    b.rule_2("<temp> celsius",
             temperature_check!(),
             b.reg(r#"c(?:elsius)?\.?"#)?,
             |temp, _| Ok(TemperatureValue {
                 value: temp.value().value,
                 unit: Some("celsius"),
                 latent: false,
             })
    );
    b.rule_2("<temp> kelvin",
             temperature_check!(),
             b.reg(r#"k(?:elvin)?"#)?,
             |temp, _| Ok(TemperatureValue {
                 value: temp.value().value,
                 unit: Some("kelvin"),
                 latent: false,
             })
    );
    b.rule_2("<temp> fahrenheit",
             temperature_check!(),
             b.reg(r#"f(?:ah?rh?enh?eit)?"#)?,
             |temp, _| Ok(TemperatureValue {
                 value: temp.value().value,
                 unit: Some("fahrenheit"),
                 latent: false,
             })
    );
    b.rule_2("<temp> °F",
             temperature_check!(),
             b.reg(r#"f"#)?,
             |temp, _| Ok(TemperatureValue {
                 value: temp.value().value,
                 unit: Some("fahrenheit"),
                 latent: false,
             })
    );
    Ok(())
}

pub fn rules_numbers(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("intersect",
             number_check!(|number: &NumberValue| number.grain().unwrap_or(0) > 1),
             number_check!(),
             |a, b| helpers::compose_numbers(&a.value(), &b.value()));

    b.rule_3("intersect",
            number_check!(|number: &NumberValue| number.grain().unwrap_or(0) > 1),
            b.reg(r#"und"#)?,
            number_check!(),
            |a, _, b| helpers::compose_numbers(&a.value(), &b.value()));

    b.rule_1_terminal("null",
                      b.reg(r#"kein(?:er|en|e?s?)|null|nichts"#)?,
                      |_| IntegerValue::new(0)
    );
    b.rule_1_terminal("integer one",
                      b.reg(r#"ein(?:e(?:n|r|s|m)?|s)?"#)?,
                      |_| IntegerValue::new(1)
    );
    b.rule_1_terminal("integer (2..19)",
                      b.reg(r#"(zwei|drei(?:zehn)?|vier(?:zehn)?|f[üu]nf(?:zehn)?|sech(?:s|zehn)|sieb(?:en|zehn)|acht(?:zehn)?|neun(?:zehn)?|elf|zw[öo]lf)"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "zwei" => 2,
                              "drei" => 3,
                              "vier" => 4,
                              "fünf" => 5,
                              "funf" => 5,
                              "sechs" => 6,
                              "sieben" => 7,
                              "acht" => 8,
                              "neun" => 9,
                              "elf" => 11,
                              "zwölf" => 12,
                              "zwolf" => 12,
                              "dreizehn" => 13,
                              "vierzehn" => 14,
                              "fünfzehn" => 15,
                              "funfzehn" => 15,
                              "sechzehn" => 16,
                              "siebzehn" => 17,
                              "achtzehn" => 18,
                              "neunzehn" => 19,
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          IntegerValue::new(value)
                      }
    );
    b.rule_1_terminal("ten",
                      b.reg(r#"zehn"#)?,
                      |_| IntegerValue::new_with_grain(10, 1)
    );
    b.rule_1_terminal("dozen",
                      b.reg(r#"dutzend"#)?,
                      |_| Ok(IntegerValue {
                          value: 12,
                          grain: Some(1),
                          group: true,
                          ..IntegerValue::default()
                      })
    );
    b.rule_1_terminal("hundred",
                      b.reg(r#"hundert"#)?,
                      |_| IntegerValue::new_with_grain(100, 2)
    );
    b.rule_1_terminal("thousand",
                      b.reg(r#"tausend"#)?,
                      |_| IntegerValue::new_with_grain(1_000, 3)
    );
    b.rule_1_terminal("million",
                      b.reg(r#"million(?:en)?"#)?,
                      |_| IntegerValue::new_with_grain(1_000_000, 6)
    );
    b.rule_1_terminal("billion",
                      b.reg(r#"milliarden?"#)?,
                      |_| IntegerValue::new_with_grain(1_000_000, 6)
    );
    b.rule_1_terminal("couple",
                      b.reg(r#"(?:ein )?paar"#)?,
                      |_| IntegerValue::new(2)
    );
    b.rule_1_terminal("few",
                      b.reg(r#"mehrere"#)?,
                      |_| Ok(IntegerValue {
                          value: 3,
                          grain: Some(1),
                          precision: Approximate,
                          ..IntegerValue::default()
                      })
    );
    b.rule_1_terminal("integer (20..90)",
                      b.reg(r#"(zwanzig|drei(?:ss|β|ß)ig|vierzig|f[üu]nfzig|sechzig|siebzig|achtzig|neunzig)"#)?,
                      |text_match| {
                        let value = match text_match.group(1).as_ref() {
                            "zwanzig" => 20,
                            "dreissig" => 30,
                            "dreißig" =>  30,
                            "dreiβig" =>  30,
                            "vierzig" => 40,
                            "funfzig" => 50,
                            "fünfzig" => 50,
                            "sechzig" => 60,
                            "siebzig" => 70,
                            "achtzig" => 80,
                            "neunzig" => 90,
                            _ => return Err(RuleError::Invalid.into()),
                        };
                        IntegerValue::new_with_grain(value, 1)
                      }
    );
    b.rule_3("integer ([2-9][1-9])",
             integer_check_by_range!(1, 9),
             b.reg(r#"und"#)?,
             integer_check_by_range!(10, 90, |integer: &IntegerValue| integer.value % 10 == 0),
             |a, _, b| IntegerValue::new(a.value().value + b.value().value)
    );
    b.rule_1_terminal("integer (numeric)",
                      b.reg(r#"(\d{1,18})"#)?,
                      |text_match| IntegerValue::new(text_match.group(1).parse()?)
    );
    b.rule_1_terminal("integer with thousands separator .",
                      b.reg(r#"(\d{1,3}(\.\d\d\d){1,5})"#)?,
                      |text_match| IntegerValue::new(text_match.group(1).replace(".", "").parse()?)
    );

    b.rule_2("number hundreds",
        integer_check_by_range!(1, 99),
        b.reg(r#"hundert"#)?,
        |a, _| Ok(IntegerValue {
            value: a.value().value * 100,
            grain: Some(2),
            ..IntegerValue::default()
        })
    );

    b.rule_2("number thousands",
        integer_check_by_range!(1, 999),
        b.reg(r#"tausend"#)?,
        |a, _| Ok(IntegerValue {
            value: a.value().value * 1_000,
            grain: Some(3),
            ..IntegerValue::default()
        })
    );

    b.rule_2("number millions",
        integer_check_by_range!(1, 999),
         b.reg(r#"million(?:en)?"#)?,
        |a, _| Ok(IntegerValue {
            value: a.value().value * 1_000_000,
            grain: Some(6),
            ..IntegerValue::default()
        })
    );
    b.rule_2("number billion",
        integer_check_by_range!(1, 999),
         b.reg(r#"milliarden?"#)?,
        |a, _| Ok(IntegerValue {
            value: a.value().value * 1_000_000_000,
            grain: Some(9),
            ..IntegerValue::default()
        })
    );
    b.rule_1_terminal("decimal number",
                      b.reg(r#"(\d*,\d+)"#)?,
                      |text_match| FloatValue::new(text_match.group(1).replace(",", ".").parse()?)
    );
    b.rule_3("number dot number",
             number_check!(|number: &NumberValue| !number.prefixed()),
             b.reg(r#"komma"#)?,
             number_check!(|number: &NumberValue| !number.suffixed()),
             |a, _, b| {
                 let power = b.value().value().to_string().chars().count();
                 let coeff = 10.0_f32.powf(-1.0 * power as f32);
                 Ok(FloatValue {
                     value: b.value().value() * coeff + a.value().value(),
                     ..FloatValue::default()
                 })
             }
    );
    b.rule_4("number dot zero ... number",
             number_check!(|number: &NumberValue| !number.prefixed()),
             b.reg(r#"komma"#)?,
             b.reg(r#"(?:(?:null )*(?:null))"#)?,
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
                      |text_match| FloatValue::new(text_match.group(1).replace(".", "").replace(",", ".").parse()?)
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
    b.rule_1_terminal("ordinals (first..19th)",
                      b.reg(r#"(nullte|erste|zweite|dritte|vierte|f[üu]nfte|sechste|sieb(?:en)?te|achte|neunte|zehnte|elfte|zw[öo]lfte|dreizehnte|vierzehnte|f[üu]nfzehnte|sechzehnte|siebzehnte|achtzehnte|neunzehnte)[rsnm]?"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "nullte" => 0,
                              "erste" => 1,
                              "zweite" => 2,
                              "dritte" => 3,
                              "vierte" => 4,
                              "funfte" => 5,
                              "fünfte" => 5,
                              "sechste" => 6,
                              "siebte" => 7,
                              "siebente" => 7,
                              "achte" => 8,
                              "neunte" => 9,
                              "zehnte" => 10,
                              "elfte" => 11,
                              "zwölfte" => 12,
                              "zwolfte" => 12,
                              "dreizehnte" => 13,
                              "vierzehnte" => 14,
                              "fünfzehnte" => 15,
                              "funfzehnte" => 15,
                              "sechzehnte" => 16,
                              "siebzehnte" => 17,
                              "achtzehnte" => 18,
                              "neunzehnte" => 19,
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          Ok(OrdinalValue::new(value))
                      }
    );
    b.rule_1_terminal("ordinal (digits)",
                      b.reg(r#"0*(\d+)(?:\.| ?(?:te(?:n|r|s)?)|(?:ste(?:n|r|s)?))"#)?,
                      |text_match| Ok(OrdinalValue::new(text_match.group(1).parse()?))
    );
    b.rule_2("der <ordinal>",
             b.reg(r#"de[rsnm]|das|die"#)?,
             ordinal_check!(),
             |_, ordinal| Ok(ordinal.value().clone().prefixed())
    );
    b.rule_1_terminal("ordinal (20..90)",
                      b.reg(r#"(zwanzigste|drei(?:ss|β|ß)igste|vierzigste|f[üu]nfzigste|sechzigste|siebzigste|achtzigste|neunzigste)(?:r|n|m|s)?"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "zwanzigste" => 20,
                              "dreissigste" => 30,
                              "dreiβigste" => 30,
                              "dreißigste" => 30,
                              "vierzigste" => 40,
                              "funfzigste" => 50,
                              "fünfzigste" => 50,
                              "sechzigste" => 60,
                              "siebzigste" => 70,
                              "achtzigste" => 80,
                              "neunzigste" => 90,
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          Ok(OrdinalValue::new(value))
                      }
    );
    b.rule_2("ordinal (200..900, 2_000..9_000, 2_000_000..9_000_000_000)",
        integer_check_by_range!(1, 999),
        b.reg(r#"(hundert|tausend|million|milliard)ste[rnms]?"#)?,
        |integer, text_match| {
            let (value, grain) = match text_match.group(1).as_ref() {
                "hundert" => (100, 2),
                "tausend" => (1_000, 3),
                "million" => (1_000_000, 6),
                "milliard" => (1_000_000_000, 9),
                _ => return Err(RuleError::Invalid.into()),
            };
            Ok(OrdinalValue::new_with_grain(integer.value().value * value, grain))
        }
    );
    b.rule_1_terminal("ordinal (100, 1_000, 1_000_000)",
        b.reg(r#"(hundert|tausend|million|milliard)ste[rnms]?"#)?,
        |text_match| {
            let (value, grain) = match text_match.group(1).as_ref() {
                "hundert" => (100, 2),
                "tausend" => (1_000, 3),
                "million" => (1_000_000, 6),
                "milliard" => (1_000_000_000, 9),
                _ => return Err(RuleError::Invalid.into()),
            };
            Ok(OrdinalValue::new_with_grain(value, grain))
        }
    );
    b.rule_3("ordinal [2-9][1-9]",
             integer_check_by_range!(1, 9),
             b.reg(r#"und"#)?,
             ordinal_check!(|ordinal: &OrdinalValue| ordinal.value % 10 == 0),
             |integer, _, ordinal| Ok(OrdinalValue::new(integer.value().value + ordinal.value().value))
    );
    b.rule_2("ordinal (102...9_999_999)",
        integer_check!(|integer: &IntegerValue| integer.value >= 100 || integer.value % 100 == 0),
        ordinal_check_by_range!(2, 99),
        |integer, ordinal| {
            Ok(OrdinalValue::new(integer.value().value + ordinal.value().value))
        }
    );
    b.rule_3("ordinal (102...9_999_999)",
        integer_check!(|integer: &IntegerValue| integer.value >= 100 || integer.value % 100 == 0),
        b.reg(r#"und"#)?,
        ordinal_check_by_range!(2, 99),
        |integer, _, ordinal| {
            Ok(OrdinalValue::new(integer.value().value + ordinal.value().value))
        }
    );
    b.rule_2("ordinal (1_1_000..9_999_999_000)",
        integer_check_by_range!(1000, 99_999_999_000),
        ordinal_check!(|ordinal: &OrdinalValue| {
            let grain = ordinal.grain.unwrap_or(0);
            grain == 2 || (grain % 3 == 0 && grain != 0)
        }),
        |integer, ordinal| {
            let grain = ordinal.value().grain.unwrap_or(0);
            let next_grain = (grain / 3) * 3 + 3;
            if integer.value().value % 10i64.pow(next_grain as u32) != 0 { return Err(RuleError::Invalid.into()); }
            Ok(OrdinalValue::new(integer.value().value + ordinal.value().value))
        }
    );
    b.rule_2("ordinal (101, 201, 301, ...)",
        integer_check!(|integer: &IntegerValue| integer.value >= 100 || integer.value % 100 == 0),
        b.reg(r#"(?:und )?erste[rnms]?"#)?,
        |integer, _| {
            Ok(OrdinalValue::new(integer.value().value + 1))
        }
    );
    Ok(())
}
