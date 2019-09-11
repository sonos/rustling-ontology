use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::helpers;
use rustling_ontology_values::dimension::Precision::*;
use rustling_ontology_moment::{Weekday, Grain};


pub fn rules_datetime(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("intersect",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |a, b| a.value().intersect(b.value())
    );
    b.rule_3("intersect by `de`",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             b.reg(r#"del?"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |a, _, b| a.value().intersect(b.value())
    );
    b.rule_3("two time tokens separated by \",\"",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             b.reg(r#","#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |a, _, b| a.value().intersect(b.value())
    );
//    // Not latent intersects
//    b.rule_3("intersect <date> at <datetime>",
//             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
//             b.reg(r#"de"#)?,
//             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
//             |a, _, b| a.value().intersect(b.value())
//    );
    // Add constraints? en + year, en + celebration, para + Part-of-day, por + part-of-day, etc?
    b.rule_2("for <datetime>",
             b.reg(r#"para|por|en|durante"#)?,
             datetime_check!(|datetime: &DatetimeValue| !!!datetime.latent),
             |_, a| Ok(a.value().clone())
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"lunes|lun?\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Mon)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"martes|mar?\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Tue)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"mi[eé]\.?(?:rcoles)?|mx|mier?\."#)?,
                      |_| helpers::day_of_week(Weekday::Wed)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"jueves|jue|jue\."#)?,
                      |_| helpers::day_of_week(Weekday::Thu)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"viernes|vie|vie\."#)?,
                      |_| helpers::day_of_week(Weekday::Fri)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"s[áa]bado|s(?:á|a)b\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Sat)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"domingo|dom\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Sun)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"enero|ene\.?"#)?,
                      |_| helpers::month(1)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"febrero|feb\.?"#)?,
                      |_| helpers::month(2)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"marzo|mar\.?"#)?,
                      |_| helpers::month(3)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"abril|abr\.?"#)?,
                      |_| helpers::month(4)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"mayo?\.?"#)?,
                      |_| helpers::month(5)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"junio|jun\.?"#)?,
                      |_| helpers::month(6)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"julio|jul\.?"#)?,
                      |_| helpers::month(7)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"agosto|ago\.?"#)?,
                      |_| helpers::month(8)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"septiembre|sept?\.?"#)?,
                      |_| helpers::month(9)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"octubre|oct\.?"#)?,
                      |_| helpers::month(10)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"noviembre|nov\.?"#)?,
                      |_| helpers::month(11)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"diciembre|dic\.?"#)?,
                      |_| helpers::month(12)
    );
    b.rule_1_terminal("right now",
                      b.reg(r#"(?:justo )?ahor(?:it)?a(?: mismo)?|ya|en\s?seguida|cuanto antes|en este preciso (?:istante|momento)|inmediatamente|a estas horas|actualmente"#)?,
                      |_| helpers::cycle_nth(Grain::Second, 0)
    );
    b.rule_1_terminal("now / today",
                      b.reg(r#"hoy|(?:en este momento)|actualmente|en la actualidad|de momento"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 0)
    );
    b.rule_1_terminal("tomorrow",
                      b.reg(r#"ma[nñ]ana|el d[iíì]a (?:siguiente|(?:de )?despu[eéè]s)"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 1)
    );
    b.rule_1_terminal("yesterday",
                      b.reg(r#"ayer|el d[iíì]a (?:anterior|de antes)|la v[iíì]spera"#)?,
                      |_| helpers::cycle_nth(Grain::Day, -1)
    );
    b.rule_1_terminal("the day after tomorrow",
                      b.reg(r#"pasados? ma[nñ]ana"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 2)
    );
    b.rule_1_terminal("the day before yesterday",
                      b.reg(r#"anteayer|antes de (?:ayer|anoche)|antier"#)?,
                      |_| helpers::cycle_nth(Grain::Day, -2)
    );
    b.rule_2("this <day-of-week>", //assumed to be in the future
             b.reg(r#"este"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             |_, datetime| datetime.value().the_nth_not_immediate(0)
    );
    b.rule_2("this <month>", //assumed to be in the future
             b.reg(r#"este mes de"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, datetime| datetime.value().the_nth_not_immediate(0)
    );
    b.rule_2("this <datetime>",
             b.reg(r#"est[ea]"#)?,
             datetime_check!(),
             |_, datetime| datetime.value().the_nth(0)
    );
    b.rule_2("during <date>",
             b.reg(r#"durante"#)?,
             datetime_check!(|datetime: &DatetimeValue| datetime.form.is_day()),
             |_, datetime| Ok(datetime.value().clone())
    );
    b.rule_2("in <named-month>",
             b.reg(r#"(?:durante|en)(?: el mes de)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, a| Ok(a.value().clone())
    );
    b.rule_2("beginning <named-month>(interval)",
             b.reg(r#"a(?: principios|l comienzo) de(?:l mes de)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, month| {
                 let start = month.value().intersect(&helpers::day_of_month(1)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(5)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_1_terminal("end of month",
                      b.reg(r#"a fin(?:ales)? de mes"#)?,
                      |_| {
                          let month = helpers::cycle_nth(Grain::Month, 1)?;
                        Ok(helpers::cycle_nth_after(Grain::Day, -10, &month)?
                            .span_to(&month, false)?
                            .latent()
                            .form(Form::PartOfMonth))
                    }
    );
    b.rule_2("end <named-month>(interval)",
             b.reg(r#"a fin(?:ales)? de(?:l mes de)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, month| {
                 let start = month.value().intersect(&helpers::day_of_month(25)?)?;
                 let end = helpers::cycle(Grain::Day)?.last_of(month.value())?;
                 start.span_to(&end, true)
             }
    );
    b.rule_2("beginning <named-month>(interval)",
             b.reg(r#"a(?: principios|l comienzo) de(?:l mes de)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, month| {
                 let start = month.value().intersect(&helpers::day_of_month(1)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(5)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_2("first 15 days of <named-month>(interval)",
             b.reg(r#"primera quincena de(?:l mes de)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, month| {
                 let start = month.value().intersect(&helpers::day_of_month(1)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(14)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_2("second 15 days of <named-month>(interval)",
             b.reg(r#"segunda quincena de(?:l mes de)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, month| {
                 let start = month.value().intersect(&helpers::day_of_month(15)?)?;
                 let end = helpers::cycle(Grain::Day)?.last_of(month.value())?;
                 start.span_to(&end, true)
             }
    );
    b.rule_2("next <named-month|named-day>",
             b.reg(r#"(?:el|la )?pr[oóò]xim[oa]"#)?,
             datetime_check!(),
             |_, datetime| datetime.value().the_nth_not_immediate(0)
    );
    b.rule_2("last <named-month|named-day>",
             b.reg(r#"(?:el|la )?pasad[oa]"#)?,
             datetime_check!(),
             |_, datetime| datetime.value().the_nth(1)
    );
    b.rule_2("<named-month|named-day> next",
             datetime_check!(),
             b.reg(r#"que vienen?|pr[oóò]xim[oa]"#)?,
             |datetime, _| datetime.value().the_nth_not_immediate(0)
    );
    b.rule_3("the <datetime> next",
             b.reg(r#"el|la"#)?,
             datetime_check!(),
             b.reg(r#"que vienen?|pr[oóò]xim[oa]"#)?,
             |_, datetime, _| datetime.value().the_nth(0)
    );

    b.rule_3("the <day-of-week> of next week",
             b.reg(r#"el"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"de la (?:semana que viene|pr[oóò]xima semana)"#)?,
             |_, datetime, _| datetime.value().the_nth(1)
    );
    b.rule_2("<day-of-week> of next week",
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"de la (?:semana que viene|pr[oóò]xima semana)"#)?,
             |datetime, _| datetime.value().the_nth(1)
    );
    b.rule_2("<named-month|named-day> past",
             datetime_check!(),
             b.reg(r#"pasad[oa]"#)?,
             |datetime, _| datetime.value().the_nth(-1)
    );
    b.rule_1("year",
             integer_check_by_range!(1000, 2100),
             |integer| {
                 helpers::year(integer.value().value as i32)
             }
    );
    b.rule_1("year (latent)",
             integer_check_by_range!(-1000, 999),
             |integer| {
                 Ok(helpers::year(integer.value().value as i32)?.latent())
             }
    );
    b.rule_1("year (latent)",
             integer_check_by_range!(2101, 2200),
             |integer| {
                 Ok(helpers::year(integer.value().value as i32)?.latent())
             }
    );
    b.rule_2("del <year>", //latin america mostly
             b.reg(r#"(?:d?el )?(?:a[ñn]o)?"#)?,
             integer_check_by_range!(1000, 2100),
             |_, integer| helpers::year(integer.value().value as i32)
    );
    b.rule_1_terminal("day of month (1st)",
             b.reg(r#"el prim(?:er)?o|uno|prim\.?|1o"#)?,
             |_| helpers::day_of_month(1)
    );
    b.rule_2("el dia <day-of-month> (non ordinal) (not latent)",
             b.reg(r#"d?el(?: d[iíì]a)?"#)?,
             integer_check_by_range!(1, 31),
             |_, integer| Ok(helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_2("el <day-of-month> (non ordinal) (latent)",
             b.reg(r#"d?el"#)?,
             integer_check_by_range!(1, 31),
             |_, integer| Ok(helpers::day_of_month(integer.value().value as u32)?.latent())
    );
    b.rule_3("the <day-of-month> (ordinal)",
             b.reg(r#"el"#)?,
             ordinal_check!(),
             b.reg(r#"d[iíì]a"#)?,
             |_, ordinal, _| Ok((*ordinal.value()).prefixed()))
    ;
    b.rule_2("<day-of-month> <named-month>",
             integer_check_by_range!(1, 31),
             datetime_check!(form!(Form::Month(_))),
             |integer, month| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_3("<day-of-month> de <named-month>",
             integer_check_by_range!(1, 31),
             b.reg(r#"de(?:l mes de)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |integer, _, month| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_4("el <day-of-month> de <named-month>",
             b.reg(r#"el(?: d[iíì]a)?"#)?,
             integer_check_by_range!(1, 31),
             b.reg(r#"de(?:l mes de)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, integer, _, month| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_4("<day-of-week> <day-of-month> de <named-month>",
             datetime_check!(form!(Form::DayOfWeek{..})),
             integer_check_by_range!(1, 31),
             b.reg(r#"de(?:l mes de)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, integer, _, month| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_4("ultimo <day-of-week> de <datetime>",
             b.reg(r#"[ú|u]ltimo"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"de|en"#)?,
             datetime_check!(),
             |_, dow, _, datetime| dow.value().last_of(datetime.value())
    );
    b.rule_4("the <cycle> of <datetime>",
             b.reg(r#"el|la"#)?,
             cycle_check!(),
             b.reg(r#"del?"#)?,
             datetime_check!(),
             |_, cycle, _, datetime| helpers::cycle_nth_after_not_immediate(cycle.value().grain, 0, datetime.value())
    );
    b.rule_4("nth <datetime> de <datetime>",
             ordinal_check!(),
             datetime_check!(),
             b.reg(r#"del?|en"#)?,
             datetime_check!(),
             |ordinal, a, _, b| b.value().intersect(a.value())?.the_nth(ordinal.value().value - 1)
    );
    b.rule_5("the nth <datetime> de <datetime>",
             b.reg(r#"el|la"#)?,
             ordinal_check!(),
             datetime_check!(),
             b.reg(r#"de|en"#)?,
             datetime_check!(),
             |_, ordinal, a, _, b| b.value().intersect(a.value())?.the_nth(ordinal.value().value - 1)
    );
    b.rule_4("ultimo <cycle> de <datetime>",
             b.reg(r#"[ú|u]ltimo"#)?,
             cycle_check!(),
             b.reg(r#"de|en"#)?,
             datetime_check!(),
             |_, cycle, _, datetime| helpers::cycle_nth_after_not_immediate(cycle.value().grain, -1, datetime.value())
    );
    b.rule_4("nth <cycle> de <datetime>",
             ordinal_check!(),
             cycle_check!(),
             b.reg(r#"de|en"#)?,
             datetime_check!(),
             |ordinal, cycle, _, datetime| helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value - 1, datetime.value())
    );
    b.rule_5("the nth <cycle> de <datetime>",
             b.reg(r#"el|la"#)?,
             ordinal_check!(),
             cycle_check!(),
             b.reg(r#"del?|en"#)?,
             datetime_check!(),
             |_, ordinal, cycle, _, datetime| helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value - 1, datetime.value())
    );
    b.rule_3("<ordinal> week-end of <named-month>",
             ordinal_check!(),
             b.reg(r#"week[ -]?end|fin(?:de)?(?: de semana)? de(?:l mes de)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |ordinal, _, datetime| {
                 let week_day_start = helpers::day_of_week(Weekday::Fri)?.intersect(&helpers::hour(18, false)?)?;
                 let week_day_end = helpers::day_of_week(Weekday::Mon)?.intersect(&helpers::hour(0, false)?)?;
                 let week_day = week_day_start.span_to(&week_day_end, false)?;
                 let week_ends_of_time = datetime.value().intersect(&week_day)?;
                 week_ends_of_time.the_nth(ordinal.value().value - 1)
             }
    );
    b.rule_2("last week-end of <named-month>",
             b.reg(r#"[ú|u]ltimo (?:week[ -]?end|fin(?:de)?(?: de semana)?) de(?:l mes de)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, datetime| {
                 let week_day_start = helpers::day_of_week(Weekday::Fri)?.intersect(&helpers::hour(18, false)?)?;
                 let week_day_end = helpers::day_of_week(Weekday::Mon)?.intersect(&helpers::hour(0, false)?)?;
                 let week_day = week_day_start.span_to(&week_day_end, false)?;
                 week_day.last_of(datetime.value())
             }
    );
    b.rule_2("<named-month> <day-of-month>",
             datetime_check!(form!(Form::Month(_))),
             integer_check_by_range!(1, 31),
             |month, integer| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_3("el <day-of-week> <day-of-month>",
             b.reg(r#"el"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             integer_check_by_range!(1, 31),
             |_, dow, integer| dow.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_2("<day-of-week> <day-of-month>",
             datetime_check!(form!(Form::DayOfWeek{..})),
             integer_check_by_range!(1, 31),
             |dow, integer| dow.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_1("time-of-day (latent)",
             integer_check_by_range!(1, 23),
             |integer| Ok(helpers::hour(integer.value().value as u32, integer.value().value < 12)?.latent())
    );
    b.rule_1("time-of-day (latent)",
             integer_check_by_range!(0, 0),
             |_| Ok(helpers::hour(0, false)?.latent())
    );
    b.rule_1_terminal("noon",
                      b.reg(r#"mediod[iíi]a"#)?,
                      |_| helpers::hour(12, false)
    );
    b.rule_1_terminal("midnight",
                      b.reg(r#"medianoche|las doce de la noche"#)?,
                      |_| helpers::hour(0, false)
    );
    b.rule_3("la <time-of-day> horas",
             b.reg(r#"las?"#)?,
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))),
             b.reg(r#"h(?:\.|oras)?"#)?,
             |_, datetime, _| Ok(datetime.value().clone().not_latent())
    );
    b.rule_2("la <time-of-day>",
             b.reg(r#"las?"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, datetime| Ok(datetime.value().clone().not_latent())
    );
    b.rule_2("a las <time-of-day>",
             b.reg(r#"a(?: las)?"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, tod| Ok(tod.value().clone().not_latent())
    );
    b.rule_3("a las <hour-min>(time-of-day)",
             b.reg(r#"a(?: las)?"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"h(?:\.|oras)?"#)?,
             |_, tod, _| Ok(tod.value().clone().not_latent())
    );
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
    b.rule_1_terminal("quarter (relative minutes)",
                      b.reg(r#"(?:un )?(?:cuarto|1/4)(?: de hora)?"#)?,
                      |_| Ok(RelativeMinuteValue(15))
    );
    b.rule_1_terminal("half (relative minutes)",
                      b.reg(r#"media"#)?,
                      |_| Ok(RelativeMinuteValue(30))
    );
    b.rule_1_terminal("3 quarter (relative minutes)",
                      b.reg(r#"(3|tres) cuartos?(?: de hora)?"#)?,
                      |_| Ok(RelativeMinuteValue(45))
    );
    b.rule_1("number (as relative minutes)",
             integer_check_by_range!(1, 59),
             |integer| Ok(RelativeMinuteValue(integer.value().value as i32))
    );
    b.rule_2("<integer> minutes (as relative minutes)",
             integer_check_by_range!(1, 59),
             b.reg(r#"min\.?(?:uto)?s?"#)?,
             |integer, _| Ok(RelativeMinuteValue(integer.value().value as i32))
    );
    b.rule_2("<integer> minutes (as relative minutes)",
             b.reg(r#"y"#)?,
             integer_check_by_range!(1, 59),
             |_, integer| Ok(RelativeMinuteValue(integer.value().value as i32))
    );
    b.rule_2("<hour-of-day> <integer> (as relative minutes)",
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))),
             relative_minute_check!(),
             |datetime, relative_minute| helpers::hour_relative_minute(
                 datetime.value().form_time_of_day()?.full_hour(),
                 relative_minute.value().0,
                 datetime.value().form_time_of_day()?.is_12_clock())
    );
    b.rule_3("<hour-of-day> minus <integer> (as relative minutes)",
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))),
             b.reg(r#"menos\s?"#)?,
             relative_minute_check!(),
             |datetime, _, relative_minute| helpers::hour_relative_minute(
                 datetime.value().form_time_of_day()?.full_hour(),
                 -1 * relative_minute.value().0,
                 datetime.value().form_time_of_day()?.is_12_clock())
    );
    b.rule_3("<hour-of-day> and <relative minutes>",
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))),
             b.reg(r#"y"#)?,
             relative_minute_check!(),
             |datetime, _, relative_minute| helpers::hour_relative_minute(
                 datetime.value().form_time_of_day()?.full_hour(),
                 relative_minute.value().0,
                 datetime.value().form_time_of_day()?.is_12_clock())
    );
    // Written dates in numeric formats
    b.rule_1_terminal("yyyy-mm-dd - ISO",
                      b.reg(r#"(\d{4})[-/](0?[1-9]|1[0-2])[-/](3[01]|[12]\d|0?[1-9])"#)?,
                      |text_match| helpers::year_month_day(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(3).parse()?)
    );
    b.rule_1_terminal("dd/mm/yy or dd/mm/yyyy",
                      b.reg(r#"(0?[1-9]|[12]\d|3[01])[-\./](0?[1-9]|1[0-2])[-\./](\d{2,4})"#)?,
                      |text_match| helpers::year_month_day(
                          text_match.group(3).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(1).parse()?,
                      )
    );
    b.rule_1_terminal("dd/mm",
                      b.reg(r#"(0?[1-9]|[12]\d|3[01])[\./](1[0-2]|0?[1-9])"#)?,
                      |text_match| helpers::month_day(
                          text_match.group(2).parse()?,
                          text_match.group(1).parse()?)
    );
    // End of Written dates in numeric formats
    b.rule_1_terminal("beginning of day",
                      b.reg(r#"al (?:inicio|empezar) d?el d[iíì]a|a primera hora"#)?,
                      |_| {
                          Ok(helpers::hour(6, false)?
                              .span_to(&helpers::hour(10, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Morning)))
                      }
    );
    b.rule_1_terminal("middle of day",
                      b.reg(r#"(?:al?|en) (?:la mitad|medio) (?:del )?d[iíì]a|a mediod[iíì]a"#)?,
                      |_| {
                          Ok(helpers::hour(11, false)?
                              .span_to(&helpers::hour(16, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::None)))
                      }
    );
    b.rule_1_terminal("end of day",
                      b.reg(r#"al (?:final|acabar) d?el d[iíì]a|a última hora"#)?,
                      |_| {
                          Ok(helpers::hour(17, false)?
                              .span_to(&helpers::hour(21, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );

    b.rule_1_terminal("morning",
                      b.reg(r#"ma[ñn]ana"#)?,
                      |_| Ok(helpers::hour(4, false)?.span_to(&helpers::hour(12, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Morning))
                          .latent())
    );
    b.rule_1_terminal("beginning of morning",
                      b.reg(r#"(?:pronto por|a primera hora (?:por|de)) la mañana|la mañana a primera hora"#)?,
                      |_| Ok(helpers::hour(4, false)?
                          .span_to(&helpers::hour(9, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Morning)))
    );
    b.rule_1_terminal("end of morning",
                      b.reg(r#"[uúù]ltima hora de la mañana|la mañana a [uúù]ltima hora"#)?,
                      |_| Ok(helpers::hour(10, false)?
                          .span_to(&helpers::hour(12, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Morning)))
    );
    b.rule_1_terminal("lunch",
                      b.reg(r#"(?:la )?(?:comida|hora de comer)"#)?,
                      |_| Ok(helpers::hour(12, false)?
                          .span_to(&helpers::hour(14, false)?, false)?
                          .latent()
                          .form(Form::Meal))
    );
    b.rule_1_terminal("after lunch",
                      b.reg(r#"despu[eéè]s de (?:la hora de )?comer"#)?,
                      |_| {
                          let period = helpers::hour(13, false)?
                              .span_to(&helpers::hour(17, false)?, false)?;
                          Ok(helpers::cycle_nth(Grain::Day, 0)?.intersect(&period)?.form(Form::PartOfDay(PartOfDayForm::Afternoon)))
                      }
    );
    b.rule_1_terminal("before lunch",
                      b.reg(r#"antes de comer"#)?,
                      |_| {
                          let period = helpers::hour(10, false)?
                              .span_to(&helpers::hour(12, false)?, false)?;
                          Ok(helpers::cycle_nth(Grain::Day, 0)?.intersect(&period)?.form(Form::PartOfDay(PartOfDayForm::Morning)))
                      }
    );
    b.rule_1_terminal("after work",
                      b.reg(r#"después del trabajo|al salir de trabajar"#)?,
                      |_| {
                          let period = helpers::hour(17, false)?
                              .span_to(&helpers::hour(21, false)?, false)?;
                          Ok(helpers::cycle_nth(Grain::Day, 0)?.intersect(&period)?.form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );
    b.rule_1_terminal("afternoon",
                      b.reg(r#"(?:la )?tarde"#)?,
                      |_| Ok(helpers::hour(12, false)?.span_to(&helpers::hour(19, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Afternoon))
                          .latent())
    );
    b.rule_1_terminal("middle afternoon",
                      b.reg(r#"(?:media|plena) tarde"#)?,
                      |_| {
                          Ok(helpers::hour(15, false)?
                              .span_to(&helpers::hour(17, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
                      }
    );
    b.rule_1_terminal("middle of day",
                      b.reg(r#"(?:la )?(?:mitad|medio) ?(?:del )?d[iíì]a"#)?,
                      |_| {
                          Ok(helpers::hour(11, false)?
                              .span_to(&helpers::hour(16, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::None)))
                      }
    );
    b.rule_1_terminal("evening",
                      b.reg(r#"(?:la )?noche"#)?,
                      |_| Ok(helpers::hour(18, false)?.span_to(&helpers::hour(0, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Evening))
                          .latent())
    );
    b.rule_1_terminal("beginning of evening",
                      b.reg(r#"(?:primera hora de|pronto por) la (?:tarde|noche)|por la (?:tarde|noche) (?:a primera hora|pronto)"#)?,
                      |_| {
                          Ok(helpers::hour(18, false)?
                              .span_to(&helpers::hour(21, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );
    b.rule_1_terminal("end of evening",
                      b.reg(r#"tarde por la noche|por la (?:noche|tarde) a [uúù]ltima hora|(?:al final|(?:a|en las?) [uúù]ltimas? horas?) de la (?:velada|tarde)"#)?,
                      |_| {
                          Ok(helpers::hour(21, false)?
                              .span_to(&helpers::hour(0, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );
    b.rule_2("in the <part-of-day>",
             b.reg(r#"(?:a|en|de|por|durante)"#)?,
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             |_, pod| Ok(pod.value().clone().not_latent())
    );
    b.rule_2("this <part-of-day>",
             b.reg(r#"est(?:e|a)"#)?,
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             |_, pod| Ok(helpers::cycle_nth(Grain::Day, 0)?
                 .intersect(pod.value())?
                 .form(pod.value().form.clone()))
    );
    b.rule_1_terminal("del mediodía",
                      b.reg(r#"del mediod[ií]a"#)?,
                      |_| Ok(helpers::hour(12, false)?.span_to(&helpers::hour(17, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Afternoon))
                          .latent())
    );
    b.rule_2("<time-of-day> <part-of-day>",
             datetime_check!(excluding_form!(Form::PartOfDay(_))),
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             |a, b| a.value().intersect(b.value())
    );
    b.rule_3("<time-of-day> prep <part-of-day>",
             datetime_check!(excluding_form!(Form::TimeOfDay(_))),
             b.reg(r#"por(?: la| el)?"#)?,
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             |a, _, b| a.value().intersect(b.value())
    );
    b.rule_2("<dim time> de la tarde",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:a|en|de) la tarde"#)?,
             |datetime, _| {
                 let period = helpers::hour(12, false)?
                     .span_to(&helpers::hour(21, false)?, false)?;
                 datetime.value().intersect(&period)
             }
    );
    b.rule_2("<dim time> de la manana",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:a|en|de) la (?:ma[ñn]ana|madrugada)"#)?,
             |datetime, _| {
                 let period = helpers::hour(0, false)?
                     .span_to(&helpers::hour(12, false)?, false)?;
                 datetime.value().intersect(&period)
             }
    );
    b.rule_2("<dim time> in the evening",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"de la (?:media)?noche"#)?,
             |a, _| {
                 let period = helpers::hour(16, false)?
                     .span_to(&helpers::hour(0, false)?, false)?;
                 a.value().intersect(&period)
             }
    );
    b.rule_3("<integer> in the <part-of-day>",
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             b.reg(r#"(?:a|en|de|por) la"#)?,
             datetime_check!(),
             |pod, _, datetime| datetime.value().intersect(pod.value())
    );
    b.rule_3("the <day-of-month> at <datetime>",
             b.reg(r#"el(?: d[iíì]a)?"#)?,
             integer_check_by_range!(1, 31),
             datetime_check!(),
             |_, integer, datetime| {
                 let day_of_month = helpers::day_of_month(integer.value().value as u32)?;
                 day_of_month.intersect(&datetime.value())
             }
    );
    b.rule_1_terminal("beginning of week",
                      b.reg(r#"a principios de (?:esta )?semana"#)?,
                      |_| helpers::day_of_week(Weekday::Mon)
                          ?.span_to(&helpers::day_of_week(Weekday::Tue)?, false)
    );
    b.rule_1_terminal("middle of week",
                      b.reg(r#"a (?:mitad de|media) semana"#)?,
                      |_| helpers::day_of_week(Weekday::Wed)
                          ?.span_to(&helpers::day_of_week(Weekday::Thu)?, false)
    );
    b.rule_1_terminal("end of week (not weekend)",
                      b.reg(r#"a finales de la semana"#)?,
                      |_| helpers::day_of_week(Weekday::Thu)
                          ?.span_to(&helpers::day_of_week(Weekday::Sun)?, false)
    );
    b.rule_1_terminal("during the week",
                      b.reg(r#"durante la semana"#)?,
                      |_| helpers::day_of_week(Weekday::Mon)
                          ?.span_to(&helpers::day_of_week(Weekday::Fri)?, false)
    );
    b.rule_1_terminal("week-end",
                      b.reg(r#"week[ -]?end|fin(?:de)?(?: de semana)?"#)?,
                      |_| {
                          let friday = helpers::day_of_week(Weekday::Fri)?
                              .intersect(&helpers::hour(18, false)?)?;
                          let monday = helpers::day_of_week(Weekday::Mon)?
                              .intersect(&helpers::hour(0, false)?)?;
                          friday.span_to(&monday, false)
                      }
    );
    b.rule_1_terminal("season",
                      b.reg(r#"verano"#)?,
                      |_| helpers::month_day(6, 21)?
                          .span_to(&helpers::month_day(9, 23)?, false)
    );
    b.rule_1_terminal("season",
                      b.reg(r#"oto[ñn]o"#)?,
                      |_| helpers::month_day(9, 23)?
                          .span_to(&helpers::month_day(12, 21)?, false)
    );
    b.rule_1_terminal("season",
                      b.reg(r#"invierno"#)?,
                      |_| helpers::month_day(12, 21)?
                          .span_to(&helpers::month_day(3, 20)?, false)
    );
    b.rule_1_terminal("season",
                      b.reg(r#"primavera"#)?,
                      |_| helpers::month_day(3, 20)?
                          .span_to(&helpers::month_day(6, 21)?, false)
    );
    b.rule_2("el <datetime>",
             b.reg(r#"el|la"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |_, datetime| Ok(datetime.value().clone())
    );
    b.rule_5("dd-dd <month>(interval)",
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             b.reg(r#"\-|al?"#)?,
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             b.reg(r#"de(?:l mes de)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |a, _, b, _, month| {
                 let start = month.value().intersect(&helpers::day_of_month(a.group(1).parse()?)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(b.group(1).parse()?)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_4("dd-dd <month>(interval)",
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             b.reg(r#"\-|al?"#)?,
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             datetime_check!(form!(Form::Month(_))),
             |a, _, b, month| {
                 let start = month.value().intersect(&helpers::day_of_month(a.group(1).parse()?)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(b.group(1).parse()?)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_6("dd-dd <month>(interval)",
             b.reg(r#"entre(?: el)?"#)?,
             b.reg(r#"(0?[1-9]|[12]\d|3[01])"#)?,
             b.reg(r#"y(?: el)?"#)?,
             b.reg(r#"(0?[1-9]|[12]\d|3[01])"#)?,
             b.reg(r#"de(?:l mes de)?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, a, _, b, _, month| {
                 let start = month.value().intersect(&helpers::day_of_month(a.group(1).parse()?)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(b.group(1).parse()?)?)?;
                 start.span_to(&end, true)
             }
    );
    // FIXME: not working
    b.rule_4("dd <month> - dd <month>(interval)",
             b.reg(r#"(0?[1-9]|[12]\d|3[01])"#)?,
             datetime_check!(form!(Form::Month(_))),
             b.reg(r#" *- *(0?[1-9]|[12]\d|3[01])"#)?,
             datetime_check!(form!(Form::Month(_))),
             |a, b, c, d| {
        let start = b.value().intersect(&helpers::day_of_month(a.group(1).parse()?)?)?;
        let end = d.value().intersect(&helpers::day_of_month(c.group(1).parse()?)?)?;
        start.span_to(&end, true)
    }
    );

    b.rule_4("<datetime> - <datetime> (interval)",
             b.reg(r#"del"#)?,
             datetime_check!(),
             b.reg(r#"\-|al?"#)?,
             datetime_check!(),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );
    b.rule_4("between <datetime> and <datetime> (interval)",
             b.reg(r#"entre"#)?,
             datetime_check!(),
             b.reg(r#"y"#)?,
             datetime_check!(),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );
    b.rule_4("from <time-of-day> to <time-of-day> (interval)",
             b.reg(r#"de(?: las?)?"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"a(?: las?)?"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );
    b.rule_2("before <datetime>",
             b.reg(r#"antes de|hasta"#)?,
             datetime_check!(),
             |_, datetime| Ok(datetime.value().clone().mark_before_end())
    );
    b.rule_2("approx <time-of-day>",
             b.reg(r#"sobre|cerca de|hacia|alrededor de"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, a| Ok(a.value().clone().not_latent().precision(Approximate))
    );
    b.rule_2("<time-of-day> approx",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"m[aáà]s o menos|aproximadamente"#)?,
             |a, _| Ok(a.value().clone().not_latent().precision(Approximate))
    );
    b.rule_2("<time-of-day> exact",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"exactamente|exactas|en punto"#)?,
             |a, _| Ok(a.value().clone().not_latent())
    );
    b.rule_2("from <time-of-day>",
             b.reg(r#"(?:a partir|despu[eéè]s) del?|desde"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, datetime| Ok(datetime.value().clone().mark_after_start())
    );
    b.rule_3("from <time-of-day> on",
             b.reg(r#"(?:a partir |despu[eéè]s )?del?|desde"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"en adelante"#)?,
             |_, datetime, _| Ok(datetime.value().clone().mark_after_start())
    );
    b.rule_2("(from) <time-of-day> on",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"en adelante"#)?,
             |datetime, _| Ok(datetime.value().clone().mark_after_start())
    );
    b.rule_2("from <datetime>",
             b.reg(r#"(?:a partir|despu[eéè]s) del?|desde"#)?,
             datetime_check!(excluding_form!(Form::TimeOfDay(_))),
             |_, datetime| Ok(datetime.value().clone().mark_after_start())
    );
    b.rule_3("from <datetime> on",
             b.reg(r#"(?:a partir |despu[eéè]s )?del?|desde"#)?,
             datetime_check!(excluding_form!(Form::TimeOfDay(_))),
             b.reg(r#"en adelante"#)?,
             |_, datetime, _| Ok(datetime.value().clone().mark_after_start())
    );
    b.rule_2("(from) <datetime> on",
             datetime_check!(excluding_form!(Form::TimeOfDay(_))),
             b.reg(r#"en adelante"#)?,
             |datetime, _| Ok(datetime.value().clone().mark_after_start())
    );
    Ok(())
}


pub fn rules_datetime_with_cycle(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("this <cycle>",
             b.reg(r#"(?:durante )?(?:est(?:e|a|os)|en (?:el|l[oa]s?) ?)"#)?,
             cycle_check!(),
             |_, cycle| helpers::cycle_nth(cycle.value().grain, 0)
    );
    b.rule_3("the <cycle> past",
             b.reg(r#"(?:el|l[oa]s?|est[ea]) ?"#)?,
             cycle_check!(),
             b.reg(r#"(?:pasad|[uúù]ltim)[oa]s?"#)?,
             |_, cycle, _| helpers::cycle_nth(cycle.value().grain, -1)
    );
    b.rule_2("the past <cycle>",
             b.reg(r#"(?:(?:el|l[oa]s?|est[ea]) )?(?:pasad|[uúù]ltim)[oa]s?"#)?,
             cycle_check!(),
             |_, cycle| helpers::cycle_nth(cycle.value().grain, -1)
    );
    b.rule_3("the <cycle> next",
             b.reg(r#"(?:el|l[oa]s?) ?"#)?,
             cycle_check!(),
             b.reg(r#"(?:pr[oóò]xim[oa]s?|que vienen?|siguientes?)"#)?,
             |_, cycle, _| helpers::cycle_nth(cycle.value().grain, 1)
    );
    b.rule_2("the next <cycle>",
             b.reg(r#"(?:(?:el|l[oa]s?|est[ea]) )?pr[oóò]xim[oa]s?|siguientes?"#)?,
             cycle_check!(),
             |_, cycle| helpers::cycle_nth(cycle.value().grain, 1)
    );
    b.rule_4("the <cycle> after <datetime>",
             b.reg(r#"(?:el|l[oa]s?)"#)?,
             cycle_check!(),
             b.reg(r#"(?:pr[oóò]xim[oa]s?|que vienen?|siguientes?)"#)?,
             datetime_check!(),
             |_, cycle, _, datetime| helpers::cycle_nth_after(cycle.value().grain, 1, datetime.value())
    );
    b.rule_4("the <cycle> before <datetime>",
             b.reg(r#"(?:el|l[oa]s?)"#)?,
             cycle_check!(),
             b.reg(r#"antes de"#)?,
             datetime_check!(),
             |_, cycle, _, datetime| helpers::cycle_nth_after(cycle.value().grain, -1, datetime.value())
    );
    b.rule_3("past n <cycle>",
             b.reg(r#"(?:pasad|[uúù]ltim)[oa]s?"#)?,
             integer_check_by_range!(2, 9999),
             cycle_check!(),
             |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    b.rule_3("n past <cycle>",
             integer_check_by_range!(2, 9999),
             b.reg(r#"(?:pasad|[uúù]ltim)[oa]s?"#)?,
             cycle_check!(),
             |integer, _, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    b.rule_4("the n past <cycle>",
             b.reg(r#"(?:el|l[oa]s?)"#)?,
             integer_check_by_range!(2, 9999),
             b.reg(r#"(?:pasad|[uúù]ltim)[oa]s?"#)?,
             cycle_check!(),
             |_, integer, _, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    b.rule_3("the past n <cycle>",
             b.reg(r#"(?:el|l[oa]s?) (?:pasad|[uúù]ltim)[oa]s?"#)?,
             integer_check_by_range!(2, 9999),
             cycle_check!(),
             |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    b.rule_4("the n <cycle> past",
             b.reg(r#"(?:el|l[oa]s?)"#)?,
             integer_check_by_range!(2, 9999),
             cycle_check!(),
             b.reg(r#"(?:pasad|[uúù]ltim)[oa]s?"#)?,
             |_, integer, cycle, _| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    b.rule_3("next n <cycle>",
             b.reg(r#"(?:(?:el|l[oa]s?) )?pr[oóò]xim[oa]s?"#)?,
             integer_check_by_range!(2, 9999),
             cycle_check!(),
             |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    b.rule_3("n next <cycle>",
             integer_check_by_range!(2, 9999),
             b.reg(r#"pr[oóò]xim[oa]s?"#)?,
             cycle_check!(),
             |integer, _, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    b.rule_4("the n next <cycle>",
             b.reg(r#"(?:el|l[oa]s?)"#)?,
             integer_check_by_range!(2, 9999),
             b.reg(r#"pr[oóò]xim[oa]s?"#)?,
             cycle_check!(),
             |_, integer, _, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    b.rule_3("n <cycle> next",
             integer_check_by_range!(2, 9999),
             cycle_check!(),
             b.reg(r#"(?:pr[oóò]xim[oa]s?|que vienen?|siguientes?)"#)?,
             |integer, cycle, _| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    b.rule_4("the n <cycle> next",
             b.reg(r#"(?:el|l[oa]s?)"#)?,
             integer_check_by_range!(2, 9999),
             cycle_check!(),
             b.reg(r#"(?:pr[oóò]xim[oa]s?|que vienen?|siguientes?)"#)?,
             |_, integer, cycle, _| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    b.rule_2("<ordinal> quarter",
             ordinal_check_by_range!(1, 4),
             cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Quarter),
             |ordinal, _| helpers::cycle_nth_after(
                 Grain::Quarter,
                 ordinal.value().value - 1,
                 &helpers::cycle_nth(Grain::Year, 0)?
             )
    );
    b.rule_4("<ordinal> quarter <year>",
             ordinal_check_by_range!(1, 4),
             cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Quarter),
             b.reg(r#"del? ?"#)?,
             datetime_check!(),
             |ordinal, _, _, datetime| helpers::cycle_nth_after(
                 Grain::Quarter,
                 ordinal.value().value - 1,
                 datetime.value()
             )
    );
    Ok(())
}


pub fn rules_datetime_with_duration(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
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
    b.rule_2("in <duration> (future moment)",
             b.reg(r#"(?:en|dentro(?: de)?)(?: (?:el|la|los|las) pr[oóò]xim[oa]s)?"#)?,
             duration_check!(),
             |_, duration| duration.value().in_present()
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
                      b.reg(r#"d[iíì]as?"#)?,
                      |_| CycleValue::new(Grain::Day)
    );
    b.rule_1_terminal("week (cycle)",
                      b.reg(r#"semanas?"#)?,
                      |_| CycleValue::new(Grain::Week)
    );
    b.rule_1_terminal("month (cycle)",
                      b.reg(r#"mes(?:es)?"#)?,
                      |_| CycleValue::new(Grain::Month)
    );
    b.rule_1_terminal("trimester (cycle)",
                      b.reg(r#"trimestres?"#)?,
                      |_| CycleValue::new(Grain::Quarter)
    );
    b.rule_1_terminal("year (cycle)",
                      b.reg(r#"a(?:n|ñ)os?"#)?,
                      |_| CycleValue::new(Grain::Year)
    );
    Ok(())
}
