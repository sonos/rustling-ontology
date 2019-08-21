use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::helpers;
use rustling_ontology_moment::{Grain, PeriodComp, Period};


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
