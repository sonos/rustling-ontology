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
                      b.reg(r#"gbp|libras? esterlinas?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("GBP") })
    );
    b.rule_1_terminal("USD",
        b.reg(r#"d[oó]lar(?:es)? americanos?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("USD") })
    );
    b.rule_1_terminal("CAD",
                      b.reg(r#"d[oó]lar(?:es)? canadenses?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("CAD") })
    );
    b.rule_1_terminal("AUD",
                      b.reg(r#"d[oó]lar(?:es) australianos?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("AUD") })
    );
    b.rule_1_terminal("Bitcoin",
        b.reg(r#"฿|bitcoins?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("฿") })
    );
    b.rule_1_terminal("GBP",
        b.reg(r#"libras? inglesas?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("GBP") })
    );
    b.rule_1_terminal("JPY",
                      b.reg(r#"jpy|[yi]en(?:es)?(?: japoneses?)?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("JPY") })
    );
    b.rule_1_terminal("¥",
                      b.reg(r#"¥"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("¥") })
    );
    b.rule_1_terminal("KRW",
                      b.reg(r#"wons?|₩"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("KRW") })
    );
//    b.rule_1_terminal("RMB|CNH|CNY",
//                      b.reg(r#""#)?,
//                      |_| Ok(MoneyUnitValue { unit: Some("CNY") })
//    );
    b.rule_1_terminal("INR",
                      b.reg(r#"r[uú]pias?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("INR") })
    );
//    b.rule_1_terminal("HKD",
//                      b.reg(r#""#)?,
//                      |_| Ok(MoneyUnitValue { unit: Some("HKD") })
//    );
    b.rule_1_terminal("CHF",
                      b.reg(r#"francos? suíços?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("CHF") })
    );
//    b.rule_1_terminal("KR",
//                      b.reg(r#""#)?,
//                      |_| Ok(MoneyUnitValue { unit: Some("KR") })
//    );
//    b.rule_1_terminal("DKK",
//                      b.reg(r#""#)?,
//                      |_| Ok(MoneyUnitValue { unit: Some("DKK") })
//    );
//    b.rule_1_terminal("NOK",
//                      b.reg(r#""#)?,
//                      |_| Ok(MoneyUnitValue { unit: Some("NOK") })
//    );
//    b.rule_1_terminal("SEK",
//                      b.reg(r#""#)?,
//                      |_| Ok(MoneyUnitValue { unit: Some("SEK") })
//    );
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
             b.reg(r#"cerca de"#)?,
             amount_of_money_check!(),
             |_, a| {
                 Ok(AmountOfMoneyValue {
                     precision: Approximate,
                     ..a.value().clone()
                 })
             });
//    b.rule_2("<amount-of-money> about",
//             amount_of_money_check!(),
//             b.reg(r#""#)?,
//             |a, _| {
//                 Ok(AmountOfMoneyValue {
//                     precision: Approximate,
//                     ..a.value().clone()
//                 })
//             });
    b.rule_2("exactly <amount-of-money>",
             b.reg(r#"exatamente"#)?,
             amount_of_money_check!(),
             |_, a| {
                 Ok(AmountOfMoneyValue {
                     precision: Exact,
                     ..a.value().clone()
                 })
             });
//    b.rule_2("<amount-of-money> exactly",
//             amount_of_money_check!(),
//             b.reg(r#""#)?,
//             |a, _| {
//                 Ok(AmountOfMoneyValue {
//                     precision: Exact,
//                     ..a.value().clone()
//                 })
//             }
//    );
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
             b.reg(r#"e meia"#)?,
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
             b.reg(r#"por"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().prefixed())
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
             b.reg(r#"de"#)?,
             datetime_check!(|time: &TimeValue| !time.latent),
             |a, _, b| a.value().intersect(b.value())
    );
    b.rule_3("two time tokens separated by \",\"",
             datetime_check!(|time: &TimeValue| !time.latent),
             b.reg(r#","#)?,
             datetime_check!(|time: &TimeValue| !time.latent),
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
                      b.reg(r#"agora"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 0)
    );
    b.rule_1_terminal("today",
                      b.reg(r#"hoje"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 0)
    );
    b.rule_1_terminal("tomorrow",
                      b.reg(r#"amanh[aã]"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 1)
    );
    b.rule_1_terminal("yesterday",
                      b.reg(r#"ontem"#)?,
                      |_| helpers::cycle_nth(Grain::Day, -1)
    );
//    b.rule_1_terminal("the day after tomorrow",
//                      b.reg(r#""#)?,
//                      |_| helpers::cycle_nth(Grain::Day, 2)
//    );
//    b.rule_1_terminal("the day before yesterday",
//                      b.reg(r#""#)?,
//                      |_| helpers::cycle_nth(Grain::Day, -2)
//    );

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
    b.rule_1_terminal("dd[/-.]mm[/-.]yyyy",
                      b.reg(r#"(3[01]|[12]\d|0?[1-9])[-/.](0?[1-9]|1[0-2])[-/.](\d{2,4})"#)?,
                      |text_match| helpers::year_month_day(
                          text_match.group(3).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(1).parse()?
                      )
    );
    b.rule_1_terminal("yyyy-mm-dd",
                      b.reg(r#"(\d{2,4})-(0?[1-9]|1[0-2])-(3[01]|[12]\d|0?[1-9])"#)?,
                      |text_match| helpers::year_month_day(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(3).parse()?
                      )
    );
    b.rule_1_terminal("dd[/-]mm",
                      b.reg(r#"(3[01]|[12]\d|0?[1-9])[-/](0?[1-9]|1[0-2])"#)?,
                      |text_match| helpers::month_day(
                          text_match.group(2).parse()?,
                          text_match.group(1).parse()?
                      )
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
             b.reg(r#"abaixo de cero"#)?,
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
    b.rule_1_terminal("number (0..10)",
                      b.reg(r#"(cero|uma?|dois|duas|tr[eéê]s|quatro|cinco|s[eé]is|sete|oito|nove|dez)"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "cero" => 0,
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
                              "sete" => 7,
                              "oito" => 8,
                              "nove" => 9,
                              "dez" => 10,
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          IntegerValue::new(value)
                      }
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
    b.rule_1_terminal("ordinals (primero..10)",
                      b.reg(r#"(primer|segund|terceir|quart|quint|sext|s[eéè]tim|oitav|non|d[eéè]cim)(?:[oa]s?)?"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "primer" => 1,
                              "segund" => 2,
                              "terceir" => 3,
                              "quart" => 4,
                              "quint" => 5,
                              "sext" => 6,
                              "séptim" => 7,
                              "sèptim" => 7,
                              "septim" => 7,
                              "oitav" => 8,
                              "noven" => 9,
                              "décim" => 10,
                              "dècim" => 10,
                              "decim" => 10,
                              _ => return Err(RuleError::Invalid.into())
                          };
                          Ok(OrdinalValue::new(value))
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
