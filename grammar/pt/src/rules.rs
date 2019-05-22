use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::dimension::Precision::*;
use rustling_ontology_values::helpers;
use rustling_ontology_moment::{Weekday, Grain, PeriodComp, Period};

pub fn rules_percentage(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("<number> per cent",
        number_check!(),
        b.reg(r#"(?:%|por ?cento)"#)?,
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
             b.reg(r#"e"#)?,
             amount_of_money_check!(|money: &AmountOfMoneyValue| money.unit == Some("cent")),
             |a, _, b| helpers::compose_money(&a.value(), &b.value()));
    b.rule_2("intersect",
             amount_of_money_check!(|money: &AmountOfMoneyValue| money.unit != Some("cent")),
             number_check!(),
             |a, b| helpers::compose_money_number(&a.value(), &b.value()));

    b.rule_1_terminal("$",
        b.reg(r#"\$|d[oó]lar(?:es)?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("$") })
    );
    b.rule_1_terminal("EUR",
        b.reg(r#"€|euros?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("EUR") })
    );
    b.rule_1_terminal("£",
        b.reg(r#"£|libras?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("£") })
    );
    b.rule_1_terminal("GBP",
                      b.reg(r#"gbp|libras? esterlinas?|libras? inglesas?|libras? britânicas?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("GBP") })
    );
    b.rule_1_terminal("USD",
        b.reg(r#"d[oó]lar(?:es)? americanos?|d[oó]lar(?:es)? estadunidenses?|us$|usd"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("USD") })
    );
    b.rule_1_terminal("CAD",
                      b.reg(r#"d[oó]lar(?:es)? canadenses?|cad"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("CAD") })
    );
    b.rule_1_terminal("AUD",
                      b.reg(r#"d[oó]lar(?:es) australianos?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("AUD") })
    );
    b.rule_1_terminal("Bitcoin",
        b.reg(r#"฿|bitcoins?|btc|xbt"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("฿") })
    );
    b.rule_1_terminal("JPY",
                      b.reg(r#"jpy|[yi]en(?:es)?(?: japoneses?)?|ienes?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("JPY") })
    );
    b.rule_1_terminal("¥",
                      b.reg(r#"¥"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("¥") })
    );
    b.rule_1_terminal("₽",
                          b.reg(r#"₽"#)?,
                          |_| Ok(MoneyUnitValue { unit: Some("₽") })
    );
    b.rule_1_terminal("KRW",
                      b.reg(r#"krw|₩|won(?:es)? (?:sul[- ])?coreanos?|won(?:es)?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("KRW") })
    );
    b.rule_1_terminal("RMB|CNH|CNY",
                      b.reg(r#"yuan(?:es)?(?: chineses?)?|renminbis?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("CNY") })
    );
    b.rule_1_terminal("INR",
                      b.reg(r#"r[uú]pias?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("INR") })
    );
    b.rule_1_terminal("INR",
                          b.reg(r#"r[uú]pias? indianas?"#)?,
                          |_| Ok(MoneyUnitValue { unit: Some("INR") })
        );
    b.rule_1_terminal("HKD",
                      b.reg(r#"d[oó]lar(?:es) de hong kong"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("HKD") })
    );
    b.rule_1_terminal("CHF",
                      b.reg(r#"francos? su[íi]ços?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("CHF") })
    );
    b.rule_1_terminal("KR",
                      b.reg(r#"kr|coroas?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("KR") })
    );
    b.rule_1_terminal("DKK",
                      b.reg(r#"dkk|coroas? dinamarquesas?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("DKK") })
    );
    b.rule_1_terminal("NOK",
                      b.reg(r#"nok|coroas? norueguesas?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("NOK") })
    );
    b.rule_1_terminal("SEK",
                      b.reg(r#"sek|coroas? suecas?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("SEK") })
    );

    b.rule_1_terminal("RUB",
                      b.reg(r#"rub"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("RUB") })
    );
    b.rule_1_terminal("RUB",
                      b.reg(r#"rublos?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("RUB") })
    );
    b.rule_1_terminal("RUB",
                      b.reg(r#"rublos? russos?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("RUB") })
    );

    b.rule_1_terminal("cent",
                      b.reg(r#"centavos?"#)?,
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
             b.reg(r#"quase|aproximadamente|cerca de|por cerca de|por volta de|em torno de|uns|umas"#)?,
             amount_of_money_check!(),
             |_, a| {
                 Ok(AmountOfMoneyValue {
                     precision: Approximate,
                     ..a.value().clone()
                 })
             });
    b.rule_2("<amount-of-money> about",
             amount_of_money_check!(),
             b.reg(r#"aproximadamente|mais ou menos"#)?,
             |a, _| {
                 Ok(AmountOfMoneyValue {
                     precision: Approximate,
                     ..a.value().clone()
                 })
             });
    b.rule_2("exactly <amount-of-money>",
             b.reg(r#"exatamente|precisamente"#)?,
             amount_of_money_check!(),
             |_, a| {
                 Ok(AmountOfMoneyValue {
                     precision: Exact,
                     ..a.value().clone()
                 })
             });
    b.rule_2("<amount-of-money> exactly",
             amount_of_money_check!(),
             b.reg(r#"exatamente|precisamente|exatos"#)?,
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
                      b.reg(r#"segundos?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Second))
    );
    b.rule_1_terminal("minute (unit-of-duration)",
                      b.reg(r#"minutos?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Minute))
    );
    b.rule_1_terminal("hour (unit-of-duration)",
                      b.reg(r#"horas?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Hour))
    );
    b.rule_1_terminal("day (unit-of-duration)",
                      b.reg(r#"dias?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Day))
    );
    b.rule_1_terminal("week (unit-of-duration)",
                      b.reg(r#"semanas?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Week))
    );
    b.rule_1_terminal("month (unit-of-duration)",
                      b.reg(r#"m[eê]s(?:es)?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Month))
    );
    b.rule_1_terminal("year (unit-of-duration)",
                      b.reg(r#"anos?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Year))
    );
    b.rule_1_terminal("trimester (unit-of-duration)",
                      b.reg(r#"trimestres?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Quarter))
    );
    b.rule_1_terminal("quarter of an hour",
                      b.reg(r#"quarto de hora"#)?,
                      |_| Ok(DurationValue::new(PeriodComp::minutes(15).into()))
    );
    b.rule_1_terminal("half an hour",
                      b.reg(r#"meia hora"#)?,
                      |_| Ok(DurationValue::new(PeriodComp::minutes(30).into()))
    );
    b.rule_1_terminal("three-quarters of an hour",
                      b.reg(r#"tr[eê]s quartos de hora"#)?,
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
             b.reg(r#"e mei[ao]"#)?,
             |integer, uod, _| {
                 let half_period: Period = uod.value().grain.half_period().map(|a| a.into()).ok_or_else(|| RuleError::Invalid)?;
                 Ok(DurationValue::new(half_period + PeriodComp::new(uod.value().grain, integer.value().value)))
             }
    );
//    b.rule_3("<integer> <unit-of-duration> and a quarter",
//             integer_check_by_range!(0),
//             unit_of_duration_check!(),
//             b.reg(r#""#)?,
//             |integer, uod, _| {
//                 let half_period: Period = uod.value().grain.half_period().map(|a| a.into()).ok_or_else(|| RuleError::Invalid)?;
//                 Ok(DurationValue::new(half_period + PeriodComp::new(uod.value().grain, integer.value().value)))
//             }
//    );
    b.rule_2("in <duration> (future moment)",
             b.reg(r#"em"#)?,
             duration_check!(),
             |_, duration| duration.value().in_present()
    );
    b.rule_3("<duration> y <duration>",
             duration_check!(|duration: &DurationValue| !duration.suffixed),
             b.reg(r#"e"#)?,
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
             duration_check!(),
             b.reg(r#"atr[aá]s"#)?,
             |duration, _| duration.value().ago()
    );
    b.rule_2("<duration> later",
             duration_check!(),
             b.reg(r#"depois"#)?,
             |duration, _| duration.value().in_present()
    );
    b.rule_2("during <duration>",
             b.reg(r#"por|durante"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().prefixed())
    );
    b.rule_2("approx <duration>",
             b.reg(r#"aproximadamente|cerca de|por cerca de|por volta de|em torno de"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().precision(Precision::Approximate))
    );
    b.rule_2("approx <duration>",
             duration_check!(),
             b.reg(r#"aproximadamente"#)?,
             |duration, _| Ok(duration.value().clone().precision(Precision::Approximate))
    );
    b.rule_2("precisely <duration>",
             b.reg(r#"exactamente|precisamente"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().precision(Precision::Exact))
    );
    b.rule_2("precisely <duration>",
             duration_check!(),
             b.reg(r#"exactamente|precisamente"#)?,
             |duration , _| Ok(duration.value().clone().precision(Precision::Exact))
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
                      b.reg(r#"dias?"#)?,
                      |_| CycleValue::new(Grain::Day)
    );
    b.rule_1_terminal("week (cycle)",
                      b.reg(r#"semanas?"#)?,
                      |_| CycleValue::new(Grain::Week)
    );
    b.rule_1_terminal("month (cycle)",
                      b.reg(r#"m[eê]s(?:es)?"#)?,
                      |_| CycleValue::new(Grain::Month)
    );
    b.rule_1_terminal("year (cycle)",
                      b.reg(r#"anos?"#)?,
                      |_| CycleValue::new(Grain::Year)
    );
    b.rule_1_terminal("trimester (cycle)",
                          b.reg(r#"trimestres?"#)?,
                          |_| CycleValue::new(Grain::Year)
    );
    b.rule_2("next <cycle> ",
             b.reg(r#"próxim[oa]s?"#)?,
             cycle_check!(),
             |_, cycle| helpers::cycle_nth(cycle.value().grain, 1)
    );
    b.rule_2("<cycle> next",
             cycle_check!(),
             b.reg(r#"que vem"#)?,
             |cycle, _| helpers::cycle_nth(cycle.value().grain, 1)
    );
    b.rule_2("last <cycle> ",
             b.reg(r#"hà"#)?,
             cycle_check!(),
             |_, cycle| helpers::cycle_nth(cycle.value().grain, -1)
    );
    b.rule_2("<cycle> last",
             cycle_check!(),
             b.reg(r#"atr[aá]s"#)?,
             |cycle, _| helpers::cycle_nth(cycle.value().grain, -1)
    );
    b.rule_3("in <cycle>",
             b.reg(r#"daqui a|dentro de"#)?,
             integer_check_by_range!(2, 9999),
             cycle_check!(),
             |_, integer, cycle| helpers::cycle_nth(cycle.value().grain, integer.value().value)
    );
    Ok(())
}

pub fn rules_time(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("intersect",
             time_check!(|time: &TimeValue| !time.latent),
             time_check!(|time: &TimeValue| !time.latent),
             |a, b| a.value().intersect(b.value())
    );
    b.rule_3("intersect by `de`",
             time_check!(|time: &TimeValue| !time.latent),
             b.reg(r#"de"#)?,
             time_check!(|time: &TimeValue| !time.latent),
             |a, _, b| a.value().intersect(b.value())
    );
    b.rule_3("two time tokens separated by \",\"",
             time_check!(|time: &TimeValue| !time.latent),
             b.reg(r#","#)?,
             time_check!(|time: &TimeValue| !time.latent),
             |a, _, b| a.value().intersect(b.value())
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"segunda(?:[- ]feira)?"#)?,
                      |_| helpers::day_of_week(Weekday::Mon)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"ter[cç]a(?:[- ]feira)?"#)?,
                      |_| helpers::day_of_week(Weekday::Tue)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"quarta(?:[- ]feira)?"#)?,
                      |_| helpers::day_of_week(Weekday::Wed)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"quinta(?:[- ]feira)?"#)?,
                      |_| helpers::day_of_week(Weekday::Thu)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"sexta(?:[- ]feira)?"#)?,
                      |_| helpers::day_of_week(Weekday::Fri)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"s[aá]bado"#)?,
                      |_| helpers::day_of_week(Weekday::Sat)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"domingo"#)?,
                      |_| helpers::day_of_week(Weekday::Sun)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"janeiro"#)?,
                      |_| helpers::month(1)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"fevereiro"#)?,
                      |_| helpers::month(2)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"mar[cç]o"#)?,
                      |_| helpers::month(3)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"abril"#)?,
                      |_| helpers::month(4)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"maio"#)?,
                      |_| helpers::month(5)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"junho"#)?,
                      |_| helpers::month(6)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"julho"#)?,
                      |_| helpers::month(7)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"agosto"#)?,
                      |_| helpers::month(8)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"setembro"#)?,
                      |_| helpers::month(9)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"outubro"#)?,
                      |_| helpers::month(10)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"novembro"#)?,
                      |_| helpers::month(11)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"dezembro"#)?,
                      |_| helpers::month(12)
    );
    b.rule_1_terminal("right now",
                      b.reg(r#"agora mesmo"#)?,
                      |_| helpers::cycle_nth(Grain::Second, 0)
    );
    b.rule_1_terminal("now",
                      b.reg(r#"agora|neste momento"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 0)
    );
    // Date
    b.rule_1_terminal("Christmas",
                      b.reg(r#"natal"#)?,
                      |_| helpers::month_day(12, 25)
    );
    // Date
    b.rule_1_terminal("today",
                      b.reg(r#"hoje"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 0)
    );
    // Date
    b.rule_1_terminal("tomorrow",
                      b.reg(r#"amanh[aã]"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 1)
    );
    // Date
    b.rule_1_terminal("yesterday",
                      b.reg(r#"ontem"#)?,
                      |_| helpers::cycle_nth(Grain::Day, -1)
    );
    // Date
    b.rule_1_terminal("the day after tomorrow",
                      b.reg(r#"o dia depois de amanhã"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 2)
    );
    // Date
    b.rule_1_terminal("the day before yesterday",
                      b.reg(r#"anteontem"#)?,
                      |_| helpers::cycle_nth(Grain::Day, -2)
    );
    // Date
    b.rule_2("this <day-of-week>", //assumed to be in the future
             b.reg(r#"est[ea]"#)?,
             time_check!(form!(Form::DayOfWeek{..})),
             |_, time| time.value().the_nth_not_immediate(0)
    );
    // DateTime
    b.rule_2("this <datetime>",
             b.reg(r#"est[ea]|próximo"#)?,
             time_check!(),
             |_, time| time.value().the_nth(0)
    );
    b.rule_2("in <datetime>",
             b.reg(r#"durante|em|para(?: o)?|n[oa]"#)?,
             time_check!(),
             |_, a| Ok(a.value().clone())
    );
    b.rule_2("in <part-of-day>",
             b.reg(r#"pela"#)?,
             time_check!(|time: &TimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
             |_, a| Ok(a.value().clone())
    );
    b.rule_2("last <time>",
             b.reg(r#"hà"#)?,
             time_check!(),
             |_, a| {
                 a.value().the_nth(-1)
             }
    );
    // Date-period
    b.rule_2("beginning <named-month>(interval)",
             b.reg(r#"o? (começo|início) de"#)?,
             time_check!(form!(Form::Month(_))),
             |_, month| {
                 let start = month.value().intersect(&helpers::day_of_month(1)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(5)?)?;
                 start.span_to(&end, true)
             }
    );
    // Date-period
    b.rule_1_terminal("end of month",
                      b.reg(r#"o fim do mês"#)?,
                      |_| {
                          let month = helpers::cycle_nth(Grain::Month, 1)?;
                        Ok(helpers::cycle_nth_after(Grain::Day, -10, &month)?
                            .span_to(&month, false)?
                            .latent()
                            .form(Form::PartOfMonth))
                    }
    );
    // Date-period
    b.rule_2("end <named-month>(interval)",
             b.reg(r#"o fim de"#)?,
             time_check!(form!(Form::Month(_))),
             |_, month| {
                 let start = month.value().intersect(&helpers::day_of_month(25)?)?;
                 let end = helpers::cycle(Grain::Day)?.last_of(month.value())?;
                 start.span_to(&end, true)
             }
    );
    // Date period
    b.rule_2("next <named-month>",
             b.reg(r#"o próximo mês de"#)?,
             time_check!(form!(Form::Month(_))),
             |_, time| time.value().the_nth_not_immediate(0)
    );
    // Date period
    b.rule_2("for next <named-month>",
             b.reg(r#"(para|durante) o próximo mês de"#)?,
             time_check!(form!(Form::Month(_))),
             |_, time| time.value().the_nth_not_immediate(0)
    );
    // Date period
    b.rule_2("last <named-month>",
             b.reg(r#"o último mês de"#)?,
             time_check!(form!(Form::Month(_))),
             |_, time| time.value().the_nth(-1)
    );
    // Date period
    b.rule_2("for last <named-month>",
             b.reg(r#"(para|durante) o último mês de"#)?,
             time_check!(form!(Form::Month(_))),
             |_, time| time.value().the_nth(-1)
    );
    // Date
    b.rule_2("next <named-day>",
             b.reg(r#"no próximo"#)?,
             time_check!(form!(Form::DayOfWeek{..})),
             |_, time| time.value().the_nth_not_immediate(0)
    );
    // Date
    b.rule_2("for next <named-day>",
             b.reg(r#"(para|nest[ea]) no próximo"#)?,
             time_check!(form!(Form::DayOfWeek{..})),
             |_, time| time.value().the_nth_not_immediate(0)
    );
    // Date
    b.rule_2("last <named-day>",
             b.reg(r#"na última"#)?,
             time_check!(form!(Form::DayOfWeek{..})),
             |_, time| time.value().the_nth(-1)
    );
    // Date
    b.rule_2("for last <named-day>",
             b.reg(r#"(para|nest[ea]) na última"#)?,
             time_check!(form!(Form::DayOfWeek{..})),
             |_, time| time.value().the_nth(-1)
    );
    // Date-Period
    b.rule_1("year",
             integer_check_by_range!(1000, 2100),
             |integer| {
                 helpers::year(integer.value().value as i32)
             }
    );
    // Date-Period
    b.rule_1("year (latent)",
             integer_check_by_range!(-1000, 999),
             |integer| {
                 Ok(helpers::year(integer.value().value as i32)?.latent())
             }
    );
    // Date-Period
    b.rule_1("year (latent)",
             integer_check_by_range!(2101, 2200),
             |integer| {
                 Ok(helpers::year(integer.value().value as i32)?.latent())
             }
    );
    // Date-Period
    b.rule_2("in year",
             b.reg(r#"n?o ano( de)?"#)?,
             integer_check_by_range!(1000, 2100),
             |_, integer| helpers::year(integer.value().value as i32)
    );
    // Date-Period
    b.rule_2("in year",
             b.reg(r#"em"#)?,
             integer_check_by_range!(1000, 2100),
             |_, integer| helpers::year(integer.value().value as i32)
    );
    // Date-Period
    b.rule_2("in year",
             b.reg(r#"(para|durante) o ano( de)?"#)?,
             integer_check_by_range!(1000, 2100),
             |_, integer| helpers::year(integer.value().value as i32)
    );
    // Date
    b.rule_2("<day-of-month> ordinal",
             b.reg(r#"o"#)?,
             ordinal_check_by_range!(1, 31),
             |_, integer| Ok(helpers::day_of_month(integer.value().value as u32)?)
    );
    // Date
    b.rule_3("<day-of-week> day <day-of-month>",
             time_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"dia"#)?,
             integer_check_by_range!(1, 31),
             |dow, _, integer| dow.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    // Date
    b.rule_3("<day-of-month> de <named-month>",
             integer_check_by_range!(1, 31),
             b.reg(r#"de"#)?,
             time_check!(form!(Form::Month(_))),
             |integer, _, month| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    // Date domingo dia trinta e um de dezembro
    b.rule_4("<day-of-week> <day-of-month>(ordinal) de <named-month>",
             time_check!(form!(Form::DayOfWeek{..})),
             ordinal_check_by_range!(1, 31),
             b.reg(r#"de"#)?,
             time_check!(form!(Form::Month(_))),
             |_, integer, _, month| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    // Date
    b.rule_4("<day-of-week> <day-of-month>(integer) de <named-month>",
             time_check!(form!(Form::DayOfWeek{..})),
             integer_check_by_range!(1, 31),
             b.reg(r#"de"#)?,
             time_check!(form!(Form::Month(_))),
             |_, integer, _, month| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    // Date
    b.rule_5("<day-of-week> dia <day-of-month>(integer) de <named-month>",
             time_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"dia"#)?,
             integer_check_by_range!(1, 31),
             b.reg(r#"de"#)?,
             time_check!(form!(Form::Month(_))),
             |_, _, integer, _, month| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    // Date period
    b.rule_1_terminal("beginning of week",
                      b.reg(r#"o início da semana"#)?,
                      |_| helpers::day_of_week(Weekday::Mon)
                          ?.span_to(&helpers::day_of_week(Weekday::Tue)?, false)
    );
    // Date period
    b.rule_1_terminal("middle of week",
                      b.reg(r#"o meio da semana"#)?,
                      |_| helpers::day_of_week(Weekday::Wed)
                          ?.span_to(&helpers::day_of_week(Weekday::Thu)?, false)
    );
    // Date period
    b.rule_1_terminal("end of week (not weekend)",
                      b.reg(r#"o fim da semana"#)?,
                      |_| helpers::day_of_week(Weekday::Thu)
                          ?.span_to(&helpers::day_of_week(Weekday::Sun)?, false)
    );
    // Date period
    b.rule_1_terminal("during the week",
                      b.reg(r#"(para|durante )?esta semana"#)?,
                      |_| helpers::day_of_week(Weekday::Mon)
                          ?.span_to(&helpers::day_of_week(Weekday::Fri)?, false)
    );
    // Date period
    b.rule_1_terminal("week-end",
                      b.reg(r#"para? (o|este) (fim|final) de semana"#)?,
                      |_| {
                          let friday = helpers::day_of_week(Weekday::Fri)?
                              .intersect(&helpers::hour(18, false)?)?;
                          let monday = helpers::day_of_week(Weekday::Mon)?
                              .intersect(&helpers::hour(0, false)?)?;
                          friday.span_to(&monday, false)
                      }
    );
    // Date period
    b.rule_1_terminal("season",
                      b.reg(r#"verão"#)?,
                      |_| helpers::month_day(6, 21)?
                          .span_to(&helpers::month_day(9, 23)?, false)
    );
    // Date period
    b.rule_1_terminal("season",
                      b.reg(r#"outono"#)?,
                      |_| helpers::month_day(9, 23)?
                          .span_to(&helpers::month_day(12, 21)?, false)
    );
    // Date period
    b.rule_1_terminal("season",
                      b.reg(r#"inverno"#)?,
                      |_| helpers::month_day(12, 21)?
                          .span_to(&helpers::month_day(3, 20)?, false)
    );
    // Date period
    b.rule_1_terminal("season",
                      b.reg(r#"primavera"#)?,
                      |_| helpers::month_day(3, 20)?
                          .span_to(&helpers::month_day(6, 21)?, false)
    );

    // Time
    b.rule_1_terminal("noon",
                      b.reg(r#"(?:o )?meio[- ]dia"#)?,
                      |_| helpers::hour(12, false)
    );
    // Time
    b.rule_1_terminal("midnight",
                      b.reg(r#"(?:a )?meia[- ]noite"#)?,
                      |_| helpers::hour(0, false)
    );
    b.rule_1("time-of-day (latent) (1 to 23)",
             integer_check_by_range!(1, 23),
             |integer| {
                 Ok(helpers::hour(integer.value().value as u32, integer.value().value <= 12)?.latent())
             }
    );
    b.rule_1("time-of-day (latent) (0)",
             integer_check_by_range!(0, 0),
             |_| Ok(helpers::hour(0, false)?.latent())
    );
    b.rule_1("time-of-day (latent) (half)",
            number_check!(|number: &NumberValue| {
                let hour = (number.value() - 0.5) as u32;
                hour as f32 == (number.value() - 0.5) && hour >= 1 && hour <= 23
            }),
             |number| {
                let hour = number.value().value() as u32;
                Ok(helpers::hour_minute(hour, 30, hour <= 12)?.latent())
             }
    );
    b.rule_1("time-of-day (latent) (quarter)",
            number_check!(|number: &NumberValue| {
                let hour = (number.value() - 0.25) as u32;
                hour as f32 == (number.value() - 0.25) && hour >= 1 && hour <= 23
            }),
             |number| {
                let hour = number.value().value() as u32;
                Ok(helpers::hour_minute(hour, 15, hour <= 12)?.latent())
             }
    );
    // Time
    b.rule_2("<time-of-day> hour",
             time_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"horas?"#)?,
             |time, _| Ok(time.value().clone().not_latent())
    );
    // Time
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
    // Time period
    b.rule_1_terminal("morning",
                      b.reg(r#"manh[aã]"#)?,
                      |_| Ok(helpers::hour(4, false)?.span_to(&helpers::hour(12, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Morning))
                          .latent())
    );
    // Time period
    b.rule_1_terminal("beginning of morning",
                      b.reg(r#"(?:para |n)?o começo da manh[aã]"#)?,
                      |_| Ok(helpers::hour(4, false)?
                          .span_to(&helpers::hour(9, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Morning)))
    );
    // Time period
    b.rule_1_terminal("end of morning",
                      b.reg(r#"(?:para |no )fim da manh[aã]"#)?,
                      |_| Ok(helpers::hour(10, false)?
                          .span_to(&helpers::hour(12, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Morning)))
    );
    // Time period
    b.rule_1_terminal("end of morning",
                      b.reg(r#"o fim da manh[aã]"#)?,
                      |_| Ok(helpers::hour(10, false)?
                          .span_to(&helpers::hour(12, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Morning)))
    );
    // Time period
    b.rule_1_terminal("afternoon",
                      b.reg(r#"tarde"#)?,
                      |_| Ok(helpers::hour(12, false)?.span_to(&helpers::hour(19, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Afternoon))
                          .latent())
    );
    // Time period
    b.rule_1_terminal("middle afternoon",
                      b.reg(r#"(?:para |n)?o meio da tarde"#)?,
                      |_| {
                          Ok(helpers::hour(15, false)?
                              .span_to(&helpers::hour(17, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
                      }
    );
    // Time period
    b.rule_1_terminal("end of afternoon",
                      b.reg(r#"(?:para |no )fim da tarde"#)?,
                      |_| {
                          Ok(helpers::hour(15, false)?
                              .span_to(&helpers::hour(17, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
                      }
    );
    // Time period
    b.rule_1_terminal("end of afternoon",
                      b.reg(r#"o fim da tarde"#)?,
                      |_| {
                          Ok(helpers::hour(15, false)?
                              .span_to(&helpers::hour(17, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
                      }
    );
    // Time period
    b.rule_1_terminal("evening",
                      b.reg(r#"noite"#)?,
                      |_| Ok(helpers::hour(18, false)?.span_to(&helpers::hour(0, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Evening))
                          .latent())
    );
    // Time period
    b.rule_1_terminal("beginning of evening",
                      b.reg(r#"(?:para |n)?o começo da noite"#)?,
                      |_| {
                          Ok(helpers::hour(18, false)?
                              .span_to(&helpers::hour(21, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );
    // Time period
    b.rule_1_terminal("end of evening",
                      b.reg(r#"(?:para |n)?o fim da noite"#)?,
                      |_| {
                          Ok(helpers::hour(21, false)?
                              .span_to(&helpers::hour(0, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );
    // Time period
    b.rule_1_terminal("lunch",
                      b.reg(r#"almoço"#)?,
                      |_| Ok(helpers::hour(12, false)?
                          .span_to(&helpers::hour(14, false)?, false)?
                          .latent()
                          .form(Form::Meal))
    );
    // Time period
    b.rule_1_terminal("lunch",
                      b.reg(r#"a hora doalmoço"#)?,
                      |_| Ok(helpers::hour(12, false)?
                          .span_to(&helpers::hour(14, false)?, false)?
                          .latent()
                          .form(Form::Meal))
    );
    // Time period
    b.rule_2("this <part-of-day>",
             b.reg(r#"d?esta|d[ea]"#)?,
             time_check!(|time: &TimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
             |_, pod| Ok(helpers::cycle_nth(Grain::Day, 0)?
                 .intersect(pod.value())?
                 .form(pod.value().form.clone()))
    );
    // Time
    b.rule_2("<time-of-day> <part-of-day>",
             time_check!(excluding_form!(Form::PartOfDay(_))),
             time_check!(|time: &TimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
             |a, b| a.value().intersect(b.value())
    );
    // Time period
    b.rule_3("<part-of-day> de <time>",
            time_check!(|time: &TimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
            b.reg(r#"d?esta|d[ea]"#)?,
            time_check!(|time: &TimeValue| excluding_form!(Form::Year(_))(time) && excluding_form!(Form::Month(_))(time)),
            |part_of_day, _, time| time.value().intersect(part_of_day.value())
    );
    // Time period
    b.rule_3("<time> à <part-of-day>",
            time_check!(|time: &TimeValue| excluding_form!(Form::Year(_))(time) && excluding_form!(Form::Month(_))(time)),
            b.reg(r#"à"#)?,
            time_check!(|time: &TimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
            |time, _, part_of_day| time.value().intersect(part_of_day.value())
    );
    // Time period
    b.rule_3("<time-of-day> de <time>",
            time_check!(excluding_form!(Form::PartOfDay(_))),
            b.reg(r#"d?esta|d[ea]"#)?,
            time_check!(|time: &TimeValue| excluding_form!(Form::Year(_))(time) && excluding_form!(Form::Month(_))(time)),
            |part_of_day, _, time| time.value().intersect(part_of_day.value())
    );
    b.rule_1_terminal("half (relative minutes)",
                      b.reg(r#"meia"#)?,
                      |_| Ok(RelativeMinuteValue(30))
    );
    b.rule_1("number (as relative minutes)",
             integer_check_by_range!(1, 59),
             |integer| Ok(RelativeMinuteValue(integer.value().value as i32))
    );
    b.rule_2("<integer> minutes (as relative minutes)",
             integer_check_by_range!(1, 59),
             b.reg(r#"minutos?"#)?,
             |integer, _| Ok(RelativeMinuteValue(integer.value().value as i32))
    );
    // Time (ambiguity with Duration) ex: seis horas e vinte minutos
    b.rule_3("<hour-of-day> and <relative minutes>",
             time_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))),
             b.reg(r#"e"#)?,
             relative_minute_check!(),
             |time, _, relative_minute| helpers::hour_relative_minute(
                 time.value().form_time_of_day()?.full_hour(),
                 relative_minute.value().0,
                 time.value().form_time_of_day()?.is_12_clock())
    );
    // Time
    b.rule_2("at <time-of-day>",
             b.reg(r#"às?|para"#)?,
             time_check!(form!(Form::TimeOfDay(_))),
             |_, tod| Ok(tod.value().clone().not_latent())
    );
    // Time period
    b.rule_1_terminal("beginning of day",
                      b.reg(r#"(?:n?o )?(começo|início) do dia"#)?,
                      |_| {
                          Ok(helpers::hour(6, false)?
                              .span_to(&helpers::hour(10, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Morning)))
                      }
    );
    // Time period
    b.rule_1_terminal("beginning of day",
                      b.reg(r#"(?:n?o )?início do dia"#)?,
                      |_| {
                          Ok(helpers::hour(6, false)?
                              .span_to(&helpers::hour(10, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Morning)))
                      }
    );
    // Time period
    b.rule_1_terminal("middle of day",
                      b.reg(r#"(?:n?o )?meio do dia"#)?,
                      |_| {
                          Ok(helpers::hour(11, false)?
                              .span_to(&helpers::hour(16, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::None)))
                      }
    );
    // Time period
    b.rule_1_terminal("middle of day",
                      b.reg(r#"metade do dia"#)?,
                      |_| {
                          Ok(helpers::hour(11, false)?
                              .span_to(&helpers::hour(16, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::None)))
                      }
    );
    // Time period
    b.rule_1_terminal("end of day",
                      b.reg(r#"(?:para |n)?o fim do dia"#)?,
                      |_| {
                          Ok(helpers::hour(17, false)?
                              .span_to(&helpers::hour(21, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );
    // Time period
    b.rule_4("between <datetime> and <datetime> (interval)",
             b.reg(r#"entre(?: as?)?"#)?,
             time_check!(),
             b.reg(r#"e|e a"#)?,
             time_check!(),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );
    // Time period
    b.rule_4("between <datetime> and <datetime> (interval)",
             b.reg(r#"entre(?: as?)?"#)?,
             time_check!(),
             b.reg(r#"e as"#)?,
             time_check!(),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );
    // Time period
    b.rule_4("between <time-of-day> e as <time-of-day> (interval)",
             b.reg(r#"entre(?: as?)?"#)?,
             time_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"e( a)?"#)?,
             time_check!(form!(Form::TimeOfDay(_))),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );
    // Time period
    b.rule_4("between <time-of-day> e as <time-of-day> (interval)",
             b.reg(r#"entre(?: as?)?"#)?,
             time_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"e as"#)?,
             time_check!(form!(Form::TimeOfDay(_))),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );
    // Time period
    b.rule_4("from <time-of-day> to <time-of-day> (interval)",
             b.reg(r#"das"#)?,
             time_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"às"#)?,
             time_check!(),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );
    // Time period
    b.rule_2("from <time-of-day>",
             b.reg(r#"a partir( das?| de| desta)"#)?,
             time_check!(form!(Form::TimeOfDay(_))),
             |_, time| Ok(time.value().clone().mark_after_start())
    );
    // Time period
    b.rule_2("from <time-of-day>",
             b.reg(r#"do início do|desde as?"#)?,
             time_check!(form!(Form::TimeOfDay(_))),
             |_, time| Ok(time.value().clone().mark_after_start())
    );
    // Time period
    b.rule_2("after <date-time>",
             b.reg(r#"(a partir|depois)( desta| das?| de)"#)?,
             time_check!(),
             |_, time| Ok(time.value().clone().mark_after_start())
    );
    // Time period
    b.rule_2("from <date-time>",
             b.reg(r#"desde as?"#)?,
             time_check!(),
             |_, time| Ok(time.value().clone().mark_after_start())
    );
    // Time period
    b.rule_2("from <time-of-day>",
             b.reg(r#"desde as?"#)?,
             time_check!(form!(Form::TimeOfDay(_))),
             |_, time| Ok(time.value().clone().mark_after_start())
    );
    // Time period
    b.rule_3("from <time-of-day> on",
             b.reg(r#"do|de|das"#)?,
             time_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"em diante"#)?,
             |_, time, _| Ok(time.value().clone().mark_after_start())
    );
    // Time period
    b.rule_3("from <part-of-day> on",
             b.reg(r#"do|de|das"#)?,
             time_check!(|time: &TimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
             b.reg(r#"em diante"#)?,
             |_, pod, _| Ok(pod.value().clone().mark_after_start())
    );
    // Time period
    b.rule_3("from <datetime> on",
             b.reg(r#"do|de|das"#)?,
             time_check!(),
             b.reg(r#"em diante"#)?,
             |_, pod, _| Ok(pod.value().clone().mark_after_start())
    );
    // Time period
    b.rule_4("from <date-time> to <date-time> (interval)",
             b.reg(r#"do|de|das"#)?,
             time_check!(),
             b.reg(r#"às"#)?,
             time_check!(),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );
    // Time period
    b.rule_4("from <date-time> to <date-time> (interval)",
             b.reg(r#"a partir (d[ae]|das|desta)"#)?,
             time_check!(),
             b.reg(r#"para as"#)?,
             time_check!(),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );
    // Time period
    b.rule_2("before <datetime>",
             b.reg(r#"antes d[ao]s?"#)?,
             time_check!(),
             |_, time| Ok(time.value().clone().mark_before_end())
    );
    // Time period
    b.rule_2("before <datetime>",
             b.reg(r#"àté(?: as?| o| de)?"#)?,
             time_check!(),
             |_, time| Ok(time.value().clone().mark_before_end())
    );
    // Time period
    b.rule_2("before <part-of-day>",
             b.reg(r#"àté as|àté o|àté de|àté"#)?,
             time_check!(|time: &TimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
             |_, pod| Ok(pod.value().clone().mark_before_end())
    );
    // Time period
    b.rule_2("before <part-of-day>",
             b.reg(r#"àté"#)?,
             time_check!(|time: &TimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
             |_, pod| Ok(pod.value().clone().mark_before_end())
    );
    // Time
    b.rule_1_terminal("hh(:|h)mm (time-of-day)",
                      b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:h\.]([0-5]\d)"#)?,
                      |text_match| {
                          let hour: u32 = text_match.group(1).parse()?;
                          let minute: u32 = text_match.group(2).parse()?;
                          helpers::hour_minute(hour, minute, hour < 12)
                      }
    );
    // Time
    b.rule_1_terminal("hh:mm:ss",
                      b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:.]([0-5]\d)[:.]([0-5]\d)"#)?,
                      |text_match| helpers::hour_minute_second(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(3).parse()?,
                          true
                      )
    );
    // Date
    b.rule_1_terminal("dd[/-.]mm[/-.]yyyy",
                      b.reg(r#"(3[01]|[12]\d|0?[1-9])[-/.](0?[1-9]|1[0-2])[-/.](\d{2,4})"#)?,
                      |text_match| helpers::year_month_day(
                          text_match.group(3).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(1).parse()?
                      )
    );
    // Date
    b.rule_1_terminal("yyyy-mm-dd",
                      b.reg(r#"(\d{2,4})-(0?[1-9]|1[0-2])-(3[01]|[12]\d|0?[1-9])"#)?,
                      |text_match| helpers::year_month_day(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(3).parse()?
                      )
    );
    // Date
    b.rule_1_terminal("dd[/-]mm",
                      b.reg(r#"(3[01]|[12]\d|0?[1-9])[-/](0?[1-9]|1[0-2])"#)?,
                      |text_match| helpers::month_day(
                          text_match.group(2).parse()?,
                          text_match.group(1).parse()?
                      )
    );
    // Date time complement
    b.rule_3("<time> <part-of-day>",
            time_check!(|time: &TimeValue| excluding_form!(Form::Year(_))(time) && excluding_form!(Form::Month(_))(time)),
            b.reg(r#"à|de"#)?,
            time_check!(|time: &TimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
            |time, _, part_of_day| time.value().intersect(part_of_day.value())
    );
    // Date time complement
    b.rule_3("<datetime> <part-of-day>",
            time_check!(),
            b.reg(r#"à|de"#)?,
            time_check!(|time: &TimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
            |time, _, part_of_day| time.value().intersect(part_of_day.value())
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
             b.reg(r#"°|graus?"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("degree"),
                     latent: false,
                 })
             });
    b.rule_2("<temp> Celsius",
             temperature_check!(),
             b.reg(r#"c\.?(?:elsius|ent[ií]grados?)?"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("celsius"),
                     latent: false,
                 })
             });
    b.rule_2("<temp> Kelvin",
             temperature_check!(),
             b.reg(r#"k\.?(?:elvin)?"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("kelvin"),
                     latent: false,
                 })
             });
    b.rule_2("<temp> Fahrenheit",
             temperature_check!(),
             b.reg(r#"f\.?(?:ahrenheit)?"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("fahrenheit"),
                     latent: false,
                 })
             });
    b.rule_2("<latent temp> below zero",
             temperature_check!(),
             b.reg(r#"abaixo de zero"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: -1.0 * a.value().value,
                     latent: false,
                     ..*a.value()
                 })
             });
    b.rule_2("<latent temp> below zero",
             b.reg(r#"menos"#)?,
             temperature_check!(),
             |_, a| {
                 Ok(TemperatureValue {
                     value: -1.0 * a.value().value,
                     latent: false,
                     ..*a.value()
                 })
             });
    Ok(())
}

pub fn rules_numbers(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {

    b.rule_2("intersect numbers",
             number_check!(|number: &NumberValue| number.grain().unwrap_or(0) > 1),
             number_check!(),
             |a, b| helpers::compose_numbers(&a.value(), &b.value())
    );

    b.rule_3("intersect numbers",
             number_check!(|number: &NumberValue| number.grain().unwrap_or(0) > 1),
             b.reg(r#"e"#)?,
             number_check!(),
             |a, _,b| helpers::compose_numbers(&a.value(), &b.value())
    );

    b.rule_1_terminal("numbers (0..9)",
                      b.reg(r#"(zero|uma?|dois|duas|tr[eéê]s|quatro|cinco|s[eé]is|meia|sete|oito|nove)"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "zero" => 0,
                              "um" => 1,
                              "uma" => 1,
                              "dois" => 2,
                              "duas" => 2,
                              "tres" => 3,
                              "trés" => 3,
                              "três" => 3,
                              "quatro" => 4,
                              "cinco" => 5,
                              "seis" => 6,
                              "séis" => 6,
                              "meia" => 6,
                              "sete" => 7,
                              "oito" => 8,
                              "nove" => 9,
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          IntegerValue::new(value)
                      }
    );

    b.rule_1_terminal("numbers (10..19)",
                      b.reg(r#"(dezesseis|dezasseis|dezessete|dezoito|dezenove|dezanove|dez|onze|doze|treze|quatorze|catorze|quinze)"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "dez" => 10,
                              "onze" => 11,
                              "doze" => 12,
                              "treze" => 13,
                              "quatorze" => 14,
                              "catorze" => 14,
                              "quinze" => 15,
                              "dezesseis" => 16,
                              "dezasseis" => 16,
                              "dezessete" => 17,
                              "dezoito" => 18,
                              "dezenove" => 19,
                              "dezanove" => 19,
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          IntegerValue::new(value)
                      }
    );

    b.rule_1_terminal("numbers (20..90)",
                 b.reg(r#"(vinte|trinta|quarenta|cinquenta|sessenta|setenta|oitenta|noventa)"#)?,
                 |text_match| {
                     let value = match text_match.group(1).as_ref() {
                         "vinte" => 20,
                         "trinta" => 30,
                         "quarenta" => 40,
                         "cinquenta" => 50,
                         "sessenta" => 60,
                         "setenta" => 70,
                         "oitenta" => 80,
                         "noventa" => 90,
                         _ => return Err(RuleError::Invalid.into()),
                     };
                     IntegerValue::new(value)
                 }
        );

    b.rule_3("numbers (21...99)",
                 integer_check_by_range!(20, 90, |integer: &IntegerValue| integer.value % 10 == 0),
                 b.reg(r#"e"#)?,
                 integer_check_by_range!(1, 9),
                 |x, _, y| IntegerValue::new(x.value().value + y.value().value)
    );

    b.rule_1_terminal("cem",
                      b.reg(r#"cem"#)?,
                      |_| IntegerValue::new_with_grain(100,2)
    );

    b.rule_3("numbers (101...199)",
                 b.reg(r#"cento"#)?,
                 b.reg(r#"e"#)?,
                 integer_check_by_range!(1, 99),
                 |_, _, y| IntegerValue::new_with_grain(100 + y.value().value, 2)
    );

    b.rule_1_terminal("numbers (200..900)",
             b.reg(r#"(duzent|trezent|quatrocent|quinhent|seiscent|setecent|oitocent|novecent)(?:[oa]s)"#)?,
             |text_match| {
                 let value = match text_match.group(1).as_ref() {
                     "duzent" => 200,
                     "trezent" => 300,
                     "quatrocent" => 400,
                     "quinhent" => 500,
                     "seiscent" => 600,
                     "setecent" => 700,
                     "oitocent" => 800,
                     "novecent" => 900,
                     _ => return Err(RuleError::Invalid.into()),
                 };
                 IntegerValue::new_with_grain(value,2)
             }
    );

    b.rule_1_terminal("thousand",
        b.reg(r#"mil"#)?,
        |_| IntegerValue::new_with_grain(1000, 3)
    );

    b.rule_2("thousands",
        integer_check_by_range!(1, 999),
        b.reg(r#"mil"#)?,
        |a, _| {
            Ok(IntegerValue {
                   value: a.value().value * 1000,
                   grain: Some(3),
                   ..IntegerValue::default()
               })
    });

    b.rule_2("one million",
        integer_check! (|integer: &IntegerValue| integer.value == 1),
        b.reg(r#"milhão"#)?,
        |_,_| IntegerValue::new_with_grain(1000000, 6)
    );

    b.rule_2("millions",
        integer_check_by_range!(2, 999),
        b.reg(r#"milhões"#)?,
        |a, _| {
            Ok(IntegerValue {
                   value: a.value().value * 1000000,
                   grain: Some(6),
                   ..IntegerValue::default()
               })
    });

    b.rule_2("one billion",
        integer_check! (|integer: &IntegerValue| integer.value == 1),
        b.reg(r#"bilhão"#)?,
        |_,_| IntegerValue::new_with_grain(1000000000, 9)
    );

    b.rule_2("billions",
        integer_check_by_range!(2, 999),
        b.reg(r#"bilhões"#)?,
        |a, _| {
            Ok(IntegerValue {
                   value: a.value().value * 1000000000,
                   grain: Some(9),
                   ..IntegerValue::default()
               })
    });

//    b.rule_3("numbers (1,000,000...999,999,999)",
//                 integer_check_by_range!(1000000, 999000000),
//                 b.reg(r#"e?"#)?,
//                 integer_check_by_range!(1, 999999),
//                 |a, _, c| IntegerValue::new_with_grain(a.value().value + c.value().value,3)
//    );
//    b.rule_3("numbers (1,000...999,999)",
//                 integer_check_by_range!(1000, 999000),
//                 b.reg(r#"e?"#)?,
//                 integer_check_by_range!(1, 999),
//                 |a, _, c| IntegerValue::new_with_grain(a.value().value + c.value().value,3)
//    );
//    b.rule_3("numbers (200...999)",
//                integer_check_by_range!(200, 900, |integer: &IntegerValue| integer.value % 100 == 0),
//                 b.reg(r#"e"#)?,
//                 integer_check_by_range!(1, 99),
//                 |x, _, y| IntegerValue::new_with_grain(x.value().value + y.value().value,2)
//    );

    b.rule_1_terminal("some",
                      b.reg(r#"algumas|alguns"#)?,
                      |_| IntegerValue::new_with_grain(3, 1)
    );
    b.rule_1_terminal("several",
                      b.reg(r#"v[àa]rios"#)?,
                      |_| IntegerValue::new_with_grain(4, 1)
    );
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
             b.reg(r#"v[íi]rgula"#)?,
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
             b.reg(r#"v[íi]rgula"#)?,
             b.reg(r#"(?:(?:zero )*(?:zero))"#)?,
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

    b.rule_1_terminal("ordinals (primero..9)",
                      b.reg(r#"(primer|segund|terceir|quart|quint|sext|s[eéè]tim|oitav|non)(?:[oa]s?)?"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "primer" => 1,
                              "segund" => 2,
                              "terceir" => 3,
                              "quart" => 4,
                              "quint" => 5,
                              "sext" => 6,
                              "sétim" => 7,
                              "sètim" => 7,
                              "setim" => 7,
                              "oitav" => 8,
                              "non" => 9,
                              _ => return Err(RuleError::Invalid.into())
                          };
                          Ok(OrdinalValue::new(value))
                      }
    );
    b.rule_1_terminal("ordinals (10..90)",
                          b.reg(r#"(d[eéè]cim|vig[eéè]sim|trig[eéè]sim|quadrag[eéè]sim|quinquag[eéè]sim|sexag[eéè]sim|septuag[eéè]sim|setuag[eéè]sim|octog[eéè]sim|nonag[eéè]sim)(?:[oa]s?)?"#)?,
                          |text_match| {
                              let value = match text_match.group(1).as_ref() {
                                  "décim" => 10,
                                  "dècim" => 10,
                                  "decim" => 10,
                                  "vigésim" => 20,
                                  "vigèsim" => 20,
                                  "vigesim" => 20,
                                  "trigésim" => 30,
                                  "trigèsim" => 30,
                                  "trigesim" => 30,
                                  "quadragésim" => 40,
                                  "quadragèsim" => 40,
                                  "quadragesim" => 40,
                                  "quinquagésim" => 50,
                                  "quinquagesim" => 50,
                                  "quinquagèsim" => 50,
                                  "sexagésim" => 60,
                                  "sexagèsim" => 60,
                                  "sexagesim" => 60,
                                  "septuagésim" => 70,
                                  "septuagèsim" => 70,
                                  "septuagesim" => 70,
                                  "setuagèsim" => 70,
                                  "setuagesim" => 70,
                                  "setuagésim" => 70,
                                  "octogésim" => 80,
                                  "octogèsim" => 80,
                                  "octogesim" => 80,
                                  "nonagésim" => 90,
                                  "nonagèsim" => 90,
                                  "nonagesim" => 90,
                                  _ => return Err(RuleError::Invalid.into())
                              };
                              Ok(OrdinalValue::new(value))
                          }
    );

    b.rule_2("ordinals (11..99)",
        ordinal_check_by_range!(10, 90),
        ordinal_check_by_range!(1, 9),
        |a, b| {
            Ok(OrdinalValue::new(a.value().value + b.value().value))
        }
    );

    b.rule_1_terminal("ordinals (100..900)",
                              b.reg(r#"(cent[eéè]sim|ducent[eéè]sim|trecent[eéè]sim|tricent[eéè]sim|quadrin?gent[eéè]sim|quingent[eéè]sim|sexcent[eéè]sim|seiscent[eéè]sim|setingent[eéè]sim|septigent[eéè]sim|septingent[eéè]sim|octingent[eéè]sim|octigent[eéè]sim|nongent[eéè]sim|noningent[eéè]sim)(?:[oa]s?)?"#)?,
                              |text_match| {
                                  let value = match text_match.group(1).as_ref() {
                                      "centésim" => 100,
                                      "centèsim" => 100,
                                      "centesim" => 100,
                                      "ducentésim" => 200,
                                      "ducentèsim" => 200,
                                      "ducentesim" => 200,
                                      "trecentésim" => 300,
                                      "trecentèsim" => 300,
                                      "trecentesim" => 300,
                                      "tricentésim" => 300,
                                      "tricentèsim" => 300,
                                      "tricentesim" => 300,
                                      "quadrigentésim" => 400,
                                      "quadrigentèsim" => 400,
                                      "quadrigentesim" => 400,
                                      "quadringentésim" => 400,
                                      "quadringentèsim" => 400,
                                      "quadringentesim" => 400,
                                      "quingentésim" => 500,
                                      "quingentesim" => 500,
                                      "quingentèsim" => 500,
                                      "sexcentésim" => 600,
                                      "sexcentèsim" => 600,
                                      "sexcentesim" => 600,
                                      "seiscentésim" => 600,
                                      "seiscentèsim" => 600,
                                      "seiscentesim" => 600,
                                      "setingentésim" => 700,
                                      "setingentèsim" => 700,
                                      "setingentesim" => 700,
                                      "septingentèsim" => 700,
                                      "septingentesim" => 700,
                                      "septingentésim" => 700,
                                      "septigentésim" => 700,
                                      "septigentèsim" => 700,
                                      "septigentesim" => 700,
                                      "octingentésim" => 800,
                                      "octingentèsim" => 800,
                                      "octingentesim" => 800,
                                      "octigentèsim" => 800,
                                      "octigentésim" => 800,
                                      "octigentesim" => 800,
                                      "nongentésim" => 900,
                                      "nongentèsim" => 900,
                                      "nongentesim" => 900,
                                      "noningentésim" => 900,
                                      "noningentèsim" => 900,
                                      "noningentesim" => 900,
                                      _ => return Err(RuleError::Invalid.into())
                                  };
                                  Ok(OrdinalValue::new(value))
                              }
    );

    b.rule_2("ordinals (101..999)",
        ordinal_check_by_range!(100, 900),
        ordinal_check_by_range!(1, 99),
        |a, b| {
            Ok(OrdinalValue::new(a.value().value + b.value().value))
        }
    );

    b.rule_1_terminal("ordinal thousand",
                      b.reg(r#"(mil[eéè]sim)(?:[oa]s?)?"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "milésim" => 1000,
                              "milèsim" => 1000,
                              "milesim" => 1000,
                              _ => return Err(RuleError::Invalid.into())
                          };
                          Ok(OrdinalValue::new_with_grain(value,3))
                      }
    );

    b.rule_2("ordinal thousands",
        integer_check_by_range!(1, 999),
        ordinal_check! (|ordinal: &OrdinalValue| ordinal.value == 1000),
        |a, b| {
            Ok(OrdinalValue::new(a.value().value * b.value().value))
        }
    );

    b.rule_2("ordinal thousands + number",
        ordinal_check_by_range!(1000,999000),
        ordinal_check_by_range!(1, 999),
        |a, b| {
            Ok(OrdinalValue::new(a.value().value + b.value().value))
        }
    );

    b.rule_1_terminal("ordinal (digits)",
                      b.reg(r#"0*(\d+)[ºªoa]"#)?,
                      |text_match| {
                          let value: i64 = text_match.group(1).parse()?;
                          Ok(OrdinalValue::new(value))
                      }
    );
    Ok(())
}
