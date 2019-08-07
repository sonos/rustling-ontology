use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::helpers;
use rustling_ontology_moment::{Grain, PeriodComp, Period};

pub fn rules_duration(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    // Basic duration units
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
                      b.reg(r#"giorn(?:[oi]|ata)"#)?,
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
    b.rule_1_terminal("1 quarter of an hour",
                      b.reg(r#"(1/4|(?:un|1) quarto) d'ora"#)?,
                      |_| Ok(DurationValue::new(PeriodComp::minutes(15).into()))
    );
    b.rule_1_terminal("1 half an hour",
                      b.reg(r#"1/2 ora|(?:una |1 )?mezz'?ora"#)?,
                      |_| Ok(DurationValue::new(PeriodComp::minutes(30).into()))
    );
    b.rule_1_terminal("3 quarters of an hour",
                      b.reg(r#"(?:3/4|tre quarti) d'ora"#)?,
                      |_| Ok(DurationValue::new(PeriodComp::minutes(45).into()))
    );
    b.rule_1_terminal("two weeks | fortnight",
                      b.reg(r#"(?:un )?paio di settimane"#)?,
                      |_| Ok(DurationValue::new(PeriodComp::days(14).into()))
    );
    // N duration units
    b.rule_2("<integer> <unit-of-duration>",
             integer_check_by_range!(0),
             unit_of_duration_check!(),
             |integer, uod| Ok(DurationValue::new(PeriodComp::new(uod.value().grain, integer.value().value).into()))
    );
    b.rule_3("<number> hours <number>",
             integer_check_by_range!(0),
             b.reg(r#"or[ae] e|h"#)?,
             integer_check_by_range!(0,59),
             |hour, _, minute| {
                 let hour_period = Period::from(PeriodComp::new(Grain::Hour, hour.value().clone().value));
                 let minute_period = Period::from(PeriodComp::new(Grain::Minute, minute.value().clone().value));
                 Ok(DurationValue::new(hour_period + minute_period))
             }
    );
    b.rule_3("<integer> <unit-of-duration> and a quarter",
             integer_check_by_range!(0),
             unit_of_duration_check!(),
             b.reg(r#"e un quarto"#)?,
             |integer, uod, _| {
                 let quarter_period: Period = uod.value().grain.quarter_period().map(|a| a.into()).ok_or_else(|| RuleError::Invalid)?;
                 Ok(DurationValue::new(quarter_period + PeriodComp::new(uod.value().grain, integer.value().value)))
             }
    );
    b.rule_3("<integer> <unit-of-duration> and a half",
             integer_check_by_range!(0),
             unit_of_duration_check!(),
             b.reg(r#"e mezz[oa]"#)?,
             |integer, uod, _| {
                 let half_period: Period = uod.value().grain.half_period().map(|a| a.into()).ok_or_else(|| RuleError::Invalid)?;
                 Ok(DurationValue::new(half_period + PeriodComp::new(uod.value().grain, integer.value().value)))
             }
    );
    b.rule_3("<integer> <unit-of-duration> and 3 quarters of an hour",
             integer_check_by_range!(0),
             unit_of_duration_check!(),
             b.reg(r#"e tre quarti"#)?,
             |integer, uod, _| {
                 let half_period: Period = uod.value().grain.half_period().map(|a| a.into()).ok_or_else(|| RuleError::Invalid)?;
                 Ok(DurationValue::new(half_period + PeriodComp::new(uod.value().grain, integer.value().value)))
             }
    );
    // Duration combinations
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
    b.rule_2("<duration> <integer>",
             duration_check!(|duration: &DurationValue| !duration.prefixed),
             integer_check_by_range!(0),
             |duration, integer| helpers::compose_duration_with_integer(duration.value(), integer.value())
    );
    // Durations with modifiers / timeline positioning
    b.rule_2("in-future <duration> (French 'dans 2 mois')",
             b.reg(r#"[tf]ra"#)?,
             duration_check!(),
             |_, duration| duration.value().in_present()
    );
    b.rule_2("<duration> later",
             duration_check!(),
             b.reg(r"(dopo|più tardi)")?,
             |duration, _| duration.value().in_present()
    );
    b.rule_2("approx <duration>",
             b.reg(r#"verso|interno a|(?:approssim|indic|orient)ativamente|(?:all'in)?circa|più o meno|pressappoco|suppergiù|grosso modo"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().precision(Precision::Approximate))
    );
    b.rule_2("<duration> approx",
             duration_check!(),
             b.reg(r#"(?:all'in)?circa|più o meno"#)?,
             |duration, _| Ok(duration.value().clone().precision(Precision::Approximate))
    );
    b.rule_2("exactly <duration>",
             b.reg(r#"esattamente"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().precision(Precision::Exact))
    );
    b.rule_2("<duration> exactly",
             duration_check!(),
             b.reg(r#"(?:esatt|precis)(?:[aoie]|amente)"#)?,
             |duration, _| Ok(duration.value().clone().precision(Precision::Exact))
    );
    b.rule_2("during <duration>",
             b.reg(r#"(?:durante|per)"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().prefixed())
    );
    b.rule_2("<duration> ago",
             duration_check!(),
             b.reg(r#"fa"#)?,
             |duration, _| duration.value().ago()
    );
    b.rule_2("since <duration>",
             b.reg(r#"da(?: |l(?:l['oaie])?)"#)?,
             duration_check!(),
             |_, duration| {
                 duration.value().ago()?
                     .span_to(&helpers::cycle_nth(Grain::Second, 0)?, false)
             });
    b.rule_3("<duration> after <datetime>",
             duration_check!(),
             b.reg(r#"dopo"#)?,
             datetime_check!(),
             |duration, _, datetime| duration.value().after(datetime.value())
    );
    b.rule_3("<duration> before <datetime>",
             duration_check!(),
             b.reg(r#"prima"#)?,
             datetime_check!(),
             |duration, _, datetime| duration.value().after(datetime.value())
    );

    Ok(())
}