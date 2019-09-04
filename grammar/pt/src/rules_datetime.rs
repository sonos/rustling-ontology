use std::f32;

use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::dimension::Precision::*;
use rustling_ontology_values::helpers;
use rustling_ontology_moment::{Weekday, Grain};


pub fn rules_datetime(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("intersect",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |a, b| a.value().intersect(b.value())
    );

    b.rule_3("intersect by prepostion",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             b.reg(r#"d[eoa]"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |a, _, b| a.value().intersect(b.value())
    );

    b.rule_3("two time tokens separated by \",\"",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             b.reg(r#","#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |a, _, b| a.value().intersect(b.value())
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"(segunda|2ª)(?:[- ]feira| f.)?"#)?,
                      |_| helpers::day_of_week(Weekday::Mon)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"(ter[cç]a|3ª)(?:[- ]feira| f.)?"#)?,
                      |_| helpers::day_of_week(Weekday::Tue)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"(quarta|4ª)(?:[- ]feira| f.)?"#)?,
                      |_| helpers::day_of_week(Weekday::Wed)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"(quinta|5ª)(?:[- ]feira| f.)?"#)?,
                      |_| helpers::day_of_week(Weekday::Thu)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"(sexta|6ª)(?:[- ]feira| f.)?"#)?,
                      |_| helpers::day_of_week(Weekday::Fri)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"s[aá]bado|s[aá]b."#)?,
                      |_| helpers::day_of_week(Weekday::Sat)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"domingo|dom."#)?,
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
                      b.reg(r#"neste exato momento"#)?,
                      |_| helpers::cycle_nth(Grain::Second, 0)
    );
    b.rule_1_terminal("now",
                      b.reg(r#"agora(?: mesmo)?|neste momento"#)?,
                      |_| helpers::cycle_nth(Grain::Second, 0)
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
                      b.reg(r#"dia depois de amanhã"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 2)
    );
    // Date
    b.rule_1_terminal("the day after tomorrow",
                      b.reg(r#"depois de amanhã"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 2)
    );
    // Date
    b.rule_1_terminal("the day before yesterday",
                      b.reg(r#"anteontem"#)?,
                      |_| helpers::cycle_nth(Grain::Day, -2)
    );

    // Date
//    b.rule_2("this <day-of-week>", //assumed to be in the future
//             b.reg(r#"n?est[ea]"#)?,
//             datetime_check!(form!(Form::DayOfWeek{..})),
//             |_, time| time.value().the_nth_not_immediate(0)
//    );
    // DateTime
    b.rule_2("this <datetime>",
             b.reg(r#"n?est[ea]|(?:n?[oa])? próxim[oa]"#)?,
             datetime_check!(),
             |_, time| time.value().the_nth(0)
    );
    b.rule_2("month 'preposition' <named-month>",
             b.reg(r#"mês de"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, a| Ok(a.value().clone())
    );

    b.rule_2("in <datetime>",
             b.reg(r#"ao|durante|em|para(?: [oa])?|n?[oa]s?"#)?,
             datetime_check!(),
             |_, a| Ok(a.value().clone())
    );
    b.rule_2("in approximately <part-of-day> (approximation)",
             b.reg(r#"pela"#)?,
             datetime_check!(|time: &DatetimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
             |_, a| Ok(a.value().clone())
    );
    // Date-period
    b.rule_2("beginning <named-month>(interval)",
             b.reg(r#"(começo|início) d[eo](?: mês de)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, month| {
                 let start = month.value().intersect(&helpers::day_of_month(1)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(5)?)?;
                 start.span_to(&end, true)
             }
    );
    // Date-period
    b.rule_1_terminal("end of this month",
                      b.reg(r#"(fim|final) (do|deste) mês"#)?,
                      |_| {
                          let month = helpers::cycle_nth(Grain::Month, 1)?;
                        Ok(helpers::cycle_nth_after(Grain::Day, -10, &month)?
                            .span_to(&month, false)?
                            .latent()
                            .form(Form::PartOfMonth))
                    }
    );
    // Date-period
    b.rule_2("end of <named-month>(interval)",
             b.reg(r#"(?:fim|final) d[eo](?: mês de)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, month| {
                 let start = month.value().intersect(&helpers::day_of_month(25)?)?;
                 let end = helpers::cycle(Grain::Day)?.last_of(month.value())?;
                 start.span_to(&end, true)
             }
    );
    // Date period
    b.rule_2("middle of <named-month>",
             b.reg(r#"(?:em )?meados de|(?:n?a )?metade(?: do mês)? de"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, month| {
                 let start = month.value().intersect(&helpers::day_of_month(10)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(19)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_2("fist half of <named-month>(interval)",
             b.reg(r#"primeira quinzena(?: do mês)? de"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, month| {
                 let start = month.value().intersect(&helpers::day_of_month(1)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(14)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_2("second half of <named-month>(interval)",
             b.reg(r#"segunda quinzena(?: do mês)? de"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, month| {
                 let start = month.value().intersect(&helpers::day_of_month(15)?)?;
                 let end = helpers::cycle(Grain::Day)?.last_of(month.value())?;
                 start.span_to(&end, true)
             }
    );
    // Date period
    b.rule_2("next <named-month>",
             b.reg(r#"próximo mês de"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, time| time.value().the_nth_not_immediate(0)
    );
    // Date period
    b.rule_2("last <named-month>",
             b.reg(r#"último mês de"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, time| time.value().the_nth(-1)
    );
    // Date
    b.rule_2("next <named-day>",
             b.reg(r#"próxim[oa]|nest[ae]"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             |_, time| time.value().the_nth_not_immediate(0)
    );
    // Date
    b.rule_2("<named-day> next",
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"(?:da semana )?que vem"#)?,
             |time, _| time.value().the_nth_not_immediate(0)
    );
    // Date
    b.rule_2("<named-day> next",
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"da próxima semana"#)?,
             |time, _| time.value().the_nth_not_immediate(0)
    );
    // Date
    b.rule_2("for next <named-day>",
             b.reg(r#"próximo"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             |_, time| time.value().the_nth_not_immediate(0)
    );
    // Date
    b.rule_2("last <named-day>",
             b.reg(r#"(?:nest[ea] )?últim[ao]"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             |_, time| time.value().the_nth(-1)
    );
    // Date
    b.rule_2("<named-day> last",
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"atr[aá]s|antes|passad[oa]|anteriore"#)?,
             |time, _| time.value().the_nth(-1)
    );
    // Date
    b.rule_2("<named-day> of last week",
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"da semana passada"#)?,
             |time, _| time.value().the_nth(-1)
    );
    // Date-Period
    b.rule_1("year",
             integer_check_by_range!(1000, 2100),
             |integer| {
                 helpers::year(integer.value().value as i32)
             }
    );
    // Date-Period
//    b.rule_1("year (latent)",
//             integer_check_by_range!(-1000, 999),
//             |integer| {
//                 Ok(helpers::year(integer.value().value as i32)?.latent())
//             }
//    );
    // Date-Period
    b.rule_1("year (latent)",
             integer_check_by_range!(2101, 2200),
             |integer| {
                 Ok(helpers::year(integer.value().value as i32)?.latent())
             }
    );
    // Date-Period
    b.rule_2("in year",
             b.reg(r#"em|[dn]?o ano(?: de)?"#)?,
             integer_check_by_range!(1000, 2100),
             |_, integer| helpers::year(integer.value().value as i32)
    );

    b.rule_3("<datetime> de <year>",
             datetime_check!(|time: &DatetimeValue| !time.latent),
             b.reg(r#"d[eo]"#)?,
             datetime_check!(form!(Form::Year(_))),
             |a, _, b| a.value().intersect(b.value())
    );
    // Date
    b.rule_2("o ordinal (<day-of-month>)",
             b.reg(r#"o"#)?,
             ordinal_check_by_range!(1, 31),
             |_, integer| Ok(helpers::day_of_month(integer.value().value as u32)?)
    );
    // Date
    b.rule_2("dia + integer",
            b.reg(r#"dia"#)?,
            integer_check_by_range!(1, 31),
            |_, integer| Ok(helpers::day_of_month(integer.value().value as u32)?)
    );
    // Date
    b.rule_3("<day-of-week> day <integer>",
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"dia"#)?,
             integer_check_by_range!(1, 31),
             |dow, _, integer| dow.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    // Date
    b.rule_3("<integer> de <named-month> (<day-of-month>)",
             integer_check_by_range!(1, 31),
             b.reg(r#"de"#)?,
             datetime_check!(form!(Form::Month(_))),
             |integer, _, month| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    // Date
    b.rule_3("<ordinal> de <named-month> (<day-of-month>)",
             ordinal_check_by_range!(1, 31),
             b.reg(r#"(?:dia )?de"#)?,
             datetime_check!(form!(Form::Month(_))),
             |integer, _, month| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    // Date
    b.rule_4("dia <integer> de <named-month> (<day-of-month>)",
             b.reg(r#"dia"#)?,
             integer_check_by_range!(1, 31),
             b.reg(r#"de"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, integer, _, month| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    // Date
    b.rule_4("dia <ordinal> de <named-month> (<day-of-month>)",
             b.reg(r#"dia"#)?,
             ordinal_check_by_range!(1, 31),
             b.reg(r#"de"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, integer, _, month| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    // Date
    b.rule_4("<day-of-week> <ordinal> de <named-month> (<day-of-month>)",
             datetime_check!(form!(Form::DayOfWeek{..})),
             ordinal_check_by_range!(1, 31),
             b.reg(r#"de"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, integer, _, month| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    // Date
    b.rule_4("<day-of-week> <integer> de <named-month> (<day-of-month>)",
             datetime_check!(form!(Form::DayOfWeek{..})),
             integer_check_by_range!(1, 31),
             b.reg(r#"de"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, integer, _, month| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    // Date
    b.rule_5("<day-of-week> dia <integer> de <named-month> (<day-of-month>)",
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"dia"#)?,
             integer_check_by_range!(1, 31),
             b.reg(r#"de"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, _, integer, _, month| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    // Date period
    b.rule_1_terminal("beginning of week",
                      b.reg(r#"(início|começo) (da|desta) semana"#)?,
                      |_| helpers::day_of_week(Weekday::Mon)
                          ?.span_to(&helpers::day_of_week(Weekday::Tue)?, false)
    );
    // Date period
    b.rule_1_terminal("middle of week",
                      b.reg(r#"(meio|metade) da semana"#)?,
                      |_| helpers::day_of_week(Weekday::Wed)
                          ?.span_to(&helpers::day_of_week(Weekday::Thu)?, false)
    );
    // Date period
    b.rule_1_terminal("end of week (not weekend)",
                      b.reg(r#"(fim|final) d[ae] semana"#)?,
                      |_| helpers::day_of_week(Weekday::Thu)
                          ?.span_to(&helpers::day_of_week(Weekday::Sun)?, false)
    );
    // Date period
    b.rule_1_terminal("week-end",
                      b.reg(r#"(fim|final) de semana"#)?,
                      |_| {
                          let friday = helpers::day_of_week(Weekday::Fri)?
                              .intersect(&helpers::hour(18, false)?)?;
                          let monday = helpers::day_of_week(Weekday::Mon)?
                              .intersect(&helpers::hour(0, false)?)?;
                          friday.span_to(&monday, false)
                      }
    );
    b.rule_2("dernier week-end de <time>",
             b.reg(r#"último (fim|final) de semana(?: de| do)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, time| {
                 let week_day_start = helpers::day_of_week(Weekday::Fri)?.intersect(&helpers::hour(18, false)?)?;
                 let week_day_end = helpers::day_of_week(Weekday::Mon)?.intersect(&helpers::hour(0, false)?)?;
                 let week_day = week_day_start.span_to(&week_day_end, false)?;
                 week_day.last_of(time.value())
             }
    );
    b.rule_3("<ordinal> week-end of <time>",
             ordinal_check!(),
             b.reg(r#"(fim|final) de semana(?: de| do)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |ordinal, _, time| {
                 let week_day_start = helpers::day_of_week(Weekday::Fri)?.intersect(&helpers::hour(18, false)?)?;
                 let week_day_end = helpers::day_of_week(Weekday::Mon)?.intersect(&helpers::hour(0, false)?)?;
                 let week_day = week_day_start.span_to(&week_day_end, false)?;
                 let week_ends_of_time = time.value().intersect(&week_day)?;
                 week_ends_of_time.the_nth(ordinal.value().value - 1)
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
                      b.reg(r#"meio[- ]dia"#)?,
                      |_| helpers::hour(12, false)
    );
    // Time
    b.rule_1_terminal("midnight",
                      b.reg(r#"meia[- ]noite"#)?,
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
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"horas?"#)?,
             |time, _| Ok(time.value().clone().not_latent())
    );
    // Time
    b.rule_2("<time-of-day> am|pm",
             datetime_check!(form!(Form::TimeOfDay(_))),
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
                      b.reg(r#"manh[aã]|madrugada"#)?,
                      |_| Ok(helpers::hour(4, false)?.span_to(&helpers::hour(12, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Morning))
                          .latent())
    );
    // Time period
    b.rule_1_terminal("beginning of morning",
                      b.reg(r#"(começo|início) da manh[aã]"#)?,
                      |_| Ok(helpers::hour(4, false)?
                          .span_to(&helpers::hour(9, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Morning)))
    );
    // Time period
    b.rule_1_terminal("beginning of morning",
                      b.reg(r#"logo (à|de|no início da) manh[aã]"#)?,
                      |_| Ok(helpers::hour(4, false)?
                          .span_to(&helpers::hour(9, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Morning)))
    );
    b.rule_1_terminal("beginning of morning",
                      b.reg(r#"primeiras horas da manh[aã]"#)?,
                      |_| Ok(helpers::hour(4, false)?
                          .span_to(&helpers::hour(9, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Morning)))
    );
    // Time period
    b.rule_1_terminal("end of morning",
                      b.reg(r#"(fim|final) da manh[aã]"#)?,
                      |_| Ok(helpers::hour(10, false)?
                          .span_to(&helpers::hour(12, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Morning)))
    );
    // Time period
    b.rule_1_terminal("end of morning",
                      b.reg(r#"nas últimas horas da manh[aã]"#)?,
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
    b.rule_1_terminal("beginning of afternoon",
        b.reg(r#"(começo|início) da tarde"#)?,
        |_| {
            Ok(helpers::hour(12, false)?
                    .span_to(&helpers::hour(15, false)?, false)?
                    .latent()
                    .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
        }
    );
    // Time period
    b.rule_1_terminal("beginning of afternoon",
        b.reg(r#"logo (à|de|no início da) tarde"#)?,
        |_| {
            Ok(helpers::hour(12, false)?
                    .span_to(&helpers::hour(15, false)?, false)?
                    .latent()
                    .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
        }
    );
    // Time period
    b.rule_1_terminal("middle afternoon",
                      b.reg(r#"meio da tarde"#)?,
                      |_| {
                          Ok(helpers::hour(15, false)?
                              .span_to(&helpers::hour(17, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
                      }
    );
    // Time period
    b.rule_1_terminal("middle afternoon",
                      b.reg(r#"(?:em )?plena tarde"#)?,
                      |_| {
                          Ok(helpers::hour(15, false)?
                              .span_to(&helpers::hour(17, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
                      }
    );
    // Time period nas últimas horas
    b.rule_1_terminal("end of afternoon",
                      b.reg(r#"(fim|final) da tarde"#)?,
                      |_| {
                          Ok(helpers::hour(15, false)?
                              .span_to(&helpers::hour(17, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
                      }
    );
    b.rule_1_terminal("end of afternoon",
                      b.reg(r#"nas últimas horas da tarde"#)?,
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
                      b.reg(r#"(começo|início) da noite"#)?,
                      |_| {
                          Ok(helpers::hour(18, false)?
                              .span_to(&helpers::hour(21, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );
    // Time period
    b.rule_1_terminal("beginning of evening",
                      b.reg(r#"logo (à|de|no início da) noite"#)?,
                      |_| {
                          Ok(helpers::hour(18, false)?
                              .span_to(&helpers::hour(21, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );
    // Time period
    b.rule_1_terminal("end of evening",
                      b.reg(r#"(fim|final|tarde) da noite"#)?,
                      |_| {
                          Ok(helpers::hour(21, false)?
                              .span_to(&helpers::hour(0, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );
    // Time period
    b.rule_1_terminal("end of evening",
                      b.reg(r#"(fim|final|tarde) da noite"#)?,
                      |_| {
                          Ok(helpers::hour(21, false)?
                              .span_to(&helpers::hour(0, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );
    // Time period
    b.rule_1_terminal("end of evening",
                      b.reg(r#"nas últimas horas da noite"#)?,
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
                      b.reg(r#"(?:d?a )?hora doalmoço"#)?,
                      |_| Ok(helpers::hour(12, false)?
                          .span_to(&helpers::hour(14, false)?, false)?
                          .latent()
                          .form(Form::Meal))
    );
    // Time period
    b.rule_2("this <part-of-day>",
             b.reg(r#"d?esta|d[ea]"#)?,
             datetime_check!(|time: &DatetimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
             |_, pod| Ok(helpers::cycle_nth(Grain::Day, 0)?
                 .intersect(pod.value())?
                 .form(pod.value().form.clone()))
    );
    // Time
    b.rule_2("<time-of-day> <part-of-day>",
             datetime_check!(|time: &DatetimeValue| excluding_form!(Form::Year(_))(time) && excluding_form!(Form::Month(_))(time)),
             datetime_check!(|time: &DatetimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
             |a, b| a.value().intersect(b.value())
    );
    // Time period
    b.rule_3("<time> preposition <part-of-day>",
            datetime_check!(|time: &DatetimeValue| excluding_form!(Form::Year(_))(time) && excluding_form!(Form::Month(_))(time)),
            b.reg(r#"à|d[ea]"#)?,
            datetime_check!(|time: &DatetimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
            |time, _, part_of_day| time.value().intersect(part_of_day.value())
    );
    // Time
    b.rule_2("<dim time> da manha",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:d[ea]|à) manh[aã]"#)?,
             |a, _| {
                 let period = helpers::hour(0, false)?
                     .span_to(&helpers::hour(12, false)?, false)?;
                 a.value().intersect(&period)
             }
    );
    // Time
    b.rule_2("<dim time> da tarde",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:d[ea]|à) tarde"#)?,
             |a, _| {
                 let period = helpers::hour(15, false)?
                     .span_to(&helpers::hour(19, false)?, false)?;
                 a.value().intersect(&period)
             }
    );
    // Time
    b.rule_2("<dim time> da noite",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:d[ea]|à) noite"#)?,
             |a, _| {
                 let period = helpers::hour(16, false)?
                     .span_to(&helpers::hour(0, false)?, false)?;
                 a.value().intersect(&period)
             }
    );
    // Time period
    b.rule_3("<part-of-day> de <time>",
             datetime_check!(|time: &DatetimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
             b.reg(r#"d?esta|d[ea]"#)?,
             datetime_check!(|time: &DatetimeValue| excluding_form!(Form::Year(_))(time) && excluding_form!(Form::Month(_))(time)),
             |part_of_day, _, time| time.value().intersect(part_of_day.value())
    );
    // Time period
//    b.rule_3("<time-of-day> de <time>",
//            datetime_check!(excluding_form!(Form::PartOfDay(_))),
//            b.reg(r#"d?esta|d[ea]"#)?,
//            datetime_check!(|time: &TimeValue| excluding_form!(Form::Year(_))(time) && excluding_form!(Form::Month(_))(time)),
//            |part_of_day, _, time| time.value().intersect(part_of_day.value())
//    );
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
             b.reg(r#"minutos?|min"#)?,
             |integer, _| Ok(RelativeMinuteValue(integer.value().value as i32))
    );
    // Time (ambiguity with Duration) ex: seis horas e vinte minutos
    b.rule_3("<hour-of-day> and <relative minutes>",
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))),
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
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, tod| Ok(tod.value().clone().not_latent())
    );
    // Time
    b.rule_2("around <time-of-day>",
             b.reg(r#"lá (?:pelas?|pelo)|por volta d(?:e|as)"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, a| Ok(a.value().clone().not_latent())
    );
    // Time
    b.rule_2("<time-of-day> approx",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"mais ou menos"#)?,
             |a, _| Ok(a.value().clone().not_latent())
    );
    // Time
    b.rule_4("<integer> (as relative minutes) para o <hour-of-day>",
             b.reg(r#"às|aos"#)?,
             relative_minute_check!(),
             b.reg(r#"para (?:o|as)?"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             //datetime_check!(|time: &DatetimeValue| !time.latent && form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))(time)),
             |_, minutes, _, time| helpers::hour_relative_minute(
                 time.value().form_time_of_day()?.full_hour(),
                 -1 * minutes.value().0,
                 time.value().form_time_of_day()?.is_12_clock()
             )
    );
    // Time
    b.rule_3("<integer> (as relative minutes) para o <hour-of-day>",
             relative_minute_check!(),
             b.reg(r#"para (?:o|as)?"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             //datetime_check!(|time: &DatetimeValue| !time.latent && form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))(time)),
             |minutes, _, time| helpers::hour_relative_minute(
                 time.value().form_time_of_day()?.full_hour(),
                 -1 * minutes.value().0,
                 time.value().form_time_of_day()?.is_12_clock()
             )
    );
    // Time period
    b.rule_1_terminal("beginning of day",
                      b.reg(r#"(começo|início) do dia"#)?,
                      |_| {
                          Ok(helpers::hour(6, false)?
                              .span_to(&helpers::hour(10, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Morning)))
                      }
    );
    // Time period
    b.rule_1_terminal("middle of day",
                      b.reg(r#"(metade|meio) do dia"#)?,
                      |_| {
                          Ok(helpers::hour(11, false)?
                              .span_to(&helpers::hour(16, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::None)))
                      }
    );
    // Time period
    b.rule_1_terminal("end of day",
                      b.reg(r#"(fim|final|acabar) d?o dia"#)?,
                      |_| {
                          Ok(helpers::hour(17, false)?
                              .span_to(&helpers::hour(21, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );
    // Time period
    b.rule_1_terminal("end of day",
                      b.reg(r#"nas últimas horas d?o dia"#)?,
                      |_| {
                          Ok(helpers::hour(17, false)?
                              .span_to(&helpers::hour(21, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );
    // Time period
    b.rule_4("between <datetime> and <datetime> (interval)",
             b.reg(r#"entre(?: as?| o)?"#)?,
             datetime_check!(|time: &DatetimeValue| excluding_form!(Form::Year(_))(time)),
             b.reg(r#"e(?: as?| a?o)?"#)?,
             datetime_check!(|time: &DatetimeValue| excluding_form!(Form::Year(_))(time)),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );
    // Time period
    b.rule_4("between <datetime> and <datetime> (interval)",
             b.reg(r#"entre(?: as?| o)?"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"e(?: as?| a?o)?"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );


    // Time period
    b.rule_4("between <datetime> and <datetime> (interval)",
             b.reg(r#"entre(?: as?| a?o)?"#)?,
             datetime_check!(|time: &DatetimeValue| excluding_form!(Form::Year(_))(time)),
             b.reg(r#"e"#)?,
             datetime_check!(|time: &DatetimeValue| excluding_form!(Form::Year(_))(time)),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );
    // Time period
    b.rule_4("between <time-of-day> e as <time-of-day> (interval)",
             b.reg(r#"entre(?: as?| o)?"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"e as"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );
    // Time period
    b.rule_4("from <time-of-day> to <time-of-day> (interval)",
             b.reg(r#"das?"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"às?"#)?,
             datetime_check!(),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );
    // Time period
    b.rule_4("from <time-of-day> to <time-of-day> (interval)",
             b.reg(r#"d[oe]"#)?,
             datetime_check!(),
             b.reg(r#"às?"#)?,
             datetime_check!(),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );
    // Time period
    b.rule_2("from <time-of-day>",
             b.reg(r#"a partir( das?| de| desta| do)"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, time| Ok(time.value().clone().mark_after_start())
    );
    // Time period
    b.rule_2("from <time-of-day>",
             b.reg(r#"do início do|desde as?"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, time| Ok(time.value().clone().mark_after_start())
    );
    // Time period
    b.rule_2("after <date-time>",
             b.reg(r#"(desde|a partir|depois)( desta| das?| de| d?o)"#)?,
             datetime_check!(),
             |_, time| Ok(time.value().clone().mark_after_start())
    );
    b.rule_2("after <date-time>",
             b.reg(r#"depois"#)?,
             datetime_check!(),
             |_, time| Ok(time.value().clone().mark_after_start())
    );
    // Time period
    b.rule_2("from <date-time>",
             b.reg(r#"desde(?: as?)?"#)?,
             datetime_check!(),
             |_, time| Ok(time.value().clone().mark_after_start())
    );
    // Time period
    b.rule_2("from <time-of-day>",
             b.reg(r#"a partir(?: d[oe])?|desde(?: as?)?"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, time| Ok(time.value().clone().mark_after_start())
    );
    // Time period
    b.rule_3("from <time-of-day> on",
             b.reg(r#"do|de|das"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"em diante"#)?,
             |_, time, _| Ok(time.value().clone().mark_after_start())
    );
    // Time period
    b.rule_3("from <part-of-day> on",
             b.reg(r#"do|de|das"#)?,
             datetime_check!(|time: &DatetimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
             b.reg(r#"em diante"#)?,
             |_, pod, _| Ok(pod.value().clone().mark_after_start())
    );
    // Time period
    b.rule_3("from <datetime> on",
             b.reg(r#"(?:do|de|das)(?: dia)?"#)?,
             datetime_check!(),
             b.reg(r#"em diante"#)?,
             |_, pod, _| Ok(pod.value().clone().mark_after_start())
    );
    // Time period
//    b.rule_4("from <date-time> to <date-time> (interval)",
//             b.reg(r#"do|de|das"#)?,
//             datetime_check!(),
//             b.reg(r#"[àa]s?|ao?|[aà]té"#)?,
//             datetime_check!(),
//             |_, a, _, b| a.value().span_to(b.value(), false)
//    );
    // Date period
    b.rule_6("from dd to dd <month>(interval)",
             b.reg(r#"do|de|das"#)?,
             integer_check_by_range!(1, 31),
             b.reg(r#"[àa]s?|ao?|[aà]té"#)?,
             integer_check_by_range!(1, 31),
             b.reg(r#"de"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, a, _, b, _,month| {
                 let start = month.value().intersect(&helpers::day_of_month(a.value().value as u32)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(b.value().value as u32)?)?;
                 start.span_to(&end, true)
             }
    );
    // Time period
//    b.rule_4("from <date-time> to <date-time> (interval)",
//             b.reg(r#"a partir (d[ae]|das|desta)"#)?,
//             datetime_check!(),
//             b.reg(r#"para as"#)?,
//             datetime_check!(),
//             |_, a, _, b| a.value().span_to(b.value(), false)
//    );
    // Time period
    b.rule_2("before <datetime>",
             b.reg(r#"antes d[ao]s?"#)?,
             datetime_check!(),
             |_, time| Ok(time.value().clone().mark_before_end())
    );
    // Time period
    b.rule_2("before <datetime>",
             b.reg(r#"[àa]té(?: as?| o| de)?"#)?,
             datetime_check!(),
             |_, time| Ok(time.value().clone().mark_before_end())
    );
    // Time period
    b.rule_2("before <part-of-day>",
             b.reg(r#"àté(?: as| o| de)?"#)?,
             datetime_check!(|time: &DatetimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
             |_, pod| Ok(pod.value().clone().mark_before_end())
    );
    // Time period
    b.rule_2("before <part-of-day>",
             b.reg(r#"àté"#)?,
             datetime_check!(|time: &DatetimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
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
    b.rule_1_terminal("hh(h) (time-of-day)",
                      b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[h\.]"#)?,
                      |text_match| {
                          let hour: u32 = text_match.group(1).parse()?;
                          helpers::hour(hour, hour < 12)
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
                      b.reg(r#"(?:no dia )?(3[01]|[12]\d|0?[1-9])[-/.](0?[1-9]|1[0-2])[-/.](\d{2,4})"#)?,
                      |text_match| helpers::year_month_day(
                          text_match.group(3).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(1).parse()?
                      )
    );
    // Date
    b.rule_1_terminal("yyyy-mm-dd",
                      b.reg(r#"(?:no dia )?(\d{2,4})-(0?[1-9]|1[0-2])-(3[01]|[12]\d|0?[1-9])"#)?,
                      |text_match| helpers::year_month_day(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(3).parse()?
                      )
    );
    // Date
    b.rule_1_terminal("dd[/-]mm",
                      b.reg(r#"(?:no dia )?(3[01]|[12]\d|0?[1-9])[-/](0?[1-9]|1[0-2])"#)?,
                      |text_match| helpers::month_day(
                          text_match.group(2).parse()?,
                          text_match.group(1).parse()?
                      )
    );
    // Date time complement
//    b.rule_3("<time> <part-of-day>",
//            datetime_check!(|time: &DatetimeValue| excluding_form!(Form::Year(_))(time) && excluding_form!(Form::Month(_))(time)),
//            b.reg(r#"à|de|da"#)?,
//            datetime_check!(|time: &DatetimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
//            |time, _, part_of_day| time.value().intersect(part_of_day.value())
//    );
    // Date time complement
//    b.rule_3("<datetime> <part-of-day>",
//            datetime_check!(excluding_form!(Form::TimeOfDay(_))),
//            b.reg(r#"à|d[ea]"#)?,
//            datetime_check!(|time: &TimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
//            |time, _, part_of_day| time.value().intersect(part_of_day.value())
//    );
    Ok(())
}


pub fn rules_datetime_with_duration(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    Ok(())
}


pub fn rules_datetime_with_cycle(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    // CURRENT
    b.rule_2("this <cycle>",
             b.reg(r#"d?est[ea]|a"#)?,
             cycle_check!(),
             |_, cycle| helpers::cycle_nth(cycle.value().grain, 0)
    );
    // FUTUR
    b.rule_2("next <cycle> ",
             b.reg(r#"próxim[oa]"#)?,
             cycle_check!(),
             |_, cycle| helpers::cycle_nth(cycle.value().grain, 1)
    );
    // FUTUR
    b.rule_2("<cycle> next",
             cycle_check!(),
             b.reg(r#"que vem|depois"#)?,
             |cycle, _| helpers::cycle_nth(cycle.value().grain, 1)
    );
    // PAST
    b.rule_2("last <cycle> ",
             b.reg(r#"últim[oa]"#)?,
             cycle_check!(),
             |_, cycle| helpers::cycle_nth(cycle.value().grain, -1)
    );
    // PAST
    b.rule_2("<cycle> last",
             cycle_check!(),
             b.reg(r#"atr[aá]s|antes|passad[oa]|anteriore"#)?,
             |cycle, _| helpers::cycle_nth(cycle.value().grain, -1)
    );
    // FUTUR (Date|Time) period
    b.rule_3("next n <cycle>",
            b.reg(r#"próxim[ao]s"#)?,
            integer_check_by_range!(2, 9999),
            cycle_check!(),
            |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    // FUTUR (Date|Time) period
    b.rule_3("n next <cycle>",
             integer_check_by_range!(2, 9999),
             b.reg(r#"próxim[ao]s"#)?,
             cycle_check!(),
             |integer, _, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    // PAST (Date|Time) period
    b.rule_3("n <cycle> last ",
            integer_check_by_range!(2, 9999),
            cycle_check!(),
            b.reg(r#"passad[oa]s|anteriores"#)?,
            |integer, cycle, _| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    // PAST (Date|Time) period
    b.rule_3("last n <cycle>",
            b.reg(r#"últim[ao]s"#)?,
            integer_check_by_range!(2, 9999),
            cycle_check!(),
            |_, integer,cycle| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    // PAST (Date|Time) period
    b.rule_3("n last <cycle>",
             integer_check_by_range!(2, 9999),
             b.reg(r#"últim[ao]s"#)?,
             cycle_check!(),
             |integer, _, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    Ok(())
}


/* DATETIME - CYCLE DEFINITIONS */
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
    Ok(())
}
