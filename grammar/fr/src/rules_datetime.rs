use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::helpers;
use rustling_ontology_moment::{Weekday, Grain};

/* DATETIME - CYCLE DEFINITIONS */
pub fn rules_cycle(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("seconde (cycle)",
        b.reg(r#"secondes?"#)?,
        |_| CycleValue::new(Grain::Second)
    );
    b.rule_1_terminal("minute (cycle)",
        b.reg(r#"minutes?"#)?,
        |_| CycleValue::new(Grain::Minute)
    );
    b.rule_1_terminal("heure (cycle)",
        b.reg(r#"heures?"#)?,
        |_| CycleValue::new(Grain::Hour)
    );
    b.rule_1_terminal("jour (cycle)",
        b.reg(r#"jour(?:n[ée]e?)?s?"#)?,
        |_| CycleValue::new(Grain::Day)
    );
    b.rule_1_terminal("semaine (cycle)",
        b.reg(r#"semaines?"#)?,
        |_| CycleValue::new(Grain::Week)
    );
    b.rule_1("mois (cycle)",
             b.reg(r#"mois"#)?,
             |_| CycleValue::new(Grain::Month)
    );
    b.rule_1_terminal("trimestre (cycle)",
             b.reg(r#"trimestre"#)?,
             |_| CycleValue::new(Grain::Quarter)
    );
    b.rule_1("année (cycle)",
             b.reg(r#"an(?:n[ée]e?)?s?"#)?,
             |_| CycleValue::new(Grain::Year)
    );
    b.rule_1_terminal("trimestre (cycle)",
             b.reg(r#"trimestres?"#)?,
             |_| CycleValue::new(Grain::Quarter)
    );
    Ok(())
}


pub fn rules_datetime_with_cycle(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {

    // Cycle patterns relative to now
    b.rule_2("ce|dans le <cycle>",
             b.reg(r#"(?:(?:dans )?l[ea' ]|cet?(?:te)?)"#)?,
             cycle_check!(),
             |_, cycle| helpers::cycle_nth(cycle.value().grain, 0)
    );
    b.rule_3("ce <cycle> (la ou ci)",
             b.reg(r#"cet?t?e?s?"#)?,
             cycle_check!(),
             b.reg(r#"-?ci"#)?,
             |_, cycle, _| helpers::cycle_nth(cycle.value().grain, 0)
    );
    b.rule_2("<cycle> dernier",
             cycle_check!(),
             b.reg(r#"derni[èe]re?|pass[ée]e?|pr[eé]c[eé]dente?|(?:d')? ?avant"#)?,
             |cycle, _| helpers::cycle_nth(cycle.value().grain, -1)
    );
    b.rule_3("le <cycle> dernier",
             b.reg(r#"l[ae']? ?"#)?,
             cycle_check!(),
             b.reg(r#"derni[èe]re?|pass[ée]e?"#)?,
             |_, cycle, _| helpers::cycle_nth(cycle.value().grain, -1)
    );
    b.rule_3("n derniers <cycle>",
             integer_check_by_range!(2, 9999),
             b.reg(r#"derni.re?s?"#)?,
             cycle_check!(),
             |integer, _, cycle| {
                 let mut res = helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)?;
                 // All grains except Day will trigger the right datetime_kind
                 if cycle.value().grain == Grain::Day {
                     res = res.datetime_kind(DatetimeKind::DatePeriod);
                 }
                 Ok(res)
             }
    );
    b.rule_4("(pendant/durant/dans) les n derniers <cycle>",
             b.reg(r#"(?:pendant |durant |dans )?[cld]es"#)?,
             integer_check_by_range!(2, 9999),
             b.reg(r#"derni.re?s?"#)?,
             cycle_check!(),
             |_, integer, _, cycle| {
                 let mut res = helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)?;
                 // All grains except Day will trigger the right datetime_kind
                 if cycle.value().grain == Grain::Day {
                     res = res.datetime_kind(DatetimeKind::DatePeriod);
                 }
                 Ok(res)
             }
    );
    b.rule_3("n <cycle> passes|precedents",
             integer_check_by_range!(2, 9999),
             cycle_check!(),
             b.reg(r#"pass[eèé][eèé]?s?|pr[eé]c[eé]dente?s?|(?:d')? ?avant|plus t[oô]t"#)?,
             |integer, cycle, _| {
                 let mut res = helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)?;
                 // All grains except Day will trigger the right datetime_kind
                 if cycle.value().grain == Grain::Day {
                     res = res.datetime_kind(DatetimeKind::DatePeriod);
                 }
                 Ok(res)
             }
    );
    b.rule_4("(pendant/durant/dans) les n <cycle> passes|precedents",
             b.reg(r#"(?:pendant |durant |dans )?[cld]es"#)?,
             integer_check_by_range!(2, 9999),
             cycle_check!(),
             b.reg(r#"pass[eèé][eèé]?s?|pr[eé]c[eé]dente?s?|(?:d')? ?avant|plus t[oô]t"#)?,
             |_, integer, cycle, _| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    // Incorrect resolution if some <datetime> follows the expression,
    // e.g. "suivant le <date>" (unsupported)
    b.rule_2("<cycle> prochain|suivant|d'après",
             cycle_check!(),
             b.reg(r#"prochaine?|suivante?|qui suit|(?:d')? ?apr[eèé]s"#)?,
             |cycle, _| helpers::cycle_nth(cycle.value().grain, 1)
    );
    b.rule_3("le <cycle> prochain|suivant|d'après",
             b.reg(r#"l[ae']? ?|une? ?"#)?,
             cycle_check!(),
             b.reg(r#"prochaine?|suivante?|qui suit|(?:d'? ?)?apr[eèé]s"#)?,
             |_, cycle, _| helpers::cycle_nth(cycle.value().grain, 1)
    );
    b.rule_3("n prochains <cycle>",
             integer_check_by_range!(2, 9999),
             b.reg(r#"prochaine?s?|suivante?s?|apr[eèé]s"#)?,
             cycle_check!(),
             |integer, _, cycle| {
                 let mut res = helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)?;
                 // All grains except Day will trigger the right datetime_kind
                 if cycle.value().grain == Grain::Day {
                     res = res.datetime_kind(DatetimeKind::DatePeriod);
                 }
                 Ok(res)
             }
    );
    b.rule_4("(pendant/durant/dans) les n prochains <cycle>",
             b.reg(r#"(?:pendant |durant |dans )?[cld]es"#)?,
             integer_check_by_range!(2, 9999),
             b.reg(r#"prochaine?s?|suivante?s?|apr[eèé]s"#)?,
             cycle_check!(),
             |_, integer, _, cycle| {
                 let mut res = helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)?;
                 // All grains except Day will trigger the right datetime_kind
                 if cycle.value().grain == Grain::Day {
                     res = res.datetime_kind(DatetimeKind::DatePeriod);
                 }
                 Ok(res)
             }
    );
    b.rule_3("n <cycle> suivants",
             integer_check_by_range!(2, 9999),
             cycle_check!(),
             b.reg(r#"prochaine?s?|suivante?s?|apr[eèé]s|qui sui(?:t|ves?)|plus tard"#)?,
             |integer, cycle, _| {
                 let mut res = helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)?;
                 // All grains except Day will trigger the right datetime_kind
                 if cycle.value().grain == Grain::Day {
                     res = res.datetime_kind(DatetimeKind::DatePeriod);
                 }
                 Ok(res)
             }
    );
    b.rule_4("(pendant/durant/dans) les n <cycle> suivants",
             b.reg(r#"(?:pendant |durant |dans )?[cld]es"#)?,
             integer_check_by_range!(2, 9999),
             cycle_check!(),
             b.reg(r#"prochaine?s?|suivante?s?|apr[eèé]s|qui sui(?:t|ves?)|plus tard"#)?,
             |_, integer, cycle, _| {
                 let mut res = helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)?;
                 // All grains except Day will trigger the right datetime_kind
                 if cycle.value().grain == Grain::Day {
                     res = res.datetime_kind(DatetimeKind::DatePeriod);
                 }
                 Ok(res)
             }
    );
    b.rule_3("n <cycle> avant",
             integer_check_by_range!(2, 9999),
             cycle_check!(),
             b.reg(r#"(?:d')? ?avant|plus t[oô]t"#)?,
             |integer, cycle, _| {
                 let mut res = helpers::cycle_nth(cycle.value().grain, -1 * integer.value().value)?;
                 // All grains except Day will trigger the right datetime_kind
                 if cycle.value().grain == Grain::Day {
                     res = res.datetime_kind(DatetimeKind::DatePeriod);
                 }
                 Ok(res)
             }
    );
    b.rule_3("n <cycle> après",
             integer_check_by_range!(2, 9999),
             cycle_check!(),
             b.reg(r#"(?:d')? ?apr[eèé]s|qui sui(?:t|ves?)|plus tard"#)?,
             |integer, cycle, _| {
                 let mut res = helpers::cycle_nth(cycle.value().grain, integer.value().value)?;
                 // All grains except Day will trigger the right datetime_kind
                 if cycle.value().grain == Grain::Day {
                     res = res.datetime_kind(DatetimeKind::DatePeriod);
                 }
                 Ok(res)
             }
    );
    // Cycle patterns relative to another datetime
    b.rule_4("le <cycle> après|suivant <datetime>",
             b.reg(r#"l[ea']? ?"#)?,
             cycle_check!(),
             b.reg(r#"suivante?|apr[eèé]s"#)?,
             datetime_check!(),
             |_, cycle, _, datetime| helpers::cycle_nth_after(cycle.value().grain, 1, datetime.value())
    );
    b.rule_4("le <cycle> avant|précédent <datetime>",
             b.reg(r#"l[ea']? ?"#)?,
             cycle_check!(),
             b.reg(r#"avant|pr[ée]c[ée]dent"#)?,
             datetime_check!(),
             |_, cycle, _, datetime| helpers::cycle_nth_after(cycle.value().grain, -1, datetime.value())
    );
    b.rule_1_terminal("fin du mois",
                      b.reg(r#"(?:(?:(?:[aà] )?la|en)? )?fin (?:du|de) mois"#)?,
                      |_| {
                          let month = helpers::cycle_nth(Grain::Month, 1)?;
                          Ok(helpers::cycle_nth_after(Grain::Day, -10, &month)?
                              .span_to(&month, false)?
                              .latent()
                              .form(Form::PartOfMonth))
                      }
    );
    b.rule_5("le <ordinal> <cycle> de <datetime>",
             b.reg(r#"l[ea]"#)?,
             ordinal_check_by_range!(1, 9999),
             cycle_check!(),
             b.reg(r#"d['eu]|en"#)?,
             datetime_check!(),
             |_, ordinal, cycle, _, datetime| helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value - 1, datetime.value())
    );
    b.rule_4("le <cycle> de <datetime>",
             b.reg(r#"l[ea]"#)?,
             cycle_check!(),
             b.reg(r#"d['eu]|en"#)?,
             datetime_check!(),
             |_, cycle, _, datetime| helpers::cycle_nth_after_not_immediate(cycle.value().grain, 0, datetime.value())
    );
    b.rule_2("le lendemain du <datetime>",
             b.reg(r#"(?:le|au)? ?lendemain du"#)?,
             datetime_check!(),
             |_, datetime| helpers::cycle_nth_after_not_immediate(Grain::Day, 1, datetime.value())
    );
    b.rule_2("la veille du <datetime>",
             b.reg(r#"(la )?veille du"#)?,
             datetime_check!(),
             |_, datetime| helpers::cycle_nth_after_not_immediate(Grain::Day, -1, datetime.value())
    );
    Ok(())
}

pub fn rules_datetime_with_duration(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("il y a <duration>",
             b.reg(r#"il y a"#)?,
             duration_check!(),
             |_, duration| duration.value().ago()
    );
    // With "depuis/d'ici", interpretation of duration ambiguous with time-of-day we choose time-of-day
    // I.e. "x heures (y)", but not "x heures et y minutes", "x minutes", etc.
    // FIXME: some time-of-day patterns should be removed, e.g. "x heures y minutes" - they are not
    // proper time-of-day, but they can be duration expressions
    // TODO: check if Grain::Second here breaks DatePeriod - cf. implem. of equivalent in English?
    b.rule_2("depuis <duration>",
             b.reg(r#"depuis|[cç]a fait"#)?,
             duration_check!(),
             |_, duration| {
                 if duration.value().get_coarser_grain() == Grain::Hour {
                     return Err(RuleError::Invalid.into())
                 }
                 duration.value().ago()?
                     .span_to(&helpers::cycle_nth(Grain::Second, 0)?, false)
             });
    b.rule_2("d'ici <duration>",
             b.reg(r#"d'ici|dans l(?:'|es?)"#)?,
             duration_check!(),
             |_, duration| {
                 let duration_grain = duration.value().get_coarser_grain();
                 // Priority to d'ici <time-of-day>
                 if duration_grain == Grain::Hour &&
                     // FIXME: There must be a better way to do this check!
                     duration.value().period.0.get(Grain::Hour as usize).unwrap_or(&0) <= &23 {
                     return Err(RuleError::Invalid.into())
                 }
                 let grain = if duration_grain.is_date_grain() { Grain::Day } else { Grain::Second };
                 let start = helpers::cycle_nth(grain, 0)?;
                 let end = if grain == Grain::Day { duration.value().in_present_day()? } else { duration.value().in_present()? };
                 start.span_to(&end, true)
             }
    );
    b.rule_2("dans le <duration>",
             b.reg(r#"dans l(?:'|es?)"#)?,
             duration_check!(),
             |_, duration| {
                 let duration_grain = duration.value().get_coarser_grain();
                 let grain = if duration_grain.is_date_grain() { Grain::Day } else { Grain::Second };
                 let start = helpers::cycle_nth(grain, 0)?;
                 let end = if grain == Grain::Day { duration.value().in_present_day()? } else { duration.value().in_present()? };
                 start.span_to(&end, false)
             }
    );
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
    Ok(())
}

pub fn rules_datetime(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("intersect",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |a, b| a.value().intersect(b.value())
    );
    b.rule_3("intersect by 'de' or ','",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             b.reg(r#"de|,"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |a, _, b| a.value().intersect(b.value())
    );
    b.rule_3("intersect by 'mais/par exemple/plutôt'",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             b.reg(r#"mais|par exemple|plutôt|plutot"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |a, _, b| a.value().intersect(b.value())
    );
    b.rule_2("en <named-month>",
             b.reg(r#"en|au mois d[e']|du mois d[e']"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, a| Ok(a.value().clone())
    );
    b.rule_2("pour <datetime>",
             b.reg(r#"pour"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             |_, a| Ok(a.value().clone())
    );
    b.rule_1_terminal("named-day",
        b.reg(r#"lun\.?(?:di)?"#)?,
        |_| helpers::day_of_week(Weekday::Mon)
    );
    b.rule_1_terminal("named-day",
        b.reg(r#"mar\.?(?:di)?"#)?,
        |_| helpers::day_of_week(Weekday::Tue)
    );
    b.rule_1_terminal("named-day",
        b.reg(r#"mer\.?(?:credi)?"#)?,
        |_| helpers::day_of_week(Weekday::Wed)
    );
    b.rule_1_terminal("named-day",
        b.reg(r#"jeu\.?(?:di)?"#)?,
        |_| helpers::day_of_week(Weekday::Thu)
    );
    b.rule_1_terminal("named-day",
        b.reg(r#"ven\.?(?:dredi)?"#)?,
        |_| helpers::day_of_week(Weekday::Fri)
    );
    b.rule_1_terminal("named-day",
        b.reg(r#"sam\.?(?:edi)?"#)?,
        |_| helpers::day_of_week(Weekday::Sat)
    );
    b.rule_1_terminal("named-day",
        b.reg(r#"dim\.?(?:anche)?"#)?,
        |_| helpers::day_of_week(Weekday::Sun)
    );
    b.rule_1_terminal("named-month",
        b.reg(r#"janvier|janv\.?"#)?,
        |_| helpers::month(1)
    );
    b.rule_1_terminal("named-month",
        b.reg(r#"fevrier|février|fev|fév\.?"#)?,
        |_| helpers::month(2)
    );
    b.rule_1_terminal("named-month",
        b.reg(r#"mars|mar\.?"#)?,
        |_| helpers::month(3)
    );
    b.rule_1_terminal("named-month",
        b.reg(r#"avril|avr\.?"#)?,
        |_| helpers::month(4)
    );
    b.rule_1_terminal("named-month",
        b.reg(r#"mai"#)?,
        |_| helpers::month(5)
    );
    b.rule_1_terminal("named-month",
        b.reg(r#"juin|jun\.?"#)?,
        |_| helpers::month(6)
    );
    b.rule_1_terminal("named-month",
        b.reg(r#"juillet|juil?\."#)?,
        |_| helpers::month(7)
    );
    b.rule_1_terminal("named-month",
        b.reg(r#"aout|août|aou\.?"#)?,
        |_| helpers::month(8)
    );
    b.rule_1_terminal("named-month",
//      b.reg(r#"septembre|sept?\.?"#)?, // "sept" with no dot forbidden (confusion with nb "sept" in "à trois heures trente sept")
        b.reg(r#"septembre|sept\.|sep\.?"#)?,
        |_| helpers::month(9)
    );
    b.rule_1_terminal("named-month",
        b.reg(r#"octobre|oct\.?"#)?,
        |_| helpers::month(10)
    );
    b.rule_1_terminal("named-month",
        b.reg(r#"novembre|nov\.?"#)?,
        |_| helpers::month(11)
    );
    b.rule_1_terminal("named-month",
        b.reg(r#"décembre|decembre|déc\.?|dec\.?"#)?,
        |_| helpers::month(12)
    );

    b.rule_1_terminal("maintenant",
        b.reg(r#"maintenant|tout de suite|en ce moment"#)?,
        |_| helpers::cycle_nth(Grain::Second, 0)
    );
    b.rule_1_terminal("aujourd'hui",
        b.reg(r#"(?:aujourd'? ?hui)|(?:ce jour)|(?:dans la journ[ée]e?)"#)?,
        |_| helpers::cycle_nth(Grain::Day, 0)
    );
    // FIXME: "le lendemain" interpreted as demain, not as relative to another date
    // but there is a rule "le lendemain du <date>" - inconsistent
    b.rule_1_terminal("demain",
        b.reg(r#"(?:demain)|(?:le lendemain)"#)?,
        |_| helpers::cycle_nth(Grain::Day, 1)
    );
    b.rule_1_terminal("hier",
        b.reg(r#"hier|la veille"#)?,
        |_| helpers::cycle_nth(Grain::Day, -1)
    );
    b.rule_1_terminal("fin du mois",
        b.reg(r#"(?:(?:[aà] )?la )?fin (?:du|de) mois"#)?,
        |_| {
            let month = helpers::cycle_nth(Grain::Month, 1)?;
            Ok(helpers::cycle_nth_after(Grain::Day, -10, &month)?
                .span_to(&month, false)?
                .latent()
                .form(Form::PartOfMonth))
        }
    );
    b.rule_1_terminal("après-demain",
        b.reg(r#"apr(?:e|è)s[- ]?demain"#)?,
        |_| helpers::cycle_nth(Grain::Day, 2)
    );
    b.rule_1_terminal("avant-hier",
        b.reg(r#"avant[- ]?hier"#)?,
        |_| helpers::cycle_nth(Grain::Day, -2)
    );
    b.rule_2("ce <day-of-week>",
             b.reg(r#"ce"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             |_, datetime| datetime.value().the_nth_not_immediate(0)
    );
    b.rule_2("ce <datetime>",
             b.reg(r#"ce"#)?,
             datetime_check!(),
             |_, datetime| Ok(datetime.value().the_nth(0)?
                 .datetime_kind(datetime.value().datetime_kind.clone()))
    );
    b.rule_2("<day-of-week> prochain",
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"prochain"#)?,
             |datetime, _| datetime.value().the_nth_not_immediate(0)
    );
    b.rule_2("<date> prochain",
             datetime_check!(|datetime: &DatetimeValue| datetime.form.is_day()),
             b.reg(r#"prochaine?"#)?,
             |datetime, _| datetime.value().the_nth_not_immediate(0)
    );
    // TODO: add restrictions on datetime form?
    b.rule_2("au prochain <datetime>",
             b.reg(r#"(au |(?:[aà] )?l[ae] )prochaine?"#)?,
             datetime_check!(|datetime: &DatetimeValue| !form!(Form::PartOfDay(_))(datetime) && !form!(Form::Meal)(datetime)),
             |_, a| {
                 Ok(a.value().the_nth(0)?
                     .form(a.value().form.clone())
                     .datetime_kind(a.value().datetime_kind.clone()))
             }
    );
    // TODO: add restrictions on datetime form?
    b.rule_2("au dernier <datetime>",
             b.reg(r#"(au |(?:[aà] )?l[ea] )derni[eè]re?"#)?,
             datetime_check!(|datetime: &DatetimeValue| !form!(Form::PartOfDay(_))(datetime) && !form!(Form::Meal)(datetime)),
             |_, a| {
                 Ok(a.value().the_nth(-1)?
                     .form(a.value().form.clone())
                     .datetime_kind(a.value().datetime_kind.clone()))
             }
    );
    b.rule_2("au prochain <date>",
             b.reg(r#"(au |(?:[aà] )?la )prochaine?"#)?,
             datetime_check!(|datetime: &DatetimeValue| datetime.form.is_day()),
             |_, datetime| datetime.value().the_nth_not_immediate(0)
    );
    b.rule_2("au dernier <date>",
             b.reg(r#"(au |(?:[aà] )?la )dernier?"#)?,
             datetime_check!(|datetime: &DatetimeValue| datetime.form.is_day()),
             |_, datetime| datetime.value().the_nth_not_immediate(-1)
    );
    b.rule_2("<named-month> prochain",
             // The direction check is to avoid application of datetime_check(month) on rule result
             // "avant <named-month>"
             datetime_check!(|datetime: &DatetimeValue| form!(Form::Month(_))(datetime) && !datetime.direction.is_some()),
             b.reg(r#"prochain"#)?,
             |datetime, _| datetime.value().the_nth_not_immediate(0)
    );
    b.rule_2("<named-month|named-day> suivant|d'après",
             datetime_check!(),
             b.reg(r#"suivante?s?|d'apr[eéè]s"#)?,
             |datetime, _| datetime.value().the_nth(1)
    );
    b.rule_2("<named-month|named-day> dernier|passé",
             datetime_check!(),
             b.reg(r#"derni[eéè]re?|pass[ée]e?"#)?,
             |datetime, _| datetime.value().the_nth(-1)
    );
    b.rule_2("<named-day> en huit",
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"en (?:huit|8)"#)?,
             |datetime, _| datetime.value().the_nth(1)
    );
    b.rule_2("<named-day> en quinze",
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"en (quinze|15)"#)?,
             |datetime, _| datetime.value().the_nth(2)
    );
    b.rule_4("dernier <day-of-week> de <datetime> (latent)",
             b.reg(r#"derni[eéè]re?"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"d['e]"#)?,
             datetime_check!(),
             |_, dow, _, datetime| dow.value().last_of(datetime.value())
    );
    b.rule_4("dernier <day-of-week> de <datetime> (latent)",
             b.reg(r#"derni[eéè]re?"#)?,
             cycle_check!(),
             b.reg(r#"d['e]"#)?,
             datetime_check!(),
             |_, cycle, _, datetime| cycle.value().last_of(datetime.value())
    );
    b.rule_4("<ordinal> <datetime> de <datetime>",
             ordinal_check!(), // the first
             datetime_check!(), // Thursday
             b.reg(r#"d[e']"#)?, // of
             datetime_check!(), // march
             |ordinal, a, _, b| {
                 b.value().intersect(a.value())?.the_nth(ordinal.value().value - 1)
             }
    );
    b.rule_3("<ordinal> week-end de <datetime>",
             ordinal_check!(),
             b.reg(r#"week(?:\s|-)?end (?:d['eu]|en|du mois de)"#)?,
             datetime_check!(form!(Form::Month(_))),
             |ordinal, _, datetime| {
                 let weekend = helpers::weekend()?;
                 let nth_week_end = datetime.value().intersect(&weekend)?;
                 nth_week_end.the_nth(ordinal.value().value - 1)
             }
    );
    b.rule_2("dernier week-end de <datetime>",
             b.reg(r#"(?:le )?dernier week(?:\s|-)?end (?:du mois d[e']|d['eu]|en)"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, datetime| {
                 let weekend = helpers::weekend()?;
                 weekend.last_of(datetime.value())
             }
    );
    // FIXME: change latency ranges for years? E.g. latent until 1900?
    b.rule_1("year",
             integer_check_by_range!(1000, 2100),
             |integer| helpers::year(integer.value().value as i32)
    );
    b.rule_1("year (latent)",
             integer_check_by_range!(-1000, 999),
             |integer| Ok(helpers::year(integer.value().value as i32)?.latent())
    );
    b.rule_2("l'année <year>",
             b.reg(r#"l[' ]an(?:n[eé]+)?"#)?,
             integer_check!(),
             |_, integer| helpers::year(integer.value().value as i32)
    );
    b.rule_2("en <year>",
             b.reg(r#"en"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && form!(Form::Year(_))(datetime)),
             |_, year| Ok(year.value().clone())
    );
    b.rule_1("year (latent)",
             integer_check_by_range!(2101, 3000),
             |integer| Ok(helpers::year(integer.value().value as i32)?.latent())
    );
    b.rule_1_terminal("day of month (premier)",
        b.reg(r#"premier|prem\.?|1er|1 er"#)?,
        |_| helpers::day_of_month(1)
    );
    b.rule_2("le <day-of-month> (non ordinal)",
             b.reg(r#"le"#)?,
             integer_check_by_range!(1, 31),
             |_, integer| helpers::day_of_month(integer.value().value as u32)
    );
    b.rule_4("le <day-of-month> à <time-of-day>",
             b.reg(r#"le"#)?,
             integer_check_by_range!(1, 31),
             b.reg(r#"[aà]"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && form!(Form::TimeOfDay(_))(datetime)),
             |_, integer, _, datetime| {
                 let day_of_month = helpers::day_of_month(integer.value().value as u32)?;
                 day_of_month.intersect(&datetime.value())
             }
    );
    b.rule_2("<day-of-month> <named-month>",
             integer_check_by_range!(1, 31),
             datetime_check!(form!(Form::Month(_))),
             |integer, month| Ok(month.value()
                 .intersect(&helpers::day_of_month(integer.value().value as u32)?)?
                 .form(Form::DayOfMonth))
    );
    b.rule_2("<day-of-week> <day-of-month>",
             datetime_check!(form!(Form::DayOfWeek{..})), // Weird it is not used in the production of the rule
             integer_check_by_range!(1, 31),
             |_, integer| helpers::day_of_month(integer.value().value as u32)
    );
    b.rule_4("<day-of-week> <day-of-month> à <time-of-day>)",
             datetime_check!(form!(Form::DayOfWeek{..})), // Weird it is not used in the production of the rule
             integer_check_by_range!(1, 31),
             b.reg(r#"[aà]"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && form!(Form::TimeOfDay(_))(datetime)),
             |_, integer, _, tod| helpers::day_of_month(integer.value().value as u32)
                 ?.intersect(tod.value())
    );
    b.rule_1("<time-of-day> (latent)",
             integer_check_by_range!(1, 23),
             |integer| Ok(helpers::hour(integer.value().value as u32, integer.value().value < 12)?.latent())
    );
    b.rule_1("<time-of-day> (latent)",
             integer_check_by_range!(0, 0),
             |_| Ok(helpers::hour(0, false)?.latent())
    );
    b.rule_1_terminal("midi",
        b.reg(r#"midi(?: pile| exactement| pr[eé]cises)?"#)?,
        |_| helpers::hour(12, false)
    );
    b.rule_1_terminal("minuit",
        b.reg(r#"minuit(?: pile| exactement| pr[eé]cises)?"#)?,
        |_| helpers::hour(0, false)
    );
    b.rule_2("<time-of-day> heures",
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))),
             b.reg(r#"h\.?(?:eure)?s?(?: pile| exactement| pr[eé]cises?)?"#)?,
             |a, _| Ok(a.value().clone().not_latent())
    );
    b.rule_2("<time-of-day> (heures) pile",
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))),
             b.reg(r#"pile"#)?,
             |a, _| Ok(a.value().clone().not_latent())
    );
    b.rule_2("à|vers <time-of-day>",
             b.reg(r#"(?:vers|autour de|[aà] environ|aux alentours de|[aà])"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, a| Ok(a.value().clone().not_latent())
    );
    b.rule_1_terminal("hh(:|h)mm (time-of-day)",
        b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:h]([0-5]\d)"#)?,
        |text_match| {
            let hour: u32 = text_match.group(1).parse()?;
            let minute: u32 = text_match.group(2).parse()?;
            helpers::hour_minute(hour, minute, hour < 12)
        }
    );
    b.rule_3_terminal("hh(:|h)mm - hh(:|h)mm (time-of-day interval)",
                      b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:h]([0-5]\d)"#)?,
                      b.reg(r#" ?\- ?"#)?,
                      b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:h]([0-5]\d)"#)?,
                      |a, _, b| {
                          let hour_start: u32 = a.group(1).parse()?;
                          let minute_start: u32 = a.group(2).parse()?;
                          let hour_end: u32 = b.group(1).parse()?;
                          let minute_end: u32 = b.group(2).parse()?;
                          let start = helpers::hour_minute(hour_start, minute_start, hour_start < 12)?;
                          let end = helpers::hour_minute(hour_end, minute_end, hour_end < 12)?;
                          start.smart_span_to(&end, false)
                      }
    );
    b.rule_1_terminal("hh:mm:ss",
        b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:.]([0-5]\d)[:.]([0-5]\d)"#)?,
        |text_match| helpers::hour_minute_second(
                text_match.group(1).parse()?,
                text_match.group(2).parse()?,
                text_match.group(3).parse()?,
                false
                )

    );
    b.rule_1_terminal("hhmm (military time-of-day)",
        b.reg(r#"((?:[01]?\d)|(?:2[0-3]))([0-5]\d)"#)?,
        |text_match| Ok(helpers::hour_minute(
            text_match.group(1).parse()?,
            text_match.group(2).parse()?,
            false
            )?.latent())
    );
    b.rule_1_terminal("quart (relative minutes)",
        b.reg(r#"(?:un )?quart"#)?,
        |_| Ok(RelativeMinuteValue(15))
    );
    b.rule_1_terminal("demi (relative minutes)",
        b.reg(r#"demie?"#)?,
        |_| Ok(RelativeMinuteValue(30))
    );
    b.rule_1_terminal("trois quarts (relative minutes)",
        b.reg(r#"(?:3|trois) quarts?"#)?,
        |_| Ok(RelativeMinuteValue(45))
    );
    b.rule_1("number (as relative minutes)",
             integer_check_by_range!(1, 59),
             |a| Ok(RelativeMinuteValue(a.value().value as i32))
    );
    b.rule_2("number minutes (as relative minutes)",
             integer_check_by_range!(1, 59),
             b.reg(r#"min\.?(?:ute)?s?"#)?,
             |a, _| Ok(RelativeMinuteValue(a.value().value as i32))
    );
    b.rule_2("<hour-of-day> <integer> (as relative minutes)",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))(datetime)),
             relative_minute_check!(),
             |datetime, minutes| helpers::hour_relative_minute(
                 datetime.value().form_time_of_day()?.full_hour(),
                 minutes.value().0,
                 datetime.value().form_time_of_day()?.is_12_clock()
             )
    );
    b.rule_3("<hour-of-day> <integer> (as relative minutes) exactly",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))(datetime)),
             relative_minute_check!(),
             b.reg(r#"pile|exactement|pr[ée]cises?"#)?,
             |datetime, minutes, _| helpers::hour_relative_minute(
                 datetime.value().form_time_of_day()?.full_hour(),
                 minutes.value().0,
                 datetime.value().form_time_of_day()?.is_12_clock()
             )
    );
    b.rule_3("<hour-of-day> moins <integer> (as relative minutes)",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))(datetime)),
             b.reg(r#"moins(?: le)?"#)?,
             relative_minute_check!(),
             |datetime, _, minutes| helpers::hour_relative_minute(
                 datetime.value().form_time_of_day()?.full_hour(),
                 -1 * minutes.value().0,
                 datetime.value().form_time_of_day()?.is_12_clock()
             )
    );
    b.rule_4("<hour-of-day> moins <integer> (as relative minutes) exactly ",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))(datetime)),
             b.reg(r#"moins(?: le)?"#)?,
             relative_minute_check!(),
             b.reg(r#"pile|exactement|pr[ée]cises?"#)?,
             |datetime, _, minutes, _| helpers::hour_relative_minute(
                 datetime.value().form_time_of_day()?.full_hour(),
                 -1 * minutes.value().0,
                 datetime.value().form_time_of_day()?.is_12_clock()
             )
    );
    b.rule_3("<hour-of-day> et|passé de <relative minutes>",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))(datetime)),
             b.reg(r#"et|pass[ée]e?s? de"#)?,
             relative_minute_check!(),
             |datetime, _, minutes| helpers::hour_relative_minute(
                 datetime.value().form_time_of_day()?.full_hour(),
                 minutes.value().0,
                 datetime.value().form_time_of_day()?.is_12_clock()
             )
    );
    b.rule_4("<hour-of-day> et|passé de <relative minutes> exactly",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))(datetime)),
             b.reg(r#"et|pass[ée]e?s? de"#)?,
             relative_minute_check!(),
             b.reg(r#"pile|exactement|pr[ée]cises?"#)?,
             |datetime, _, minutes, _| helpers::hour_relative_minute(
                 datetime.value().form_time_of_day()?.full_hour(),
                 minutes.value().0,
                 datetime.value().form_time_of_day()?.is_12_clock()
             )
    );
    b.rule_4("<time-of-day> de <part-of-day> exactly",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:d[eu] |dans )(?:l[ 'a])?"#)?,
             datetime_check!(form!(Form::PartOfDay(_))),
             b.reg(r#"pile|exactement|pr[eé]cises?"#)?,
             |a, _, b, _| Ok(a.value().intersect(b.value())?.form(a.value().form.clone()))
    );
    // Adding "pour" here makes time-of-day ambiguous w/ Duration
    // Duration has less priority than Datetime types, therefore duration will be output only
    // if the output kind filter is set for Duration
    b.rule_2("à <time-of-day>",
             b.reg(r#"[aà]|pour"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, a| Ok(a.value().clone().not_latent())
    );
    b.rule_2("à <time-of-day>",
             b.reg(r#"[aà]|pour"#)?,
             datetime_check!(form!(Form::PartOfDay(_))),
             |_, a| Ok(a.value().clone().not_latent())
    );
    b.rule_2("vers <time-of-day>",
             b.reg(r#"(?:plut[ôo]t )?(?:vers|autour de|[aà] environ|aux alentours de)"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, a| Ok(a.value().clone().not_latent().precision(Precision::Approximate))
    );
    // Written time/date in numeric formats
    b.rule_1_terminal("hh(:|h)mm (time-of-day)",
                      b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:h]([0-5]\d)"#)?,
                      |text_match| {
                          let hour: u32 = text_match.group(1).parse()?;
                          let minute: u32 = text_match.group(2).parse()?;
                          helpers::hour_minute(hour, minute, hour < 12)
                      }
    );
    b.rule_3_terminal("hh(:|h)mm - hh(:|h)mm (time-of-day interval)",
                      b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:h]([0-5]\d)"#)?,
                      b.reg(r#" ?\- ?"#)?,
                      b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:h]([0-5]\d)"#)?,
                      |a, _, b| {
                          let hour_start: u32 = a.group(1).parse()?;
                          let minute_start: u32 = a.group(2).parse()?;
                          let hour_end: u32 = b.group(1).parse()?;
                          let minute_end: u32 = b.group(2).parse()?;
                          let start = helpers::hour_minute(hour_start, minute_start, hour_start < 12)?;
                          let end = helpers::hour_minute(hour_end, minute_end, hour_end < 12)?;
                          start.smart_span_to(&end, false)
                      }
    );
    b.rule_1_terminal("hh:mm:ss",
                      b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:.]([0-5]\d)[:.]([0-5]\d)"#)?,
                      |text_match| helpers::hour_minute_second(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(3).parse()?,
                          false
                      )

    );
    b.rule_1_terminal("hhmm (military time-of-day)",
                      b.reg(r#"((?:[01]\d)|(?:2[0-3]))([0-5]\d)"#)?,
                      |text_match| Ok(helpers::hour_minute(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?,
                          false
                      )?.latent())
    );
    b.rule_1_terminal("yyyy-mm-dd - ISO",
                      b.reg(r#"(\d{4})[-/](0?[1-9]|1[0-2])[-/](3[01]|[12]\d|0?[1-9])"#)?,
                      |text_match| helpers::year_month_day(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(3).parse()?)
    );
    // Supporting these date formats also with whitespace as a separator for legacy
    // But this seems too permissive?
    b.rule_1_terminal("dd/mm/yy or dd/mm/yyyy",
                      b.reg(r#"(0?[1-9]|[12]\d|3[01])[-\./ ](0?[1-9]|1[0-2])[-\./ ](\d{2,4})"#)?,
                      |text_match| helpers::year_month_day(
                          text_match.group(3).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(1).parse()?,
                      )
    );
    b.rule_1_terminal("dd/mm",
                      b.reg(r#"(0?[1-9]|[12]\d|3[01])[\./ ](1[0-2]|0?[1-9])"#)?,
                      |text_match| helpers::month_day(
                          text_match.group(2).parse()?,
                          text_match.group(1).parse()?)
    );
    // End of Written time/date in numeric formats
    b.rule_1_terminal("matin",
        b.reg(r#"mat(?:in[ée]?e?)?"#)?,
        |_| Ok(helpers::hour(4, false)?
                .span_to(&helpers::hour(12, false)?, false)?
                .latent()
                .form(Form::PartOfDay(PartOfDayForm::Morning)))
    );
    b.rule_1_terminal("début de matinée",
        b.reg(r#"(?:le matin (?:tr[eè]s )?t[ôo]t|(?:tr[eè]s )?t[ôo]t le matin|d[ée]but de matin[ée]e)"#)?,
        |_| Ok(helpers::hour(4, false)?
                .span_to(&helpers::hour(9, false)?, false)?
                .latent()
                .form(Form::PartOfDay(PartOfDayForm::Morning)))
    );
    b.rule_1_terminal("lever du soleil",
                      b.reg(r#"lever d[ue] soleil|aurore|aube"#)?,
                      |_| Ok(helpers::hour(4, false)?
                          .span_to(&helpers::hour(8, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Morning)))
    );
    b.rule_1_terminal("lever du soleil",
                      b.reg(r#"lever d[ue] soleil|aurore|aube"#)?,
                      |_| Ok(helpers::hour(4, false)?
                          .span_to(&helpers::hour(8, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Morning)))
    );
    b.rule_1_terminal("petit dejeuner",
        b.reg(r#"petit[- ]d[ée]jeuner"#)?,
        |_| Ok(helpers::hour(5, false)?
                .span_to(&helpers::hour(10, false)?, false)?
                .latent()
                .form(Form::Meal))
    );
    b.rule_1_terminal("milieu de matinée",
         b.reg(r#"(?:le )?milieu de (?:la )?matin[ée]e"#)?,
         |_| Ok(helpers::hour(9, false)?
                .span_to(&helpers::hour(11, false)?, false)?
                .latent()
                .form(Form::PartOfDay(PartOfDayForm::Morning)))
    );
    b.rule_1_terminal("brunch",
        b.reg(r#"brunch"#)?,
        |_| Ok(helpers::hour(10, false)?
                .span_to(&helpers::hour(15, false)?, false)?
                .latent()
                .form(Form::Meal))
    );
    b.rule_1_terminal("fin de matinée",
                      b.reg(r#"fin de (?:la )?matin[ée]e"#)?,
                      |_| Ok(helpers::hour(10, false)?
                          .span_to(&helpers::hour(12, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Morning)))
    );
    b.rule_1_terminal("déjeuner",
        b.reg(r#"d[eéè]jeuner"#)?,
        |_| Ok(helpers::hour(12, false)?
                .span_to(&helpers::hour(14, false)?, false)?
                .latent()
                .form(Form::Meal))
    );
    b.rule_1_terminal("après le déjeuner",
        b.reg(r#"apr[eè]s (?:le )?d[eéè]jeuner"#)?,
        |_| {
            let period = helpers::hour(13, false)?
                    .span_to(&helpers::hour(17, false)?, false)?;
            Ok(helpers::cycle_nth(Grain::Day, 0)?.intersect(&period)?.form(Form::PartOfDay(PartOfDayForm::Afternoon)))
        }
    );
    b.rule_1_terminal("avant le déjeuner",
        b.reg(r#"avant (?:le )?d[eéè]jeuner"#)?,
        |_| {
            let period = helpers::hour(10, false)?
                    .span_to(&helpers::hour(12, false)?, false)?;
            Ok(helpers::cycle_nth(Grain::Day, 0)?.intersect(&period)?.form(Form::PartOfDay(PartOfDayForm::Morning)))
        }
    );
    b.rule_1_terminal("avant le travail",
        b.reg(r#"avant le travail"#)?,
        |_| {
            let period = helpers::hour(7, false)?
                    .span_to(&helpers::hour(10, false)?, false)?;
            Ok(helpers::cycle_nth(Grain::Day, 0)?.intersect(&period)?.form(Form::PartOfDay(PartOfDayForm::Morning)))
        }
    );
    b.rule_1_terminal("pendant le travail",
        b.reg(r#"pendant le travail"#)?,
        |_| {
            let period = helpers::hour(9, false)?
                    .span_to(&helpers::hour(19, false)?, false)?;
            Ok(helpers::cycle_nth(Grain::Day, 0)?.intersect(&period)?.form(Form::PartOfDay(PartOfDayForm::None)))
        }
    );
    b.rule_1_terminal("après le travail",
        b.reg(r#"apr[eè]s (?:le )?travail"#)?,
        |_| {
            let period = helpers::hour(17, false)?
                    .span_to(&helpers::hour(21, false)?, false)?;
            Ok(helpers::cycle_nth(Grain::Day, 0)?.intersect(&period)?.form(Form::PartOfDay(PartOfDayForm::Evening)))
        }
    );
    b.rule_1_terminal("après-midi",
        b.reg(r#"apr[eéè]s?[ \-]?midi|aprem"#)?,
        |_| {
            Ok(helpers::hour(12, false)?
                    .span_to(&helpers::hour(19, false)?, false)?
                    .latent()
                    .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
        }
    );
    b.rule_1_terminal("début d'après-midi",
        b.reg(r#"d[ée]but (?:d'|de l')(?:apr[eéè]s?[ \-]?midi|aprem)"#)?,
        |_| {
            Ok(helpers::hour(12, false)?
                    .span_to(&helpers::hour(15, false)?, false)?
                    .latent()
                    .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
        }
    );
    b.rule_1_terminal("milieu d'après-midi",
        b.reg(r#"milieu (?:d'|de l')(?:apr[eéè]s?[ \-]?midi|aprem)"#)?,
        |_| {
            Ok(helpers::hour(15, false)?
                    .span_to(&helpers::hour(17, false)?, false)?
                    .latent()
                    .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
        }
    );
    b.rule_1_terminal("gouter",
                      b.reg(r#"(?:(?:[àa] )?l[' ]heure du|au(?: moment du)?|pendant le|pour le) go[uû]ter"#)?,
                      |_| Ok(helpers::hour(16, false)?
                          .span_to(&helpers::hour(18, false)?, false)?
                          .form(Form::Meal))
    );
    b.rule_1_terminal("thé",
        b.reg(r#"(?:(?:[àa] )?l[' ]heure du|au moment du|pendant le|pour le) th[eé]"#)?,
        |_| Ok(helpers::hour(15, false)?
                .span_to(&helpers::hour(17, false)?, false)?
                .form(Form::Meal))
    );
    b.rule_1_terminal("cafe",
        b.reg(r#"(?:(?:[àa] )?l[' ]heure du|au moment du|pendant le|pour le) caf[eé]"#)?,
        |_| Ok(helpers::hour(14, false)?
                .span_to(&helpers::hour(16, false)?, false)?
                .form(Form::Meal))
    );
    b.rule_1_terminal("fin d'après-midi",
        b.reg(r#"fin (?:d'|de l')(?:apr[eéè]s?[ \-]?midi|aprem)"#)?,
        |_| {
            Ok(helpers::hour(17, false)?
                    .span_to(&helpers::hour(19, false)?, false)?
                    .latent()
                    .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
        }
    );
    // TODO: APERO
    b.rule_1_terminal("début de journée",
        b.reg(r#"d[ée]but de (?:la )?journ[ée]e"#)?,
        |_| {
            Ok(helpers::hour(6, false)?
                    .span_to(&helpers::hour(10, false)?, false)?
                    .latent()
                    .form(Form::PartOfDay(PartOfDayForm::Morning)))
        }
    );
    b.rule_1_terminal("milieu de journée",
                      b.reg(r#"(?:milieu de (?:la )?|(?:(?:[àa] )?la )?mi[ -])journ[ée]e"#)?,
                      |_| {
                          Ok(helpers::hour(12, false)?
                              .span_to(&helpers::hour(16, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::None)))
                      }
    );
    b.rule_1_terminal("fin de journée",
        b.reg(r#"fin de (?:la )?journ[ée]e"#)?,
        |_| {
            Ok(helpers::hour(17, false)?
                    .span_to(&helpers::hour(21, false)?, false)?
                    .latent()
                    .form(Form::PartOfDay(PartOfDayForm::Evening)))
        }
    );
    b.rule_1_terminal("soir",
        b.reg(r#"soir[ée]?e?"#)?,
        |_| {
            Ok(helpers::hour(18, false)?
                    .span_to(&helpers::hour(0, false)?, false)?
                    .latent()
                    .form(Form::PartOfDay(PartOfDayForm::Evening)))
        }
    );
    b.rule_1_terminal("coucher du soleil",
                      b.reg(r#"coucher d[eu] soleil|cr[eé]puscule|tomb[ée]e de la nuit"#)?,
                      |_| {
                          Ok(helpers::hour(19, false)?
                              .span_to(&helpers::hour(22, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );
    b.rule_1_terminal("coucher du soleil",
                      b.reg(r#"coucher du soleil|cr[eé]puscule|tomb[ée]e de la nuit"#)?,
                      |_| {
                          Ok(helpers::hour(19, false)?
                              .span_to(&helpers::hour(22, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );
    b.rule_1_terminal("début de soirée",
        b.reg(r#"d[ée]but de (?:la )?soir[ée]e?"#)?,
        |_| {
            Ok(helpers::hour(18, false)?
                    .span_to(&helpers::hour(21, false)?, false)?
                    .latent()
                    .form(Form::PartOfDay(PartOfDayForm::Evening)))
        }
    );
    b.rule_1_terminal("fin de soirée",
        b.reg(r#"fin de (?:la )?soir[ée]e?"#)?,
        |_| {
            Ok(helpers::hour(21, false)?
                    .span_to(&helpers::hour(0, false)?, false)?
                    .latent()
                    .form(Form::PartOfDay(PartOfDayForm::Evening)))
        }
    );
    b.rule_1_terminal("diner",
        b.reg(r#"d[iî]ner|souper"#)?,
        |_| Ok(helpers::hour(18, false)?
                .span_to(&helpers::hour(23, false)?, false)?
                .form(Form::Meal))
    );
    b.rule_1_terminal("nuit",
        b.reg(r#"nuit"#)?,
        |_| {
            Ok(helpers::hour(22, false)?
                    .span_to(&helpers::hour(6, false)?, false)?
                    .latent()
                    .form(Form::PartOfDay(PartOfDayForm::Night)))
        }
    );
    b.rule_1_terminal("milieu de la nuit",
                      b.reg(r#"milieu de la nuit"#)?,
                      |_| {
                          Ok(helpers::hour(2, false)?
                              .span_to(&helpers::hour(4, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Night)))
                      }
    );
    b.rule_1_terminal("milieu de la nuit",
                      b.reg(r#"milieu de la nuit"#)?,
                      |_| {
                          Ok(helpers::hour(2, false)?
                              .span_to(&helpers::hour(4, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Night)))
                      }
    );
    b.rule_2("a l'heure de <meal>",
             b.reg(r#"(?:[àa] )?l[' ]heure du|au moment du|pendant l[ea']|au|pour l[ea']|l[ea']"#)?,
             datetime_check!(|datetime: &DatetimeValue| form!(Form::Meal)(datetime)),
             |_, a| Ok(a.value().clone().not_latent())
    );
    b.rule_2("prep? & article <part-of-day>", // This is very catch-all/junky
             b.reg(r#"(?:pendant |durant |dans |d[eè]s )?l[ae']?|en|au"#)?,
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime)),
             |_, a| Ok(a.value().clone().not_latent())
    );
    b.rule_2("ce <part-of-day>",
             b.reg(r#"cet?t?e?"#)?,
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             |_, datetime| Ok(helpers::cycle_nth(Grain::Day, 0)?
                 .intersect(datetime.value())?
                 .form(datetime.value().form.clone())
                 .datetime_kind(DatetimeKind::DatetimeComplement { date_and_time: true, today: true }))
    );
    b.rule_2("intersect <date> <part-of-day|meal>",
             datetime_check!(|datetime: &DatetimeValue| datetime.form.is_day()),
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             |a, b| a.value().intersect(b.value())
    );
    b.rule_2("intersect <part-of-day|meal> <date>",
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             datetime_check!(|datetime: &DatetimeValue| datetime.form.is_day()),
             |a, b| a.value().intersect(b.value())
    );
    b.rule_2("<time-of-day> du matin",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:(?:du|dans|de) )?(?:(?:au|le|la) )?mat(?:in[ée]?e?)?(?: pile| exactement| pr[eé]cises?)?"#)?,
             |a, _| {
                 let period = helpers::hour(0, false)?
                     .span_to(&helpers::hour(12, false)?, false)?;
                 Ok(a.value().intersect(&period)?.form(a.value().form.clone()))
             }
    );
    b.rule_2("<time-of-day> de l'apres-midi",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:dans |de )?l[' ]apr[eè]s[\- ]midi(?: pile| exactement| pr[eé]cises?)?"#)?,
             |a, _| {
                 let period = helpers::hour(12, false)?
                     .span_to(&helpers::hour(19, false)?, false)?;
                 Ok(a.value().intersect(&period)?.form(a.value().form.clone()))
             }
    );
    b.rule_2("<time-of-day> du soir",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:(?:du|dans|de) )?(?:(?:au|le|la) )?soir[ée]?e?(?: pile| exactement| pr[eé]cises?)?"#)?,
             |a, _| {
                 let period = helpers::hour(16, false)?
                     .span_to(&helpers::hour(0, false)?, false)?;
                 Ok(a.value().intersect(&period)?.form(a.value().form.clone()))
             }
    );
    b.rule_3("<part-of-day> du <date>",
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             b.reg(r#"du"#)?,
             datetime_check!(|datetime: &DatetimeValue| datetime.form.is_day()),
             |a, _, b| b.value().intersect(a.value())
    );
    b.rule_1_terminal("(ce/le) week-end",
        b.reg(r#"(?:[cl]e )?week(?:\s|-)?end"#)?,
        |_| helpers::weekend()
    );
    b.rule_1_terminal("le week-end dernier",
        b.reg(r#"le week(?:\s|-)?end dernier"#)?,
        |_| {
            let weekend = helpers::weekend()?;
            Ok(weekend.the_nth(-1)?.datetime_kind(DatetimeKind::DatePeriod))
        }
    );
    b.rule_1_terminal("le week-end prochain",
        b.reg(r#"le week(?:\s|-)?end prochain|le prochain week(?:\s|-)?end"#)?,
        |_| {
            let weekend = helpers::weekend()?;
            Ok(weekend.the_nth(1)?.datetime_kind(DatetimeKind::DatePeriod))
        }
    );
    b.rule_1_terminal("début de semaine",
        b.reg(r#"(?:en |au )?d[ée]but de (?:cette |la )?semaine"#)?,
        |_| helpers::day_of_week(Weekday::Mon)
                    ?.span_to(&helpers::day_of_week(Weekday::Tue)?, false)
    );
    b.rule_1_terminal("milieu de semaine",
        b.reg(r#"(?:en |au )?milieu de (?:cette |la )?semaine"#)?,
        |_| helpers::day_of_week(Weekday::Wed)
                    ?.span_to(&helpers::day_of_week(Weekday::Thu)?, false)
    );
    b.rule_1_terminal("fin de semaine (Warning: this is the weekend in Quebec)",
        b.reg(r#"(?:en |à la )?fin de (?:cette |la )?semaine"#)?,
        |_| helpers::day_of_week(Weekday::Thu)
                    ?.span_to(&helpers::day_of_week(Weekday::Sun)?, false)
    );
    b.rule_1_terminal("en semaine",
        b.reg(r#"(?:pendant la |en )semaine"#)?,
        |_| helpers::day_of_week(Weekday::Mon)
                    ?.span_to(&helpers::day_of_week(Weekday::Fri)?, false)
    );
    b.rule_1_terminal("season",
        b.reg(r#"(?:cet )?(?:été|ete)"#)?,
        |_| helpers::month_day(6, 21)?.span_to(&helpers::month_day(9, 23)?, false)
    );
    b.rule_1_terminal("season",
        b.reg(r#"(?:cet )?automne"#)?,
        |_| helpers::month_day(9, 23)?.span_to(&helpers::month_day(12, 21)?, false)
    );
    b.rule_1_terminal("season",
        b.reg(r#"(?:cet )?hiver"#)?,
        |_| helpers::month_day(12, 21)?.span_to(&helpers::month_day(3, 20)?, false)
    );
    b.rule_1_terminal("season",
        b.reg(r#"(?:ce )?printemps"#)?,
        |_| helpers::month_day(3, 20)?.span_to(&helpers::month_day(6, 21)?, false)
    );
    b.rule_1_terminal("début de l'été",
                      b.reg(r#"début de (?:cet |l')?(?:été|ete)"#)?,
                      |_| helpers::month_day(6, 21)?.span_to(&helpers::month_day(7, 15)?, false)
    );
    b.rule_1_terminal("milieu de l'été",
                      b.reg(r#"milieu de (?:cet |l')?(?:été|ete)"#)?,
                      |_| helpers::month_day(7, 15)?.span_to(&helpers::month_day(8, 15)?, false)
    );
    b.rule_1_terminal("fin de l'été",
                      b.reg(r#"fin de (?:cet |l')?(?:été|ete)"#)?,
                      |_| helpers::month_day(8, 15)?.span_to(&helpers::month_day(9, 21)?, false)
    );
    b.rule_1_terminal("début de l'automne",
                      b.reg(r#"début de (?:cet |l')?automne"#)?,
                      |_| helpers::month_day(9, 21)?.span_to(&helpers::month_day(10, 15)?, false)
    );
    b.rule_1_terminal("milieu de l'automne",
                      b.reg(r#"milieu de (?:cet |l')?automne"#)?,
                      |_| helpers::month_day(10, 15)?.span_to(&helpers::month_day(11, 15)?, false)
    );
    b.rule_1_terminal("fin de l'automne",
                      b.reg(r#"fin de (?:cet |l')?automne"#)?,
                      |_| helpers::month_day(11, 15)?.span_to(&helpers::month_day(12, 21)?, false)
    );
    b.rule_1_terminal("début de l'hiver",
                      b.reg(r#"début de (?:cet |l')?hiver"#)?,
                      |_| helpers::month_day(12, 21)?.span_to(&helpers::month_day(1, 15)?, false)
    );
    b.rule_1_terminal("milieu de l'hiver",
                      b.reg(r#"milieu de (?:cet |l')?hiver"#)?,
                      |_| helpers::month_day(1, 15)?.span_to(&helpers::month_day(2, 15)?, false)
    );
    b.rule_1_terminal("fin de l'hiver",
                      b.reg(r#"fin de (?:cet |l')?hiver"#)?,
                      |_| helpers::month_day(2, 15)?.span_to(&helpers::month_day(3, 21)?, false)
    );
    b.rule_1_terminal("début du printemps",
                      b.reg(r#"début (?:du|de ce)? printemps"#)?,
                      |_| helpers::month_day(3, 21)?.span_to(&helpers::month_day(4, 15)?, false)
    );
    b.rule_1_terminal("milieu du printemps",
                      b.reg(r#"milieu (?:du|de ce)? printemps"#)?,
                      |_| helpers::month_day(4, 15)?.span_to(&helpers::month_day(5, 15)?, false)
    );
    b.rule_1_terminal("fin du printemps",
                      b.reg(r#"fin (?:du|de ce)? printemps"#)?,
                      |_| helpers::month_day(5, 15)?.span_to(&helpers::month_day(6, 21)?, false)
    );
    b.rule_1_terminal("fin de cette année",
                      b.reg(r#"fin (?:de (?:l'|cette )|d')?ann[ée]e"#)?,
                      |_| {
                          let current_year = helpers::cycle_nth(Grain::Year, 0)?;
                          let start = current_year.intersect(&helpers::month(10)?)?;
                          let end = current_year.intersect(&helpers::month(12)?)?;
                          start.span_to(&end, true)
                      }
    );
    b.rule_1_terminal("début de cette année",
                      b.reg(r#"d[ée]but (?:de (?:l'|cette )|d')?ann[ée]e"#)?,
                      |_| {
                          let current_year = helpers::cycle_nth(Grain::Year, 0)?;
                          let start = current_year.intersect(&helpers::month(1)?)?;
                          let end = current_year.intersect(&helpers::month(2)?)?;
                          start.span_to(&end, true)
                      }
    );
    b.rule_1_terminal("début de l'été",
                      b.reg(r#"début de (?:cet |l')?(?:été|ete)"#)?,
                      |_| helpers::month_day(6, 21)?.span_to(&helpers::month_day(7, 15)?, false)
    );
    b.rule_1_terminal("milieu de l'été",
                      b.reg(r#"milieu de (?:cet |l')?(?:été|ete)"#)?,
                      |_| helpers::month_day(7, 15)?.span_to(&helpers::month_day(8, 15)?, false)
    );
    b.rule_1_terminal("fin de l'été",
                      b.reg(r#"fin de (?:cet |l')?(?:été|ete)"#)?,
                      |_| helpers::month_day(8, 15)?.span_to(&helpers::month_day(9, 21)?, false)
    );
    b.rule_1_terminal("début de l'automne",
                      b.reg(r#"début de (?:cet |l')?(?:été|ete)"#)?,
                      |_| helpers::month_day(9, 21)?.span_to(&helpers::month_day(10, 15)?, false)
    );
    b.rule_1_terminal("milieu de l'automne",
                      b.reg(r#"milieu de (?:cet |l')?(?:été|ete)"#)?,
                      |_| helpers::month_day(10, 15)?.span_to(&helpers::month_day(11, 15)?, false)
    );
    b.rule_1_terminal("fin de l'automne",
                      b.reg(r#"fin de (?:cet |l')?(?:été|ete)"#)?,
                      |_| helpers::month_day(11, 15)?.span_to(&helpers::month_day(12, 21)?, false)
    );
    b.rule_1_terminal("début de l'hiver",
                      b.reg(r#"début de (?:cet |l')?(?:été|ete)"#)?,
                      |_| helpers::month_day(12, 21)?.span_to(&helpers::month_day(1, 15)?, false)
    );
    b.rule_1_terminal("milieu de l'hiver",
                      b.reg(r#"milieu de (?:cet |l')?(?:été|ete)"#)?,
                      |_| helpers::month_day(1, 15)?.span_to(&helpers::month_day(2, 15)?, false)
    );
    b.rule_1_terminal("fin de l'hiver",
                      b.reg(r#"fin de (?:cet |l')?(?:été|ete)"#)?,
                      |_| helpers::month_day(2, 15)?.span_to(&helpers::month_day(3, 21)?, false)
    );
    b.rule_1_terminal("début du printemps",
                      b.reg(r#"début de (?:cet |l')?(?:été|ete)"#)?,
                      |_| helpers::month_day(3, 21)?.span_to(&helpers::month_day(4, 15)?, false)
    );
    b.rule_1_terminal("milieu du printemps",
                      b.reg(r#"milieu de (?:cet |l')?(?:été|ete)"#)?,
                      |_| helpers::month_day(4, 15)?.span_to(&helpers::month_day(5, 15)?, false)
    );
    b.rule_1_terminal("fin du printemps",
                      b.reg(r#"fin de (?:cet |l')?(?:été|ete)"#)?,
                      |_| helpers::month_day(5, 15)?.span_to(&helpers::month_day(6, 21)?, false)
    );
    b.rule_2("le <datetime>",
             //b.reg(r#"l[ea]"#)?,
             b.reg(r#"l[ea]|en|au|à|pour"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |_, a| Ok(a.value().clone())
    );
    b.rule_4("dd-dd <month>(interval)",
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             b.reg(r#"\-|(?:jusqu')?au"#)?,
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             datetime_check!(form!(Form::Month(_))),
             |a, _, b, month| {
                 let start = month.value().intersect(&helpers::day_of_month(a.group(1).parse()?)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(b.group(1).parse()?)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_4("<datetime>-dd <month>(interval)",
             datetime_check!(),
             b.reg(r#"\-|(?:jusqu')?au"#)?,
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             datetime_check!(form!(Form::Month(_))),
             |datetime, _, text_match, month| {
                 let start = month.value().intersect(datetime.value())?;
                 let end = month.value().intersect(&helpers::day_of_month(text_match.group(1).parse()?)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_5("<datetime>-<day-of-week> dd <month>(interval)",
             datetime_check!(),
             b.reg(r#"\-|(?:jusqu')?au"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             datetime_check!(form!(Form::Month(_))),
             |datetime, _, _, text_match, month| {
                 let start = month.value().intersect(datetime.value())?;
                 let end = month.value().intersect(&helpers::day_of_month(text_match.group(1).parse()?)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_6("<day-of-week> 1er-<day-of-week> dd <month>(interval)",
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"premier|prem\.?|1er|1 er"#)?,
             b.reg(r#"\-|(?:jusqu')?au"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, _, _, _, text_match, month| {
                 let start = month.value().intersect(&helpers::day_of_month(1)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(text_match.group(1).parse()?)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_6("du dd-<day-of-week> dd <month>(interval)",
             b.reg(r#"du"#)?,
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             b.reg(r#"\-|(?:jusqu')?au"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, a, _, _, b, month| {
                 let start = month.value().intersect(&helpers::day_of_month(a.group(1).parse()?)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(b.group(1).parse()?)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_6("du dd-<day-of-week> dd <month>(interval)",
             b.reg(r#"du"#)?,
             datetime_check!(),
             b.reg(r#"\-|(?:jusqu')?au"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, datetime, _, _, text_match, month| {
                 let start = month.value().intersect(datetime.value())?;
                 let end = month.value().intersect(&helpers::day_of_month(text_match.group(1).parse()?)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_4("la nuit <day-of-week> <day-of-week>",
             b.reg(r#"(dans|pendant|durant) la nuit (?:du|de)"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"\-|(?:jusqu')?au"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             |_, start, _, end| {
            let start = start.value().intersect(&helpers::hour(22, false)?)?;
            let end = end.value().intersect(&helpers::hour(6, false)?)?;
            start.span_to(&end, false)
        }
    );
    b.rule_5("entre dd et dd <month>(interval)",
             b.reg(r#"entre(?: le)?"#)?,
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             b.reg(r#"et(?: le)?"#)?,
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, a, _, b, month| {
                 let start = month.value().intersect(&helpers::day_of_month(a.group(1).parse()?)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(b.group(1).parse()?)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_4_terminal("du dd au dd(interval)",
        b.reg(r#"du"#)?,
        b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
        b.reg(r#"(?:jusqu')?au"#)?,
        b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
        |_, a, _, b| {
            let start = helpers::day_of_month(a.group(1).parse()?)?;
            let end = helpers::day_of_month(b.group(1).parse()?)?;
            start.span_to(&end, true)
        }
    );
    b.rule_2("fin <named-month>(interval)",
             b.reg(r#"fin(?: du mois d[e']? ?)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, month| {
                 let start = month.value().intersect(&helpers::day_of_month(25)?)?;
                 let end = helpers::cycle(Grain::Day)?.last_of(month.value())?;
                 start.span_to(&end, true)
             }
    );
    b.rule_2("début <named-month>(interval)",
             b.reg(r#"d[ée]but(?: du mois d[e'] ?)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, month| {
                 let start = month.value().intersect(&helpers::day_of_month(1)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(5)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_2("première quinzaine de <named-month>(interval)",
             b.reg(r#"(?:premi[èe]re|1 ?[èe]re) (?:quinzaine|15 ?aine) d[e']"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, month| {
                 let start = month.value().intersect(&helpers::day_of_month(1)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(14)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_2("deuxième quinzaine de <named-month>(interval)",
             b.reg(r#"(?:deuxi[èe]me|2 ?[èe]me) (?:quinzaine|15 ?aine) d[e']"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, month| {
                 let start = month.value().intersect(&helpers::day_of_month(15)?)?;
                 let end = helpers::cycle(Grain::Day)?.last_of(month.value())?;
                 start.span_to(&end, true)
             }
    );
    b.rule_2("<named-month>",
             b.reg(r#"mi[- ]"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, month| {
                 let start = month.value().intersect(&helpers::day_of_month(10)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(19)?)?;
                 start.span_to(&end, true)
             }
    );

    /* Intervals */
    b.rule_3("<datetime> - <datetime> (interval)",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             b.reg(r#" \- |(?:jusqu')?(?:au|[aà])"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             |a, _, b| a.value().span_to(b.value(), true)
    );
    b.rule_3("<datetime> avant <time-of-day> (interval)",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             b.reg(r#"jusqu'(?:au|[aà])|avant"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |a, _, b| a.value().span_to(b.value(), false)
    );
    b.rule_4("de <datetime> - <datetime> (interval)",
             b.reg(r#"depuis|d[e'u]?"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             b.reg(r#" \- |(?:jusqu')?(?:au|[aà])"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             |_, a, _, b| a.value().span_to(b.value(), true)
    );
    b.rule_4("entre <datetime> et <datetime> (interval)",
             b.reg(r#"entre"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             b.reg(r#"et"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             |_, a, _, b| a.value().span_to(b.value(), true)
    );
    b.rule_4("entre <part-of-day> et <time-of-day> (interval)",
             b.reg(r#"entre"#)?,
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             b.reg(r#"et"#)?,
             datetime_check!(|datetime: &DatetimeValue| form!(Form::TimeOfDay(_))(datetime)),
             |_, a, _, b| a.value().span_to(b.value(), true)
    );
    b.rule_4("entre <time-of-day> et <part-of-day> (interval)",
             b.reg(r#"(?:[aà] partir )?d[eu]"#)?,
             datetime_check!(|datetime: &DatetimeValue| form!(Form::TimeOfDay(_))(datetime)),
             b.reg(r#"(?:jusqu')?(?:à|au)"#)?,
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             |_, a, _, b| a.value().span_to(b.value(), true)
    );
    b.rule_4("entre <part-of-day> et <time-of-day> (interval)",
             b.reg(r#"(?:[aà] partir )?d[eu]"#)?,
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             b.reg(r#"(?:jusqu')?(?:à|au)"#)?,
             datetime_check!(|datetime: &DatetimeValue| form!(Form::TimeOfDay(_))(datetime)),
             |_, a, _, b| a.value().span_to(b.value(), true)
    );
    b.rule_4("entre <time-of-day> et <part-of-day> (interval)",
             b.reg(r#"entre"#)?,
             datetime_check!(|datetime: &DatetimeValue| form!(Form::TimeOfDay(_))(datetime)),
             b.reg(r#"et"#)?,
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             |_, a, _, b| a.value().span_to(b.value(), true)
    );
    // Specific case with years
    b.rule_5("de <datetime> - <datetime> <year> (interval)",
             b.reg(r#"depuis|d[e'u]?"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             b.reg(r#" \- |(?:jusqu')?(?:au|[aà])"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime) && datetime.is_coarse_grain_greater_than(Grain::Year)),
             datetime_check!(form!(Form::Year(_))),
             |_, a, _, b, year| a.value().span_to(b.value(), true)?.intersect(year.value())
    );
    b.rule_5("entre <datetime> et <datetime> <year> (interval)",
             b.reg(r#"entre"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             b.reg(r#"et"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime) && datetime.is_coarse_grain_greater_than(Grain::Year)),
             datetime_check!(form!(Form::Year(_))),
             |_, a, _, b, year| a.value().span_to(b.value(), true)?.intersect(year.value())
    );
    b.rule_3("<time-of-day> - <time-of-day> (interval)",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && form!(Form::TimeOfDay(_))(datetime)),
             b.reg(r#" \- |(?:jusqu')?(?:au|[aà])"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && form!(Form::TimeOfDay(_))(datetime)),
             |a, _, b| a.value().smart_span_to(b.value(), false)
    );
    b.rule_4("de <time-of-day> - <time-of-day> (interval)",
             b.reg(r#"(?:[aà] partir )?d['e]"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:jusqu')?(?:au|[aà])"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, a, _, b| a.value().smart_span_to(b.value(), false)
    );
    b.rule_4("de <part-of-day> - <part-of-day> (interval)",
             b.reg(r#"(?:[aà] partir )?d['eu]"#)?,
             datetime_check!(form!(Form::PartOfDay(_))),
             b.reg(r#"(?:jusqu')?(?:au|[aà])"#)?,
             datetime_check!(form!(Form::PartOfDay(_))),
             |_, a, _, b| a.value().smart_span_to(b.value(), false)
    );
    b.rule_4("de <meal> - <meal> (interval)",
             b.reg(r#"(?:[aà] partir )?d['e]"#)?,
             datetime_check!(form!(Form::Meal)),
             b.reg(r#"(?:jusqu')?(?:au|[aà])"#)?,
             datetime_check!(form!(Form::Meal)),
             |_, a, _, b| a.value().smart_span_to(b.value(), false)
    );
    b.rule_2("de maintenant - <time-of-day> (interval)",
             b.reg(r#"(?:[aà] partir )?de maintenant (?:jusqu')?(?:au|[aà])"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, a| helpers::cycle_nth(Grain::Second, 0)?.smart_span_to(a.value(), false)
    );
    b.rule_3("de <time-of-day> - maintenant (interval)",
             b.reg(r#"(?:[aà] partir )?d['e]"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:jusqu')?(?:au|[aà]) maintenant"#)?,
             |_, a, _| {
                 let now = helpers::cycle_nth(Grain::Second, 0)?;
                 a.value().smart_span_to(&now, false)
             }
    );
    b.rule_4("entre <time-of-day> et <time-of-day> (interval)",
             b.reg(r#"entre"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"et"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, a, _, b| a.value().smart_span_to(b.value(), false)
    );
    b.rule_2("jusqu'à <datetime>",
             b.reg(r#"(?:n[ ']importe quand )?jusqu'(?:au|[aà]|en)?"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |_, datetime| Ok(datetime.value().clone().mark_before_end())
    );
    b.rule_2("avant <datetime>",
             b.reg(r#"(?:n[ ']importe quand )?avant"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |_, datetime| Ok(datetime.value().clone().mark_before_start())
    );
    b.rule_2("avant <part-of-day>",
             b.reg(r#"(?:n[ ']importe quand )?(avant|jusqu'(?:au|[aà]|en))"#)?,
             datetime_check!(form!(Form::PartOfDay(_))),
             |_, datetime| Ok(datetime.value().clone().mark_before_start())
    );
    b.rule_2("après <datetime>",
             b.reg(r#"apr[eè]s"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |_, datetime| Ok(datetime.value().clone().mark_after_end())
    );
    b.rule_2("à partir de <datetime>",
             b.reg(r#"[aà] partir d['eu]"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |_, datetime| Ok(datetime.value().clone().mark_after_start())
    );
    b.rule_2("à partir de <part-of-day>",
             b.reg(r#"[aà] partir d['eu](?:l[ 'a])?"#)?,
             datetime_check!(form!(Form::PartOfDay(_))),
             |_, datetime| Ok(datetime.value().clone().mark_after_start())
    );
    b.rule_2("après <part-of-day>",
             b.reg(r#"après"#)?,
             datetime_check!(form!(Form::PartOfDay(_))),
             |_, datetime| Ok(datetime.value().clone().mark_after_end())
    );
    b.rule_2("après le <day-of-month>",
             b.reg(r#"apr(?:e|è)s le"#)?,
             integer_check_by_range!(1, 31),
             |_, integer| Ok(helpers::day_of_month(integer.value().value as u32)?.mark_after_end())
    );
    b.rule_2("après le <day-of-month>",
             b.reg(r#"[aà] partir d['eu]"#)?,
             integer_check_by_range!(1, 31),
             |_, integer| Ok(helpers::day_of_month(integer.value().value as u32)?.mark_after_start())
    );
    b.rule_2("il y a <duration>",
             b.reg(r#"il y a"#)?,
             duration_check!(),
             |_, duration| duration.value().ago()
    );
    // With "depuis/d'ici", interpretation of duration ambiguous with time-of-day we choose time-of-day
    // I.e. "x heures (y)", but not "x heures et y minutes", "x minutes", etc.
    // FIXME: some time-of-day patterns should be removed, e.g. "x heures y minutes" - they are not
    // proper time-of-day, but they can be duration expressions
    // TODO: check if Grain::Second here breaks DatePeriod - cf. implem. of equivalent in English?
    b.rule_2("depuis <duration>",
             b.reg(r#"depuis|[cç]a fait"#)?,
             duration_check!(),
             |_, duration| {
                 if duration.value().get_coarser_grain() == Grain::Hour {
                     return Err(RuleError::Invalid.into())
                 }
                 duration.value().ago()?
                         .span_to(&helpers::cycle_nth(Grain::Second, 0)?, false)
             });
    b.rule_2("depuis <datetime>",
             b.reg(r#"depuis"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |_, datetime| Ok(datetime.value().the_nth(-1)?.mark_after_start())
    );
    b.rule_2("d'ici <duration>",
             b.reg(r#"d'ici|dans l(?:'|es?)"#)?,
             duration_check!(),
             |_, duration| {
                 let duration_grain = duration.value().get_coarser_grain();
                 // Priority to d'ici <time-of-day>
                 if duration_grain == Grain::Hour &&
                     // FIXME: There must be a better way to do this check!
                     duration.value().period.0.get(Grain::Hour as usize).unwrap_or(&0) <= &23 {
                     return Err(RuleError::Invalid.into())
                 }
                 let grain = if duration_grain.is_date_grain() { Grain::Day } else { Grain::Second };
                 let start = helpers::cycle_nth(grain, 0)?;
                 let end = if grain == Grain::Day { duration.value().in_present_day()? } else { duration.value().in_present()? };
                 start.span_to(&end, true)
             }
    );
    b.rule_2("dans le <duration>",
             b.reg(r#"dans l(?:'|es?)"#)?,
             duration_check!(),
             |_, duration| {
                 let duration_grain = duration.value().get_coarser_grain();
                 let grain = if duration_grain.is_date_grain() { Grain::Day } else { Grain::Second };
                 let start = helpers::cycle_nth(grain, 0)?;
                 let end = if grain == Grain::Day { duration.value().in_present_day()? } else { duration.value().in_present()? };
                 start.span_to(&end, false)
             }
    );
    b.rule_2("d'ici <time-of-day>",
             b.reg(r#"d'ici"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && form!(Form::TimeOfDay(_))(datetime)),
             |_, tod| {
                 // FIXME: This adds one second to the value of now+then
                 let now = helpers::cycle_nth(Grain::Second, 0)?;
                 let then = tod.value().clone().mark_before_start();
                 now.span_to(&then, false)
             }
    );
    b.rule_2("d'ici <date>",
             b.reg(r#"d'ici"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             |_, date| {
                 // FIXME: This adds one second to the value of now+then
                 let today = helpers::cycle_nth(Grain::Day, 0)?;
                 let then = date.value().clone().mark_before_start();
                 today.span_to(&then, false)
             }
    );
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
