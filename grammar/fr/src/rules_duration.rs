use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::helpers;
use rustling_ontology_moment::{Grain, PeriodComp, Period};

pub fn rules_duration(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("seconde (unit-of-duration)",
        b.reg(r#"sec(?:onde)?s?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Second))
    );
    b.rule_1_terminal("minute (unit-of-duration)",
        b.reg(r#"min(?:ute)?s?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Minute))
    );
    b.rule_1_terminal("heure (unit-of-duration)",
        b.reg(r#"h(?:eure)?s?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Hour))
    );
    b.rule_1_terminal("jour (unit-of-duration)",
        b.reg(r#"jour(?:n[ée]e?)?s?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Day))
    );
    b.rule_1_terminal("semaine (unit-of-duration)",
        b.reg(r#"semaines?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Week))
    );
    b.rule_1_terminal("mois (unit-of-duration)",
        b.reg(r#"mois?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Month))
    );
    b.rule_1_terminal("année (unit-of-duration)",
        b.reg(r#"an(?:n[ée]e?)?s?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Year))
    );
    b.rule_1_terminal("trimestre (unit-of-duration)",
        b.reg(r#"trimestres?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Quarter))
    );
    b.rule_1_terminal("un quart heure",
        b.reg(r#"(1/4\s?h(?:eure)?|(?:un|1) quart d'heure)"#)?,
        |_| Ok(DurationValue::new(PeriodComp::minutes(15).into()))
    );
    b.rule_1_terminal("une demi heure",
        b.reg(r#"(?:1/2\s?h(?:eure)?|(?:1|une) demi(?:e)?(?:\s|-)heure)"#)?,
        |_| Ok(DurationValue::new(PeriodComp::minutes(30).into()))
    );
    b.rule_1_terminal("trois quarts d'heure",
        b.reg(r#"(?:3/4\s?h(?:eure)?|(?:3|trois) quart(?:s)? d'heure)"#)?,
        |_| Ok(DurationValue::new(PeriodComp::minutes(45).into()))
    );
    b.rule_2("<integer> <unit-of-duration>",
             integer_check_by_range!(0),
             unit_of_duration_check!(),
             |integer, unit| Ok(DurationValue::new(PeriodComp::new(unit.value().grain, integer.value().value).into()))
    );
    b.rule_3("<integer> de <unit-of-duration>",
        integer_check!(|integer: &IntegerValue| integer.value >= 0 && integer.group),
        b.reg(r#"d[e']"#)?,
        unit_of_duration_check!(),
        |integer, _, unit| Ok(DurationValue::new(PeriodComp::new(unit.value().grain, integer.value().value).into()))
    );
    b.rule_4("<number> h <number>",
             integer_check_by_range!(0),
             b.reg(r#"h(?:eures?)?"#)?,
             integer_check_by_range!(0,59),
             b.reg(r#"m(?:inutes?)?"#)?,
             |hour, _, minute, _| {
                 let hour_period = Period::from(PeriodComp::new(Grain::Hour, hour.value().clone().value));
                 let minute_period = Period::from(PeriodComp::new(Grain::Minute, minute.value().clone().value));
                 Ok(DurationValue::new(hour_period + minute_period))
             }
    );
    b.rule_3("<integer> <unit-of-duration> et quart",
        integer_check_by_range!(0),
        unit_of_duration_check!(),
        b.reg(r#"et quart"#)?,
        |integer, uod, _| {
           let quarter_period: Period = uod.value().grain.quarter_period().map(|a| a.into()).ok_or_else(|| RuleError::Invalid)?;
           Ok(DurationValue::new(quarter_period + PeriodComp::new(uod.value().grain, integer.value().value)))
        }
    );
    b.rule_3("<integer> <unit-of-duration> et demie",
        integer_check_by_range!(0),
        unit_of_duration_check!(),
        b.reg(r#"et demie?"#)?,
        |integer, uod, _| {
           let half_period: Period = uod.value().grain.half_period().map(|a| a.into()).ok_or_else(|| RuleError::Invalid)?;
           Ok(DurationValue::new(half_period + PeriodComp::new(uod.value().grain, integer.value().value)))
        }
    );
    b.rule_3("<duration> et <duration>",
             duration_check!(|duration: &DurationValue| !duration.suffixed),
             b.reg(r#"et"#)?,
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
    b.rule_2("dans <duration>",
             b.reg(r#"dans"#)?,
             duration_check!(),
             |_, duration| duration.value().in_present()
    );
    b.rule_2("<duration> plus tard",
        duration_check!(),
        b.reg(r"plus tard")?,
        |duration, _| duration.value().in_present()
    );
    b.rule_2("environ <duration>",
             b.reg(r#"environ|approximativement|à peu près|presque"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().precision(Precision::Approximate))
    );
    b.rule_2("<duration> environ",
             duration_check!(),
             b.reg(r#"environ|approximativement|à peu près"#)?,
             |duration, _| Ok(duration.value().clone().precision(Precision::Approximate))
    );
    b.rule_2("exactement <duration> ",
             b.reg(r#"exactement|précisément"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().precision(Precision::Exact))
    );
    b.rule_2("<duration> exactement",
             duration_check!(),
             b.reg(r#"exactement|précisément|pile"#)?,
             |duration, _| Ok(duration.value().clone().precision(Precision::Exact))
    );
    b.rule_2("pendant <duration>",
             b.reg(r#"pendant|durant|pour"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().prefixed())
    );
    b.rule_2("une durée de <duration>",
             b.reg(r#"une dur[ée]e d['e]"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().prefixed())
    );
    b.rule_2("il y a <duration>",
             b.reg(r#"il y a"#)?,
             duration_check!(),
             |_, duration| duration.value().ago()
    );
    b.rule_2("depuis <duration>",
        b.reg(r#"depuis|[cç]a fait"#)?,
        duration_check!(),
        |_, duration| {
            duration.value().ago()?
                .span_to(&helpers::cycle_nth(Grain::Second, 0)?, false)
    });
    b.rule_3("<duration> apres <datetime>",
             duration_check!(),
             b.reg(r#"apr[eè]s"#)?,
             datetime_check!(),
             |duration, _, datetime| duration.value().after(datetime.value())
    );
    b.rule_3("<duration> avant <datetime>",
             duration_check!(),
             b.reg(r#"avant"#)?,
             datetime_check!(),
             |duration, _, datetime| duration.value().before(datetime.value())
    );
    Ok(())
}