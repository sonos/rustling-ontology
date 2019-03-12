use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::helpers;
use rustling_ontology_moment::{Grain, PeriodComp, Period};

pub fn rules_duration(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("second (unit-of-duration)",
                      b.reg(r#"sec(?:ond)?s?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Second))
    );
    b.rule_1_terminal("minute (unit-of-duration)",
                      b.reg(r#"min(?:ute)?s?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Minute))
    );
    b.rule_1_terminal("hour (unit-of-duration)",
                      b.reg(r#"h(?:(?:(?:ou)?rs?)|r)?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Hour))
    );
    b.rule_1_terminal("day (unit-of-duration)",
                      b.reg(r#"days?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Day))
    );
    b.rule_1_terminal("week (unit-of-duration)",
                      b.reg(r#"weeks?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Week))
    );
    b.rule_1_terminal("month (unit-of-duration)",
                      b.reg(r#"months?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Month))
    );
    b.rule_1_terminal("quarter (unit-of-duration)",
                      b.reg(r#"quarters?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Quarter))
    );
    b.rule_1_terminal("year (unit-of-duration)",
                      b.reg(r#"years?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Year))
    );
    b.rule_1_terminal("quarter of an hour",
                      b.reg(r#"1/4\s?h(?:our)?|(?:a\s)?quarter(?: of an |-)hour"#)?,
                      |_| Ok(DurationValue::new(PeriodComp::minutes(15).into()))
    );
    b.rule_1_terminal("half an hour",
                      b.reg(r#"1/2\s?h(?:our)?|half an? hour|an? half hour"#)?,
                      |_| Ok(DurationValue::new(PeriodComp::minutes(30).into()))
    );
    b.rule_1_terminal("three-quarters of an hour",
                      b.reg(r#"3/4\s?h(?:our)?|three(?:\s|-)quarters of an hour"#)?,
                      |_| Ok(DurationValue::new(PeriodComp::minutes(45).into()))
    );
    b.rule_1_terminal("fortnight",
                      b.reg(r#"(?:a|one)? fortnight"#)?,
                      |_| Ok(DurationValue::new(PeriodComp::days(14).into()))
    );
    b.rule_2("<integer> <unit-of-duration>",
             integer_check_by_range!(0),
             unit_of_duration_check!(),
             |integer, uod| Ok(DurationValue::new(PeriodComp::new(uod.value().grain, integer.value().value).into()))
    );
    b.rule_3("<integer> more <unit-of-duration>",
             integer_check_by_range!(0),
             b.reg(r#"more|less"#)?,
             unit_of_duration_check!(),
             |integer, _, uod| Ok(DurationValue::new(PeriodComp::new(uod.value().grain, integer.value().value).into()))
    );
    b.rule_2_terminal("number.number hours",
                      b.reg(r#"(\d+)\.(\d+)"#)?,
                      b.reg(r#"hours?"#)?,
                      |text_match, _| {
                          Ok(DurationValue::new(
                              PeriodComp::minutes(
                                  helpers::decimal_hour_in_minute(text_match.group(1), text_match.group(2))?
                              ).into()
                          )
                          )
                      }
    );
    b.rule_2("<integer> and a half hours",
             integer_check_by_range!(0),
             b.reg(r#"and (?:an? )?half hours?"#)?,
             |integer, _| Ok(DurationValue::new(PeriodComp::minutes(integer.value().value * 60 + 30).into()))
    );
    b.rule_3("<integer> <unit-of-duration> and a half",
             integer_check_by_range!(0),
             unit_of_duration_check!(),
             b.reg(r#"and (?:an? )?half"#)?,
             |integer, uod, _| {
                 let half_period: Period = uod.value().grain.half_period().map(|a| a.into()).unwrap_or_else(|| Period::default());
                 Ok(DurationValue::new(half_period + PeriodComp::new(uod.value().grain, integer.value().value)))
             }
    );
    b.rule_3("<integer> <unit-of-duration> and a quarter",
             integer_check_by_range!(0),
             unit_of_duration_check!(),
             b.reg(r#"and (?:a? )?quarter"#)?,
             |integer, uod, _| {
                 let quarter_period: Period = uod.value().grain.quarter_period().map(|a| a.into()).unwrap_or_else(|| Period::default());
                 Ok(DurationValue::new(quarter_period + PeriodComp::new(uod.value().grain, integer.value().value)))
             }
    );
    b.rule_3("<integer> and a half <unit-of-duration>",
             integer_check_by_range!(0),
             b.reg(r#"and (?:an? )?half"#)?,
             unit_of_duration_check!(),
             |integer, _, uod| {
                 let half_period: Period = uod.value().grain.half_period().map(|a| a.into()).unwrap_or_else(|| Period::default());
                 Ok(DurationValue::new(half_period + PeriodComp::new(uod.value().grain, integer.value().value)))
             }
    );
    b.rule_3("<integer> and a quarter <unit-of-duration>",
             integer_check_by_range!(0),
             b.reg(r#"and (?:a? )?quarter"#)?,
             unit_of_duration_check!(),
             |integer, _, uod| {
                 let quarter_period: Period = uod.value().grain.quarter_period().map(|a| a.into()).unwrap_or_else(|| Period::default());
                 Ok(DurationValue::new(quarter_period + PeriodComp::new(uod.value().grain, integer.value().value)))
             }
    );
    b.rule_3("<number> h <number>",
             integer_check_by_range!(0),
             b.reg(r#"h(?:ours?)?"#)?,
             integer_check_by_range!(0,59),
             |hour, _, minute| {
                 let hour_period = Period::from(PeriodComp::new(Grain::Hour, hour.value().clone().value));
                 let minute_period = Period::from(PeriodComp::new(Grain::Minute, minute.value().clone().value));
                 Ok(DurationValue::new(hour_period + minute_period))
             }
    );
    b.rule_2("a <unit-of-duration>",
             b.reg(r#"an?"#)?,
             unit_of_duration_check!(),
             |_, unit| Ok(DurationValue::new(PeriodComp::new(unit.value().grain, 1).into()))
    );
    b.rule_2("in <duration>",
             b.reg(r#"in"#)?,
             duration_check!(),
             |_, duration| duration.value().in_present()
    );
    b.rule_3("in <duration>",
             b.reg(r#"in"#)?,
             duration_check!(),
             b.reg(r#"(?:' )? times?"#)?,
             |_, duration, _| duration.value().in_present()
    );
    b.rule_2("for <duration>",
             b.reg(r#"for"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().prefixed())
    );
    b.rule_2("during <duration>",
             b.reg(r#"during"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().prefixed())
    );
    b.rule_2("after <duration>",
             b.reg(r#"after"#)?,
             duration_check!(),
             |_, duration| Ok(duration
                 .value()
                 .in_present()?
                 .mark_after_start())
    );
    b.rule_3("<duration> and <duration>",
             duration_check!(|duration: &DurationValue| !duration.suffixed),
             b.reg(r#"and"#)?,
             duration_check!(|duration: &DurationValue| !duration.prefixed),
             |a, _, b| Ok(a.value() + b.value())
    );

    b.rule_2("<duration> <duration>",
             duration_check!(|duration: &DurationValue| !duration.suffixed),
             duration_check!(|duration: &DurationValue| !duration.prefixed),
             |a, b| Ok(a.value() + b.value())
    );

    b.rule_2("<duration> from now", // "10 minutes from now"
             duration_check!(),
             b.reg(r#"from (?:today|now)"#)?,
             |a, _| a.value().in_present()
    );

    b.rule_3("for <duration> from now", // "for 10 minutes from now"
             b.reg(r#"for"#)?,
             duration_check!(),
             b.reg(r#"from (?:today|now)"#)?,
             |_, duration, _| {
                 let start = helpers::cycle_nth(Grain::Second, 0)?;
                 let end = duration.value().in_present()?;
                 start.span_to(&end, false)
             }
    );

    b.rule_2("<duration> ago",
             duration_check!(),
             b.reg(r#"ago"#)?,
             |a, _| a.value().ago()
    );

    b.rule_2("<duration> hence",
             duration_check!(),
             b.reg(r#"hence"#)?,
             |a, _| a.value().in_present()
    );

    b.rule_3("<duration> after <datetime>",
             duration_check!(),
             b.reg(r#"after"#)?,
             datetime_check!(),
             |duration, _, datetime| duration.value().after(datetime.value())
    );

    b.rule_3("<duration> before <datetime>",
             duration_check!(),
             b.reg(r#"before"#)?,
             datetime_check!(),
             |duration, _, datetime| duration.value().before(datetime.value())
    );

    b.rule_2("about <duration>",
             b.reg(r#"(?:about|around|approximately|roughly)"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().precision(Precision::Approximate))
    );

    b.rule_2("<duration> approximately",
             duration_check!(),
             b.reg(r#"(?:about|around|approximately|roughly)"#)?,
             |duration, _| Ok(duration.value().clone().precision(Precision::Approximate))
    );

    b.rule_2("exactly <duration>",
             b.reg(r#"exactly|precisely"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().precision(Precision::Exact))
    );

    b.rule_2("<duration> exactly",
             duration_check!(),
             b.reg(r#"exactly|precisely"#)?,
             |duration, _| Ok(duration.value().clone().precision(Precision::Exact))
    );
    Ok(())
}