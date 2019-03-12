use std::f32;

use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::dimension::Precision::*;
use rustling_ontology_values::helpers;
use rustling_ontology_moment::{Weekday, Grain};


/** Datetime rules
  - 4 specified datetime types:
    - <datetime>
    - <TimePeriod>
    - <Date>
    - <DatePeriod>
  - 1 underspecified datetime type:
    - Any other supported datetime pattern is a DatetimeComplement. Mainly, these are combinations
    of date+time, e.g. "tomorrow at 5pm". Its inherent type is <Datetime>.
  - Datetime is the inclusion of the 4 types + DatetimeComplement.
  - All rules matching one of the 4 types have a Datetime counterpart, i.e. a rule for the same
  match with the type <Datetime> (instead of e.g. <Date>).
  - When the queried entity type is <Datetime>, the <Datetime> version of the match will be selected.
  - DatetimeComplement matches can happen only when the queried entity type is <Datetime>.
*/


/** Time Rules */
pub fn rules_time(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {

    Ok(())

}


/** TimePeriod Rules */
pub fn rules_time_period(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {

    Ok(())

}


/** Date Rules */
pub fn rules_date(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {

    Ok(())

}


/** DatePeriod Rules */
pub fn rules_date_period(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {

    Ok(())

}


/** Rules for Datetime complement */
pub fn rules_datetime_complement(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {

    Ok(())

}


/** Rules repeating the Time, TimePeriod, Date, DatePeriod matches with the Datetime type */
pub fn rules_datetime(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {

    /* DATETIME - COMPLEX RULES */
    // DATETIME#1
    // TODO: split date/time combinations
    b.rule_2("intersect <datetime>",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |a, b| a.value().intersect(b.value())
    );
    // DATETIME#2
    // TODO: split date/time combinations
    b.rule_3("intersect by \"of\", \"from\", \"'s\"",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             b.reg(r#"of|from|for|'s"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |a, _, b| a.value().intersect(b.value())
    );
    // DATETIME#3
    // TODO: split date/time combinations
    b.rule_3("intersect by \",\"",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             b.reg(r#","#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |a, _, b| a.value().intersect(b.value())
    );
    /* END OF DATETIME - COMPLEX RULES */

    /* DATETIME - DATE - PREPOSITION + DATES */
    // DATETIME#4
    // TODO: split date/time combinations
    b.rule_2("on|in <date>",
             b.reg(r#"[oi]n"#)?,
             datetime_check!(),
             |_, a| Ok(a.value().clone().not_latent())
    );
    // DATETIME#5
    // TODO: restrict to valid datetime forms - dates and parts of day
    b.rule_2("during <date>",
             b.reg(r#"during"#)?,
             datetime_check!(),
             |_, a| Ok(a.value().clone().not_latent())
    );
    // DATETIME#6
    // TODO: restrict to valid datetime forms - shouldn't it be durations only?
    b.rule_2("within <date>",
             b.reg(r#"within"#)?,
             datetime_check!(),
             |_, a| Ok(a.value().clone().not_latent())
    );
    // DATETIME#7
    // TODO: split date/time combinations + restrict valid prepositions
    b.rule_2("for <date>",
             b.reg(r#"(?:for|at|on)"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |_, a| Ok(a.value().clone().not_latent())
    );
    // DATETIME#8
    // TODO: output::time
    b.rule_2("for <meal>",
             b.reg(r#"for"#)?,
             datetime_check!(form!(Form::Meal)),
             |_, a| Ok(a.value().clone().not_latent())
    );
    // DATETIME#9
    // TODO: remove this? The semantics is not a datetime with resolution
    // Add on+weekday - merge w/ #7
    b.rule_2("on a <named-day>",
             b.reg(r#"on a"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             |_, a| Ok(a.value().clone())
    );
    // DATETIME#10
    // TODO: output::date-period + add for, during
    b.rule_2("in <named-month>",
             b.reg(r#"in"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, a| Ok(a.value().clone())
    );
    /* END OF DATETIME - DATE - PREPOSITION + DATES */

    /* DATETIME - DATE - STANDALONE SINGLE GRAIN */

    // DATETIME#11
    // TODO: output::date
    b.rule_1_terminal("named-day",
                      b.reg(r#"monday|mon\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Mon)
    );
    // DATETIME#12
    // TODO: output::date
    b.rule_1_terminal("named-day",
                      b.reg(r#"tuesday|tues?\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Tue)
    );
    // DATETIME#13
    // TODO: output::date
    b.rule_1_terminal("named-day",
                      b.reg(r#"wed?nesday|wed\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Wed)
    );
    // DATETIME#14
    // TODO: output::date
    b.rule_1_terminal("named-day",
                      b.reg(r#"thursday|thu(?:rs?)?\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Thu)
    );
    // DATETIME#15
    // TODO: output::date
    b.rule_1_terminal("named-day",
                      b.reg(r#"friday|fri\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Fri)
    );
    // DATETIME#16
    // TODO: output::date
    b.rule_1_terminal("named-day",
                      b.reg(r#"saturday|sat\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Sat)
    );
    // DATETIME#17
    // TODO: output::date
    b.rule_1_terminal("named-day",
                      b.reg(r#"sunday|sun\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Sun)
    );
    // DATETIME#18
    // TODO: output::date
    b.rule_1_terminal("named-month",
                      b.reg(r#"january|jan\.?"#)?,
                      |_| helpers::month(1)
    );
    // DATETIME#19
    // TODO: output::date
    b.rule_1_terminal("named-month",
                      b.reg(r#"february|feb\.?"#)?,
                      |_| helpers::month(2)
    );
    // DATETIME#20
    // TODO: output::date
    b.rule_1_terminal("named-month",
                      b.reg(r#"march|mar\.?"#)?,
                      |_| helpers::month(3)
    );
    // DATETIME#21
    // TODO: output::date
    b.rule_1_terminal("named-month",
                      b.reg(r#"april|apr\.?"#)?,
                      |_| helpers::month(4)
    );
    // DATETIME#22
    // TODO: output::date
    b.rule_1_terminal("named-month",
                      b.reg(r#"may"#)?,
                      |_| helpers::month(5)
    );
    // DATETIME#23
    // TODO: output::date
    b.rule_1_terminal("named-month",
                      b.reg(r#"june|jun\.?"#)?,
                      |_| helpers::month(6)
    );
    // DATETIME#24
    // TODO: output::date
    b.rule_1_terminal("named-month",
                      b.reg(r#"july|jul\.?"#)?,
                      |_| helpers::month(7)
    );
    // DATETIME#25
    // TODO: output::date
    b.rule_1_terminal("named-month",
                      b.reg(r#"august|aug\.?"#)?,
                      |_| helpers::month(8)
    );
    // DATETIME#26
    // TODO: output::date
    b.rule_1_terminal("named-month",
                      b.reg(r#"september|sept?\.?"#)?,
                      |_| helpers::month(9)
    );
    // DATETIME#27
    // TODO: output::date
    b.rule_1_terminal("named-month",
                      b.reg(r#"october|oct\.?"#)?,
                      |_| helpers::month(10)
    );
    // DATETIME#28
    // TODO: output::date
    b.rule_1_terminal("named-month",
                      b.reg(r#"november|nov\.?"#)?,
                      |_| helpers::month(11)
    );
    // DATETIME#29
    // TODO: output::date
    b.rule_1_terminal("named-month",
                      b.reg(r#"december|dec\.?"#)?,
                      |_| helpers::month(12)
    );
    /* END OF DATETIME - DATE - STANDALONE SINGLE GRAIN */

    /* DATETIME - DATE - CELEBRATIONS AND HOLIDAYS */
    // DATETIME#31
    // TODO: output::date - add special days / holidays form/tag
    b.rule_1_terminal("christmas",
                      b.reg(r#"(?:xmas|christmas)(?: day)?"#)?,
                      |_| helpers::month_day(12, 25)
    );
    // DATETIME#32
    // TODO: output::date - add special days / holidays form/tag
    b.rule_1_terminal("christmas eve",
                      b.reg(r#"(?:xmas|christmas)(?: day)?(?:'s)? eve"#)?,
                      |_| helpers::month_day(12, 24)
    );
    // DATETIME#33
    // TODO: output::date - add special days / holidays form/tag
    b.rule_1_terminal("new year's eve",
                      b.reg(r#"new year'?s? eve"#)?,
                      |_| helpers::month_day(12, 31)
    );
    // DATETIME#34
    // TODO: output::date - add special days / holidays form/tag
    b.rule_1_terminal("new year's day",
                      b.reg(r#"new year'?s?(?: day)?"#)?,
                      |_| helpers::month_day(1, 1)
    );
    // DATETIME#35
    // TODO: output::date - add special days / holidays form/tag
    b.rule_1_terminal("valentine's day",
                      b.reg(r#"valentine'?s?(?: day)?"#)?,
                      |_| helpers::month_day(2, 14)
    );
    // DATETIME#36
    // TODO: output::date - add special days / holidays form/tag
    b.rule_1_terminal("MLK Day",
                      b.reg(r#"(?:MLK|Martin Luther King,?)(?: Jr.?| Junior)? day"#)?,
                      |_| {
                          let third_week_january =
                              helpers::cycle_nth_after(Grain::Week, 3, &helpers::month_day(1, 1)?)?;
                          let january = helpers::month(1)?;
                          let monday = helpers::day_of_week(Weekday::Mon)?;
                          january.intersect(&third_week_january)?.intersect(&monday)
                      });
    // DATETIME#37
    // TODO:
    b.rule_1_terminal("Palm sunday",
                      b.reg(r#"(?:palm|passion) sunday"#)?,
                      |_| Ok(helpers::cycle_nth_after(Grain::Day, -7, &helpers::easter()?)?
                          .form(Form::Celebration))
    );
    // DATETIME#38
    // TODO: output::date - add special days / holidays form/tag + combination w/ prep
    b.rule_1_terminal("Holy Thursday",
                      b.reg(r#"(?:holy|maundy) thursday"#)?,
                      |_| Ok(helpers::cycle_nth_after(Grain::Day, -3, &helpers::easter()?)?
                          .form(Form::Celebration))
    );
    // DATETIME#39
    // TODO: output::date - add special days / holidays form/tag + combination w/ prep
    b.rule_1_terminal("Holy Friday",
                      b.reg(r#"good friday"#)?,
                      |_| Ok(helpers::cycle_nth_after(Grain::Day, -2, &helpers::easter()?)?
                          .form(Form::Celebration))
    );
    // DATETIME#40
    // TODO: output::date - add special days / holidays form/tag + combination w/ prep
    b.rule_1_terminal("Holy Saturday",
                      b.reg(r#"(?:holy|black) saturday|easter vigil"#)?,
                      |_| Ok(helpers::cycle_nth_after(Grain::Day, -1, &helpers::easter()?)?
                          .form(Form::Celebration))
    );
    // DATETIME#41
    // TODO: output::date - add special days / holidays form/tag + combination w/ prep
    b.rule_1_terminal("Easter",
                      b.reg(r#"easter sunday"#)?,
                      |_| Ok(helpers::easter()?.form(Form::Celebration))
    );
    // DATETIME#42
    // TODO: output::date - add special days / holidays form/tag + combination w/ prep
    b.rule_1_terminal("Easter Monday",
                      b.reg(r#"easter monday"#)?,
                      |_| Ok(helpers::cycle_nth_after(Grain::Day, 1, &helpers::easter()?)?
                          .form(Form::Celebration))
    );
    // DATETIME#43
    // TODO: output::date - add special days / holidays form/tag + combination w/ prep
    b.rule_1_terminal("Ascension",
                      b.reg(r#"(?:(?:the )?feast of (?:the )?)?ascension(?: holiday|thursday|day)?"#)?,
                      |_| Ok(helpers::cycle_nth_after(Grain::Day, 39, &helpers::easter()?)?
                          .form(Form::Celebration))

    );
    // DATETIME#44
    // TODO: output::date - add special days / holidays form/tag + combination w/ prep
    b.rule_1_terminal("Pentecost",
                      b.reg(r#"(?:(?:the )?(?:feast|day) of )?pentecost"#)?,
                      |_| Ok(helpers::cycle_nth_after(Grain::Day, 49, &helpers::easter()?)?
                          .form(Form::Celebration))
    );
    // DATETIME#45
    // TODO: output::date - add special days / holidays form/tag + combination w/ prep
    b.rule_1_terminal("memorial day",
                      b.reg(r#"memorial day"#)?,
                      |_| {
                          let monday = helpers::day_of_week(Weekday::Mon)?;
                          let may = helpers::month(5)?;
                          monday.last_of(&may)
                      });
    // DATETIME#46
    // TODO: output::date - add special days / holidays form/tag + combination w/ prep
    b.rule_1_terminal("memorial day weekend",
                      b.reg(r#"memorial day week(?:\s|-)?end"#)?,
                      |_| {
                          let monday = helpers::day_of_week(Weekday::Mon)?;
                          let tuesday = helpers::day_of_week(Weekday::Tue)?;
                          let may = helpers::month(5)?;
                          let start = helpers::cycle_nth_after(Grain::Day, -3, &monday.last_of(&may)?)?
                              .intersect(&helpers::hour(18, false)?)?;
                          let end = tuesday.last_of(&may)?
                              .intersect(&helpers::hour(0, false)?)?;
                          start.span_to(&end, false)
                      });
    // DATETIME#47
    // TODO: output::date - add special days / holidays form/tag + combination w/ prep
    b.rule_1_terminal("US independence day",
                      b.reg(r#"(independence|national) day"#)?,
                      |_| helpers::month_day(7, 4)
    );
    // DATETIME#48
    // TODO: output::date - add special days / holidays form/tag + combination w/ prep
    b.rule_1_terminal("labor day",
                      b.reg(r#"labor day"#)?,
                      |_| {
                          helpers::month(9)?.intersect(&helpers::day_of_week(Weekday::Mon)?)
                      }
    );
    // DATETIME#49
    // TODO: output::date - add special days / holidays form/tag + combination w/ prep
    b.rule_1_terminal("flag day",
                      b.reg(r#"flag day"#)?,
                      |_| {
                          helpers::month_day(6, 14)
                      }
    );
    // DATETIME#50
    // TODO: output::date - add special days / holidays form/tag + combination w/ prep
    b.rule_1_terminal("patriot day",
                      b.reg(r#"patriot day"#)?,
                      |_| helpers::month_day(9, 11)
    );
    // DATETIME#51
    // TODO: output::date - add special days / holidays form/tag + combination w/ prep
    b.rule_1_terminal("women's equality day",
                      b.reg(r#"wom[ea]n'?s equality day"#)?,
                      |_| helpers::month_day(8, 26)
    );
    // DATETIME#52
    // TODO: output::date - add special days / holidays form/tag + combination w/ prep
    b.rule_1_terminal("labor day weekend",
                      b.reg(r#"labor day week(?:\s|-)?end"#)?,
                      |_| {
                          let start = helpers::cycle_nth_after(Grain::Day, -3, &helpers::month(9)?.intersect(&helpers::day_of_week(Weekday::Mon)?)?)?
                              .intersect(&helpers::hour(18, false)?)?;
                          let end = helpers::month(9)?.intersect(&helpers::day_of_week(Weekday::Tue)?)?
                              .intersect(&helpers::hour(0, false)?)?;
                          start.span_to(&end, false)
                      }
    );
    // DATETIME#53
    // TODO: output::date - add special days / holidays form/tag + combination w/ prep
    b.rule_1_terminal("Father's Day",
                      b.reg(r#"father'?s?'? day"#)?,
                      |_| {
                          let sundays_of_june = helpers::month(6)?.intersect(&helpers::day_of_week(Weekday::Sun)?)?;
                          let second_week_of_june = helpers::cycle_nth_after(Grain::Week, 2, &helpers::month_day(6, 1)?)?;
                          sundays_of_june.intersect(&second_week_of_june) // third sunday of June
                      }
    );
    // DATETIME#54
    // TODO: output::date - add special days / holidays form/tag + combination w/ prep
    b.rule_1_terminal("Mother's Day",
                      b.reg(r#"mother'?s? day"#)?,
                      |_| {
                          let sundays_of_may = helpers::month(5)?.intersect(&helpers::day_of_week(Weekday::Sun)?)?;
                          let first_week_of_may = helpers::cycle_nth_after(Grain::Week, 1, &helpers::month_day(5, 1)?)?;
                          sundays_of_may.intersect(&first_week_of_may) // second sunday of May
                      }
    );
    // DATETIME#55
    // TODO: output::date - add special days / holidays form/tag + combination w/ prep
    b.rule_1_terminal("halloween day",
                      b.reg(r#"hall?owe?en(?: day)?"#)?,
                      |_| {
                          helpers::month_day(10, 31)
                      }
    );
    // DATETIME#56
    // TODO: output::date - add special days / holidays form/tag + combination w/ prep
    b.rule_1_terminal("thanksgiving day",
                      b.reg(r#"thanks?giving(?: day)?"#)?,
                      |_| {
                          let thursday_november = helpers::month(11)?.intersect(&helpers::day_of_week(Weekday::Thu)?)?;
                          let fourth_week_of_november = helpers::cycle_nth_after(Grain::Week, 4, &helpers::month_day(11, 1)?)?;
                          thursday_november.intersect(&fourth_week_of_november) // fourth thursday of november
                      }
    );
    // DATETIME#57
    // TODO: output::date - add special days / holidays form/tag + combination w/ prep
    b.rule_1_terminal("black friday",
                      b.reg(r#"black frid?day"#)?,
                      |_| {
                          let thursday_november = helpers::month(11)?.intersect(&helpers::day_of_week(Weekday::Fri)?)?;
                          let fourth_week_of_november = helpers::cycle_nth_after(Grain::Week, 4, &helpers::month_day(11, 1)?)?;
                          thursday_november.intersect(&fourth_week_of_november) // fourth friday of november
                      }
    );
    // DATETIME#58
    // TODO: output::date - add special days / holidays form/tag + combination w/ prep
    b.rule_2("absorption of , after named day",
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#","#)?,
             |a, _| Ok(a.value().clone())
    );
    /* END OF DATETIME - DATE - CELEBRATIONS AND HOLIDAYS */

    /* DATETIME - DATE - DEICTICS */

    // DATETIME#59
    // TODO: output::date
    b.rule_1_terminal("now",
                      b.reg(r#"(?:just|right)? ?now|immediately|at this very moment|at the present time"#)?,
                      |_| helpers::cycle_nth(Grain::Second, 0)
    );
    // DATETIME#60
    // TODO: output::date
    b.rule_1_terminal("today",
                      b.reg(r#"todays?|(?:at this time)"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 0)
    );
    // DATETIME#61
    // TODO: output::date
    b.rule_1_terminal("tomorrow",
                      b.reg(r#"(?:tmrw?|tomm?or?rows?)"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 1)
    );
    // DATETIME#62
    // TODO: output::date
    b.rule_1_terminal("yesterday",
                      b.reg(r#"yesterdays?"#)?,
                      |_| helpers::cycle_nth(Grain::Day, -1)
    );
    // DATETIME#63
    // TODO: check support and expected resolution + add to dates compatible with prep=by, not standalone
    b.rule_1_terminal("end of week",
                      b.reg(r#"(?:the )?end of (?:the )?week"#)?,
                      |_| helpers::day_of_week(Weekday::Thu)
                          ?.span_to(&helpers::day_of_week(Weekday::Sun)?, false)
    );
    // DATETIME#64
    // TODO: see #63 for support/merge
    b.rule_1_terminal("by the end of week",
                      b.reg(r#"by (?:the )?end of (?:the )?week"#)?,
                      |_| helpers::cycle_nth(Grain::Second, 0)?
                          .span_to(&helpers::day_of_week(Weekday::Sun)?, true)
    );
    // DATETIME#65
    // TODO: check support and expected resolution + add to dates compatible with prep=by, not standalone
    b.rule_1_terminal("EOM|End of month",
                      b.reg(r#"(?:the )?(?:eom|end of (?:the )?month)"#)?,
                      |_| {
                          let month = helpers::cycle_nth(Grain::Month, 1)?;
                          Ok(helpers::cycle_nth_after(Grain::Day, -10, &month)?
                              .span_to(&month, false)?
                              .latent()
                              .form(Form::PartOfMonth))
                      }
    );
    // DATETIME#66
    // TODO: see #65 for support/merge
    b.rule_1_terminal("by the end of month",
                      b.reg(r#"by (?:the )?(?:eom|end of (?:the )?month)"#)?,
                      |_| helpers::cycle_nth(Grain::Second, 0)?
                          .span_to(&helpers::cycle_nth(Grain::Month, 0)?, true)
    );
    // DATETIME#67
    // TODO: check support and expected resolution + add to dates compatible with prep=by, not standalone
    b.rule_1_terminal("EOY|End of year",
                      b.reg(r#"(?:the )?(?:eoy|end of (?:the )?year)"#)?,
                      |_| {
                          let current_year = helpers::cycle_nth(Grain::Year, 0)?;
                          let start = current_year.intersect(&helpers::month(10)?)?;
                          let end = current_year.intersect(&helpers::month(12)?)?;
                          start.span_to(&end, true)
                      }
    );
    // DATETIME#68
    // TODO: see #67 for support/merge
    b.rule_1_terminal("by the end of year",
                      b.reg(r#"by (?:the )?(?:eoy|end of (?:the )?year)"#)?,
                      |_| {
                          let current_year = helpers::cycle_nth(Grain::Year, 0)?;
                          let end = current_year.intersect(&helpers::month(12)?)?;
                          helpers::cycle_nth(Grain::Second, 0)?
                              .span_to(&end, true)
                      }
    );
    // DATETIME#69
    // TODO: output::date + add celebrations
    b.rule_2("this|next <day-of-week>",
             b.reg(r#"this|(?:the )?next"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             |_, a| {
                 a.value().the_nth_not_immediate(0)
             }
    );
    // DATETIME#70
    // TODO: restrict to date forms and/or rework this completely
    b.rule_2("this <datetime>",
             b.reg(r#"the|this|current|coming"#)?,
             datetime_check!(),
             |_, a| {
                 a.value().the_nth(0)
             }
    );
    // DATETIME#71
    // TODO: same as #70
    b.rule_2("next <datetime>",
             b.reg(r#"(?:the |this )?next"#)?,
             datetime_check!(),
             |_, a| {
                 a.value().the_nth(0)
             }
    );
    // DATETIME#72
    // TODO: same as 71
    b.rule_2("last <datetime>",
             b.reg(r#"(?:this past|(?:the |this )?last)"#)?,
             datetime_check!(),
             |_, a| {
                 a.value().the_nth(-1)
             }
    );
    // DATETIME#73
    // TODO: restrict to week day forms + add other valid variants
    b.rule_2("<datetime> after next",
             datetime_check!(),
             b.reg(r#"after next"#)?,
             |a, _| {
                 a.value().the_nth_not_immediate(1)
             }
    );
    // DATETIME#74
    // TODO: same as #73 for past days
    b.rule_2("<datetime> before last",
             datetime_check!(),
             b.reg(r#"before last"#)?,
             |a, _| {
                 a.value().the_nth(-2)
             }
    );
    /* END OF DATETIME - DATE - DEICTICS */

    /* DATETIME - NTH CYCLE & CO. - NOT SUPPORTED - REMOVE */

    // DATETIME#30
    // TODO: [rm] no support for nth cycle - or is that supposed to be a holiday?
    b.rule_2("nth sunday of advent",
             ordinal_check!(),
             b.reg(r#"sunday of advent"#)?,
             |ordinal, _| {
                 helpers::day_of_week(Weekday::Sun)?.the_nth_after(-(4 - ordinal.value().value) - 1, &helpers::month_day(12, 25)?)
             }
    );
    // DATETIME#75
    // TODO: [rm] no support for nth cycle
    b.rule_4("last <day-of-week> of <datetime>",
             b.reg(r#"(?:the )?last"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"of"#)?,
             datetime_check!(),
             |_, a, _, b| {
                 a.value().last_of(b.value())
             }
    );
    // DATETIME#76
    // TODO: [rm] no support for nth cycle
    b.rule_4("last <cycle> of <datetime>",
             b.reg(r#"(?:the )?last"#)?,
             cycle_check!(),
             b.reg(r#"of|in"#)?,
             datetime_check!(),
             |_, cycle, _, datetime| {
                 cycle.value().last_of(datetime.value())
             }
    );
    // DATETIME#77
    // TODO: [rm] no support for nth cycle
    b.rule_4("nth <datetime> of <datetime>",
             ordinal_check!(), // the first
             datetime_check!(), // Thursday
             b.reg(r#"of|in"#)?, // of
             datetime_check!(), // march
             |ordinal, a, _, b| {
                 b.value().intersect(a.value())?.the_nth(ordinal.value().value - 1)
             }
    );
    // DATETIME#78
    // TODO: [rm] no support for nth cycle
    b.rule_5("nth <datetime> of <datetime>",
             b.reg(r#"the"#)?,
             ordinal_check!(),
             datetime_check!(),
             b.reg(r#"of|in"#)?,
             datetime_check!(),
             |_, ordinal, a, _, b| {
                 b.value().intersect(a.value())?.the_nth(ordinal.value().value - 1)
             }
    );
    // DATETIME#79
    // TODO: [rm] no support for nth cycle
    b.rule_4("nth <datetime> after <datetime>",
             ordinal_check!(),
             datetime_check!(),
             b.reg(r#"after"#)?,
             datetime_check!(),
             |ordinal, a, _, b| {
                 a.value().the_nth_after(ordinal.value().value - 1, b.value())
             }
    );
    // DATETIME#80
    // TODO: [rm] no support for nth cycle
    b.rule_5("nth <datetime> after <datetime>",
             b.reg(r#"the"#)?,
             ordinal_check!(),
             datetime_check!(),
             b.reg(r#"after"#)?,
             datetime_check!(),
             |_, ordinal, a, _, b| {
                 a.value().the_nth_after(ordinal.value().value - 1, b.value())
             }
    );
    /* END OF DATETIME - NTH CYCLE & CO. - NOT SUPPORTED - REMOVE */

    /* DATETIME - DATE - YEAR */

    // DATETIME#81
    // TODO: output::date-period
    b.rule_2("the year integer",
             b.reg(r#"(?:the )?year"#)?,
             integer_check_by_range!(1000, 2100),
             |_, integer| {
                 helpers::year(integer.value().value as i32)
             }
    );
    // DATETIME#82
    // TODO: output::date-period
    b.rule_3("the year composed",
             b.reg(r#"(?:the )?year"#)?,
             integer_check_by_range!(19, 21),
             integer_check_by_range!(10, 99),
             |_, a, b| {
                 let y = a.value().value * 100 + b.value().value;
                 Ok(helpers::year(y as i32)?.latent())
             }
    );
    // DATETIME#83
    // TODO: output::date-period - check need for latency and contextual resolution
    b.rule_1("year",
             integer_check_by_range!(1000, 2100),
             |integer| {
                 helpers::year(integer.value().value as i32)
             }
    );
    // DATETIME#84
    // TODO: same as #83
    b.rule_1("year short",
             integer_check_by_range!(01, 99),
             |integer| {
                 Ok(helpers::year(integer.value().value as i32)?.latent())
             }
    );
    // DATETIME#85
    // TODO: same as #83
    b.rule_2("year composed",
             integer_check_by_range!(19, 21),
             integer_check_by_range!(10, 99),
             |a, b| {
                 let y = a.value().value * 100 + b.value().value;
                 Ok(helpers::year(y as i32)?.latent())
             }
    );
    // DATETIME#86
    // TODO: same as #83
    b.rule_1("year (latent)",
             integer_check_by_range!(-1000, 999),
             |integer| {
                 Ok(helpers::year(integer.value().value as i32)?.latent())
             }
    );
    // DATETIME#87
    // TODO: same as #83
    b.rule_1("year (latent)",
             integer_check_by_range!(2101, 2200),
             |integer| {
                 Ok(helpers::year(integer.value().value as i32)?.latent())
             }
    );
    /* END OF DATETIME - DATE - YEAR */

    /* DATETIME - DATE - DATES */
    // TODO: list supported variants for dates
    /* Supported:
    -

    */

    // DATETIME#88
    // TODO: output::date
    b.rule_2("the <day-of-month> (ordinal)",
             b.reg(r#"the"#)?,
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
             |_, ordinal| {
                 Ok(helpers::day_of_month(ordinal.value().value as u32)?.latent())
             }
    );
    // DATETIME#89
    // TODO: output::date
    b.rule_1("<day-of-month> (ordinal)",
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
             |ordinal| {
                 Ok(helpers::day_of_month(ordinal.value().value as u32)?.latent())
             }
    );
    // DATETIME#90
    // TODO: check - incorrect?
    b.rule_2("the <day-of-month> (non ordinal)",
             b.reg(r#"the"#)?,
             integer_check_by_range!(1, 31),
             |_, integer| {
                 Ok(helpers::day_of_month(integer.value().value as u32)?.latent())
             }
    );
    // DATETIME#91
    // TODO: output::date
    b.rule_2("<named-day> <day-of-month> (ordinal)",
             datetime_check!(form!(Form::DayOfWeek{..})),
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
             |a, ordinal| {
                 a.value().intersect(&helpers::day_of_month(ordinal.value().value as u32)?)
             }
    );
    // DATETIME#92
    // TODO: output::date
    b.rule_2("<named-day> <month-day>",
             datetime_check!(form!(Form::DayOfWeek{..})),
             datetime_check!(form!(Form::MonthDay(_))),
             |dow, month_day| {
                 month_day.value().intersect(&dow.value())
             }
    );
    // DATETIME#93
    // TODO: output::date
    b.rule_2("<month-day> <year>",
             datetime_check!(form!(Form::MonthDay(_))),
             datetime_check!(form!(Form::Year(_))),
             |month_day, year| {
                 year.value().intersect(&month_day.value())
             }
    );
    // DATETIME#94
    // TODO: output::date
    b.rule_2("<named-month> <day-of-month> (ordinal)", // march 12th
             datetime_check!(form!(Form::Month{..})),
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
             |month, ordinal| {
                 let m = month.value().form_month()?;
                 let form = Form::MonthDay(Some(MonthDayForm { month: m,  day_of_month: ordinal.value().value as u32}));
                 Ok(month.value().intersect(&helpers::day_of_month(ordinal.value().value as u32)?)?
                     .form(form))

             }
    );
    // DATETIME#95
    // TODO: output::date
    b.rule_2("<named-month> <day-of-month> (non ordinal)",
             datetime_check!(form!(Form::Month(_))),
             integer_check_by_range!(1, 31),
             |month, integer| {
                 let m = month.value().form_month()?;
                 let form = Form::MonthDay(Some(MonthDayForm { month: m,  day_of_month: integer.value().value as u32}));
                 Ok(month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)?.form(form))
             }
    );
    // DATETIME#96
    // TODO: output::date
    b.rule_3("<named-month> the <day-of-month> (non ordinal)",
             datetime_check!(form!(Form::Month(_))),
             b.reg(r#"the"#)?,
             integer_check_by_range!(1, 31),
             |month, _, integer| {
                 let m = month.value().form_month()?;
                 let form = Form::MonthDay(Some(MonthDayForm { month: m,  day_of_month: integer.value().value as u32}));
                 Ok(month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)?.form(form))
             }
    );
    // DATETIME#97
    // TODO: output::date
    b.rule_3("<day-of-month> (ordinal) of <named-month>",
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
             b.reg(r#"of|in"#)?,
             datetime_check!(form!(Form::Month(_))),
             |ordinal, _, month| {
                 let m = month.value().form_month()?;
                 let form = Form::MonthDay(Some(MonthDayForm { month: m,  day_of_month: ordinal.value().value as u32}));
                 Ok(month.value().intersect(&helpers::day_of_month(ordinal.value().value as u32)?)?.form(form))
             }
    );
    // DATETIME#98
    // TODO: output::date
    b.rule_3("<day-of-month> (non ordinal) of <named-month>",
             integer_check_by_range!(1, 31),
             b.reg(r#"of|in"#)?,
             datetime_check!(form!(Form::Month(_))),
             |integer, _, month| {
                 let m = month.value().form_month()?;
                 let form = Form::MonthDay(Some(MonthDayForm { month: m,  day_of_month: integer.value().value as u32}));
                 Ok(month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)?.form(form))
             }
    );
    // DATETIME#99
    // TODO: output::date
    b.rule_2("<day-of-month> (non ordinal) <named-month>",
             integer_check_by_range!(1, 31),
             datetime_check!(form!(Form::Month(_))),
             |integer, month| {
                 let m = month.value().form_month()?;
                 let form = Form::MonthDay(Some(MonthDayForm { month: m,  day_of_month: integer.value().value as u32}));
                 Ok(month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)?.form(form))
             }
    );
    // DATETIME#100
    // TODO: output::date
    b.rule_2("<day-of-month>(ordinal) <named-month>",
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
             datetime_check!(form!(Form::Month(_))),
             |ordinal, month| {
                 let m = month.value().form_month()?;
                 let form = Form::MonthDay(Some(MonthDayForm { month: m,  day_of_month: ordinal.value().value as u32}));
                 Ok(month.value().intersect(&helpers::day_of_month(ordinal.value().value as u32)?)?.form(form))
             }
    );
    // DATETIME#101
    // TODO: output::date - check if should be supported (supported in Duckling)
    b.rule_2("the ides of <named-month>",
             b.reg(r#"the ides? of"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, a| {
                 let day = match a.value().form_month()? {
                     3 | 5 | 7 | 10 => 15,
                     _ => 13,
                 };
                 a.value().intersect(&helpers::day_of_month(day)?)
             }
    );
    /* END OF DATETIME - DATE - DATES */

    /* DATETIME - TIME - TIME OF DAY */

    // DATETIME#102
    // TODO: output::time
    b.rule_1("time-of-day (latent) (1 to 23)",
             integer_check_by_range!(1, 23),
             |integer| {
                 Ok(helpers::hour(integer.value().value as u32, integer.value().value <= 12)?.latent())
             }
    );
    // DATETIME#103
    // TODO: output::time
    b.rule_1("time-of-day (latent) (0)",
             integer_check_by_range!(0, 0),
             |_| Ok(helpers::hour(0, false)?.latent())
    );
    // DATETIME#104
    // TODO: output::time
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
    // DATETIME#105
    // TODO: output::time
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
    // DATETIME#106
    // TODO: output::time
    b.rule_2("at <time-of-day>",
             b.reg(r#"at|@"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, a| Ok(a.value().clone().not_latent())
    );
    // DATETIME#107
    // TODO: output::time
    b.rule_2("<time-of-day> o'clock",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"o.?clock"#)?,
             |a, _| Ok(a.value().clone().not_latent())
    );
    // DATETIME#108
    // TODO: output::time
    b.rule_3("at <time-of-day> hours",
             b.reg(r#"at"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"hours"#)?,
             |_, a, _| Ok(a.value().clone().not_latent())
    );
    // DATETIME#111
    // TODO: check - invalid? military + am/pm
    b.rule_2_terminal("hhmm (military) am|pm",
                      b.reg(r#"((?:1[012]|0?\d))([0-5]\d)"#)?,
                      b.reg(r#"([ap])\.?m?\.?"#)?,
                      |a, b| {
                          let day_period = if b.group(1) == "a" {
                              helpers::hour(0, false)?.span_to(&helpers::hour(12, false)?, false)?
                          } else {
                              helpers::hour(12, false)?.span_to(&helpers::hour(0, false)?, false)?
                          };
                          let anchor = helpers::hour_minute(
                              a.group(1).parse()?,
                              a.group(2).parse()?,
                              true)?;
                          let form = anchor.form.clone();
                          Ok(anchor.intersect(&day_period)?.form(form))
                      }
    );
    // DATETIME#112
    // TODO: output::time
    b.rule_2("<time-of-day> am|pm",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:in the )?([ap])(?:\s|\.)?m?\.?"#)?,
             |a, text_match| {
                 let day_period = if text_match.group(1) == "a" {
                     helpers::hour(0, false)?.span_to(&helpers::hour(12, false)?, false)?
                 } else {
                     helpers::hour(12, false)?.span_to(&helpers::hour(0, false)?, false)?
                 };
                 Ok(a.value().intersect(&day_period)?.form(a.value().form.clone()))
             }
    );
    // DATETIME#113
    // TODO: output::time
    b.rule_2("<time-of-day> in the morning|afternoon",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"in the (morning|afternoon|evening)"#)?,
             |a, text_match| {
                 let day_period = if text_match.group(1) == "morning" {
                     helpers::hour(0, false)?.span_to(&helpers::hour(12, false)?, false)?
                 } else {
                     helpers::hour(12, false)?.span_to(&helpers::hour(0, false)?, false)?
                 };
                 Ok(a.value().intersect(&day_period)?.form(a.value().form.clone()))
             }
    );
    // DATETIME#114
    // TODO: output::time
    b.rule_1_terminal("noon",
                      b.reg(r#"noon|midday"#)?,
                      |_| helpers::hour(12, false)
    );
    // DATETIME#115
    // TODO: output::time + check if keep EOD as midnight
    b.rule_1_terminal("midnight|EOD|end of day",
                      b.reg(r#"midni(?:ght|te)|(?:the )?(?:eod|end of (?:the )?day)"#)?,
                      |_| helpers::hour(0, false)
    );
    // DATETIME#116
    // TODO: output::time
    b.rule_1_terminal("quarter (relative minutes)",
                      b.reg(r#"(?:a|one)? ?quarter"#)?,
                      |_| Ok(RelativeMinuteValue(15))
    );
    // DATETIME#117
    // TODO: output::time
    b.rule_1_terminal("half (relative minutes)",
                      b.reg(r#"half"#)?,
                      |_| Ok(RelativeMinuteValue(30))
    );
    // DATETIME#118
    // TODO: output::time
    b.rule_1("number (as relative minutes)",
             integer_check_by_range!(1, 59),
             |a| Ok(RelativeMinuteValue(a.value().value as i32))
    );
    // DATETIME#119
    // TODO: output::time
    b.rule_2("number <minutes> (as relative minutes)",
             integer_check_by_range!(1, 59),
             b.reg(r#"minutes?"#)?,
             |a, _| Ok(RelativeMinuteValue(a.value().value as i32))
    );
    // DATETIME#120
    // TODO: output::time
    b.rule_2("<hour-of-day> <integer> (as relative minutes)",
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour {.. }))),
             relative_minute_check!(),
             |datetime, relative_minute| Ok(helpers::hour_relative_minute(
                 datetime.value().form_time_of_day()?.full_hour(),
                 relative_minute.value().0,
                 true)?
                 .precision(datetime.value().precision))
    );
    // DATETIME#121
    // TODO: output::time
    b.rule_5("at <hour-of-day> hours <integer> minutes",
             b.reg(r#"at"#)?,
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour {.. }))),
             b.reg(r#"hours?(?: and)?"#)?,
             relative_minute_check!(),
             b.reg(r#"minutes?"#)?,
             |_, datetime, _, relative_minute, _| Ok(helpers::hour_relative_minute(
                 datetime.value().form_time_of_day()?.full_hour(),
                 relative_minute.value().0,
                 true)?
                 .precision(datetime.value().precision))
    );
    // DATETIME#122
    // TODO: output::time
    b.rule_3("relative minutes to|till|before <integer> (hour-of-day)",
             relative_minute_check!(),
             b.reg(r#"to|till|before|of"#)?,
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour {.. }))),
             |relative_minute, _, datetime| Ok(helpers::hour_relative_minute(
                 datetime.value().form_time_of_day()?.full_hour(),
                 -1 * relative_minute.value().0,
                 true)?
                 .precision(datetime.value().precision))
    );
    // DATETIME#123
    // TODO: output::time
    b.rule_3("relative minutes after|past <integer> (hour-of-day)",
             relative_minute_check!(),
             b.reg(r#"after|past"#)?,
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour {.. }))),
             |relative_minute, _, datetime| Ok(helpers::hour_relative_minute(
                 datetime.value().form_time_of_day()?.full_hour(),
                 relative_minute.value().0,
                 true)?.precision(datetime.value().precision))
    );
    // DATETIME#124
    // TODO: output::time
    b.rule_2("half <integer> (UK style hour-of-day)",
             b.reg(r#"half"#)?,
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour {.. }))),
             |_, a| Ok(helpers::hour_relative_minute(
                 a.value().form_time_of_day()?.full_hour(),
                 30,
                 true)?.precision(a.value().precision))
    );
    /* END OF DATETIME - TIME - TIME OF DAY */

    /* DATETIME - TIME - TIME OF DAY - WRITTEN FORMS */
    // DATETIME#109
    // TODO: output::time - written form only
    b.rule_1_terminal("hh:mm",
                      b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:.]([0-5]\d)"#)?,
                      |text_match| helpers::hour_minute(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?,
                          true)
    );
    // DATETIME#110
    // TODO: output::time - written form only
    b.rule_1_terminal("hh:mm:ss",
                      b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:.]([0-5]\d)[:.]([0-5]\d)"#)?,
                      |text_match| helpers::hour_minute_second(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(3).parse()?,
                          true)
    );
    /* END OF DATETIME - TIME - TIME OF DAY - WRITTEN FORMS */

    /* DATETIME - DATE - DATES - WRITTEN FORMS */
    // DATETIME#125
    // TODO: output::date - written form only
    b.rule_1_terminal("yyyy-mm-dd - ISO",
                      b.reg(r#"(\d{4})[-/](0?[1-9]|1[0-2])[-/](3[01]|[12]\d|0?[1-9])"#)?,
                      |text_match| helpers::year_month_day(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(3).parse()?)
    );

    /* Notice for accepted forms and resolution:
    Non ambiguous month-day combinations: 13<=d<=31 && 01<=m<=12
    Ambiguous month-day combinations: 01<=d<=12 && 01<=m<=12
    regexes:
    month and ambiguous day: (0?[1-9]|1[0-2])
    non ambiguous day: (1[3-9]|2\d|3[01])
    */
    // DATETIME#126
    // TODO: output::date - written form only
    b.rule_1_terminal("dd/mm/yy or dd/mm/yyyy - Non ambiguous cases - Non US standard",
                      b.reg(r#"(1[3-9]|2\d|3[01])[-/\.](0?[1-9]|1[0-2])[-/\.](\d{2,4})"#)?,
                      |text_match| helpers::year_month_day(
                          text_match.group(3).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(1).parse()?)
    );
    // DATETIME#127
    // TODO: output::date - written form only
    b.rule_1_terminal("mm/dd/yy or mm/dd/yyyy - Non ambiguous cases - US standard",
                      b.reg(r#"(0?[1-9]|1[0-2])[-/\.](1[3-9]|2\d|3[01])[-/\.](\d{2,4})"#)?,
                      |text_match| helpers::year_month_day(
                          text_match.group(3).parse()?,
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?)
    );
    // DATETIME#128
    // TODO: output::date - written form only
    b.rule_1_terminal("mm/dd/yy or mm/dd/yyyy - Ambiguous cases - interpret as US standard",
                      b.reg(r#"(0?[1-9]|1[0-2])[-/\.](0?[1-9]|1[0-2])[-/\.](\d{2,4})"#)?,
                      |text_match| helpers::year_month_day(
                          text_match.group(3).parse()?,
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?)
    );
    // DATETIME#129
    // TODO: output::date - written form only
    b.rule_1_terminal("dd/mm - Non ambiguous cases - Non US standard",
                      b.reg(r#"(1[3-9]|2\d|3[01])[/\.](0?[1-9]|1[0-2])"#)?,
                      |text_match| helpers::month_day(
                          text_match.group(2).parse()?,
                          text_match.group(1).parse()?)
    );
    // DATETIME#130
    // TODO: output::date - written form only
    b.rule_1_terminal("mm/dd - Non ambiguous cases - US standard",
                      b.reg(r#"(0?[1-9]|1[0-2])[/\.](3[01]|2\d|1[3-9])"#)?,
                      |text_match| helpers::month_day(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?)
    );
    // DATETIME#131
    // TODO: output::date - written form only
    b.rule_1_terminal("mm/dd - Ambiguous cases - interpret as US standard",
                      b.reg(r#"(0?[1-9]|1[0-2])[/\.](0?[1-9]|1[0-2])"#)?,
                      |text_match| helpers::month_day(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?)
    );
    /* END OF DATETIME - DATE - DATES - WRITTEN FORMS */

    /* DATETIME - TIME - PARTS OF DAY */
    // DATETIME#132
    // TODO: output::time
    b.rule_1_terminal("morning",
                      b.reg(r#"morning"#)?,
                      |_| {
                          Ok(helpers::hour(4, false)?
                              .span_to(&helpers::hour(12, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Morning)))
                      }
    );
    // DATETIME#133
    // TODO: output::time
    b.rule_1_terminal("breakfast",
                      b.reg(r#"breakfast"#)?,
                      |_| Ok(helpers::hour(5, false)?
                          .span_to(&helpers::hour(10, false)?, false)?
                          .latent()
                          .form(Form::Meal))
    );
    // DATETIME#134
    // TODO: output::time - check support
    b.rule_1_terminal("early morning",
                      b.reg(r#"early (?:(?:in|hours of) the )?morning"#)?,
                      |_| {
                          Ok(helpers::hour(4, false)?
                              .span_to(&helpers::hour(9, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Morning)))
                      }
    );
    // DATETIME#135
    // TODO: output::time - check support
    b.rule_1_terminal("before work",
                      b.reg(r#"before work"#)?,
                      |_| {
                          let period = helpers::hour(7, false)?
                              .span_to(&helpers::hour(10, false)?, false)?;
                          Ok(helpers::cycle_nth(Grain::Day, 0)?.intersect(&period)?.form(Form::PartOfDay(PartOfDayForm::Morning)))
                      }
    );
    // DATETIME#136
    // TODO: output::time - check support
    b.rule_1_terminal("work",
                      b.reg(r#"during work(?: time)?"#)?,
                      |_| {
                          let period = helpers::hour(9, false)?
                              .span_to(&helpers::hour(19, false)?, false)?;
                          Ok(helpers::cycle_nth(Grain::Day, 0)?.intersect(&period)?.form(Form::PartOfDay(PartOfDayForm::None)))
                      }
    );
    // DATETIME#137
    // TODO: output::time
    b.rule_1_terminal("afternoon",
                      b.reg(r#"after ?noo?n"#)?,
                      |_| {
                          Ok(helpers::hour(12, false)?
                              .span_to(&helpers::hour(19, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
                      }
    );
    // DATETIME#138
    // TODO: output::time
    b.rule_1_terminal("evening",
                      b.reg(r#"evening"#)?,
                      |_| {
                          Ok(helpers::hour(18, false)?
                              .span_to(&helpers::hour(0, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );
    // DATETIME#139
    // TODO: output::time
    b.rule_1_terminal("night",
                      b.reg(r#"night"#)?,
                      |_| {
                          Ok(helpers::hour(00, false)?
                              .span_to(&helpers::hour(5, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );
    // DATETIME#140
    // TODO: output::time - check support / inclusion in other rules
    b.rule_1_terminal("brunch",
                      b.reg(r#"brunch"#)?,
                      |_| Ok(helpers::hour(10, false)?
                          .span_to(&helpers::hour(15, false)?, false)?
                          .latent()
                          .form(Form::Meal))
    );
    // DATETIME#141
    // TODO: output::time - check support / inclusion in other rules
    b.rule_1_terminal("lunch",
                      b.reg(r#"lunch"#)?,
                      |_| {
                          Ok(helpers::hour(12, false)?
                              .span_to(&helpers::hour(14, false)?, false)?
                              .latent()
                              .form(Form::Meal))
                      }
    );
    // DATETIME#142
    // TODO: output::time - check support / inclusion in other rules
    b.rule_1_terminal("dinner",
                      b.reg(r#"dinner|supper"#)?,
                      |_| Ok(helpers::hour(18, false)?
                          .span_to(&helpers::hour(23, false)?, false)?
                          .latent()
                          .form(Form::Meal))
    );
    // DATETIME#143
    // TODO: output::time - check support / inclusion in other rules
    b.rule_1_terminal("tea",
                      b.reg(r#"(?:at )?tea time"#)?,
                      |_| Ok(helpers::hour(15, false)?
                          .span_to(&helpers::hour(17, false)?, false)?
                          .form(Form::Meal))
    );
    // DATETIME#144
    // TODO: output::time - check support / inclusion in other rules
    b.rule_2("at <meal>",
             b.reg("at|for|during")?,
             datetime_check!(form!(Form::Meal)),
             |_, a| Ok(a.value().clone().not_latent())
    );
    // DATETIME#145
    // TODO: output::time - check support / inclusion in other rules
    b.rule_2("around <meal>",
             b.reg("(?:about|around|approximately)")?,
             datetime_check!(form!(Form::Meal)),
             |_, a| Ok(a.value().clone().not_latent().precision(Approximate))
    );
    // DATETIME#146
    // TODO: output::time - check support / inclusion in other rules
    b.rule_2("<meal><datetime>",
             datetime_check!(|datetime: &DatetimeValue| datetime.latent && form!(Form::Meal)(datetime)),
             b.reg("time")?,
             |a, _| Ok(a.value().clone().not_latent())
    );
    // DATETIME#147
    // TODO: output::time - check support / inclusion in other rules
    b.rule_2("the <part-of-day>",
             b.reg(r#"the"#)?,
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             |_, datetime| Ok(datetime.value().clone().latent())
    );
    // DATETIME#148
    // TODO: output::time - check support / inclusion in other rules
    b.rule_2("in|during the <part-of-day>",
             b.reg(r#"(?:in|during)(?: the)?"#)?,
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             |_, datetime| Ok(datetime.value().clone().not_latent())
    );
    // DATETIME#149
    // TODO: output::time - check support / inclusion in other rules
    b.rule_2("this <part-of-day>",
             b.reg(r#"this"#)?,
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             |_, datetime| Ok(helpers::cycle_nth(Grain::Day, 0)?
                 .intersect(datetime.value())?
                 .form(datetime.value().form.clone()))
    );
    // DATETIME#150
    // TODO: output::time - check w/ other rules and prepositions
    b.rule_1_terminal("tonight",
                      b.reg(r#"toni(?:ght|gth|te)"#)?,
                      |_| {
                          let period = helpers::hour(18, false)?.span_to(&helpers::hour(0, false)?, false)?;
                          Ok(helpers::cycle_nth(Grain::Day, 0)?
                              .intersect(&period)?
                              .form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );
    // DATETIME#151
    // TODO: output::time - check support / inclusion in other rules
    b.rule_1_terminal("after lunch",
                      b.reg(r#"after(?:-|\s)?lunch"#)?,
                      |_| {
                          let period = helpers::hour(13, false)?.span_to(&helpers::hour(17, false)?, false)?;
                          Ok(helpers::cycle_nth(Grain::Day, 0)?
                              .intersect(&period)?
                              .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
                      }
    );
    // DATETIME#152
    // TODO: output::time - check support / inclusion in other rules
    b.rule_1_terminal("after work",
                      b.reg(r#"after(?:-|\s)?work"#)?,
                      |_| {
                          let period = helpers::hour(13, false)?.span_to(&helpers::hour(17, false)?, false)?;
                          Ok(helpers::cycle_nth(Grain::Day, 0)?
                              .intersect(&period)?
                              .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
                      }
    );
    /* END OF DATETIME - TIME - PARTS OF DAY */

    /* DATETIME - DATE - DATE + PARTS OF DAY */

    // DATETIME#153
    // TODO: output::datetime - restrict combination of date/time forms
    b.rule_2("<datetime> <part-of-day>",
             datetime_check!(|datetime: &DatetimeValue| excluding_form!(Form::Year(_))(datetime) && excluding_form!(Form::Month(_))(datetime)),
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             |datetime, part_of_day| datetime.value().intersect(part_of_day.value())
    );
    // DATETIME#154
    // TODO: output::datetime - restrict combination of date/time forms - but check correctness & support
    b.rule_2("<part-of-day> <datetime>",
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             datetime_check!(|datetime: &DatetimeValue| excluding_form!(Form::Year(_))(datetime) && excluding_form!(Form::Month(_))(datetime)),
             |part_of_day, datetime| datetime.value().intersect(part_of_day.value())
    );
    // DATETIME#155
    // TODO: output::datetime - restrict combination of date/time forms - but check correctness & support
    b.rule_3("<part-of-day> of <datetime>",
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             b.reg(r#"of"#)?,
             datetime_check!(|datetime: &DatetimeValue| excluding_form!(Form::Year(_))(datetime) && excluding_form!(Form::Month(_))(datetime)),
             |part_of_day, _, datetime| datetime.value().intersect(part_of_day.value())
    );
    // DATETIME#173
    // TODO: output::date - check if supported and restrict date form to day
    b.rule_3("<datetime> before <time-of-day> (interval)",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             b.reg(r#"until|before|not after"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |a, _, b| a.value().span_to(b.value(), false)
    );

    /* END OF DATETIME - DATE - DATE + PARTS OF DAY */

    /* DATETIME - DATE-PERIOD - GRAINS AS DATE INTERVALS */

    // DATETIME#156
    // TODO: output::datetime
    b.rule_1_terminal("week-end",
                      b.reg(r#"(?:the )?(?:week(?:\s|-)?end|wkend)"#)?,
                      |_| {
                          let friday = helpers::day_of_week(Weekday::Fri)?
                              .intersect(&helpers::hour(18, false)?)?;
                          let monday = helpers::day_of_week(Weekday::Mon)?
                              .intersect(&helpers::hour(0, false)?)?;
                          friday.span_to(&monday, false)
                      }
    );
    // DATETIME#157
    // TODO: output::date-period + add dedicated form + check use in rules w/ date-period
    b.rule_1_terminal("season",
                      b.reg(r#"(?:the )?summer"#)?,
                      |_| helpers::month_day(6, 21)?.span_to(&helpers::month_day(9, 23)?, false)
    );
    // DATETIME#158
    // TODO: output::date-period + add dedicated form + check use in rules w/ date-period
    b.rule_1_terminal("season",
                      b.reg(r#"(?:the )?(?:fall|autumn)"#)?,
                      |_| helpers::month_day(9, 23)?.span_to(&helpers::month_day(12, 21)?, false)
    );
    // DATETIME#159
    // TODO: output::date-period + add dedicated form + check use in rules w/ date-period
    b.rule_1_terminal("season",
                      b.reg(r#"(?:the )?winter"#)?,
                      |_| helpers::month_day(12, 21)?.span_to(&helpers::month_day(3, 20)?, false)
    );
    // DATETIME#160
    // TODO: output::date-period + add dedicated form + check use in rules w/ date-period
    b.rule_1_terminal("season",
                      b.reg(r#"(?:the )?spring"#)?,
                      |_| helpers::month_day(3, 20)?.span_to(&helpers::month_day(6, 21)?, false)
    );

    /* END OF DATETIME - DATE-PERIOD - GRAINS AS DATE INTERVALS */

    /* DATETIME - TIME - TIME OF DAY WITH PRECISION - UNSUPPORTED */

    // DATETIME#161
    // TODO: [rm] not supported
    b.rule_1_terminal("<hour>ish",
                      b.reg(r#"(one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)ish"#)?,
                      |text_match| {
                          let hour = match text_match.group(1).as_ref() {
                              "one" => 1,
                              "two" => 2,
                              "three" => 3,
                              "four" => 4,
                              "five" => 5,
                              "six" => 6,
                              "seven" => 7,
                              "eight" => 8,
                              "nine" => 9,
                              "ten" => 10,
                              "eleven" => 11,
                              "twelve" => 12,
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          Ok(helpers::hour(hour, true)?.precision(Approximate))
                      });
    // DATETIME#162
    // TODO: [rm] not supported
    b.rule_2("<time-of-day> approximately",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:-?ish|approximately)"#)?,
             |datetime, _| Ok(datetime.value().clone().not_latent().precision(Precision::Approximate))
    );
    // DATETIME#163
    // TODO: [rm] not supported
    b.rule_2("about <time-of-day>",
             b.reg(r#"(?:about|around|approximately)"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, datetime| Ok(datetime.value().clone().not_latent().precision(Precision::Approximate))
    );
    // DATETIME#164
    // TODO: [rm] not supported
    b.rule_2("<time-of-day> sharp",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:sharp|exactly|precisely)"#)?,
             |datetime, _| Ok(datetime.value().clone().not_latent().precision(Precision::Exact))
    );
    // DATETIME#165
    // TODO: [rm] not supported
    b.rule_2("exactly <time-of-day>",
             b.reg(r#"(?:exactly|precisely)"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, datetime| Ok(datetime.value().clone().not_latent().precision(Precision::Exact))
    );
    /* END OF DATETIME - TIME - TIME OF DAY WITH PRECISION - UNSUPPORTED */

    /* DATETIME - DATE-PERIOD - FROM DATE INTERVALS */

    // DATETIME#166
    // TODO: output::date-period - split written / verbalized forms
    b.rule_4("<month> dd-dd (interval)",
             datetime_check!(form!(Form::Month(_))),
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             b.reg(r#"\-|to|th?ru|through|(?:un)?til(?:l)?"#)?,
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             |datetime, a, _, b| {
                 let start = datetime.value()
                     .intersect(&helpers::day_of_month(a.group(1).parse()?)?)?;
                 let end = datetime.value()
                     .intersect(&helpers::day_of_month(b.group(1).parse()?)?)?;
                 start.span_to(&end, true)
             }
    );
    // DATETIME#167
    // TODO: output::date-period - split written / verbalized forms
    b.rule_3("<datetime> - <datetime> (interval)",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             b.reg(r#"\-|to|th?ru|through|(?:un)?til(?:l)?"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             |a, _, b| a.value().span_to(b.value(), true)
    );
    // DATETIME#168
    // TODO: output::date-period - split written / verbalized forms
    b.rule_4("from <datetime> - <datetime> (interval)",
             b.reg(r#"from"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             b.reg(r#"(?:on )?(?:\-|to|th?ru|through|(?:un)?til(?:l)?)"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );
    // DATETIME#169
    // TODO: output::date-period - split written / verbalized forms
    b.rule_4("between <datetime> and <datetime> (interval)",
             b.reg(r#"between"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             b.reg(r#"and"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             |_, a, _, b| a.value().span_to(b.value(), true)
    );
    /* END OF DATETIME - DATE-PERIOD - FROM DATE INTERVALS */


    /* DATETIME - TIME-PERIOD - FROM TIME INTERVALS */

    // DATETIME#170
    // TODO: output::time-period - split written / verbalized forms
    b.rule_3("<time-of-day> - <time-of-day> (interval)",
             datetime_check!(|datetime: &DatetimeValue|  !datetime.latent && form!(Form::TimeOfDay(_))(datetime)),
             b.reg(r#"\-|:|to|th?ru|through|(?:un)?til(?:l)?"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |a, _, b| a.value().span_to(b.value(), false)
    );
    // DATETIME#171
    // TODO: output::time-period - split written / verbalized forms
    b.rule_4("from <time-of-day> - <time-of-day> (interval)",
             b.reg(r#"(?:later than|from)"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:(?:but )?before)|\-|to|th?ru|through|(?:un)?til(?:l)?"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );
    // DATETIME#172
    // TODO: output::time-period - split written / verbalized forms
    b.rule_4("between <time-of-day> and <time-of-day> (interval)",
             b.reg(r#"between"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"and"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );
    /* END OF DATETIME - TIME-PERIOD - FROM TIME INTERVALS */

    /* DATETIME - DATE AND TIME PERIODS - SPLIT TO DO */

    // DATETIME#174
    // TODO: split date/time period
    b.rule_2("within <duration>",
             b.reg(r#"within"#)?,
             duration_check!(),
             |_, a| helpers::cycle_nth(Grain::Second, 0)?.span_to(&a.value().in_present()?, false)
    );
    // DATETIME#175
    // TODO: split date/time period
    b.rule_2("by <datetime>",
             b.reg(r#"by"#)?,
             datetime_check!(|datetime: &DatetimeValue|  !datetime.latent),
             |_, a| helpers::cycle_nth(Grain::Second, 0)?.span_to(a.value(), false)
    );
    // DATETIME#176
    // TODO: check support - split date/time, or time shouldn't be supported here?
    b.rule_2("by the end of <datetime>",
             b.reg(r#"by (?:the )?end of"#)?,
             datetime_check!(),
             |_, a| helpers::cycle_nth(Grain::Second, 0)?.span_to(a.value(), true)
    );
    // DATETIME#177
    // TODO: split date/time period + correct regex
    b.rule_2("until <datetime>",
             b.reg(r#"(?:anytime |sometimes? )?(?:(?:un)?til(?:l)?|through|up to)"#)?,
             datetime_check!(),
             |_, a| Ok(a.value().clone().mark_before_end())
    );
    // DATETIME#178
    // TODO: split date/time period + correct regex
    b.rule_2("before <datetime>",
             b.reg(r#"(?:anytime |sometimes? )?before"#)?,
             datetime_check!(),
             |_, a| Ok(a.value().clone().mark_before_start())
    );
    // DATETIME#179
    // TODO: split date/time period + correct regex
    b.rule_2("after <time-of-day>",
             b.reg(r#"(?:anytime |sometimes? )?after"#)?,
             datetime_check!(),
             |_, a| Ok(a.value().clone().mark_after_end())
    );
    // DATETIME#180
    // TODO: split date/time period + correct regex
    b.rule_2("since <time-of-day>",
             b.reg(r#"since"#)?,
             datetime_check!(),
             |_, a| Ok(a.value().the_nth(-1)?.mark_after_start())
    );
    // DATETIME#181
    // TODO: [rm] not supported - what was that?
    b.rule_2("about <duration>",
             b.reg(r#"(?:about|around|approximately)"#)?,
             datetime_check!(|datetime: &DatetimeValue|  !datetime.latent),
             |_, datetime| Ok(datetime.value().clone().precision(Precision::Approximate))
    );
    // DATETIME#182
    // TODO: [rm] not supported - what was that?
    b.rule_2("exactly <duration>",
             b.reg(r#"exactly|precisely"#)?,
             datetime_check!(|datetime: &DatetimeValue|  !datetime.latent),
             |_, datetime| Ok(datetime.value().clone().precision(Precision::Exact))
    );
    /* END OF DATETIME - DATE AND TIME PERIODS - SPLIT TO DO */
    Ok(())
}

pub fn rules_cycle(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    /* DATETIME - CYCLE DEFINITIONS */
    // DATETIME#183
    b.rule_1_terminal("second (cycle)",
                      b.reg(r#"seconds?"#)?,
                      |_| CycleValue::new(Grain::Second)
    );
    // DATETIME#184
    b.rule_1_terminal("minute (cycle)",
                      b.reg(r#"minutes?"#)?,
                      |_| CycleValue::new(Grain::Minute)
    );
    // DATETIME#185
    b.rule_1_terminal("hour (cycle)",
                      b.reg(r#"h(?:ou)?rs?"#)?,
                      |_| CycleValue::new(Grain::Hour)
    );
    // DATETIME#186
    b.rule_1_terminal("day (cycle)",
                      b.reg(r#"days?"#)?,
                      |_| CycleValue::new(Grain::Day)
    );
    // DATETIME#187
    b.rule_1_terminal("week (cycle)",
                      b.reg(r#"weeks?"#)?,
                      |_| CycleValue::new(Grain::Week)
    );
    // DATETIME#188
    b.rule_1_terminal("month (cycle)",
                      b.reg(r#"months?"#)?,
                      |_| CycleValue::new(Grain::Month)
    );
    // DATETIME#189
    b.rule_1_terminal("quarter (cycle)",
                      b.reg(r#"(?:quarter|qtr)s?"#)?,
                      |_| CycleValue::new(Grain::Quarter)
    );
    // DATETIME#190
    b.rule_1_terminal("year (cycle)",
                      b.reg(r#"y(?:ea)?rs?"#)?,
                      |_| CycleValue::new(Grain::Year)
    );
    // DATETIME#191
    // TODO: output::date-period - grain bigger than day
    b.rule_2("this <cycle>",
             b.reg(r#"this|current|coming"#)?,
             cycle_check!(|cycle: &CycleValue| helpers::grain_greather_than_day(cycle.grain)),
             |_, a| helpers::cycle_nth(a.value().grain, 0)
    );
    // DATETIME#192
    // TODO: really keep if this is too ambiguous? also is it ever a correct datetime?
    b.rule_2("the <cycle>",
             b.reg(r#"the"#)?,
             cycle_check!(),
             |_, a| {
                 Ok(helpers::cycle_nth(a.value().grain, 0)?.too_ambiguous())
             }
    );
    // DATETIME#193
    // TODO: output::date-period - grain bigger than day
    b.rule_2("last <cycle>",
             b.reg(r#"(?:the )?(?:last|past|previous)"#)?,
             cycle_check!(|cycle: &CycleValue| helpers::grain_greather_than_day(cycle.grain)),
             |_, a| helpers::cycle_nth(a.value().grain, -1)
    );
    // DATETIME#194
    // TODO: output::date-period - grain bigger than day
    b.rule_2("next <cycle>",
             b.reg(r#"(?:the )?next|the following"#)?,
             cycle_check!(|cycle: &CycleValue| helpers::grain_greather_than_day(cycle.grain)),
             |_, a| helpers::cycle_nth(a.value().grain, 1)
    );
    // DATETIME#195
    // TODO: check but should move to unsupported
    b.rule_4("the <cycle> after <datetime>",
             b.reg(r#"the"#)?,
             cycle_check!(),
             b.reg(r#"after"#)?,
             datetime_check!(),
             |_, cycle, _, datetime| helpers::cycle_nth_after(cycle.value().grain, 1, datetime.value())
    );
    // DATETIME#196
    // TODO: same as #195
    b.rule_3("<cycle> after <datetime>",
             cycle_check!(),
             b.reg(r#"after"#)?,
             datetime_check!(),
             |cycle, _, datetime| helpers::cycle_nth_after(cycle.value().grain, 1, datetime.value())
    );
    // DATETIME#197
    // TODO: same as #195
    b.rule_4("the <cycle> before <datetime>",
             b.reg(r#"the"#)?,
             cycle_check!(),
             b.reg(r#"before"#)?,
             datetime_check!(),
             |_, cycle, _, datetime| helpers::cycle_nth_after(cycle.value().grain, -1, datetime.value())
    );
    // DATETIME#198
    // TODO: same as #195
    b.rule_3("<cycle> before <datetime>",
             cycle_check!(),
             b.reg(r#"before"#)?,
             datetime_check!(),
             |cycle, _, datetime| helpers::cycle_nth_after(cycle.value().grain, -1, datetime.value())
    );
    // DATETIME#199
    // TODO: move to unsupported
    b.rule_3("last n <cycle>",
             b.reg(r#"(?:the |these )?(?:last|past)"#)?,
             integer_check_by_range!(1, 9999),
             cycle_check!(),
             |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    // DATETIME#200
    // TODO: move to unsupported
    b.rule_3("next n <cycle>",
             b.reg(r#"(?:the |these )?next"#)?,
             integer_check_by_range!(1, 9999),
             cycle_check!(),
             |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    // DATETIME#201
    // TODO: move to unsupported
    b.rule_4("<ordinal> <cycle> of <datetime>",
             ordinal_check!(),
             cycle_check!(),
             b.reg(r#"of|in|from"#)?,
             datetime_check!(),
             |ordinal, cycle, _, datetime| helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value - 1, datetime.value())
    );
    // DATETIME#202
    // TODO: move to unsupported
    b.rule_5("the <ordinal> <cycle> of <datetime>",
             b.reg(r#"the"#)?,
             ordinal_check!(),
             cycle_check!(),
             b.reg(r#"of|in|from"#)?,
             datetime_check!(),
             |_, ordinal, cycle, _, datetime| helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value - 1, datetime.value())
    );
    // DATETIME#203
    // TODO: move to unsupported
    b.rule_4("the <cycle> of <datetime>",
             b.reg(r#"the"#)?,
             cycle_check!(),
             b.reg(r#"of"#)?,
             datetime_check!(),
             |_, cycle, _, datetime| helpers::cycle_nth_after_not_immediate(cycle.value().grain, 0, datetime.value())
    );
    // DATETIME#204
    // TODO: move to unsupported
    b.rule_4("<ordinal> <cycle> after <datetime>",
             ordinal_check!(),
             cycle_check!(),
             b.reg(r#"after"#)?,
             datetime_check!(),
             |ordinal, cycle, _, datetime| helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value - 1, datetime.value())
    );
    // DATETIME#205
    // TODO: move to unsupported
    b.rule_5("the <ordinal> <cycle> after <datetime>",
             b.reg(r#"the"#)?,
             ordinal_check!(),
             cycle_check!(),
             b.reg(r#"after"#)?,
             datetime_check!(),
             |_, ordinal, cycle, _, datetime| helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value - 1, datetime.value())
    );
    // DATETIME#206
    // TODO: output::date-period
    b.rule_2(
        "<ordinal> quarter",
        ordinal_check!(),
        cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Quarter),
        |ordinal, _| helpers::cycle_nth_after(Grain::Quarter, ordinal.value().value - 1, &helpers::cycle_nth(Grain::Year, 0)?)
    );
    // DATETIME#207
    // TODO: output::date-period
    b.rule_3("the <ordinal> quarter",
             b.reg(r#"the"#)?,
             ordinal_check!(),
             cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Quarter),
             |_, ordinal, _| helpers::cycle_nth_after(Grain::Quarter, ordinal.value().value - 1, &helpers::cycle_nth(Grain::Year, 0)?)
    );
    // DATETIME#208
    // TODO: output::date-period + adjust rule
    b.rule_3("<ordinal> quarter <year>",
             ordinal_check!(),
             cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Quarter),
             datetime_check!(),
             |ordinal, _, datetime| helpers::cycle_nth_after(Grain::Quarter, ordinal.value().value - 1, datetime.value())
    );
    Ok(())
}