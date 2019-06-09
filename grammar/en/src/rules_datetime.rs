use std::f32;

use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::dimension::Precision::*;
use rustling_ontology_values::helpers;
use rustling_ontology_moment::{Weekday, Grain};


pub fn rules_datetime(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {

    /* DATETIME - COMPLEX RULES */
    // TODO: split date/time combinations + exclude intersect w/ 1 interval?
    b.rule_2("intersect <datetime>",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |a, b| a.value().intersect(b.value())
    );
    // TODO: split date/time combinations + exclude intersect w/ 1 interval?
    b.rule_3("intersect by \"of\", \"from\", \"'s\"",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             b.reg(r#"of|from|for|'s"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |a, _, b| a.value().intersect(b.value())
    );
    // TODO: split date/time combinations + exclude intersect w/ 1 interval?
    b.rule_3("intersect by \",\"",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             b.reg(r#","#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |a, _, b| a.value().intersect(b.value())
    );
    /* END OF DATETIME - COMPLEX RULES */

    /* DATETIME - DATE - PREPOSITION + DATES */
    b.rule_2("on|in <date>",
             b.reg(r#"[oi]n"#)?,
             datetime_check!(|datetime: &DatetimeValue| form!(Form::DayOfMonth)(datetime) || form!(Form::Celebration)(datetime)),
             |_, a| Ok(a.value().clone().not_latent())
    );
    // TODO: add restrictions on datetime form
    b.rule_2("during <date>",
             b.reg(r#"during"#)?,
             datetime_check!(),
             |_, a| Ok(a.value().clone().not_latent())
    );
    // TODO: add restrictions on datetime form
    b.rule_2("for <datetime>",
             b.reg(r#"(?:for|at|on)"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |_, a| Ok(a.value().clone().not_latent())
    );
    b.rule_2("for <meal>",
             b.reg(r#"for"#)?,
             datetime_check!(form!(Form::Meal)),
             |_, a| Ok(a.value().clone().not_latent())
    );
    b.rule_2("in|for <named-month>",
             b.reg(r#"in|for"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, a| Ok(a.value().clone().not_latent())
    );

    b.rule_2("in|for <year>",
             b.reg(r#"in|for"#)?,
             datetime_check!(|datetime: &DatetimeValue| form!(Form::Year(_))(datetime) && !datetime.latent),
             |_, a| Ok(a.value().clone().not_latent())
    );

    b.rule_2("in|for <season>",
             b.reg(r#"in|for"#)?,
             datetime_check!(form!(Form::Season)),
             |_, a| Ok(a.value().clone().not_latent())
    );
    /* END OF DATETIME - DATE - PREPOSITION + DATES */

    /* DATETIME - DATE - STANDALONE SINGLE GRAIN */

    b.rule_1_terminal("named-day",
                      b.reg(r#"monday|mon\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Mon)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"tuesday|tues?\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Tue)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"wed?nesday|wed\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Wed)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"thursday|thu(?:rs?)?\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Thu)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"friday|fri\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Fri)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"saturday|sat\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Sat)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"sunday|sun\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Sun)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"january|jan\.?"#)?,
                      |_| helpers::month(1)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"february|feb\.?"#)?,
                      |_| helpers::month(2)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"march|mar\.?"#)?,
                      |_| helpers::month(3)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"april|apr\.?"#)?,
                      |_| helpers::month(4)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"may"#)?,
                      |_| helpers::month(5)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"june|jun\.?"#)?,
                      |_| helpers::month(6)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"july|jul\.?"#)?,
                      |_| helpers::month(7)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"august|aug\.?"#)?,
                      |_| helpers::month(8)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"september|sept?\.?"#)?,
                      |_| helpers::month(9)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"october|oct\.?"#)?,
                      |_| helpers::month(10)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"november|nov\.?"#)?,
                      |_| helpers::month(11)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"december|dec\.?"#)?,
                      |_| helpers::month(12)
    );
    // Quarters identified by an ordinal are similar to months
    b.rule_2("<ordinal> quarter",
             ordinal_check_by_range!(1, 4),
             cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Quarter),
             |ordinal, _| helpers::cycle_nth_after(Grain::Quarter, ordinal.value().value - 1, &helpers::cycle_nth(Grain::Year, 0)?)
    );
    b.rule_3("the <ordinal> quarter",
             b.reg(r#"the"#)?,
             ordinal_check_by_range!(1, 4),
             cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Quarter),
             |_, ordinal, _| helpers::cycle_nth_after(Grain::Quarter, ordinal.value().value - 1, &helpers::cycle_nth(Grain::Year, 0)?)
    );
    b.rule_3("<ordinal> quarter <year>",
             ordinal_check_by_range!(1, 4),
             cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Quarter),
             datetime_check!(),
             |ordinal, _, datetime| helpers::cycle_nth_after(Grain::Quarter, ordinal.value().value - 1, datetime.value())
    );
    /* END OF DATETIME - DATE - STANDALONE SINGLE GRAIN */


    /* DATETIME - DATE - DEICTICS */

    b.rule_1_terminal("today",
                      b.reg(r#"today"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 0)
    );
    b.rule_1_terminal("tomorrow",
                      b.reg(r#"(?:tmrw?|tomm?or?rows?)"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 1)
    );
    b.rule_1_terminal("the day after tomorrow",
                      b.reg(r#"(?:the )?day after (?:tmrw?|tomm?or?rows?)"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 2)
    );
    b.rule_1_terminal("yesterday",
                      b.reg(r#"yesterdays?"#)?,
                      |_| helpers::cycle_nth(Grain::Day, -1)
    );
    b.rule_1_terminal("the day before yesterday",
                      b.reg(r#"(?:the )?day before yesterdays?"#)?,
                      |_| helpers::cycle_nth(Grain::Day, -2)
    );
    b.rule_1_terminal("end of week",
                      b.reg(r#"(?:the )?end of (?:the )?week"#)?,
                      |_| helpers::day_of_week(Weekday::Thu)
                          ?.span_to(&helpers::day_of_week(Weekday::Sun)?, false)
    );
    b.rule_1_terminal("by the end of the week",
                      b.reg(r#"by (?:the )?end of (?:the )?week"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 0)?
                          .span_to(&helpers::day_of_week(Weekday::Sun)?, true)
    );
    b.rule_1_terminal("EOD|End of day",
                      b.reg(r#"(?:eod|end of (?:the )?day)"#)?,
                      |_| helpers::hour(20, false)
    );
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
    b.rule_1_terminal("by the end of month",
                      b.reg(r#"by (?:the )?(?:eom|end of (?:the )?month)"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 0)?
                          .span_to(&helpers::cycle_nth(Grain::Month, 0)?, true)
    );
    b.rule_1_terminal("EOY|End of year",
                      b.reg(r#"(?:the )?(?:eoy|end of (?:the )?year)"#)?,
                      |_| {
                          let current_year = helpers::cycle_nth(Grain::Year, 0)?;
                          let start = current_year.intersect(&helpers::month(10)?)?;
                          let end = current_year.intersect(&helpers::month(12)?)?;
                          start.span_to(&end, true)
                      }
    );
    b.rule_1_terminal("by the end of year",
                      b.reg(r#"by (?:the )?(?:eoy|end of (?:the )?year)"#)?,
                      |_| {
                          let current_year = helpers::cycle_nth(Grain::Year, 0)?;
                          let end = current_year.intersect(&helpers::month(12)?)?;
                          helpers::cycle_nth(Grain::Second, 0)?
                              .span_to(&end, true)
                      }
    );
    b.rule_2("this|next <day-of-week>",
             b.reg(r#"this|(?:the )?next"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             |_, a| {
                 a.value().the_nth_not_immediate(0)
             }
    );
    // TODO: add restrictions on datetime form
    b.rule_2("this <datetime>",
             b.reg(r#"the|this|current|coming"#)?,
             datetime_check!(|datetime: &DatetimeValue| !form!(Form::PartOfDay(_))(datetime) && !form!(Form::Meal)(datetime)),
             |_, a| {
                 Ok(a.value().the_nth(0)?
                     .form(a.value().form.clone())
                     .datetime_kind(a.value().datetime_kind.clone()))
             }
    );
    // TODO: add restrictions on datetime form
    b.rule_2("next <datetime>",
             b.reg(r#"(?:the |this )?next"#)?,
             datetime_check!(|datetime: &DatetimeValue| !form!(Form::PartOfDay(_))(datetime) && !form!(Form::Meal)(datetime)),
             |_, a| {
                 Ok(a.value().the_nth(0)?
                     .form(a.value().form.clone())
                     .datetime_kind(a.value().datetime_kind.clone()))
             }
    );
    // TODO: add restrictions on datetime form
    b.rule_2("last <datetime>",
             b.reg(r#"this past|(?:the |this )?last"#)?,
             datetime_check!(|datetime: &DatetimeValue| !form!(Form::PartOfDay(_))(datetime) && !form!(Form::Meal)(datetime)),
             |_, a| {
                 Ok(a.value().the_nth(-1)?
                     .form(a.value().form.clone())
                     .datetime_kind(a.value().datetime_kind.clone()))
             }
    );
    // TODO: add restrictions on datetime form (week days?)
    b.rule_2("<datetime> after next",
             datetime_check!(),
             b.reg(r#"after next"#)?,
             |a, _| {
                 Ok(a.value().the_nth_not_immediate(1)?
                     .form(a.value().form.clone())
                     .datetime_kind(a.value().datetime_kind.clone()))
             }
    );
    /* END OF DATETIME - DATE - DEICTICS */

    /* DATETIME - DATE - YEAR */

    b.rule_2("the year + integer 1000-2100",
             b.reg(r#"(?:the )?year"#)?,
             integer_check_by_range!(1000, 2100),
             |_, integer| {
                 helpers::year(integer.value().value as i32)
             }
    );
    b.rule_3("the year + composed 1900-2199",
             b.reg(r#"(?:the )?year"#)?,
             integer_check_by_range!(19, 21),
             integer_check_by_range!(10, 99),
             |_, a, b| {
                 let y = a.value().value * 100 + b.value().value;
                 Ok(helpers::year(y as i32)?.latent())
             }
    );
    b.rule_1("year as integer 1000-2100",
             integer_check_by_range!(1000, 2100),
             |integer| {
                 helpers::year(integer.value().value as i32)
             }
    );
    b.rule_1_terminal("year as short integer 00-09",
              b.reg(r#"0(\d)"#)?,
             |integer| {
                 Ok(helpers::year(integer.group(1).parse()?)?.latent())
             }
    );
    b.rule_1("year as short integer 10-99",
             integer_check_by_range!(10, 99),
             |integer| {
                 Ok(helpers::year(integer.value().value as i32)?.latent())
             }
    );
    b.rule_2("year as integer composed 1900-2199",
             integer_check_by_range!(19, 21),
             integer_check_by_range!(10, 99),
             |a, b| {
                 let y = a.value().value * 100 + b.value().value;
                 Ok(helpers::year(y as i32)?.latent())
             }
    );
    b.rule_1("year as integer -1000-999",
             integer_check_by_range!(-1000, 999),
             |integer| {
                 Ok(helpers::year(integer.value().value as i32)?.latent())
             }
    );
    b.rule_1("year as integer 2101-2200",
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

    b.rule_2("the <day-of-month> (ordinal)",
             b.reg(r#"the"#)?,
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
             |_, ordinal| {
                 Ok(helpers::day_of_month(ordinal.value().value as u32)?.latent())
             }
    );
    b.rule_1("<day-of-month> (ordinal)",
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
             |ordinal| {
                 Ok(helpers::day_of_month(ordinal.value().value as u32)?.latent())
             }
    );
// FIXME: incorrect/unsupport
    b.rule_2("the <day-of-month> (non ordinal)",
             b.reg(r#"the"#)?,
             integer_check_by_range!(1, 31),
             |_, integer| {
                 Ok(helpers::day_of_month(integer.value().value as u32)?.latent())
             }
    );
    // TODO: allow <day-of-month> as integer form too?
    b.rule_2("<named-day> <day-of-month> (ordinal)",
             datetime_check!(form!(Form::DayOfWeek{..})),
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
             |a, ordinal| {
                 a.value().intersect(&helpers::day_of_month(ordinal.value().value as u32)?)
             }
    );
    b.rule_2("<named-day> <month-day>",
             datetime_check!(form!(Form::DayOfWeek{..})),
             datetime_check!(form!(Form::MonthDay(_))),
             |dow, month_day| {
                 month_day.value().intersect(&dow.value())
             }
    );
    b.rule_2("<month-day> <year>",
             datetime_check!(form!(Form::MonthDay(_))),
             datetime_check!(form!(Form::Year(_))),
             |month_day, year| {
                 year.value().intersect(&month_day.value())
             }
    );
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
    b.rule_2("<named-month> <day-of-month> (non ordinal)",
             datetime_check!(form!(Form::Month(_))),
             integer_check_by_range!(1, 31),
             |month, integer| {
                 let m = month.value().form_month()?;
                 let form = Form::MonthDay(Some(MonthDayForm { month: m,  day_of_month: integer.value().value as u32}));
                 Ok(month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)?.form(form))
             }
    );
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
    b.rule_2("<day-of-month> (non ordinal) <named-month>",
             integer_check_by_range!(1, 31),
             datetime_check!(form!(Form::Month(_))),
             |integer, month| {
                 let m = month.value().form_month()?;
                 let form = Form::MonthDay(Some(MonthDayForm { month: m,  day_of_month: integer.value().value as u32}));
                 Ok(month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)?.form(form))
             }
    );
    b.rule_2("<day-of-month>(ordinal) <named-month>",
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
             datetime_check!(form!(Form::Month(_))),
             |ordinal, month| {
                 let m = month.value().form_month()?;
                 let form = Form::MonthDay(Some(MonthDayForm { month: m,  day_of_month: ordinal.value().value as u32}));
                 Ok(month.value().intersect(&helpers::day_of_month(ordinal.value().value as u32)?)?.form(form))
             }
    );
    /* END OF DATETIME - DATE - DATES */

    /* DATETIME - TIME - TIME OF DAY */

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
    b.rule_2("at <time-of-day>",
             b.reg(r#"at|@"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, a| Ok(a.value().clone().not_latent())
    );
    b.rule_2("<time-of-day> o'clock",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"o.?clock"#)?,
             |a, _| Ok(a.value().clone().not_latent())
    );
    b.rule_3("at <time-of-day> hours",
             b.reg(r#"at"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"hours"#)?,
             |_, a, _| Ok(a.value().clone().not_latent())
    );
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
    b.rule_1_terminal("noon",
                      b.reg(r#"noon|midday"#)?,
                      |_| helpers::hour(12, false)
    );
    b.rule_1_terminal("midnight",
                      b.reg(r#"midni(?:ght|te)"#)?,
                      |_| helpers::hour(0, false)
    );
    b.rule_1_terminal("quarter (relative minutes)",
                      b.reg(r#"(?:a|one)? ?quarter"#)?,
                      |_| Ok(RelativeMinuteValue(15))
    );
    b.rule_1_terminal("half (relative minutes)",
                      b.reg(r#"half"#)?,
                      |_| Ok(RelativeMinuteValue(30))
    );
    b.rule_1("number (as relative minutes)",
             integer_check_by_range!(1, 59),
             |a| Ok(RelativeMinuteValue(a.value().value as i32))
    );
    b.rule_2("0 or o as 0 + number (as relative minutes)",
             b.reg(r#"o|zero"#)?,
             integer_check_by_range!(1, 9),
             |_, a| Ok(RelativeMinuteValue(a.value().value as i32))
    );
    b.rule_2("number <minutes> (as relative minutes)",
             integer_check_by_range!(1, 59),
             b.reg(r#"minutes?"#)?,
             |a, _| Ok(RelativeMinuteValue(a.value().value as i32))
    );
    b.rule_3("0 or o as 0 + number <minutes> (as relative minutes)",
             b.reg(r#"o|zero"#)?,
             integer_check_by_range!(1, 9),
             b.reg(r#"minutes?"#)?,
             |_, a, _| Ok(RelativeMinuteValue(a.value().value as i32))
    );
    b.rule_2("<hour-of-day> <integer> (as relative minutes)",
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour {.. }))),
             relative_minute_check!(),
             |datetime, relative_minute| Ok(helpers::hour_relative_minute(
                 datetime.value().form_time_of_day()?.full_hour(),
                 relative_minute.value().0,
                 true)?
                 .precision(datetime.value().precision))
    );
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
    b.rule_3("relative minutes after|past <integer> (hour-of-day)",
             relative_minute_check!(),
             b.reg(r#"after|past"#)?,
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour {.. }))),
             |relative_minute, _, datetime| Ok(helpers::hour_relative_minute(
                 datetime.value().form_time_of_day()?.full_hour(),
                 relative_minute.value().0,
                 true)?.precision(datetime.value().precision))
    );
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
// Time - written form only
    b.rule_1_terminal("hh:mm",
                      b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:.]([0-5]\d)"#)?,
                      |text_match| helpers::hour_minute(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?,
                          true)
    );
// Time - written form only
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
// Date - written form only
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
// Date - written form only
    b.rule_1_terminal("dd/mm/yy or dd/mm/yyyy - Non ambiguous cases - Non US standard",
                      b.reg(r#"(1[3-9]|2\d|3[01])[-/\.](0?[1-9]|1[0-2])[-/\.](\d{2,4})"#)?,
                      |text_match| helpers::year_month_day(
                          text_match.group(3).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(1).parse()?)
    );
// Date - written form only
    b.rule_1_terminal("mm/dd/yy or mm/dd/yyyy - Non ambiguous cases - US standard",
                      b.reg(r#"(0?[1-9]|1[0-2])[-/\.](1[3-9]|2\d|3[01])[-/\.](\d{2,4})"#)?,
                      |text_match| helpers::year_month_day(
                          text_match.group(3).parse()?,
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?)
    );
// Date - written form only
    b.rule_1_terminal("mm/dd/yy or mm/dd/yyyy - Ambiguous cases - interpret as US standard",
                      b.reg(r#"(0?[1-9]|1[0-2])[-/\.](0?[1-9]|1[0-2])[-/\.](\d{2,4})"#)?,
                      |text_match| helpers::year_month_day(
                          text_match.group(3).parse()?,
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?)
    );
// Date - written form only
    b.rule_1_terminal("dd/mm - Non ambiguous cases - Non US standard",
                      b.reg(r#"(1[3-9]|2\d|3[01])[/\.](0?[1-9]|1[0-2])"#)?,
                      |text_match| helpers::month_day(
                          text_match.group(2).parse()?,
                          text_match.group(1).parse()?)
    );
// Date - written form only
    b.rule_1_terminal("mm/dd - Non ambiguous cases - US standard",
                      b.reg(r#"(0?[1-9]|1[0-2])[/\.](3[01]|2\d|1[3-9])"#)?,
                      |text_match| helpers::month_day(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?)
    );
// Date - written form only
    b.rule_1_terminal("mm/dd - Ambiguous cases - interpret as US standard",
                      b.reg(r#"(0?[1-9]|1[0-2])[/\.](0?[1-9]|1[0-2])"#)?,
                      |text_match| helpers::month_day(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?)
    );
    /* END OF DATETIME - DATE - DATES - WRITTEN FORMS */

    /* DATETIME - TIME - PARTS OF DAY */

    b.rule_1_terminal("now",
                      b.reg(r#"(?:just |right )?now|immediately|at (?:this time|(?:this very |the )moment|the present time)|for the moment"#)?,
                      |_| helpers::cycle_nth(Grain::Second, 0)
    );

    b.rule_1_terminal("morning",
                      b.reg(r#"morning"#)?,
                      |_| {
                          Ok(helpers::hour(4, false)?
                              .span_to(&helpers::hour(12, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Morning)))
                      }
    );
    b.rule_1_terminal("breakfast",
                      b.reg(r#"breakfast"#)?,
                      |_| Ok(helpers::hour(5, false)?
                          .span_to(&helpers::hour(10, false)?, false)?
                          .latent()
                          .form(Form::Meal))
    );
    b.rule_1_terminal("early morning",
                      b.reg(r#"early (?:(?:in|hours of) the )?morning"#)?,
                      |_| {
                          Ok(helpers::hour(4, false)?
                              .span_to(&helpers::hour(9, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Morning)))
                      }
    );
// FIXME: Change PartOfDayForm?
    b.rule_1_terminal("before work",
                      b.reg(r#"before work"#)?,
                      |_| {
                          Ok(helpers::hour(4, false)?
                              .span_to(&helpers::hour(9, false)?, false)?
                              .form(Form::PartOfDay(PartOfDayForm::Morning)))
                      }
    );
    b.rule_1_terminal("during work",
                      b.reg(r#"during work(?: time)?"#)?,
                      |_| {
                          Ok(helpers::hour(9, false)?
                              .span_to(&helpers::hour(19, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::None)))
                      }
    );
    b.rule_1_terminal("afternoon",
                      b.reg(r#"after ?noo?n"#)?,
                      |_| {
                          Ok(helpers::hour(12, false)?
                              .span_to(&helpers::hour(19, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
                      }
    );
    b.rule_1_terminal("evening",
                      b.reg(r#"evening"#)?,
                      |_| {
                          Ok(helpers::hour(18, false)?
                              .span_to(&helpers::hour(0, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );
    b.rule_1_terminal("night",
                      b.reg(r#"night"#)?,
                      |_| {
                          Ok(helpers::hour(00, false)?
                              .span_to(&helpers::hour(5, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );
    b.rule_1_terminal("last night",
                      b.reg(r#"last night"#)?,
                      |_| {
                          let yesterday = helpers::cycle_nth(Grain::Day, -1)?;
                          let night = helpers::hour(00, false)?
                              .span_to(&helpers::hour(5, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Evening));
                          yesterday.intersect(&night)
                      }
    );
        b.rule_1_terminal("brunch",
                      b.reg(r#"brunch"#)?,
                      |_| Ok(helpers::hour(10, false)?
                          .span_to(&helpers::hour(15, false)?, false)?
                          .latent()
                          .form(Form::Meal))
    );
    b.rule_1_terminal("lunch",
                      b.reg(r#"lunch"#)?,
                      |_| {
                          Ok(helpers::hour(12, false)?
                              .span_to(&helpers::hour(14, false)?, false)?
                              .latent()
                              .form(Form::Meal))
                      }
    );
    b.rule_1_terminal("dinner",
                      b.reg(r#"dinner|supper"#)?,
                      |_| Ok(helpers::hour(18, false)?
                          .span_to(&helpers::hour(23, false)?, false)?
                          .latent()
                          .form(Form::Meal))
    );
    b.rule_1_terminal("tea",
                      b.reg(r#"(?:at )?tea time"#)?,
                      |_| Ok(helpers::hour(15, false)?
                          .span_to(&helpers::hour(17, false)?, false)?
                          .form(Form::Meal))
    );
    b.rule_2("at <meal>",
             b.reg("at|for|during")?,
             datetime_check!(form!(Form::Meal)),
             |_, a| Ok(a.value().clone().not_latent())
    );
    b.rule_2("around <meal>",
             b.reg("(?:about|around|approximately)")?,
             datetime_check!(form!(Form::Meal)),
             |_, a| Ok(a.value().clone().not_latent().precision(Approximate))
    );
    b.rule_2("<meal><datetime>",
             datetime_check!(|datetime: &DatetimeValue| datetime.latent && form!(Form::Meal)(datetime)),
             b.reg("time")?,
             |a, _| Ok(a.value().clone().not_latent())
    );
    b.rule_2("the <part-of-day>",
             b.reg(r#"the"#)?,
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             |_, datetime| Ok(datetime.value().clone().latent())
    );
    b.rule_2("in|for≤|during the <part-of-day>",
             b.reg(r#"(?:in|for|during)(?: the)?"#)?,
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             |_, datetime| Ok(datetime.value().clone().not_latent())
    );
    b.rule_2("this <part-of-day>",
             b.reg(r#"this"#)?,
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             |_, datetime| Ok(helpers::cycle_nth(Grain::Day, 0)?
                 .intersect(datetime.value())?
                 .form(datetime.value().form.clone())
                 .datetime_kind(DatetimeKind::DatetimeComplement { date_and_time: true, today: true }))
    );
    b.rule_1_terminal("tonight",
                      b.reg(r#"toni(?:ght|gth|te)"#)?,
                      |_| {
                          let period = helpers::hour(18, false)?.span_to(&helpers::hour(0, false)?, false)?;
                          Ok(helpers::cycle_nth(Grain::Day, 0)?
                              .intersect(&period)?
                              .form(Form::PartOfDay(PartOfDayForm::Evening))
                              .datetime_kind(DatetimeKind::DatetimeComplement { date_and_time: true, today: true }))
                      }
    );
    b.rule_1_terminal("after lunch",
                      b.reg(r#"after(?:-|\s)?lunch"#)?,
                      |_| {
                          let period = helpers::hour(13, false)?.span_to(&helpers::hour(17, false)?, false)?;
                          Ok(helpers::cycle_nth(Grain::Day, 0)?
                                 .intersect(&period)?
                              .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
                      }
    );
    b.rule_1_terminal("after work - resolve as 'evening' but not latent",
                      b.reg(r#"after(?:-|\s)?work"#)?,
                      |_| {
                          Ok(helpers::hour(18, false)?
                              .span_to(&helpers::hour(0, false)?, false)?
                              .form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );
    /* END OF DATETIME - TIME - PARTS OF DAY */

    /* DATETIME - DATE - DATE + PARTS OF DAY */

    // TODO: Date ruletime - restrict combination of date/time forms
    b.rule_2("<datetime> <part-of-day>",
             datetime_check!(|datetime: &DatetimeValue| excluding_form!(Form::Year(_))(datetime) && excluding_form!(Form::Month(_))(datetime)),
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             |datetime, part_of_day| datetime.value().intersect(part_of_day.value())
    );
    // TODO: Date ruletime - restrict combination of date/time forms - but check correctness & support
    b.rule_2("<part-of-day> <datetime>",
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             datetime_check!(|datetime: &DatetimeValue| excluding_form!(Form::Year(_))(datetime) && excluding_form!(Form::Month(_))(datetime)),
             |part_of_day, datetime| datetime.value().intersect(part_of_day.value())
    );
    // TODO: Date ruletime - restrict combination of date/time forms - but check correctness & support
    b.rule_3("<part-of-day> of <datetime>",
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             b.reg(r#"of"#)?,
             datetime_check!(|datetime: &DatetimeValue| excluding_form!(Form::Year(_))(datetime) && excluding_form!(Form::Month(_))(datetime)),
             |part_of_day, _, datetime| datetime.value().intersect(part_of_day.value())
    );
    // TODO: Date rule - check if supported and restrict date form to day
    b.rule_3("<datetime> before <time-of-day> (interval)",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             b.reg(r#"until|before|not after"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |a, _, b| a.value().span_to(b.value(), false)
    );

    /* END OF DATETIME - DATE - DATE + PARTS OF DAY */

    /* DATETIME - DATE-PERIOD - GRAINS AS DATE INTERVALS */

    b.rule_1_terminal("week-end - Hour grain, from Friday evening to Sunday midnight",
                      b.reg(r#"(?:the )?(?:week(?:\s|-)?end|wkend)"#)?,
                      |_| {
                          let friday = helpers::day_of_week(Weekday::Fri)?
                              .intersect(&helpers::hour(18, false)?)?;
                          let monday = helpers::day_of_week(Weekday::Mon)?
                              .intersect(&helpers::hour(0, false)?)?;
                          Ok(friday.span_to(&monday, false)?.datetime_kind(DatetimeKind::DatePeriod))
                      }
    );
    b.rule_1_terminal("season",
                      b.reg(r#"(?:the )?summer"#)?,
                      |_| Ok(helpers::month_day(6, 21)?
                          .span_to(&helpers::month_day(9, 23)?, false)?
                          .form(Form::Season))
    );
    b.rule_1_terminal("season",
                      b.reg(r#"(?:the )?(?:fall|autumn)"#)?,
                      |_| Ok(helpers::month_day(9, 23)?
                          .span_to(&helpers::month_day(12, 21)?, false)?
                          .form(Form::Season))
    );
    b.rule_1_terminal("season",
                      b.reg(r#"(?:the )?winter"#)?,
                      |_| Ok(helpers::month_day(12, 21)?
                          .span_to(&helpers::month_day(3, 20)?, false)?
                          .form(Form::Season))
    );
    b.rule_1_terminal("season",
                      b.reg(r#"(?:the )?spring"#)?,
                      |_| Ok(helpers::month_day(3, 20)?
                          .span_to(&helpers::month_day(6, 21)?, false)?
                          .form(Form::Season))
    );

    /* END OF DATETIME - DATE-PERIOD - GRAINS AS DATE INTERVALS */

    /* DATETIME - TIME - TIME OF DAY WITH PRECISION - UNSUPPORTED */

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
    b.rule_2("<time-of-day> approximately",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:-?ish|approximately)"#)?,
             |datetime, _| Ok(datetime.value().clone().not_latent().precision(Precision::Approximate))
    );
    b.rule_2("about <time-of-day>",
             b.reg(r#"(?:about|around|approximately)"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, datetime| Ok(datetime.value().clone().not_latent().precision(Precision::Approximate))
    );
    b.rule_2("<time-of-day> sharp",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:sharp|exactly|precisely)"#)?,
             |datetime, _| Ok(datetime.value().clone().not_latent().precision(Precision::Exact))
    );
    b.rule_2("exactly <time-of-day>",
             b.reg(r#"(?:exactly|precisely)"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, datetime| Ok(datetime.value().clone().not_latent().precision(Precision::Exact))
    );
    /* END OF DATETIME - TIME - TIME OF DAY WITH PRECISION - UNSUPPORTED */

    /* DATETIME - DATE-PERIOD - FROM DATE INTERVALS */

    // TODO: split written / verbalized forms
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
    // TODO: split written / verbalized forms
    b.rule_3("<datetime> - <datetime> (interval)",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             b.reg(r#"\-|to|th?ru|through|(?:un)?til(?:l)?"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             |a, _, b| a.value().span_to(b.value(), true)
    );
    // TODO: split written / verbalized forms
    b.rule_4("from <datetime> - <datetime> (interval)",
             b.reg(r#"from"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             b.reg(r#"(?:on )?(?:\-|to|th?ru|through|(?:un)?til(?:l)?)"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );
    // TODO: split written / verbalized forms
    b.rule_4("between <datetime> and <datetime> (interval)",
             b.reg(r#"between"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             b.reg(r#"and"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             |_, a, _, b| a.value().span_to(b.value(), true)
    );
    /* END OF DATETIME - DATE-PERIOD - FROM DATE INTERVALS */


    /* DATETIME - TIME-PERIOD - FROM TIME INTERVALS */

    // TODO: split written / verbalized forms
    b.rule_3("<time-of-day> - <time-of-day> (interval)",
             datetime_check!(|datetime: &DatetimeValue|  !datetime.latent && form!(Form::TimeOfDay(_))(datetime)),
             b.reg(r#"\-|:|to|th?ru|through|(?:un)?til(?:l)?"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |a, _, b| a.value().span_to(b.value(), false)
    );
    // TODO: split written / verbalized forms
    b.rule_4("from <time-of-day> - <time-of-day> (interval)",
             b.reg(r#"(?:later than|from)"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:(?:but )?before)|\-|to|th?ru|through|(?:un)?til(?:l)?"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );
    // TODO: split written / verbalized forms
    b.rule_4("between <time-of-day> and <time-of-day> (interval)",
             b.reg(r#"between"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"and"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );
    /* END OF DATETIME - TIME-PERIOD - FROM TIME INTERVALS */

    /* DATETIME - DATE AND TIME PERIODS - SPLIT TO DO */

    b.rule_2("by <time-of-day>",
             b.reg(r#"by"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && form!(Form::TimeOfDay(_))(datetime)),
             |_, a| helpers::cycle_nth(Grain::Second, 0)?.span_to(a.value(), false)
    );
    // TODO: restrict datetime forms
    b.rule_2("by the end of <datetime>",
             b.reg(r#"by (?:the )?end of"#)?,
             datetime_check!(),
             |_, a| helpers::cycle_nth(Grain::Day, 0)?.span_to(a.value(), true)
    );
    // TODO: split date/time period + correct regex
    b.rule_2("until <datetime>",
             b.reg(r#"(?:anytime |sometimes? )?(?:(?:un)?til(?:l)?|through|up to)"#)?,
             datetime_check!(),
             |_, a| Ok(a.value().clone().mark_before_end())
    );
    // TODO: split date/time period + correct regex
    b.rule_2("before <datetime>",
             b.reg(r#"(?:anytime |sometimes? )?before"#)?,
             datetime_check!(),
             |_, a| Ok(a.value().clone().mark_before_start())
    );
    // TODO: split date/time period + correct regex
    b.rule_2("after <time-of-day>",
             b.reg(r#"(?:anytime |sometimes? )?after"#)?,
             datetime_check!(),
             |_, a| Ok(a.value().clone().mark_after_end())
    );
    // TODO: split date/time period + correct regex
    b.rule_2("since <time-of-day>",
             b.reg(r#"since"#)?,
             datetime_check!(),
             |_, a| Ok(a.value().the_nth(-1)?.mark_after_start())
    );
    b.rule_2("about <datetime>",
             b.reg(r#"(?:about|around|approximately)"#)?,
             datetime_check!(|datetime: &DatetimeValue|  !datetime.latent),
             |_, datetime| Ok(datetime.value().clone().precision(Precision::Approximate))
    );
    b.rule_2("exactly <datetime>",
             b.reg(r#"exactly|precisely"#)?,
             datetime_check!(|datetime: &DatetimeValue|  !datetime.latent),
             |_, datetime| Ok(datetime.value().clone().precision(Precision::Exact))
    );
    /* END OF DATETIME - DATE AND TIME PERIODS - SPLIT TO DO */

    /* DATETIME - MISC. / UNSUPPORTED? */

    b.rule_2("absorption of , after named day",
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#","#)?,
             |a, _| Ok(a.value().clone())
    );
// Semantics of this is not a datetime with resolution
    // b.rule_2("on a <named-day>",
    //          b.reg(r#"on a"#)?,
    //          datetime_check!(form!(Form::DayOfWeek{..})),
    //          |_, a| Ok(a.value().clone())
    // );
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
//    b.rule_2("<datetime> before last",
//             datetime_check!(),
//             b.reg(r#"before last"#)?,
//             |a, _| {
//                 a.value().the_nth(-2)
//             }
//    );
    /* END OF DATETIME - MISC. / UNSUPPORTED? */

    Ok(())
}

pub fn rules_datetime_with_duration(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {

    b.rule_2("in <duration>",
             b.reg(r#"in"#)?,
             duration_check!(),
             |_, duration| duration.value().in_present()
    );
    b.rule_3("in <duration> from now",
             b.reg(r#"in"#)?,
             duration_check!(),
             b.reg(r#"from now"#)?,
             |_, duration, _| duration.value().in_present()
    );
    b.rule_3("in <duration>",
             b.reg(r#"in"#)?,
             duration_check!(),
             b.reg(r#"(?:' )? times?"#)?,
             |_, duration, _| duration.value().in_present()
    );

    // TODO: split date/time period
    b.rule_2("within <duration>",
             b.reg(r#"within"#)?,
             duration_check!(),
             |_, a| helpers::cycle_nth(Grain::Second, 0)?.span_to(&a.value().in_present()?, false)
    );

    b.rule_2("<duration> from now/today",
             duration_check!(),
             b.reg(r#"from (today|now)"#)?,
             |a, _| {
                 a.value().in_present()
             }
    );

    // FIXME: This is not very clear
    b.rule_3("for <duration> from now/today",
             b.reg(r#"for"#)?,
             duration_check!(),
             b.reg(r#"from (today|now)"#)?,
             |_, duration, grain| {
                 let start = helpers::cycle_nth(Grain::Second, 0)?;
                 let mut end = duration.value().in_present()?;
                 if grain.group(1) == "today" {
                     end = duration.value().in_present_day()?;
                 }
                 start.span_to(&end, false)
             }
    );

    b.rule_2("<duration> ago",
             duration_check!(),
             b.reg(r#"ago"#)?,
             |a, _| {
                 a.value().ago()
             }
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

// this is not a correct phrasing
//    b.rule_2("within <date>",
//             b.reg(r#"within"#)?,
//             datetime_check!(),
//             |_, a| Ok(a.value().clone().not_latent())
//    );

    Ok(())

}

// FIXME: rename "rules_datetime_with_cycle"
pub fn rules_datetime_with_nth_cycle(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {

    b.rule_2("this <cycle>",
             b.reg(r#"this|current|coming"#)?,
             cycle_check!(|cycle: &CycleValue| cycle.grain.is_greater_than_day()),
             |_, a| helpers::cycle_nth(a.value().grain, 0)
    );
    b.rule_2("last <cycle>",
             b.reg(r#"(?:the )?(?:last|past|previous)"#)?,
             cycle_check!(|cycle: &CycleValue| cycle.grain.is_greater_than_day()),
             |_, a| helpers::cycle_nth(a.value().grain, -1)
    );
    b.rule_2("next <cycle>",
             b.reg(r#"(?:the )?next|the following"#)?,
             cycle_check!(|cycle: &CycleValue| cycle.grain.is_greater_than_day()),
             |_, a| helpers::cycle_nth(a.value().grain, 1)
    );

    b.rule_4("last <day-of-week> of <datetime>",
             b.reg(r#"(?:the )?last"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"of"#)?,
             datetime_check!(),
             |_, a, _, b| {
                 a.value().last_of(b.value())
             }
    );
    b.rule_4("last <cycle> of <datetime>",
             b.reg(r#"(?:the )?last"#)?,
             cycle_check!(),
             b.reg(r#"of|in"#)?,
             datetime_check!(),
             |_, cycle, _, datetime| {
                 cycle.value().last_of(datetime.value())
             }
    );
    b.rule_4("nth <datetime> of <datetime>",
             ordinal_check!(), // the first
             datetime_check!(), // Thursday
             b.reg(r#"of|in"#)?, // of
             datetime_check!(), // march
             |ordinal, a, _, b| {
                 b.value().intersect(a.value())?.the_nth(ordinal.value().value - 1)
             }
    );
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
    b.rule_4("nth <datetime> after <datetime>",
             ordinal_check!(),
             datetime_check!(),
             b.reg(r#"after"#)?,
             datetime_check!(),
             |ordinal, a, _, b| {
                 a.value().the_nth_after(ordinal.value().value - 1, b.value())
             }
    );
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
    b.rule_4("the <cycle> after <datetime>",
             b.reg(r#"the"#)?,
             cycle_check!(),
             b.reg(r#"after"#)?,
             datetime_check!(),
             |_, cycle, _, datetime| helpers::cycle_nth_after(cycle.value().grain, 1, datetime.value())
    );
    b.rule_3("<cycle> after <datetime>",
             cycle_check!(),
             b.reg(r#"after"#)?,
             datetime_check!(),
             |cycle, _, datetime| helpers::cycle_nth_after(cycle.value().grain, 1, datetime.value())
    );
    b.rule_4("the <cycle> before <datetime>",
             b.reg(r#"the"#)?,
             cycle_check!(),
             b.reg(r#"before"#)?,
             datetime_check!(),
             |_, cycle, _, datetime| helpers::cycle_nth_after(cycle.value().grain, -1, datetime.value())
    );
    b.rule_3("<cycle> before <datetime>",
             cycle_check!(),
             b.reg(r#"before"#)?,
             datetime_check!(),
             |cycle, _, datetime| helpers::cycle_nth_after(cycle.value().grain, -1, datetime.value())
    );
    // TODO: resolution is not correct for times, i.e. rounds at grain
    b.rule_3("last n <cycle>",
             b.reg(r#"(?:for |in )?(?:the |these )?(?:last|past)"#)?,
             integer_check_by_range!(1, 9999),
             cycle_check!(),
             |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    // TODO: same as previous
    b.rule_3("next n <cycle>",
             b.reg(r#"(?:for |in )?(?:the |these )?next"#)?,
             integer_check_by_range!(1, 9999),
             cycle_check!(),
             |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    b.rule_4("<ordinal> <cycle> of <datetime>",
             ordinal_check_by_range!(1, 9999),
             cycle_check!(),
             b.reg(r#"of|in|from"#)?,
             datetime_check!(),
             |ordinal, cycle, _, datetime| helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value - 1, datetime.value())
    );
    b.rule_5("the <ordinal> <cycle> of <datetime>",
             b.reg(r#"the"#)?,
             ordinal_check_by_range!(1, 9999),
             cycle_check!(),
             b.reg(r#"of|in|from"#)?,
             datetime_check!(),
             |_, ordinal, cycle, _, datetime| helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value - 1, datetime.value())
    );
    b.rule_4("the <cycle> of <datetime>",
             b.reg(r#"the"#)?,
             cycle_check!(),
             b.reg(r#"of"#)?,
             datetime_check!(),
             |_, cycle, _, datetime| helpers::cycle_nth_after_not_immediate(cycle.value().grain, 0, datetime.value())
    );
    b.rule_4("<ordinal> <cycle> after <datetime>",
             ordinal_check_by_range!(1, 9999),
             cycle_check!(),
             b.reg(r#"after"#)?,
             datetime_check!(),
             |ordinal, cycle, _, datetime| helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value - 1, datetime.value())
    );
    b.rule_5("the <ordinal> <cycle> after <datetime>",
             b.reg(r#"the"#)?,
             ordinal_check_by_range!(1, 9999),
             cycle_check!(),
             b.reg(r#"after"#)?,
             datetime_check!(),
             |_, ordinal, cycle, _, datetime| helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value - 1, datetime.value())
    );

    Ok(())
}


/* DATETIME - CYCLE DEFINITIONS */
pub fn rules_cycle(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("second (cycle)",
                      b.reg(r#"seconds?"#)?,
                      |_| CycleValue::new(Grain::Second)
    );
    b.rule_1_terminal("minute (cycle)",
                      b.reg(r#"minutes?"#)?,
                      |_| CycleValue::new(Grain::Minute)
    );
    b.rule_1_terminal("hour (cycle)",
                      b.reg(r#"h(?:ou)?rs?"#)?,
                      |_| CycleValue::new(Grain::Hour)
    );
    b.rule_1_terminal("day (cycle)",
                      b.reg(r#"days?"#)?,
                      |_| CycleValue::new(Grain::Day)
    );
    b.rule_1_terminal("week (cycle)",
                      b.reg(r#"weeks?"#)?,
                      |_| CycleValue::new(Grain::Week)
    );
    b.rule_1_terminal("month (cycle)",
                      b.reg(r#"months?"#)?,
                      |_| CycleValue::new(Grain::Month)
    );
    b.rule_1_terminal("quarter (cycle)",
                      b.reg(r#"(?:quarter|qtr)s?"#)?,
                      |_| CycleValue::new(Grain::Quarter)
    );
    b.rule_1_terminal("year (cycle)",
                      b.reg(r#"y(?:ea)?rs?"#)?,
                      |_| CycleValue::new(Grain::Year)
    );

    Ok(())
}
