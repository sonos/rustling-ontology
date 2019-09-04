use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::helpers;
use rustling_ontology_moment::{Grain, PeriodComp, Period};

pub fn rules_duration(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("seconde (unit-of-duration)",
                      b.reg(r#"segundos?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Second))
    );
    b.rule_1_terminal("minute (unit-of-duration)",
                      b.reg(r#"minutos?|min"#)?,
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
                      b.reg(r#"(?:um )?quarto de hora"#)?,
                      |_| Ok(DurationValue::new(PeriodComp::minutes(15).into()))
    );
    b.rule_1_terminal("half an hour",
                      b.reg(r#"(?:uma )?meia hora"#)?,
                      |_| Ok(DurationValue::new(PeriodComp::minutes(30).into()))
    );
    b.rule_1_terminal("three-quarters of an hour",
                      b.reg(r#"tr[eê]s quartos de hora"#)?,
                      |_| Ok(DurationValue::new(PeriodComp::minutes(45).into()))
    );
    b.rule_3("<number> h <number>",
             integer_check_by_range!(0),
             b.reg(r#"h(?:oras?)?"#)?,
             integer_check_by_range!(0,59),
             |hour, _, minute| {
                 let hour_period = Period::from(PeriodComp::new(Grain::Hour, hour.value().clone().value));
                 let minute_period = Period::from(PeriodComp::new(Grain::Minute, minute.value().clone().value));
                 Ok(DurationValue::new(hour_period + minute_period))
             }
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
    b.rule_3("<duration> and <duration>",
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
    b.rule_2("duration of <duration>",
             b.reg(r#"uma duração de"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().prefixed())
    );
    b.rule_2("during <duration>",
             b.reg(r#"por|durante"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().prefixed())
    );
    // FUTUR
    b.rule_2("in <duration> (future moment)",
             b.reg(r#"em|daqui a|dentro de"#)?,
             duration_check!(),
             |_, duration| duration.value().in_present()
    );
    // FUTUR
    b.rule_2("<duration> later",
             duration_check!(),
             b.reg(r#"depois"#)?,
             |duration, _| duration.value().in_present()
    );
    // PAST
    b.rule_2("<duration> ago",
             duration_check!(),
             b.reg(r#"atr[aá]s"#)?,
             |duration, _| duration.value().ago()
    );
    // PAST
    b.rule_2("ago <duration>",
             b.reg(r#"há|faz"#)?,
             duration_check!(),
             |_, duration| duration.value().ago()
    );
    b.rule_2("approx <duration>",
             b.reg(r#"aproximadamente|cerca de|por cerca de|por volta de|em torno de"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().precision(Precision::Approximate))
    );
    b.rule_2("<duration> approx",
             duration_check!(),
             b.reg(r#"aproximadamente|mais ou? menos"#)?,
             |duration, _| Ok(duration.value().clone().precision(Precision::Approximate))
    );
    b.rule_2("precisely <duration>",
             b.reg(r#"exactamente|precisamente"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().precision(Precision::Exact))
    );
    b.rule_2("<duration> precisely",
             duration_check!(),
             b.reg(r#"exactamente|precisamente"#)?,
             |duration , _| Ok(duration.value().clone().precision(Precision::Exact))
    );
    Ok(())
}
