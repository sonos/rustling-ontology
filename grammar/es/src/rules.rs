use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::dimension::Precision::*;
use rustling_ontology_values::helpers;
use rustling_ontology_moment::{Weekday, Grain, PeriodComp, Period};

pub fn rules_percentage(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("<number> per cent",
        number_check!(),
        // FIXME
        b.reg(r#"(?:%|p\.c\.|por ?cien(?:tos?))?"#)?,
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
        b.reg(r#"\$|d[oóò]lar(?:es)?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("$") })
    );
    b.rule_1_terminal("EUR",
        b.reg(r#"€|(?:[e€]uro?s?)"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("EUR") })
    );
    b.rule_1_terminal("£",
        b.reg(r#"(?:pound|libra)s?|£"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("£") })
    );
    b.rule_1_terminal("USD",
        b.reg(r#"us[d\$]|d[oóò]lar(?:es)? (?:estadounidense|americano)s?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("USD") })
    );
    b.rule_1_terminal("CAD",
                      b.reg(r#"cad|d[oóò]lar(?:es)? canadienses?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("CAD") })
    );
    b.rule_1_terminal("AUD",
                      b.reg(r#"d[oóò]lar(?:es)? australianos?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("AUD") })
    );
    b.rule_1_terminal("Bitcoin",
        b.reg(r#"฿|bitc[oóò]in(?:e?s)?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("฿") })
    );
    b.rule_1_terminal("GBP",
        b.reg(r#"gbp|libras? esterlinas?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("GBP") })
    );
    b.rule_1_terminal("JPY",
                      b.reg(r#"jpy|yen(?:es)?(?: japoneses?)?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("JPY") })
    );
    b.rule_1_terminal("¥",
                      b.reg(r#"¥"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("¥") })
    );
    b.rule_1_terminal("KRW",
                      b.reg(r#"₩|krw|won(?:es)?(?: surcoreanos?)?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("KRW") })
    );
    b.rule_1_terminal("RMB|CNH|CNY",
                      b.reg(r#"cny|cnh|rmb|yuan(?:es)?(?: chinos?)?|renmimbis?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("CNY") })
    );
    b.rule_1_terminal("INR",
                      b.reg(r#"rupias?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("INR") })
    );
    b.rule_1_terminal("HKD",
                      b.reg(r#"hkd|d[oóò]lar(?:es)? de hong[- ]kong"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("HKD") })
    );
    b.rule_1_terminal("CHF",
                      b.reg(r#"chf|francos suizos?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("CHF") })
    );
    b.rule_1_terminal("KR",
                      b.reg(r#"kr|coronas?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("KR") })
    );
    b.rule_1_terminal("DKK",
                      b.reg(r#"dkk|coronas? danesas?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("DKK") })
    );
    b.rule_1_terminal("NOK",
                      b.reg(r#"nok|coronas? noruegas?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("NOK") })
    );
    b.rule_1_terminal("SEK",
                      b.reg(r#"sek|coronas? suecas?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("SEK") })
    );
    b.rule_1_terminal("cent",
                      b.reg(r#"c[eéè]nt(?:avo|imo)s?"#)?,
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
    b.rule_3("<amount> of <unit>",
             number_check!(),
             b.reg(r#"de"#)?,
             money_unit!(),
             |a, _, b| {
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
    b.rule_2("about <amount-of-money>",
             b.reg(r#"aproximadamente|sobre|cerca de|casi|un[oa]s"#)?,
             amount_of_money_check!(),
             |_, a| {
                 Ok(AmountOfMoneyValue {
                     precision: Approximate,
                     ..a.value().clone()
                 })
             });
    b.rule_2("<amount-of-money> about",
             amount_of_money_check!(),
             b.reg(r#"m[aáà]s o menos|aproximadamente"#)?,
             |a, _| {
                 Ok(AmountOfMoneyValue {
                     precision: Approximate,
                     ..a.value().clone()
                 })
             });
    b.rule_2("exactly <amount-of-money>",
             b.reg(r#"exactamente"#)?,
             amount_of_money_check!(),
             |_, a| {
                 Ok(AmountOfMoneyValue {
                     precision: Exact,
                     ..a.value().clone()
                 })
             });
    b.rule_2("<amount-of-money> exactly",
             amount_of_money_check!(),
             b.reg(r#"exactos"#)?,
             |a, _| {
                 Ok(AmountOfMoneyValue {
                     precision: Exact,
                     ..a.value().clone()
                 })
             }
    );
    Ok(())
}

pub fn rules_duration(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("seconde (unit-of-duration)",
                      b.reg(r#"seg(?:undo)?s?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Second))
    );
    b.rule_1_terminal("minute (unit-of-duration)",
                      b.reg(r#"min(?:uto)?s?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Minute))
    );
    b.rule_1_terminal("hour (unit-of-duration)",
                      b.reg(r#"h(?:ora)?s?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Hour))
    );
    b.rule_1_terminal("day (unit-of-duration)",
                      b.reg(r#"d[iíì]as?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Day))
    );
    b.rule_1_terminal("week (unit-of-duration)",
                      b.reg(r#"semanas?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Week))
    );
    b.rule_1_terminal("month (unit-of-duration)",
                      b.reg(r#"mes(?:es)?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Month))
    );
    b.rule_1_terminal("year (unit-of-duration)",
                      b.reg(r#"a[nñ]os?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Year))
    );
    b.rule_1_terminal("quarter of an hour",
                      b.reg(r#"(?:un )?(?:cuarto|1/4)(?: de hora)?"#)?,
                      |_| Ok(DurationValue::new(PeriodComp::minutes(15).into()))
    );
    b.rule_1_terminal("half an hour",
                      b.reg(r#"(?:media|1/2) hora"#)?,
                      |_| Ok(DurationValue::new(PeriodComp::minutes(30).into()))
    );
    b.rule_1_terminal("three-quarters of an hour",
                      b.reg(r#"(?:(?:3|tres) cuartos?|3/4)(?: de hora)?"#)?,
                      |_| Ok(DurationValue::new(PeriodComp::minutes(45).into()))
    );
    b.rule_2("<integer> <unit-of-duration>",
             integer_check_by_range!(0),
             unit_of_duration_check!(),
             |integer, uod| Ok(DurationValue::new(PeriodComp::new(uod.value().grain, integer.value().value).into()))
    );
    b.rule_3("<integer> <unit-of-duration> and a half",
             integer_check_by_range!(0),
             unit_of_duration_check!(),
             b.reg(r#"y media"#)?,
             |integer, uod, _| {
                 let half_period: Period = uod.value().grain.half_period().map(|a| a.into()).ok_or_else(|| RuleError::Invalid)?;
                 Ok(DurationValue::new(half_period + PeriodComp::new(uod.value().grain, integer.value().value)))
             }
    );
    b.rule_3("<integer> <unit-of-duration> and a quarter",
             integer_check_by_range!(0),
             unit_of_duration_check!(),
             b.reg(r#"y cuarto"#)?,
             |integer, uod, _| {
                 let half_period: Period = uod.value().grain.half_period().map(|a| a.into()).ok_or_else(|| RuleError::Invalid)?;
                 Ok(DurationValue::new(half_period + PeriodComp::new(uod.value().grain, integer.value().value)))
             }
    );
    b.rule_2("in <duration> (future moment)",
             b.reg(r#"(?:en|dentro(?: de)?)(?: (?:el|la|los|las) pr[oóò]xim[oa]s)?"#)?,
             duration_check!(),
             |_, duration| duration.value().in_present()
    );
    b.rule_3("<duration> y <duration>",
             duration_check!(|duration: &DurationValue| !duration.suffixed),
             b.reg(r#"y"#)?,
             duration_check!(|duration: &DurationValue| !duration.prefixed),
             |a, _, b| Ok(a.value() + b.value())
    );
    b.rule_2("<duration> <duration>",
             duration_check!(|duration: &DurationValue| !duration.suffixed),
             duration_check!(|duration: &DurationValue| !duration.prefixed),
             |a, b| Ok(a.value() + b.value())
    );
    b.rule_2("<duration> <integer>",
             duration_check!(|duration: &DurationValue| !duration.prefixed),
             integer_check_by_range!(0),
             |duration, integer| helpers::compose_duration_with_integer(duration.value(), integer.value())
    );
    b.rule_2("<duration> ago",
             b.reg(r#"hace"#)?,
             duration_check!(),
             |_, duration| duration.value().ago()
    );
    b.rule_2("<duration> later",
             duration_check!(),
             b.reg(r#"m[aáà]s tarde|despu[eéè]s"#)?,
             |duration, _| duration.value().in_present()
    );
    Ok(())
}

pub fn rules_cycle(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("second (cycle)",
                      b.reg(r#"segundos?"#)?,
                      |_| CycleValue::new(Grain::Second)
    );
    b.rule_1_terminal("minute (cycle)",
                      b.reg(r#"minutos?"#)?,
                      |_| CycleValue::new(Grain::Minute)
    );
    b.rule_1_terminal("hour (cycle)",
                      b.reg(r#"horas?"#)?,
                      |_| CycleValue::new(Grain::Hour)
    );
    b.rule_1_terminal("day (cycle)",
                      b.reg(r#"d[iíì]as?"#)?,
                      |_| CycleValue::new(Grain::Day)
    );
    b.rule_1_terminal("week (cycle)",
                      b.reg(r#"semanas?"#)?,
                      |_| CycleValue::new(Grain::Week)
    );
    b.rule_1_terminal("month (cycle)",
                      b.reg(r#"mes(?:es)?"#)?,
                      |_| CycleValue::new(Grain::Month)
    );
    b.rule_1_terminal("trimester (cycle)",
                      b.reg(r#"trimestres?"#)?,
                      |_| CycleValue::new(Grain::Quarter)
    );
    b.rule_1_terminal("year (cycle)",
                      b.reg(r#"a(?:n|ñ)os?"#)?,
                      |_| CycleValue::new(Grain::Year)
    );
    b.rule_2("this <cycle>",
             b.reg(r#"(?:est(?:e|a|os)|en (?:el|l[oa]s?) ?)"#)?,
             cycle_check!(),
             |_, cycle| helpers::cycle_nth(cycle.value().grain, 0)
    );
    b.rule_3("the <cycle> past",
             b.reg(r#"(?:el|l[oa]s?) ?"#)?,
             cycle_check!(),
             b.reg(r#"(?:pasad|[uúù]ltim)[oa]s?"#)?,
             |_, cycle, _| helpers::cycle_nth(cycle.value().grain, -1)
    );
    b.rule_2("the past <cycle>",
             b.reg(r#"(?:(?:el|l[oa]s?) )?(?:pasad|[uúù]ltim)[oa]s?"#)?,
             cycle_check!(),
             |_, cycle| helpers::cycle_nth(cycle.value().grain, -1)
    );
    b.rule_3("the <cycle> next",
             b.reg(r#"(?:el|l[oa]s?) ?"#)?,
             cycle_check!(),
             b.reg(r#"(?:pr[oóò]xim[oa]s?|que vienen?|siguientes?)"#)?,
             |_, cycle, _| helpers::cycle_nth(cycle.value().grain, 1)
    );
    b.rule_2("the next <cycle>",
             b.reg(r#"(?:(?:el|l[oa]s?) )?pr[oóò]xim[oa]s?|siguientes?"#)?,
             cycle_check!(),
             |_, cycle| helpers::cycle_nth(cycle.value().grain, 1)
    );
    b.rule_4("the <cycle> after <time>",
             b.reg(r#"(?:el|l[oa]s?)"#)?,
             cycle_check!(),
             b.reg(r#"(?:pr[oóò]xim[oa]s?|que vienen?|siguientes?)"#)?,
             datetime_check!(),
             |_, cycle, _, time| helpers::cycle_nth_after(cycle.value().grain, 1, time.value())
    );
    b.rule_4("the <cycle> before <time>",
             b.reg(r#"(?:el|l[oa]s?)"#)?,
             cycle_check!(),
             b.reg(r#"antes de"#)?,
             datetime_check!(),
             |_, cycle, _, time| helpers::cycle_nth_after(cycle.value().grain, -1, time.value())
    );
    b.rule_3("past n <cycle>",
             b.reg(r#"(?:pasad|[uúù]ltim)[oa]s?"#)?,
             integer_check_by_range!(2, 9999),
             cycle_check!(),
             |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    b.rule_3("n past <cycle>",
             integer_check_by_range!(2, 9999),
             b.reg(r#"(?:pasad|[uúù]ltim)[oa]s?"#)?,
             cycle_check!(),
             |integer, _, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    b.rule_4("the n past <cycle>",
             b.reg(r#"(?:el|l[oa]s?)"#)?,
             integer_check_by_range!(2, 9999),
             b.reg(r#"(?:pasad|[uúù]ltim)[oa]s?"#)?,
             cycle_check!(),
             |_, integer, _, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    b.rule_3("the past n <cycle>",
             b.reg(r#"(?:el|l[oa]s?) (?:pasad|[uúù]ltim)[oa]s?"#)?,
             integer_check_by_range!(2, 9999),
             cycle_check!(),
             |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    b.rule_4("the n <cycle> past",
             b.reg(r#"(?:el|l[oa]s?)"#)?,
             integer_check_by_range!(2, 9999),
             cycle_check!(),
             b.reg(r#"(?:pasad|[uúù]ltim)[oa]s?"#)?,
             |_, integer, cycle, _| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );    
    b.rule_3("next n <cycle>",
             b.reg(r#"(?:(?:el|l[oa]s?) )?pr[oóò]xim[oa]s?"#)?,
             integer_check_by_range!(2, 9999),
             cycle_check!(),
             |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    b.rule_3("n next <cycle>",
             integer_check_by_range!(2, 9999),
             b.reg(r#"pr[oóò]xim[oa]s?"#)?,
             cycle_check!(),
             |integer, _, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    b.rule_4("the n next <cycle>",
             b.reg(r#"(?:el|l[oa]s?)"#)?,
             integer_check_by_range!(2, 9999),
             b.reg(r#"pr[oóò]xim[oa]s?"#)?,
             cycle_check!(),
             |_, integer, _, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    b.rule_3("n <cycle> next",
             integer_check_by_range!(2, 9999),
             cycle_check!(),
             b.reg(r#"(?:pr[oóò]xim[oa]s?|que vienen?|siguientes?)"#)?,
             |integer, cycle, _| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    b.rule_4("the n <cycle> next",
             b.reg(r#"(?:el|l[oa]s?)"#)?,
             integer_check_by_range!(2, 9999),
             cycle_check!(),
             b.reg(r#"(?:pr[oóò]xim[oa]s?|que vienen?|siguientes?)"#)?,
             |_, integer, cycle, _| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    b.rule_2("<ordinal> quarter",
             ordinal_check!(),
             cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Quarter),
             |ordinal, _| helpers::cycle_nth_after(
                 Grain::Quarter,
                 ordinal.value().value - 1,
                 &helpers::cycle_nth(Grain::Year, 0)?
             )
    );
    b.rule_4("<ordinal> quarter <year>",
             ordinal_check!(),
             cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Quarter),
             b.reg(r#"del? ?"#)?,
             datetime_check!(),
             |ordinal, _, _, time| helpers::cycle_nth_after(
                 Grain::Quarter,
                 ordinal.value().value - 1,
                 time.value()
             )
    );
    Ok(())
}

pub fn rules_datetime(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("intersect",
             datetime_check!(|time: &TimeValue| !time.latent),
             datetime_check!(|time: &TimeValue| !time.latent),
             |a, b| a.value().intersect(b.value())
    );
    b.rule_3("intersect by `de`",
             datetime_check!(|time: &TimeValue| !time.latent),
             b.reg(r#"del?"#)?,
             datetime_check!(|time: &TimeValue| !time.latent),
             |a, _, b| a.value().intersect(b.value())
    );
    b.rule_3("two time tokens separated by \",\"",
             datetime_check!(|time: &TimeValue| !time.latent),
             b.reg(r#","#)?,
             datetime_check!(|time: &TimeValue| !time.latent),
             |a, _, b| a.value().intersect(b.value())
    );
//    // Not latent intersects
//    b.rule_3("intersect <date> at <time>",
//             datetime_check!(|time: &TimeValue| !time.latent),
//             b.reg(r#"de"#)?,
//             datetime_check!(|time: &TimeValue| !time.latent),
//             |a, _, b| a.value().intersect(b.value())
//    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"lunes|lun?\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Mon)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"martes|mar?\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Tue)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"mi[eé]\.?(?:rcoles)?|mx|mier?\."#)?,
                      |_| helpers::day_of_week(Weekday::Wed)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"jueves|jue|jue\."#)?,
                      |_| helpers::day_of_week(Weekday::Thu)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"viernes|vie|vie\."#)?,
                      |_| helpers::day_of_week(Weekday::Fri)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"s[áa]bado|s(?:á|a)b\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Sat)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"domingo|dom\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Sun)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"enero|ene\.?"#)?,
                      |_| helpers::month(1)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"febrero|feb\.?"#)?,
                      |_| helpers::month(2)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"marzo|mar\.?"#)?,
                      |_| helpers::month(3)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"abril|abr\.?"#)?,
                      |_| helpers::month(4)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"mayo?\.?"#)?,
                      |_| helpers::month(5)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"junio|jun\.?"#)?,
                      |_| helpers::month(6)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"julio|jul\.?"#)?,
                      |_| helpers::month(7)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"agosto|ago\.?"#)?,
                      |_| helpers::month(8)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"septiembre|sept?\.?"#)?,
                      |_| helpers::month(9)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"octubre|oct\.?"#)?,
                      |_| helpers::month(10)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"noviembre|nov\.?"#)?,
                      |_| helpers::month(11)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"diciembre|dic\.?"#)?,
                      |_| helpers::month(12)
    );
    b.rule_1_terminal("Navidad",
                      b.reg(r#"(?:la )?navidad"#)?,
                      |_| helpers::month_day(12, 25)
    );
    b.rule_1_terminal("Nochevieja",
                      b.reg(r#"nochevieja"#)?,
                      |_| helpers::month_day(12, 31)
    );
    b.rule_1_terminal("ano nuevo",
                      b.reg(r#"a[nñ]o nuevo"#)?,
                      |_| helpers::month_day(1, 1)
    );
    b.rule_1_terminal("right now",
                      b.reg(r#"ahor(?:it)?a(?: mismo)?|ya|en\s?seguida|cuanto antes|en este preciso (?:istante|momento)"#)?,
                      |_| helpers::cycle_nth(Grain::Second, 0)
    );
    b.rule_1_terminal("now / today",
                      b.reg(r#"(?:hoy)|(?:en este momento)"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 0)
    );
    b.rule_1_terminal("tomorrow",
                      b.reg(r#"ma[nñ]ana|el d[iíì]a (?:siguiente|(?:de )?despu[eéè]s)"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 1)
    );
    b.rule_1_terminal("yesterday",
                      b.reg(r#"ayer|el d[iíì]a (?:anterior|de antes)|la v[iíì]spera"#)?,
                      |_| helpers::cycle_nth(Grain::Day, -1)
    );
    b.rule_1_terminal("the day after tomorrow",
                      b.reg(r#"pasados? ma[nñ]ana"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 2)
    );
    b.rule_1_terminal("the day before yesterday",
                      b.reg(r#"anteayer|antes de (?:ayer|anoche)|antier"#)?,
                      |_| helpers::cycle_nth(Grain::Day, -2)
    );
    b.rule_2("this <day-of-week>", //assumed to be in the future
             b.reg(r#"este"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             |_, time| time.value().the_nth_not_immediate(0)
    );
    b.rule_2("this <datetime>",
             b.reg(r#"este"#)?,
             datetime_check!(),
             |_, time| time.value().the_nth(0)
    );
    b.rule_2("in <named-month>",
             b.reg(r#"(?:durante|en)(?: el mes de)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, a| Ok(a.value().clone())
    );
    b.rule_2("beginning <named-month>(interval)",
             b.reg(r#"a(?: principios|l comienzo) de(?:l mes de)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, month| {
                 let start = month.value().intersect(&helpers::day_of_month(1)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(5)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_1_terminal("end of month",
                      b.reg(r#"a fin(?:ales)? de mes"#)?,
                      |_| {
                          let month = helpers::cycle_nth(Grain::Month, 1)?;
                        Ok(helpers::cycle_nth_after(Grain::Day, -10, &month)?
                            .span_to(&month, false)?
                            .latent()
                            .form(Form::PartOfMonth))
                    }
    );
    b.rule_2("end <named-month>(interval)",
             b.reg(r#"a fin(?:ales)? de(?:l mes de)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, month| {
                 let start = month.value().intersect(&helpers::day_of_month(25)?)?;
                 let end = helpers::cycle(Grain::Day)?.last_of(month.value())?;
                 start.span_to(&end, true)
             }
    );
    b.rule_2("beginning <named-month>(interval)",
             b.reg(r#"a(?: principios|l comienzo) de(?:l mes de)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, month| {
                 let start = month.value().intersect(&helpers::day_of_month(1)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(5)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_2("first 15 days of <named-month>(interval)",
             b.reg(r#"primera quincena de(?:l mes de)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, month| {
                 let start = month.value().intersect(&helpers::day_of_month(1)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(14)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_2("second 15 days of <named-month>(interval)",
             b.reg(r#"segunda quincena de(?:l mes de)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, month| {
                 let start = month.value().intersect(&helpers::day_of_month(15)?)?;
                 let end = helpers::cycle(Grain::Day)?.last_of(month.value())?;
                 start.span_to(&end, true)
             }
    );
    b.rule_2("next <named-month|named-day>",
             b.reg(r#"(?:el|la )?pr[oóò]xim[oa]"#)?,
             datetime_check!(),
             |_, time| time.value().the_nth_not_immediate(0)
    );
    b.rule_2("last <named-month|named-day>",
             b.reg(r#"(?:el|la )?pasad[oa]"#)?,
             datetime_check!(),
             |_, time| time.value().the_nth(1)
    );
    b.rule_2("<named-month|named-day> next",
             datetime_check!(),
             b.reg(r#"que vienen?|pr[oóò]xim[oa]"#)?,
             |time, _| time.value().the_nth_not_immediate(0)
    );
    b.rule_3("the <time> next",
             b.reg(r#"el|la"#)?,
             datetime_check!(),
             b.reg(r#"que vienen?|pr[oóò]xim[oa]"#)?,
             |_, time, _| time.value().the_nth(0)
    );

    b.rule_3("the <day-of-week> of next week",
             b.reg(r#"el"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"de la (?:semana que viene|pr[oóò]xima semana)"#)?,
             |_, time, _| time.value().the_nth(1)
    );
    b.rule_2("<day-of-week> of next week",
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"de la (?:semana que viene|pr[oóò]xima semana)"#)?,
             |time, _| time.value().the_nth(1)
    );
    b.rule_2("<named-month|named-day> past",
             datetime_check!(),
             b.reg(r#"pasad[oa]"#)?,
             |time, _| time.value().the_nth(-1)
    );
    b.rule_1("year",
             integer_check_by_range!(1000, 2100),
             |integer| {
                 helpers::year(integer.value().value as i32)
             }
    );
    b.rule_1("year (latent)",
             integer_check_by_range!(-1000, 999),
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
    b.rule_2("del <year>", //latin america mostly
             b.reg(r#"del(?: a[ñn]o)?"#)?,
             integer_check_by_range!(1000, 2100),
             |_, integer| helpers::year(integer.value().value as i32)
    );
    b.rule_1_terminal("day of month (1st)",
             b.reg(r#"el prim(?:er)?o|uno|prim\.?|1o"#)?,
             |_| helpers::day_of_month(1)
    );
    b.rule_2("el dia <day-of-month> (non ordinal) (not latent)",
             b.reg(r#"d?el(?: d[iíì]a)?"#)?,
             integer_check_by_range!(1, 31),
             |_, integer| Ok(helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_2("el <day-of-month> (non ordinal) (latent)",
             b.reg(r#"d?el"#)?,
             integer_check_by_range!(1, 31),
             |_, integer| Ok(helpers::day_of_month(integer.value().value as u32)?.latent())
    );
    b.rule_3("the <day-of-month> (ordinal)",
             b.reg(r#"el"#)?,
             ordinal_check!(),
             b.reg(r#"d[iíì]a"#)?,
             |_, ordinal, _| Ok((*ordinal.value()).prefixed()));
    b.rule_2("<day-of-month> <named-month>",
             integer_check_by_range!(1, 31),
             datetime_check!(form!(Form::Month(_))),
             |integer, month| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_3("<day-of-month> de <named-month>",
             integer_check_by_range!(1, 31),
             b.reg(r#"de(?:l mes de)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |integer, _, month| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_4("el <day-of-month> de <named-month>",
             b.reg(r#"el(?: d[iíì]a)?"#)?,
             integer_check_by_range!(1, 31),
             b.reg(r#"de(?:l mes de)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, integer, _, month| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_4("<day-of-week> <day-of-month> de <named-month>",
             datetime_check!(form!(Form::DayOfWeek{..})),
             integer_check_by_range!(1, 31),
             b.reg(r#"de(?:l mes de)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, integer, _, month| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_4("ultimo <day-of-week> de <time>",
             b.reg(r#"[ú|u]ltimo"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"de|en"#)?,
             datetime_check!(),
             |_, dow, _, time| dow.value().last_of(time.value())
    );
    b.rule_4("the <cycle> of <time>",
             b.reg(r#"el|la"#)?,
             cycle_check!(),
             b.reg(r#"del?"#)?,
             datetime_check!(),
             |_, cycle, _, time| helpers::cycle_nth_after_not_immediate(cycle.value().grain, 0, time.value())
    );
    b.rule_4("nth <time> de <time>",
             ordinal_check!(),
             datetime_check!(),
             b.reg(r#"del?|en"#)?,
             datetime_check!(),
             |ordinal, a, _, b| b.value().intersect(a.value())?.the_nth(ordinal.value().value - 1)
    );
    b.rule_5("the nth <time> de <time>",
             b.reg(r#"el|la"#)?,
             ordinal_check!(),
             datetime_check!(),
             b.reg(r#"de|en"#)?,
             datetime_check!(),
             |_, ordinal, a, _, b| b.value().intersect(a.value())?.the_nth(ordinal.value().value - 1)
    );
    b.rule_4("ultimo <cycle> de <time>",
             b.reg(r#"[ú|u]ltimo"#)?,
             cycle_check!(),
             b.reg(r#"de|en"#)?,
             datetime_check!(),
             |_, cycle, _, time| helpers::cycle_nth_after_not_immediate(cycle.value().grain, -1, time.value())
    );
    b.rule_4("nth <cycle> de <time>",
             ordinal_check!(),
             cycle_check!(),
             b.reg(r#"de|en"#)?,
             datetime_check!(),
             |ordinal, cycle, _, time| helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value - 1, time.value())
    );
    b.rule_5("the nth <cycle> de <time>",
             b.reg(r#"el|la"#)?,
             ordinal_check!(),
             cycle_check!(),
             b.reg(r#"del?|en"#)?,
             datetime_check!(),
             |_, ordinal, cycle, _, time| helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value - 1, time.value())
    );
    b.rule_3("<ordinal> week-end of <named-month>",
             ordinal_check!(),
             b.reg(r#"week[ -]?end|fin(?:de)?(?: de semana)? de(?:l mes de)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |ordinal, _, time| {
                 let week_day_start = helpers::day_of_week(Weekday::Fri)?.intersect(&helpers::hour(18, false)?)?;
                 let week_day_end = helpers::day_of_week(Weekday::Mon)?.intersect(&helpers::hour(0, false)?)?;
                 let week_day = week_day_start.span_to(&week_day_end, false)?;
                 let week_ends_of_time = time.value().intersect(&week_day)?;
                 week_ends_of_time.the_nth(ordinal.value().value - 1)
             }
    );
    b.rule_2("last week-end of <named-month>",
             b.reg(r#"[ú|u]ltimo (?:week[ -]?end|fin(?:de)?(?: de semana)?) de(?:l mes de)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, time| {
                 let week_day_start = helpers::day_of_week(Weekday::Fri)?.intersect(&helpers::hour(18, false)?)?;
                 let week_day_end = helpers::day_of_week(Weekday::Mon)?.intersect(&helpers::hour(0, false)?)?;
                 let week_day = week_day_start.span_to(&week_day_end, false)?;
                 week_day.last_of(time.value())
             }
    );

    b.rule_2("<named-month> <day-of-month>",
             datetime_check!(form!(Form::Month(_))),
             integer_check_by_range!(1, 31),
             |month, integer| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_3("el <day-of-week> <day-of-month>",
             b.reg(r#"el"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             integer_check_by_range!(1, 31),
             |_, dow, integer| dow.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_2("<day-of-week> <day-of-month>",
             datetime_check!(form!(Form::DayOfWeek{..})),
             integer_check_by_range!(1, 31),
             |dow, integer| dow.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_1("time-of-day (latent)",
             integer_check_by_range!(1, 23),
             |integer| Ok(helpers::hour(integer.value().value as u32, integer.value().value < 12)?.latent())
    );
    b.rule_1("time-of-day (latent)",
             integer_check_by_range!(0, 0),
             |_| Ok(helpers::hour(0, false)?.latent())
    );
    b.rule_1_terminal("noon",
                      b.reg(r#"mediod[iíi]a"#)?,
                      |_| helpers::hour(12, false)
    );
    b.rule_1_terminal("midnight",
                      b.reg(r#"medianoche|las doce de la noche"#)?,
                      |_| helpers::hour(0, false)
    );
    b.rule_3("la <time-of-day> horas",
             b.reg(r#"las?"#)?,
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))),
             b.reg(r#"h(?:\.|oras)?"#)?,
             |_, time, _| Ok(time.value().clone().not_latent())
    );
    b.rule_2("la <time-of-day>",
             b.reg(r#"las?"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, time| Ok(time.value().clone().not_latent())
    );
    b.rule_2("a las <time-of-day>",
             b.reg(r#"a(?: las)?"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, tod| Ok(tod.value().clone().not_latent())
    );
    b.rule_3("a las <hour-min>(time-of-day)",
             b.reg(r#"a(?: las)?"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"h(?:\.|oras)?"#)?,
             |_, tod, _| Ok(tod.value().clone().not_latent())
    );
    b.rule_1_terminal("hh(:|h)mm (time-of-day)",
                      b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:h\.]([0-5]\d)"#)?,
                      |text_match| {
                          let hour: u32 = text_match.group(1).parse()?;
                          let minute: u32 = text_match.group(2).parse()?;
                          helpers::hour_minute(hour, minute, hour < 12)
                      }
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
    b.rule_1_terminal("quarter (relative minutes)",
                      b.reg(r#"(?:un )?(?:cuarto|1/4)(?: de hora)?"#)?,
                      |_| Ok(RelativeMinuteValue(15))
    );
    b.rule_1_terminal("half (relative minutes)",
                      b.reg(r#"media"#)?,
                      |_| Ok(RelativeMinuteValue(30))
    );
    b.rule_1_terminal("3 quarter (relative minutes)",
                      b.reg(r#"(3|tres) cuartos?(?: de hora)?"#)?,
                      |_| Ok(RelativeMinuteValue(45))
    );
    b.rule_1("number (as relative minutes)",
             integer_check_by_range!(1, 59),
             |integer| Ok(RelativeMinuteValue(integer.value().value as i32))
    );
    b.rule_2("<integer> minutes (as relative minutes)",
             integer_check_by_range!(1, 59),
             b.reg(r#"min\.?(?:uto)?s?"#)?,
             |integer, _| Ok(RelativeMinuteValue(integer.value().value as i32))
    );
    b.rule_2("<integer> minutes (as relative minutes)",
             b.reg(r#"y"#)?,
             integer_check_by_range!(1, 59),
             |_, integer| Ok(RelativeMinuteValue(integer.value().value as i32))
    );
    b.rule_2("<hour-of-day> <integer> (as relative minutes)",
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))),
             relative_minute_check!(),
             |time, relative_minute| helpers::hour_relative_minute(
                 time.value().form_time_of_day()?.full_hour(),
                 relative_minute.value().0,
                 time.value().form_time_of_day()?.is_12_clock())
    );
    b.rule_3("<hour-of-day> minus <integer> (as relative minutes)",
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))),
             b.reg(r#"menos\s?"#)?,
             relative_minute_check!(),
             |time, _, relative_minute| helpers::hour_relative_minute(
                 time.value().form_time_of_day()?.full_hour(),
                 -1 * relative_minute.value().0,
                 time.value().form_time_of_day()?.is_12_clock())
    );
    b.rule_3("<hour-of-day> and <relative minutes>",
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))),
             b.reg(r#"y"#)?,
             relative_minute_check!(),
             |time, _, relative_minute| helpers::hour_relative_minute(
                 time.value().form_time_of_day()?.full_hour(),
                 relative_minute.value().0,
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
    b.rule_1_terminal("dd/mm",
                      b.reg(r#"(0?[1-9]|[12]\d|3[01])[\./](1[0-2]|0?[1-9])"#)?,
                      |text_match| helpers::month_day(
                          text_match.group(2).parse()?,
                          text_match.group(1).parse()?)
    );
    // End of Written dates in numeric formats
    b.rule_1_terminal("beginning of day",
                      b.reg(r#"al (?:inicio|empezar) d?el d[iíì]a|a primera hora"#)?,
                      |_| {
                          Ok(helpers::hour(6, false)?
                              .span_to(&helpers::hour(10, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Morning)))
                      }
    );
    b.rule_1_terminal("middle of day",
                      b.reg(r#"(?:al?|en) (?:la mitad|medio) (?:del )?d[iíì]a|a mediod[iíì]a"#)?,
                      |_| {
                          Ok(helpers::hour(11, false)?
                              .span_to(&helpers::hour(16, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::None)))
                      }
    );
    b.rule_1_terminal("end of day",
                      b.reg(r#"al (?:final|acabar) d?el d[iíì]a|a última hora"#)?,
                      |_| {
                          Ok(helpers::hour(17, false)?
                              .span_to(&helpers::hour(21, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );

    b.rule_1_terminal("morning",
                      b.reg(r#"ma[ñn]ana"#)?,
                      |_| Ok(helpers::hour(4, false)?.span_to(&helpers::hour(12, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Morning))
                          .latent())
    );
    b.rule_1_terminal("beginning of morning",
                      b.reg(r#"(?:pronto por|a primera hora (?:por|de)) la mañana|la mañana a primera hora"#)?,
                      |_| Ok(helpers::hour(4, false)?
                          .span_to(&helpers::hour(9, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Morning)))
    );
    b.rule_1_terminal("end of morning",
                      b.reg(r#"[uúù]ltima hora de la mañana|la mañana a [uúù]ltima hora"#)?,
                      |_| Ok(helpers::hour(10, false)?
                          .span_to(&helpers::hour(12, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Morning)))
    );
    b.rule_1_terminal("lunch",
                      b.reg(r#"(?:la )?(?:comida|hora de comer)"#)?,
                      |_| Ok(helpers::hour(12, false)?
                          .span_to(&helpers::hour(14, false)?, false)?
                          .latent()
                          .form(Form::Meal))
    );
    b.rule_1_terminal("after lunch",
                      b.reg(r#"despu[eéè]s de (?:la hora de )?comer"#)?,
                      |_| {
                          let period = helpers::hour(13, false)?
                              .span_to(&helpers::hour(17, false)?, false)?;
                          Ok(helpers::cycle_nth(Grain::Day, 0)?.intersect(&period)?.form(Form::PartOfDay(PartOfDayForm::Afternoon)))
                      }
    );
    b.rule_1_terminal("before lunch",
                      b.reg(r#"antes de comer"#)?,
                      |_| {
                          let period = helpers::hour(10, false)?
                              .span_to(&helpers::hour(12, false)?, false)?;
                          Ok(helpers::cycle_nth(Grain::Day, 0)?.intersect(&period)?.form(Form::PartOfDay(PartOfDayForm::Morning)))
                      }
    );
    b.rule_1_terminal("after work",
                      b.reg(r#"después del trabajo|al salir de trabajar"#)?,
                      |_| {
                          let period = helpers::hour(17, false)?
                              .span_to(&helpers::hour(21, false)?, false)?;
                          Ok(helpers::cycle_nth(Grain::Day, 0)?.intersect(&period)?.form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );
    b.rule_1_terminal("afternoon",
                      b.reg(r#"(?:la )?tarde"#)?,
                      |_| Ok(helpers::hour(12, false)?.span_to(&helpers::hour(19, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Afternoon))
                          .latent())
    );
    b.rule_1_terminal("middle afternoon",
                      b.reg(r#"(?:media|plena) tarde"#)?,
                      |_| {
                          Ok(helpers::hour(15, false)?
                              .span_to(&helpers::hour(17, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
                      }
    );
    b.rule_1_terminal("middle of day",
                      b.reg(r#"(?:la )?(?:mitad|medio) ?(?:del )?d[iíì]a"#)?,
                      |_| {
                          Ok(helpers::hour(11, false)?
                              .span_to(&helpers::hour(16, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::None)))
                      }
    );
    b.rule_1_terminal("evening",
                      b.reg(r#"(?:la )?noche"#)?,
                      |_| Ok(helpers::hour(18, false)?.span_to(&helpers::hour(0, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Evening))
                          .latent())
    );
    b.rule_1_terminal("beginning of evening",
                      b.reg(r#"(?:primera hora de|pronto por) la (?:tarde|noche)|por la (?:tarde|noche) (?:a primera hora|pronto)"#)?,
                      |_| {
                          Ok(helpers::hour(18, false)?
                              .span_to(&helpers::hour(21, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );
    b.rule_1_terminal("end of evening",
                      b.reg(r#"tarde por la noche|por la (?:noche|tarde) a [uúù]ltima hora|(?:al final|(?:a|en las?) [uúù]ltimas? horas?) de la (?:velada|tarde)"#)?,
                      |_| {
                          Ok(helpers::hour(21, false)?
                              .span_to(&helpers::hour(0, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );
    b.rule_2("in the <part-of-day>",
             b.reg(r#"(?:a|en|de|por|durante)"#)?,
             datetime_check!(|time: &TimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
             |_, pod| Ok(pod.value().clone().not_latent())
    );
    b.rule_2("this <part-of-day>",
             b.reg(r#"est(?:e|a)"#)?,
             datetime_check!(|time: &TimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
             |_, pod| Ok(helpers::cycle_nth(Grain::Day, 0)?
                 .intersect(pod.value())?
                 .form(pod.value().form.clone()))
    );
    b.rule_1_terminal("del mediodía",
                      b.reg(r#"del mediod[ií]a"#)?,
                      |_| Ok(helpers::hour(12, false)?.span_to(&helpers::hour(17, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Afternoon))
                          .latent())
    );
    b.rule_2("<time-of-day> <part-of-day>",
             datetime_check!(excluding_form!(Form::PartOfDay(_))),
             datetime_check!(|time: &TimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
             |a, b| a.value().intersect(b.value())
    );
    b.rule_3("<time-of-day> prep <part-of-day>",
             datetime_check!(excluding_form!(Form::TimeOfDay(_))),
             b.reg(r#"por(?: la| el)?"#)?,
             datetime_check!(|time: &TimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
             |a, _, b| a.value().intersect(b.value())
    );
    b.rule_2("<dim time> de la tarde",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:a|en|de) la tarde"#)?,
             |time, _| {
                 let period = helpers::hour(12, false)?
                     .span_to(&helpers::hour(21, false)?, false)?;
                 time.value().intersect(&period)
             }
    );
    b.rule_2("<dim time> de la manana",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:a|en|de) la (?:ma[ñn]ana|madrugada)"#)?,
             |time, _| {
                 let period = helpers::hour(0, false)?
                     .span_to(&helpers::hour(12, false)?, false)?;
                 time.value().intersect(&period)
             }
    );
    b.rule_2("<dim time> in the evening",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"de la (?:media)?noche"#)?,
             |a, _| {
                 let period = helpers::hour(16, false)?
                     .span_to(&helpers::hour(0, false)?, false)?;
                 a.value().intersect(&period)
             }
    );
    b.rule_3("<integer> in the <part-of-day>",
             datetime_check!(|time: &TimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
             b.reg(r#"(?:a|en|de|por) la"#)?,
             datetime_check!(),
             |pod, _, time| time.value().intersect(pod.value())
    );
    b.rule_3("the <day-of-month> at <datetime>",
             b.reg(r#"el(?: d[iíì]a)?"#)?,
             integer_check_by_range!(1, 31),
             datetime_check!(),
             |_, integer, time| {
                 let day_of_month = helpers::day_of_month(integer.value().value as u32)?;
                 day_of_month.intersect(&time.value())
             }
    );
    b.rule_1_terminal("beginning of week",
                      b.reg(r#"a principios de (?:esta )?semana"#)?,
                      |_| helpers::day_of_week(Weekday::Mon)
                          ?.span_to(&helpers::day_of_week(Weekday::Tue)?, false)
    );
    b.rule_1_terminal("middle of week",
                      b.reg(r#"a (?:mitad de|media) semana"#)?,
                      |_| helpers::day_of_week(Weekday::Wed)
                          ?.span_to(&helpers::day_of_week(Weekday::Thu)?, false)
    );
    b.rule_1_terminal("end of week (not weekend)",
                      b.reg(r#"a finales de la semana"#)?,
                      |_| helpers::day_of_week(Weekday::Thu)
                          ?.span_to(&helpers::day_of_week(Weekday::Sun)?, false)
    );
    b.rule_1_terminal("during the week",
                      b.reg(r#"durante la semana"#)?,
                      |_| helpers::day_of_week(Weekday::Mon)
                          ?.span_to(&helpers::day_of_week(Weekday::Fri)?, false)
    );
    b.rule_1_terminal("week-end",
                      b.reg(r#"week[ -]?end|fin(?:de)?(?: de semana)?"#)?,
                      |_| {
                          let friday = helpers::day_of_week(Weekday::Fri)?
                              .intersect(&helpers::hour(18, false)?)?;
                          let monday = helpers::day_of_week(Weekday::Mon)?
                              .intersect(&helpers::hour(0, false)?)?;
                          friday.span_to(&monday, false)
                      }
    );
    b.rule_1_terminal("season",
                      b.reg(r#"verano"#)?,
                      |_| helpers::month_day(6, 21)?
                          .span_to(&helpers::month_day(9, 23)?, false)
    );
    b.rule_1_terminal("season",
                      b.reg(r#"oto[ñn]o"#)?,
                      |_| helpers::month_day(9, 23)?
                          .span_to(&helpers::month_day(12, 21)?, false)
    );
    b.rule_1_terminal("season",
                      b.reg(r#"invierno"#)?,
                      |_| helpers::month_day(12, 21)?
                          .span_to(&helpers::month_day(3, 20)?, false)
    );
    b.rule_1_terminal("season",
                      b.reg(r#"primavera"#)?,
                      |_| helpers::month_day(3, 20)?
                          .span_to(&helpers::month_day(6, 21)?, false)
    );
    b.rule_2("el <time>",
             b.reg(r#"el|la"#)?,
             datetime_check!(|time: &TimeValue| !time.latent),
             |_, time| Ok(time.value().clone())
    );
    b.rule_5("dd-dd <month>(interval)",
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             b.reg(r#"\-|al?"#)?,
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             b.reg(r#"de(?:l mes de)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |a, _, b, _, month| {
                 let start = month.value().intersect(&helpers::day_of_month(a.group(1).parse()?)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(b.group(1).parse()?)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_4("dd-dd <month>(interval)",
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             b.reg(r#"\-|al?"#)?,
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             datetime_check!(form!(Form::Month(_))),
             |a, _, b, month| {
                 let start = month.value().intersect(&helpers::day_of_month(a.group(1).parse()?)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(b.group(1).parse()?)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_6("dd-dd <month>(interval)",
             b.reg(r#"entre(?: el)?"#)?,
             b.reg(r#"(0?[1-9]|[12]\d|3[01])"#)?,
             b.reg(r#"y(?: el)?"#)?,
             b.reg(r#"(0?[1-9]|[12]\d|3[01])"#)?,
             b.reg(r#"de(?:l mes de)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, a, _, b, _, month| {
                 let start = month.value().intersect(&helpers::day_of_month(a.group(1).parse()?)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(b.group(1).parse()?)?)?;
                 start.span_to(&end, true)
             }
    );
    // FIXME: not working
    b.rule_4("dd <month> - dd <month>(interval)",
             b.reg(r#"(0?[1-9]|[12]\d|3[01])"#)?,
             datetime_check!(form!(Form::Month(_))),
             b.reg(r#" *- *(0?[1-9]|[12]\d|3[01])"#)?,
             datetime_check!(form!(Form::Month(_))),
             |a, b, c, d| {
        let start = b.value().intersect(&helpers::day_of_month(a.group(1).parse()?)?)?;
        let end = d.value().intersect(&helpers::day_of_month(c.group(1).parse()?)?)?;
        start.span_to(&end, true)
    }
    );

    b.rule_4("<datetime> - <datetime> (interval)",
             b.reg(r#"del"#)?,
             datetime_check!(),
             b.reg(r#"\-|al?"#)?,
             datetime_check!(),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );
    b.rule_4("between <datetime> and <datetime> (interval)",
             b.reg(r#"entre"#)?,
             datetime_check!(),
             b.reg(r#"y"#)?,
             datetime_check!(),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );
    b.rule_4("from <time-of-day> to <time-of-day> (interval)",
             b.reg(r#"de(?: las?)?"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"a(?: las?)?"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );
    b.rule_2("before <datetime>",
             b.reg(r#"antes de|hasta"#)?,
             datetime_check!(),
             |_, time| Ok(time.value().clone().mark_before_end())
    );
    b.rule_2("during <duration>",
             b.reg(r#"(?:durante|por|todo) (?:el|la|una?)"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().prefixed())
    );
    b.rule_2("during <duration>",
             b.reg(r#"(?:durante|por|todo)"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().prefixed())
    );
    b.rule_2("exactly <duration>",
             b.reg(r#"(?:precis|exact)amente|justo"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().prefixed().precision(Precision::Exact))
    );
    b.rule_2("<duration> exactly",
             duration_check!(),
             b.reg(r#"(?:precis|exact)amente|justo"#)?,
             |duration, _| Ok(duration.value().clone().prefixed().precision(Precision::Exact))
    );
    b.rule_2("approx <duration>",
             b.reg(r#"sobre|cerca de"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().prefixed().precision(Precision::Approximate))
    );
    b.rule_2("<duration> approx",
             duration_check!(),
             b.reg(r#"m[aáà]s o menos|aproximadamente"#)?,
             |duration, _| Ok(duration.value().clone().prefixed().precision(Precision::Approximate))
    );
    b.rule_2("approx <time-of-day>",
             b.reg(r#"sobre|cerca de|hacia"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, a| Ok(a.value().clone().not_latent())
    );
    b.rule_2("<time-of-day> approx",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"m[aáà]s o menos|aproximadamente"#)?,
             |a, _| Ok(a.value().clone().not_latent())
    );
    b.rule_2("from <time-of-day>",
             b.reg(r#"(?:a partir|despu[eéè]s) del?|desde"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, time| Ok(time.value().clone().mark_after_start())
    );
    b.rule_3("from <time-of-day> on",
             b.reg(r#"(?:a partir|despu[eéè]s) del?|desde"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"en adelante"#)?,
             |_, time, _| Ok(time.value().clone().mark_after_start())
    );
    b.rule_2("(from) <time-of-day> on",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"en adelante"#)?,
             |time, _| Ok(time.value().clone().mark_after_start())
    );
    b.rule_2("from <datetime>",
             b.reg(r#"(?:a partir|despu[eéè]s) del?|desde"#)?,
             datetime_check!(excluding_form!(Form::TimeOfDay(_))),
             |_, time| Ok(time.value().clone().mark_after_start())
    );
    b.rule_3("from <datetime> on",
             b.reg(r#"(?:a partir|despu[eéè]s) del?|desde"#)?,
             datetime_check!(excluding_form!(Form::TimeOfDay(_))),
             b.reg(r#"en adelante"#)?,
             |_, time, _| Ok(time.value().clone().mark_after_start())
    );
    b.rule_2("(from) <datetime> on",
             datetime_check!(excluding_form!(Form::TimeOfDay(_))),
             b.reg(r#"en adelante"#)?,
             |time, _| Ok(time.value().clone().mark_after_start())
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
             b.reg(r#"(?:grados?)|°"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("degree"),
                     latent: false,
                 })
             });
    b.rule_2("<temp> Celcius",
             temperature_check!(),
             b.reg(r#"(?:cent(?:i|í)grados?|c(?:el[cs]?(?:ius)?)?\.?)"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("celsius"),
                     latent: false,
                 })
             });
    // FIXME: Check double Kelvin removal
    b.rule_2("<temp> Kelvin",
             temperature_check!(),
             b.reg(r#"k(?:elvin)?"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("kelvin"),
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
             b.reg(r#"bajo cero"#)?,
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
                      b.reg(r#"(die[cs]i(?:s[eéè]is|siete|ocho|nueve)|veinti(?:un[oa]|d[oó]s|tr[eéè]s|cuatro|cinco|s[eéè]is|siete|ocho|nueve))"#)?,
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
    b.rule_2("number millions",
             integer_check_by_range!(1, 999),
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
                      b.reg(r#"d[eé]cimo? ?(primer|segund|tercer|cuart|quint|sext|s[eéè]ptim|octav|noven)(?:[oa]s?)?"#)?,
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
