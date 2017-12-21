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
        b.reg(r"(?:%|prozent)")?,
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
    b.rule_1_terminal("cent",
                      b.reg(r#"cents?|penn(?:y|ies)|cts?|c|¢"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("cent") })
    );
    b.rule_1_terminal("₩",
                      b.reg(r#"₩|krw|(?:s[üu]dkoreanische[rnms]? )?won"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("KRW") })
    );
    b.rule_1_terminal("$",
                      b.reg(r#"\$|dollar"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("$") })
    );
    b.rule_1_terminal("€",
                      b.reg(r#"€|euro?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("EUR") })
    );
    b.rule_1_terminal("£",
                      b.reg(r#"£|pfund sterling|pfund|pfd."#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("£") })
    );
    b.rule_1_terminal("GBP",
                      b.reg(r#"gbp"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("GBP") })
    );
    b.rule_1_terminal("AUD",
                      b.reg(r#"aud|australische[rnms]? dollar"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("AUD") })
    );
    b.rule_1_terminal("USD",
                      b.reg(r#"us[d\$]|us[ -]dollar"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("USD") })
    );
    b.rule_1_terminal("PTS",
                      b.reg(r#"pta?s?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("PTS") })
    );
    b.rule_1_terminal("INR",
                      b.reg(r#"inr|₹|(?:indische[rn]? )?rupien?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("INR") })
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
             b.reg(r#"[cz]irka|nahezu|beinahe|ungef[äa]hr|fast|ca\.?"#)?,
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
                      b.reg(r#"sek(?:unden?|\.?)|s.|sec"#)?,
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
                     _ => return Err(RuleErrorKind::Invalid.into()),
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
    b.rule_2("in next <unit-of-duration>",
             b.reg(r#"in de(?:n|r|m) (?:n[äa]chste(?:n|r|m)|kommende(?:r|n|m))"#)?,
             unit_of_duration_check!(),
             |_, uod| DurationValue::new(PeriodComp::new(uod.value().grain, 1).into())
                 .in_present()
    );
    b.rule_2("in <duration>",
             b.reg(r#"in(?:\s(?:de(?:n|r|m)\s)?(?:n[äa]chste(?:n|r|m)|kommende(?:r|n|m)))?"#)?,
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
             b.reg(r#"ab (?:heute|jetzt|sofort)"#)?,
             |duration, _| duration.value().in_present()
    );
    b.rule_2("<duration> from now",
             b.reg(r#"von (?:heute|jetzt|sofort) an"#)?,
             duration_check!(),
             |_, duration| duration.value().in_present()
    );
    b.rule_3("in <duration> from now",
             b.reg(r#"in"#)?,
             duration_check!(),
             b.reg(r#"ab (?:heute|jetzt|sofort)"#)?,
             |_, duration, _| duration.value().in_present()
    );
    b.rule_2("<duration> ago",
             b.reg(r#"vor"#)?,
             duration_check!(),
             |_, duration| duration.value().ago()
    );
    // TODO wrong production rules output
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
             duration_check!(),
             b.reg(r#"und"#)?,
             duration_check!(),
             |a, _, b| Ok(a.value() + b.value())
    );
    b.rule_2("<duration> <duration>",
             duration_check!(),
             duration_check!(),
             |a, b| Ok(a.value() + b.value())
    );
    Ok(())
}

pub fn rules_cycle(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("second (cycle)",
                      b.reg(r#"sekunden?"#)?,
                      |_| CycleValue::new(Grain::Second)
    );
    b.rule_1_terminal("minute (cycle)",
                      b.reg(r#"minuten?"#)?,
                      |_| CycleValue::new(Grain::Minute)
    );
    b.rule_1_terminal("hour (cycle)",
                      b.reg(r#"stunden?"#)?,
                      |_| CycleValue::new(Grain::Hour)
    );
    b.rule_1_terminal("day (cycle)",
                      b.reg(r#"tage?n?s?"#)?,
                      |_| CycleValue::new(Grain::Day)
    );
    b.rule_1_terminal("week (cycle)",
                      b.reg(r#"wochen?"#)?,
                      |_| CycleValue::new(Grain::Week)
    );
    b.rule_1_terminal("month (cycle)",
                      b.reg(r#"monate?n?s?"#)?,
                      |_| CycleValue::new(Grain::Month)
    );
    b.rule_1_terminal("quarter (cycle)",
                      b.reg(r#"quartale?"#)?,
                      |_| CycleValue::new(Grain::Quarter)
    );
    b.rule_1_terminal("year (cycle)",
                      b.reg(r#"jahre?n?s?"#)?,
                      |_| CycleValue::new(Grain::Year)
    );
    b.rule_2("this <cycle>",
             b.reg(r#"(?:in )?diese(?:r|n|s|m)?|de[sr]"#)?,
             cycle_check!(),
             |_, cycle| helpers::cycle_nth(cycle.value().grain, 0)
    );
    b.rule_2("last <cycle>",
             b.reg(r#"letzte(?:r|n|s)?|vergangene(?:r|n|s)?|vor(?:her)?ige(?:r|n|s)?"#)?,
             cycle_check!(),
             |_, cycle| helpers::cycle_nth(cycle.value().grain, -1)
    );
    b.rule_2("before last <cycle>",
             b.reg(r#"vorvergangene[rnm]?|vorletzte[rnm]?"#)?,
             cycle_check!(),
             |_, cycle| helpers::cycle_nth(cycle.value().grain, -2)
    );
    b.rule_2("next <cycle>",
             b.reg(r#"n[äa]chste(?:r|n|s)?|kommende(?:r|n|s)?"#)?,
             cycle_check!(),
             |_, cycle| helpers::cycle_nth(cycle.value().grain, 1)
    );
    b.rule_2("after next <cycle>",
             b.reg(r#"[üu]bern[äa]chste(?:r|s|n|m)?"#)?,
             cycle_check!(),
             |_, cycle| helpers::cycle_nth(cycle.value().grain, 2)
    );
    b.rule_4("the <cycle> after <time>",
             b.reg(german_article_before_cycle())?,
             cycle_check!(),
             b.reg(r#"nach"#)?,
             time_check!(),
             |_, cycle, _, time| helpers::cycle_nth_after(cycle.value().grain, 1, time.value())
    );
    b.rule_4("the <cycle> before <time>",
             b.reg(german_article_before_cycle())?,
             cycle_check!(),
             b.reg(r#"vor"#)?,
             time_check!(),
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
             time_check!(),
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
             time_check!(),
             cycle_check!(),
             |ordinal, time, cycle| helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value - 1, time.value())
    );
    // TODO wrong production rule
    // b.rule_3("next <month> <cycle>",
    //     b.reg(r#"n[äa]chsten?|kommenden?"#)?,
    //     time_check!(form!(Form::Month(_))),
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
             time_check!(),
             |ordinal, _, time| helpers::cycle_nth_after(Grain::Quarter, ordinal.value().value - 1, time.value())
    );
    Ok(())
}

pub fn rules_time(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("intersect",
             time_check!(|time: &TimeValue| !time.latent),
             time_check!(|time: &TimeValue| !time.latent),
             |a, b| a.value().intersect(b.value())
    );
    b.rule_3("intersect by 'of', 'from', 's",
             time_check!(|time: &TimeValue| !time.latent),
             b.reg(r#"von|de(?:r|s|n|m)|im"#)?,
             time_check!(|time: &TimeValue| !time.latent),
             |a, _, b| a.value().intersect(b.value())
    );
    b.rule_3("intersect by ','",
             time_check!(|time: &TimeValue| !time.latent),
             b.reg(r#","#)?,
             time_check!(|time: &TimeValue| !time.latent),
             |a, _, b| a.value().intersect(b.value())
    );
    b.rule_2("during period of time",
             b.reg(r#"im (?:laufe|verlauf)"#)?,
             time_check!(),
             |_, time| Ok(time.value().clone())
    );
    b.rule_2("on a named-day",
             b.reg(r#"an einem|an dem"#)?,
             time_check!(form!(Form::DayOfWeek{..})),
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
             time_check!(form!(Form::Month(_))),
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
                      b.reg(r#"weih?nacht(?:en|s(?:feier)?tag)?"#)?,
                      |_| helpers::month_day(12, 25)
    );
    b.rule_1_terminal("christmas eve",
                      b.reg(r#"heilig(er)? abend"#)?,
                      |_| helpers::month_day(12, 24)
    );
    b.rule_1_terminal("three wise men",
        b.reg(r#"den heiligen drei k[öo]nigen"#)?,
        |_| helpers::month_day(1, 6)
    );
    b.rule_1_terminal("new year's eve",
                      b.reg(r#"silvester|neujahrsabend"#)?,
                      |_| helpers::month_day(12, 31)
    );
    b.rule_1_terminal("new year's day",
                      b.reg(r#"neujahr(?:s?tag)?"#)?,
                      |_| helpers::month_day(1, 1)
    );
    b.rule_1_terminal("Epiphanias",
                      b.reg(r#"heilige drei k[öo]nige "#)?,
                      |_| helpers::month_day(1, 6)
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

    b.rule_1_terminal("Lent",
        b.reg(r#"(?:in|w[aä]hrend) der fastenzeit"#)?,
        |_| Ok(helpers::cycle_nth_after(Grain::Day, -47, &helpers::easter()?)?
                          .span_to(&helpers::easter()?, false)?
                          .form(Form::Celebration))
    );

    b.rule_1_terminal("fasnet",
        b.reg(r#"(?:in|w[aä]hrend) der fasnet"#)?,
        |_| Ok(helpers::month_day(11, 11)?
                          .span_to(&helpers::cycle_nth_after(Grain::Day, -47, &helpers::easter()?)?, false)?
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
    b.rule_1_terminal("Saint Joseph",
                      b.reg(r#"sankt josef"#)?,
                      |_| Ok(helpers::month_day(3, 19)?.form(Form::Celebration))
    );
    b.rule_1_terminal("Saint Florian",
                      b.reg(r#"sankt florian"#)?,
                      |_| Ok(helpers::month_day(5, 4)?.form(Form::Celebration))
    );
    b.rule_1_terminal("Saint Rupert",
                      b.reg(r#"sankt rupert"#)?,
                      |_| Ok(helpers::month_day(9, 24)?.form(Form::Celebration))
    );
    b.rule_1_terminal("German national celebration",
                      b.reg(r#"tag (?:der)? deutsc?hen? einheit"#)?,
                      |_| Ok(helpers::month_day(10, 3)?.form(Form::Celebration))
    );
    b.rule_1_terminal("Day of popular vote",
                      b.reg(r#"tag der volksabtimmun"#)?,
                      |_| Ok(helpers::month_day(10, 10)?.form(Form::Celebration))
    );
    b.rule_1_terminal("Austrian national celebration",
                      b.reg(r#"([öo]sterreichischer?)? nationalfeiertag|national feiertag"#)?,
                      |_| Ok(helpers::month_day(10, 26)?.form(Form::Celebration))
    );
    b.rule_1_terminal("Armistice Celebration",
                      b.reg(r#"waffenstillstandserkl[äa]rung"#)?,
                      |_| Ok(helpers::month_day(11, 11)?.form(Form::Celebration))
    );
    b.rule_1_terminal("Saint Martin",
                      b.reg(r#"sankt martin"#)?,
                      |_| Ok(helpers::month_day(11, 11)?.form(Form::Celebration))
    );
    b.rule_1_terminal("Saint Leopold",
                      b.reg(r#"sankt leopold"#)?,
                      |_| Ok(helpers::month_day(11, 15)?.form(Form::Celebration))
    );
    b.rule_1_terminal("Switzerland national celebration",
                      b.reg(r#"an der bundesfeier"#)?,
                      |_| Ok(helpers::month_day(8, 1)?.form(Form::Celebration))
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
    // TODO needs the lunar calendar feature
    // b.rule_1("Ascension celebration",
    //     b.reg(r#"himmelfahrt"#)?,
    //     |_| 
    // );

    // TODO in Germany it is the same day as the ascension celebration
    // b.rule_1("Father's Day",  // third Sunday of June
    //     b.reg(r#"vatt?er(?: ?tag)?|(?:herren|m[äa]nner)tag"#)?,
    //     |_| helpers::day_of_week(Weekday::Sun)?
    //             .intersect(&helpers::month(6)?)?
    //             .intersect(&helpers::cycle_nth_after(Grain::Week, 2, &helpers::month_day(6, 1)?)?)
    // );
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
    b.rule_1_terminal("Nikolaus",
                      b.reg(r#"nikolaus(?: ?tag|abend)?|nikolo"#)?,
                      |_| Ok(helpers::month_day(12, 6)?.form(Form::Celebration))
    );
    // b.rule_2("absorption of , after named day",
    //     time_check!(form!(Form::DayOfWeek{..})),
    //     b.reg(r#","#)?,
    //     |time, _| Ok(time.value().clone())
    // );

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

    b.rule_1_terminal("now",
                      b.reg(r#"(?:genau ?)?jetzt|(?:diesen|im|in diesem) (?:moment|augenblick)|nun|sofort|gerade (?:eben|jetzt)"#)?,
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
             b.reg(r#"diese(?:n|r|s|m)?|(?:im )?laufende(?:n|r|s)"#)?,
             time_check!(),
             |_, time| time.value().the_nth(0)
    );
    b.rule_2("next <time>",
             b.reg(r#"(?:n[äa]chst|kommend)e(?:n|s|r|m)?"#)?,
             time_check!(),
             |_, time| time.value().the_nth_not_immediate(0)
    );
    b.rule_2("last <time>",
             b.reg(r#"(?:letzt|vor(?:her)?ig)e(?:n|s|m|r)?"#)?,
             time_check!(),
             |_, time| time.value().the_nth(-1)
    );
    b.rule_2("before last <time>",
             b.reg(r#"vorvergangene[rnm]?|vorletzte[rnm]?"#)?,
             time_check!(),
             |_, time| time.value().the_nth(-2)
    );
    b.rule_2("after next <time>",
             b.reg(r#"[üu]bern[äa]chste(?:r|s|n|m)?"#)?,
             time_check!(),
             |_, time| time.value().the_nth_not_immediate(1)
    );
    b.rule_2("<time> after next",
             time_check!(),
             b.reg(r#"nach de(?:m|r|n) n[äa]chsten"#)?,
             |time, _| time.value().the_nth_not_immediate(1)
    );
    b.rule_4("last <day-of-week> of <time>",
             b.reg(r#"letzte(?:r|n|s)?"#)?,
             time_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"um|im|in der"#)?,
             time_check!(),
             |_, dow, _, time| dow.value().last_of(time.value())
    );
    b.rule_3(" <day-of-week> <ordinal> <month>",
             time_check!(form!(Form::DayOfWeek{..})),
             ordinal_check_by_range!(1, 31),
             time_check!(form!(Form::Month(_))),
             |dow, ordinal, month| helpers::month_day(month.value().form_month()?, ordinal.value().value as u32)?
                .intersect(dow.value())
    );
    b.rule_4(" <day-of-week> <ordinal> <month> <year>",
            time_check!(form!(Form::DayOfWeek{..})),
            ordinal_check_by_range!(1, 31),
            time_check!(form!(Form::Month(_))),
            time_check!(|time: &TimeValue| !time.latent && form!(Form::Year(_))(time)),
             |dow, ordinal, month, year| helpers::year(year.value().form_year()?)?.intersect(&helpers::month_day(month.value().form_month()?, ordinal.value().value as u32)?)?
                .intersect(dow.value())
    );
    b.rule_4("last <cycle> of <time>",
             b.reg(r#"letzte(?:r|n|s)?"#)?,
             cycle_check!(),
             b.reg(r#"um|im|in der|des"#)?,
             time_check!(),
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
             time_check!(form!(Form::Cycle(_))),
             |_, cycle, time_cycle| cycle.value().last_of(time_cycle.value())
    );
    b.rule_3("last <month> <cycle>",
             b.reg(r#"letzte(?:r|n|s)?"#)?,
             time_check!(form!(Form::Month(_))),
             cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Day || cycle.grain == Grain::Week),
             |_, month, cycle| cycle.value().last_of(month.value())
    );
    b.rule_5("the last <cycle> of <time>",
             b.reg(r#"de(?:r|s|n|m)"#)?,
             b.reg(r#"letzte(?:r|n|s)?"#)?,
             cycle_check!(),
             b.reg(r#"um|im|in der"#)?,
             time_check!(),
             |_, _, cycle, _, time| cycle.value().last_of(time.value())
    );
    b.rule_4("nth <time> of <time>",
             ordinal_check!(),
             time_check!(),
             b.reg(r#"im|in de(?:r|n)|de(?:s|n)"#)?,
             time_check!(),
             |ordinal, a, _, b| b.value()
                 .intersect(a.value())?
                 .the_nth(ordinal.value().value - 1)
    );
    b.rule_4("nth <time> after <time>",
             ordinal_check!(),
             time_check!(),
             b.reg(r#"nach"#)?,
             time_check!(),
             |ordinal, a, _, b| a.value().the_nth_after(ordinal.value().value - 1, b.value())
    );
    b.rule_2("in <month>",
             b.reg(r#"im"#)?,
             time_check!(form!(Form::Month(_))),
             |_, month| Ok(month.value().clone())
    );
    b.rule_2("in <year>",
             b.reg(r#"im(?: jahre?n?)|in"#)?,
             time_check!(form!(Form::Year(_))),
             |_, year| Ok(year.value().clone())
    );
    b.rule_1("year",
             integer_check_by_range!(1900, 2100),
             |integer| {
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
             time_check!(form!(Form::Month(_))),
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
             |time, ordinal| time.value().intersect(&helpers::day_of_month(ordinal.value().value as u32)?)
    );
    b.rule_2("<named-month> <day-of-month> (non ordinal)",
             time_check!(form!(Form::Month(_))),
             integer_check_by_range!(1, 31),
             |time, integer| time.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_3("<day-of-month> (non ordinal) of <named-month>",
             integer_check_by_range!(1, 31),
             b.reg(r#"vom|von|im"#)?,
             time_check!(form!(Form::Month(_))),
             |integer, _, month| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_2("<day-of-month> (non ordinal) <named-month>",
             integer_check_by_range!(1, 31),
             time_check!(form!(Form::Month(_))),
             |integer, month| month.value()
                 .intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_4("the <day-of-month> (non ordinal) of <named-month>",
             b.reg(r#"de[rnms]"#)?,
             integer_check_by_range!(1, 31),
             b.reg(r#"vom|von|im"#)?,
             time_check!(form!(Form::Month(_))),
             |_, integer, _, month| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_3("the <day-of-month> (non ordinal) <named-month>",
             b.reg(r#"de[rnms]"#)?,
             integer_check_by_range!(1, 31),
             time_check!(form!(Form::Month(_))),
             |_, integer, month| month.value()
                 .intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_2("<day-of-month>(ordinal) <named-month>",
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
             time_check!(form!(Form::Month(_))),
             |ordinal, month| month.value()
                 .intersect(&helpers::day_of_month(ordinal.value().value as u32)?)
    );

    b.rule_3("<day-of-month>(ordinal) <named-month> year",
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
             time_check!(form!(Form::Month(_))),
             b.reg(r#"(\d{2,4})"#)?,
             |ordinal, month, text_match| month.value()
                 .intersect(&helpers::day_of_month(ordinal.value().value as u32)?)?
                 .intersect(&helpers::year(text_match.group(1).parse()?)?)
    );
    b.rule_2("the ides of <named-month>",
             b.reg(r#"die iden (des?)"#)?,
             time_check!(form!(Form::Month(_))),
             |_, month| {
                 let day_of_month = match month.value().form_month()? {
                     3 | 5 | 7 | 10 => 15,
                     _ => 13,
                 };
                 month.value().intersect(&helpers::day_of_month(day_of_month)?)
             }
    );
    b.rule_1("time-of-day (latent)",
             integer_check_by_range!(0, 23),
             |integer| Ok(helpers::hour(integer.value().value as u32, integer.value().value < 12)?.latent())
    );
    b.rule_1("midnight (latent)",
             integer_check_by_range!(24, 24),
             |_| Ok(helpers::hour(0, false)?.latent())
    );
    b.rule_2("<time-of-day> o'clock",
             time_check!(form!(Form::TimeOfDay(_))),
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
             time_check!(form!(Form::TimeOfDay(_))),
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
                      b.reg(r#"mittag|zwolf(?: uhr)?"#)?,
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
    b.rule_1("number (as relative minutes)",
             integer_check_by_range!(1, 59),
             |integer| Ok(RelativeMinuteValue(integer.value().value as i32))
    );
    b.rule_2("number <minutes> (as relative minutes)",
             integer_check_by_range!(1, 59),
             b.reg(r#"minuten?"#)?,
             |a, _| Ok(RelativeMinuteValue(a.value().value as i32))
    );
    b.rule_3("<hour-of-day> <integer> (as relative minutes)",
             time_check!(|time: &TimeValue| !time.latent && form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))(time)),
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
             time_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))),
             |relative_minute, _, time| helpers::hour_relative_minute(
                 time.value().form_time_of_day()?.full_hour(),
                 -1 * relative_minute.value().0,
                 time.value().form_time_of_day()?.is_12_clock())
    );
    b.rule_3("relative minutes after|past <integer> (hour-of-day)",
             relative_minute_check!(),
             b.reg(r#"nach"#)?,
             time_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))),
             |relative_minute, _, time| helpers::hour_relative_minute(
                 time.value().form_time_of_day()?.full_hour(),
                 relative_minute.value().0,
                 time.value().form_time_of_day()?.is_12_clock())
    );
    b.rule_2("viertel <integer> (german style hour-of-day)",
             b.reg(r#"vie?rtel"#)?,
             time_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))),
             |_, time| helpers::hour_relative_minute(
                 time.value().form_time_of_day()?.full_hour(),
                 -45,
                 time.value().form_time_of_day()?.is_12_clock())
    );
    b.rule_2("half <integer> (german style hour-of-day)",
             b.reg(r#"halbe?"#)?,
             time_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))),
             |_, time| helpers::hour_relative_minute(
                 time.value().form_time_of_day()?.full_hour(),
                 -30,
                 time.value().form_time_of_day()?.is_12_clock())
    );
    b.rule_2("dreiviertel <integer> (german style hour-of-day)",
             b.reg(r#"dreivie?rtel"#)?,
             time_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))),
             |_, time| helpers::hour_relative_minute(
                 time.value().form_time_of_day()?.full_hour(),
                 -15,
                 time.value().form_time_of_day()?.is_12_clock())
    );
    b.rule_1_terminal("dd/mm/yyyy",
                      b.reg(r#"([012]?[1-9]|10|20|30|31)[\./](0?[1-9]|10|11|12)[\./](\d{2,4})"#)?,
                      |text_match| helpers::ymd(
                          text_match.group(3).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(1).parse()?,
                      )
    );
    b.rule_1_terminal("dd-mm-yyyy",
                      b.reg(r#"([012]?[1-9]|10|20|30|31)-(0?[1-9]|10|11|12)-(\d{2,4})"#)?,
                      |text_match| helpers::ymd(
                          text_match.group(3).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(1).parse()?,
                      )
    );
    b.rule_1_terminal("mm.dd.",
                      b.reg(r#"([012]?[1-9]|10|20|30|31)\.(0?[1-9]|10|11|12)\."#)?,
                      |text_match| helpers::month_day(
                          text_match.group(2).parse()?,
                          text_match.group(1).parse()?)
    );
    b.rule_1_terminal("dd/mm",
                      b.reg(r#"(10|20|30|31|[012]?[1-9])[/\.](10|11|12|0?[1-9])"#)?,
                      |text_match| helpers::month_day(
                          text_match.group(2).parse()?,
                          text_match.group(1).parse()?)
    );
    b.rule_1_terminal("breakfast (latent)",
                      b.reg(r#"fr[üu]hst[üu]ck(?:szeit|spause)?"#)?,
                      |_| Ok(helpers::hour(6, false)?
                          .span_to(&helpers::hour(9, false)?, false)?
                          .latent()
                          .form(Form::Meal))
    );
    b.rule_1("lunch (latent)",
             b.reg(r#"mittag(?:szeit|pause|essen(?:szeit))"#)?,
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
    b.rule_1_terminal("dinner",
                      b.reg(r#"dinner|souper|abendessen(?:szeit)|abendbrot(?:zeit)|vesper(?:zeit)"#)?,
                      |_| Ok(helpers::hour_minute(17, 30, false)?
                          .span_to(&helpers::hour(21, false)?, false)?
                          .latent()
                          .form(Form::Meal))
    );
    b.rule_1_terminal("early morning",
                      b.reg(r#"fr[üu]hen vormittag|tagesanbruch|morgen(?:grauen|fr[üu]he)|fr[üu]h(?:en )?morgens?|am morgen fruh"#)?,
                      |_| Ok(helpers::hour(4, false)?
                          .span_to(&helpers::hour(9, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Morning)).latent())
    );
    b.rule_1_terminal("morning",
                      b.reg(r#"morgens|in der fr[üu]h|vor ?mittag(?:s(?:zeit)?)?|am morgen"#)?,
                      |_| Ok(helpers::hour(3, false)?
                          .span_to(&helpers::hour(12, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Morning)))
    );
    b.rule_1_terminal("late morning",
                      b.reg(r#"(?:kurz|am sp[äa]ten) vor ?mittag"#)?,
                      |_| Ok(helpers::hour(11, false)?
                          .span_to(&helpers::hour(13, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Morning)))
    );
    b.rule_1_terminal("early afternoon (latent)",
                      b.reg(r#"fr[üu]hen nachmittags?(?:stunden?)?"#)?,
                      |_| Ok(helpers::hour(13, false)?
                          .span_to(&helpers::hour(17, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
    );
    b.rule_1_terminal("early afternoon",
                      b.reg(r#"nach dem mittagessen|kurz nach mittag"#)?,
                      |_| Ok(helpers::hour(13, false)?
                          .span_to(&helpers::hour(16, false)?, false)?
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
                      |_| Ok(helpers::hour(17, false)?
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
                      b.reg(r#"abend"#)?,
                      |_| Ok(helpers::hour(18, false)?
                          .span_to(&helpers::hour(0, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Evening)))
    );
    b.rule_1_terminal("evening",
                      b.reg(r#"abends"#)?,
                      |_| Ok(helpers::hour(18, false)?
                          .span_to(&helpers::hour(0, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Evening)))
    );
    b.rule_1_terminal("late evening (latent)",
                      b.reg(r#"sp[äa]te[nr] abend(?:stunden?)?|abend sp[äa]t"#)?,
                      |_| Ok(helpers::hour(21, false)?
                          .span_to(&helpers::hour(0, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Evening)))
    );
    b.rule_1_terminal("late evening",
                      b.reg(r#"sp[äa]t(?: am )?abends?|abends sp[äa]t|sp[äa]t abends"#)?,
                      |_| Ok(helpers::hour(21, false)?
                          .span_to(&helpers::hour(0, false)?, false)?
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
                      |_| Ok(helpers::hour(0, false)?
                          .span_to(&helpers::hour(4, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Night)))
    );
    b.rule_1_terminal("night",
                      b.reg(r#"nachts"#)?,
                      |_| Ok(helpers::hour(0, false)?
                          .span_to(&helpers::hour(4, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Night)))
    );
    b.rule_1_terminal("late night",
                      b.reg(r#"(?:sp[äa]t|tief)(?: in der)? ?nachts?"#)?,
                      |_| Ok(helpers::hour(0, false)?
                          .span_to(&helpers::hour(4, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Night)))
    );

    b.rule_2("<time-of-day> morning",
        time_of_day_check_hour!(1, 12),
        time_check!(form!(Form::PartOfDay(PartOfDayForm::Morning))),
        |tod, _| {
            let period = helpers::hour(1, false)?
                     .span_to(&helpers::hour(12, false)?, true)?;
            Ok(period.intersect(tod.value())?
                .form(tod.value().form.clone()))
        }
    );

    b.rule_2("<time-of-day> afternoon",
        time_of_day_check_hour!(1, 7),
        time_check!(form!(Form::PartOfDay(PartOfDayForm::Afternoon))),
        |tod, _| {
            let period = helpers::hour(13, false)?
                     .span_to(&helpers::hour(19, false)?, true)?;
            Ok(period.intersect(tod.value())?
                .form(tod.value().form.clone()))
        }
    );

    b.rule_2("<time-of-day> evening",
        time_of_day_check_hour!(7, 11),
        time_check!(form!(Form::PartOfDay(PartOfDayForm::Afternoon))),
        |tod, _| {
            let period = helpers::hour(19, false)?
                     .span_to(&helpers::hour(24, false)?, true)?;
            Ok(period.intersect(tod.value())?
                .form(tod.value().form.clone()))
        }
    );

    b.rule_2("<time-of-day> night",
        time_of_day_check_hour!(1, 4),
        time_check!(form!(Form::PartOfDay(PartOfDayForm::Night))),
        |tod, _| {
            let period = helpers::hour(1, false)?
                     .span_to(&helpers::hour(4, false)?, true)?;
            Ok(period.intersect(tod.value())?
                .form(tod.value().form.clone()))
        }
    );

    b.rule_2("<article> <time>",
             b.reg(german_article_regex())?,
             time_check!(),
             |_, time| Ok(time.value().clone().not_latent())
    );

    b.rule_2("<article> <time>",
        b.reg(r#"a[nm]"#)?,
        time_check!(excluding_form!(Form::TimeOfDay(_))),
        |_, time| Ok(time.value().clone().not_latent())
    );

    b.rule_2("this <part-of-day>",
        b.reg(r#"diese[snm]?|heute"#)?,
        time_check!(|time: &TimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
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
    b.rule_2("<time> <part-of-day/meal>",
        time_check!(),
        time_check!(|time: &TimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
        |a, b| b.value().intersect(a.value())
    );
    b.rule_3("<part-of-day/meal> of <time>",
        time_check!(|time: &TimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
        b.reg(r#"de[sr]|vo[nm]|am"#)?,
        time_check!(),
        |a, _, b| a.value().intersect(b.value())
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
        b.reg(r#"an der sonnenwende"#)?,
        |_| Ok(helpers::month_day(6, 21)?.form(Form::Celebration))
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
        b.reg(r#"an der wintersonnwende"#)?,
        |_| Ok(helpers::month_day(12, 21)?.form(Form::Celebration))
    );
    b.rule_1_terminal("season",
                      b.reg(r#"(?:fr[üu]hlings?|fr[üu]hjahr(?:es)?)(?:zeit)?"#)?,
                      |_| Ok(helpers::month_day(3, 20)?
                          .span_to(&helpers::month_day(6, 21)?, false)?
                          .form(Form::PartOfYear))
    );
    b.rule_2("im <part-of-year>",
             b.reg(r#"(?:(?:in )?(?:de[nrms]|die|das)|im|ins)"#)?,
             time_check!(form!(Form::PartOfYear)),
             |_, time| Ok(time.value().clone())
    );
    b.rule_2("<time-of-day> approximately",
             time_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:um )?zirka|ungef[äa]hr|etwa"#)?,
             |time, _| Ok(time.value().clone().not_latent().precision(Approximate))
    );
    b.rule_2("<time-of-day> exactly",
             time_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"genau|exakt|p[üu]nktlich|punkt(?: um)?"#)?,
             |time, _| Ok(time.value().clone().not_latent().precision(Exact))
    );
    b.rule_2("about <time-of-day>",
             b.reg(r#"(?:um )?zirka|ungef[äa]hr|etwa|gegen"#)?,
             time_check!(form!(Form::TimeOfDay(_))),
             |_, time| Ok(time.value().clone().not_latent().precision(Approximate))
    );
    b.rule_2("exactly <time-of-day>",
             b.reg(r#"genau|exakt|p[üu]nktlich|punkt(?: um)?"#)?,
             time_check!(form!(Form::TimeOfDay(_))),
             |_, time| Ok(time.value().clone().not_latent().precision(Exact))
    );
    b.rule_4("dd-dd (interval) <month>",
             b.reg(r#"([012]?\d|30|31)(?:ter|\.)?"#)?,
             b.reg(r#"\-|bis"#)?,
             b.reg(r#"([012]?\d|30|31)(?:ter|\.)?"#)?,
             time_check!(form!(Form::Month(_))),
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
             time_check!(form!(Form::Month(_))),
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
             time_check!(form!(Form::Month(_))),
             |_, d1, _, d2, month| {
                 let start = month.value()
                     .intersect(&helpers::day_of_month(d1.value().value as u32)?)?;
                 let end = month.value()
                     .intersect(&helpers::day_of_month(d2.value().value as u32)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_3("<datetime> - <datetime> (interval)",
             time_check!(|time: &TimeValue| !time.latent),
             b.reg(r#"\-|bis"#)?,
             time_check!(|time: &TimeValue| !time.latent),
             |start, _, end| {
                 start.value().smart_span_to(end.value(), true)
             }
    );
    b.rule_4("between <datetime> and <datetime> (interval)",
             b.reg(r#"zwischen"#)?,
             time_check!(|time: &TimeValue| !time.latent && excluding_form!(Form::TimeOfDay(_))(time)),
             b.reg(r#"und"#)?,
             time_check!(|time: &TimeValue| !time.latent && excluding_form!(Form::TimeOfDay(_))(time)),
             |_, start, _, end| start.value().smart_span_to(end.value(), true)
    );
    b.rule_3("<time-of-day> - <time-of-day> (interval)",
             time_check!(|time: &TimeValue| !time.latent && form!(Form::TimeOfDay(_))(time)),
             b.reg(r#"\-|bis"#)?,
             time_check!(form!(Form::TimeOfDay(_))),
             |start, _, end| start.value().smart_span_to(end.value(), true)
    );
    b.rule_4("from <time> to <time>",
             b.reg(r#"vo[nm]|ab|nach"#)?,
             time_check!(|time: &TimeValue| !time.latent),
             b.reg(r#"bis(?: zum?r?)?|auf"#)?,
             time_check!(|time: &TimeValue| !time.latent),
             |_, start, _, end| start.value().span_to(end.value(), true)
    );
    b.rule_4("from <time-of-day> - <time-of-day> (interval)",
             b.reg(r#"(?:vo[nm]|nach|ab|(?:fr[üu]h|sp[äa]t)estens(?: um| ab)?)"#)?,
             time_check!(|time: &TimeValue| form!(Form::TimeOfDay(_))(time)),
             b.reg(r#"(?:(?:noch|aber|jedoch)? vor)|\-|bis"#)?,
             time_check!(|time: &TimeValue| form!(Form::TimeOfDay(_))(time)),
             |_, start, _, end| {
                start.value().smart_span_to(end.value(), true)
            }
    );
    b.rule_4("between <time-of-day> and <time-of-day> (interval)",
             b.reg(r#"zwischen"#)?,
             time_check!(|time: &TimeValue| form!(Form::TimeOfDay(_))(time)),
             b.reg(r#"und"#)?,
             time_check!(|time: &TimeValue| form!(Form::TimeOfDay(_))(time)),
             |_, start, _, end| start.value().smart_span_to(end.value(), true)
    );
    b.rule_2("within <duration>",
             b.reg(r#"binnen|innerhalb(?: von)?"#)?,
             duration_check!(),
             |_, duration| helpers::cycle_nth(Grain::Second, 0)?
                 .span_to(&duration.value().in_present()?, false)
    );
    b.rule_2("by the end of <time>",
             b.reg(r#"bis (?:zum)? ende (?:von)?|(?:noch )?vor "#)?,
             time_check!(),
             |_, time| helpers::cycle_nth(Grain::Second, 0)?.span_to(time.value(), true)
    );
    b.rule_2("before <time>",
             b.reg(r#"vor |bis(?:(?: zu[rm]?) |in d(?:en|ie|as))?"#)?,
             time_check!(),
             |_, time| Ok(time.value().clone().mark_before_start())
    );
    b.rule_2("until <time>",
        b.reg(r#"sp[äa]testens"#)?,
        time_check!(),
        |_, tod| Ok(tod.value().clone().mark_before_end().not_latent())
    );
    b.rule_2("until <time>",
        b.reg(r#"kurz vor"#)?,
        time_check!(form!(Form::TimeOfDay(_))),
        |_, tod| Ok(tod.value().clone().mark_before_start().not_latent())
    );
    b.rule_2("after <time>",
             b.reg(r#"nach"#)?,
             time_check!(),
             |_, time| Ok(time.value().clone().mark_after_end())
    );
    b.rule_2("after <time-of-day>",
        b.reg(r#"kurz nach"#)?,
        time_check!(form!(Form::TimeOfDay(_))),
        |_, tod| Ok(tod.value().clone().mark_after_end().not_latent())
    );
    b.rule_1_terminal("start of week",
                      b.reg(r#"(?:de[rnms]|zu )?(anfang|beginn) der woche"#)?,
                      |_| {
                          let current_week = helpers::cycle_nth(Grain::Week, 0)?;
                          let start = current_week.intersect(&helpers::day_of_week(Weekday::Mon)?)?;
                          let end = current_week.intersect(&helpers::day_of_week(Weekday::Tue)?)?;
                          start.span_to(&end, true)
                      }
    );
    b.rule_2("start of week",
             b.reg(r#"(?:de[rmns] )?(anfang|beginn) der"#)?,
             time_check!(form!(Form::Cycle(Grain::Week))),
             |_, week| {
                 let start = week.value().intersect(&helpers::day_of_week(Weekday::Mon)?)?;
                 let end = week.value().intersect(&helpers::day_of_week(Weekday::Tue)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_2("middle of week",
             b.reg(r#"(?:in |im )?(?:der |die )?mitte der"#)?,
             time_check!(form!(Form::Cycle(Grain::Week))),
             |_, week| {
                 let start = week.value().intersect(&helpers::day_of_week(Weekday::Fri)?)?;
                 let end = week.value().intersect(&helpers::day_of_week(Weekday::Sun)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_2("end of week",
             b.reg(r#"(?:das )?ende der"#)?,
             time_check!(form!(Form::Cycle(Grain::Week))),
             |_, week| {
                 let start = week.value().intersect(&helpers::day_of_week(Weekday::Fri)?)?;
                 let end = week.value().intersect(&helpers::day_of_week(Weekday::Sun)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_1_terminal("end of week",
                      b.reg(r#"ende der woche"#)?,
                      |_| {
                          let current_week = helpers::cycle_nth(Grain::Week, 0)?;
                          let end = current_week.intersect(&helpers::day_of_week(Weekday::Sun)?)?;
                          helpers::cycle_nth(Grain::Second, 0)?.span_to(&end, true)
                      }
    );
    b.rule_1_terminal("start of month",
                      b.reg(r#"(?:de[rnms]|zu )?(anfang|beginn) des monate?s"#)?,
                      |_| {
                          let current_month = helpers::cycle_nth(Grain::Month, 0)?;
                          let start = current_month.intersect(&helpers::day_of_month(1)?)?;
                          let end = current_month.intersect(&helpers::day_of_month(10)?)?;
                          start.span_to(&end, true)
                      }
    );
    b.rule_2("start of month",
             b.reg(r#"(?:de[rmns] )?(anfang|beginn)(?: des)?"#)?,
             time_check!(|time: &TimeValue| {
            match time.form {
                Form::Month(_) | Form::Cycle(Grain::Month) => true,
                _ => false
            }
        }),
             |_, month| {
                 let start = month.value().intersect(&helpers::day_of_month(1)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(10)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_1_terminal("beginning of year",
                      b.reg(r#"(?:de[rmsn] )?jahres(?:anfang|beginn)|(?:de[rmsn] )?(?:anfang|beginn) des jahres"#)?,
                      |_| {
                          let current_year = helpers::cycle_nth(Grain::Year, 0)?;
                          let start = current_year.intersect(&helpers::month(1)?)?;
                          let end = current_year.intersect(&helpers::month(3)?)?;
                          start.span_to(&end, true)
                      }
    );
    b.rule_2("beginning of year",
             b.reg(r#"(?:de[rmsn] )?anfang(?: de[sr])?"#)?,
             time_check!(|time: &TimeValue| {
            match time.form {
                Form::Year(_) | Form::Cycle(Grain::Year) => true,
                _ => false
            }
        }),
             |_, year| {
                 let start = year.value().intersect(&helpers::month(1)?)?;
                 let end = year.value().intersect(&helpers::month(3)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_1_terminal("end of year",
                      b.reg(r#"(?:(?:das|de[mnsr]) )?(?:jahr(?:es)?(?:ende|schluss)|ende (?:des|vom) jahr(?:e?s)?)"#)?,
                      |_| {
                          let current_year = helpers::cycle_nth(Grain::Year, 0)?;
                          let start = current_year.intersect(&helpers::month(10)?)?;
                          let end = current_year.intersect(&helpers::month(12)?)?;
                          start.span_to(&end, true)
                      }
    );
    b.rule_2("end of year",
            b.reg(r#"(?:(?:das|de[ms] ))?ende(?: de[sr])?"#)?,
            time_check!(|time: &TimeValue| {
                match time.form {
                    Form::Year(_) | Form::Cycle(Grain::Year) => true,
                    _ => false
                }
            }),
            |_, year| {
                 let start = year.value().intersect(&helpers::month(10)?)?;
                 let end = year.value().intersect(&helpers::month(12)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_2("end of month",
             b.reg(r#"ende"#)?,
             time_check!(|time: &TimeValue| {
            match time.form {
                Form::Month(_) | Form::Cycle(Grain::Month) => true,
                _ => false
            }
        }),
             |_, month| {
                 let start = month.value().intersect(&helpers::day_of_month(25)?)?;
                 let end = helpers::cycle(Grain::Day)?.last_of(month.value())?;
                 start.span_to(&end, true)
             }
    );
    b.rule_1("end of month",
             b.reg(r#"(?:(?:das|am) )?ende (?:des|vom) monate?s?|monatsende"#)?,
             |_| {
                 let current_month = helpers::cycle_nth(Grain::Month, 0)?;
                 let start = current_month.intersect(&helpers::day_of_month(25)?)?;
                 let end = helpers::cycle(Grain::Day)?.last_of(&current_month)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_2("since <time> (past)",
             b.reg(r#"seit(?: de[rm])?"#)?,
             time_check!(),
             |_, a| Ok(a.value().the_nth(-1)?.mark_after_start().not_latent())
    );
    b.rule_2("since <time> (futur)",
             b.reg(r#"ab(?: de[rm])?"#)?,
             time_check!(),
             |_, a| Ok(a.value().clone().mark_after_start().not_latent())
    );
    b.rule_3("since <time>",
             b.reg(r#"vo[nm](?: de[rm])?"#)?,
             time_check!(),
             b.reg(r#"an"#)?,
             |_, time, _| Ok(time.value().clone().mark_after_start().not_latent())
    );
    Ok(())
}

pub fn rules_temperature(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    // TODO
    // b.rule_2("under <temp>",
    //     b.reg(r#"bis(?: zu)?|unter|weniger als"#)?,
    //     temperature_check!(|temp: &TemperatureValue| !temp.latent),
    // );
    // b.rule_2("above <temp>",
    //     b.reg(r#"[uü]ber|mehr als"#)?,
    //     temperature_check!(|temp: &TemperatureValue| !temp.latent),
    // );

    b.rule_2("<article> temp",
             b.reg(r#"bei"#)?,
             temperature_check!(),
             |_, temp| Ok(TemperatureValue {
                 value: temp.value().value,
                 unit: temp.value().unit,
                 latent: false,
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
             b.reg(r#"grad|°"#)?,
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
             b.reg(r#"f(?:ahrenheit)?"#)?,
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
                              _ => return Err(RuleErrorKind::Invalid.into()),
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
                      b.reg(r#"(zwanzig|drei(?:ss|β)ig|vierzig|f[üu]nfzig|sechzig|siebzig|achtzig|neunzig)"#)?,
                      |text_match| {
                        let value = match text_match.group(1).as_ref() {
                            "zwanzig" => 20,
                            "dreissig" => 30,
                            "dreiβig" =>  30,
                            "vierzig" => 40,
                            "funfzig" => 50,
                            "fünfzig" => 50,
                            "sechzig" => 60,
                            "siebzig" => 70,
                            "achtzig" => 80,
                            "neunzig" => 90,
                            _ => return Err(RuleErrorKind::Invalid.into()),
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
            value: a.value().value * 1000,
            grain: Some(3),
            ..IntegerValue::default()
        })
    );

    b.rule_2("number millions",
        integer_check_by_range!(1, 999),
         b.reg(r#"million(?:en)?"#)?,
        |a, _| Ok(IntegerValue {
            value: a.value().value * 1000000,
            grain: Some(6),
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
             |a, _, b| FloatValue::new(b.value().value() * 0.1 + a.value().value())
    );
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
    b.rule_1_terminal("ordinals (first..19th)",
                      b.reg(r#"(erste|zweite|dritte|vierte|f[üu]nfte|sechste|sieb(?:en)?te|achte|neunte|zehnte|elfte|zw[öo]lfte|dreizehnte|vierzehnte|f[üu]nfzehnte|sechzehnte|siebzehnte|achtzehnte|neunzehnte)(?:r|s|n|m)?"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
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
                              _ => return Err(RuleErrorKind::Invalid.into()),
                          };
                          Ok(OrdinalValue::new(value))
                      }
    );
    b.rule_1_terminal("ordinal (digits)",
                      b.reg(r#"0*(\d+)(?:\.| ?(?:te(?:n|r|s)?)|(?:ste(?:n|r|s)?))"#)?,
                      |text_match| Ok(OrdinalValue::new(text_match.group(1).parse()?))
    );
    b.rule_2("der <ordinal>",
             b.reg(r#"de(?:r|s|n|m)|das|die"#)?,
             ordinal_check!(),
             |_, ordinal| Ok(ordinal.value().clone().prefixed())
    );
    b.rule_1_terminal("ordinal (20..90)",
                      b.reg(r#"(zwanzigste|drei(?:ss|β)igste|vierzigste|f[üu]nfzigste|sechzigste|siebzigste|achtzigste|neunzigste)(?:r|n|m|s)?"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "zwanzigste" => 20,
                              "dreissigste" => 30,
                              "dreiβigste" => 30,
                              "vierzigste" => 40,
                              "funfzigste" => 50,
                              "fünfzigste" => 50,
                              "sechzigste" => 60,
                              "siebzigste" => 70,
                              "achtzigste" => 80,
                              "neunzigste" => 90,
                              _ => return Err(RuleErrorKind::Invalid.into()),
                          };
                          Ok(OrdinalValue::new(value))
                      }
    );
    b.rule_3("ordinal [2-9][1-9]",
             integer_check_by_range!(1, 9),
             b.reg(r#"und"#)?,
             ordinal_check!(|ordinal: &OrdinalValue| ordinal.value % 10 == 0),
             |integer, _, ordinal| Ok(OrdinalValue::new(integer.value().value + ordinal.value().value))
    );
    Ok(())
}
