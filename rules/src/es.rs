use rustling::*;
use values::dimension::*;
use values::helpers;
use moment::{Weekday, Grain, PeriodComp};

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
                      b.reg(r#"d(?:í|i)as?"#)?,
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
                      b.reg(r#"a(?:n|ñ)os?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Year))
    );
    b.rule_2("<integer> <unit-of-duration>",
             integer_check_by_range!(0),
             unit_of_duration_check!(),
             |integer, uod| Ok(DurationValue::new(PeriodComp::new(uod.value().grain, integer.value().value).into()))
    );
    b.rule_2("en <duration>",
             b.reg(r#"en"#)?,
             duration_check!(),
             |_, duration| duration.value().in_present()
    );
    b.rule_2("hace <duration>",
             b.reg(r#"hace"#)?,
             duration_check!(),
             |_, duration| duration.value().ago()
    );
    Ok(())
}

pub fn rules_cycle(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("segundo (cycle)",
                      b.reg(r#"segundos?"#)?,
                      |_| CycleValue::new(Grain::Second)
    );
    b.rule_1_terminal("minutos (cycle)",
                      b.reg(r#"minutos?"#)?,
                      |_| CycleValue::new(Grain::Minute)
    );
    b.rule_1_terminal("hora (cycle)",
                      b.reg(r#"horas?"#)?,
                      |_| CycleValue::new(Grain::Hour)
    );
    b.rule_1_terminal("dia (cycle)",
                      b.reg(r#"d(?:í|i)as?"#)?,
                      |_| CycleValue::new(Grain::Day)
    );
    b.rule_1_terminal("semana (cycle)",
                      b.reg(r#"semanas?"#)?,
                      |_| CycleValue::new(Grain::Week)
    );
    b.rule_1_terminal("mes (cycle)",
                      b.reg(r#"mes(?:es)?"#)?,
                      |_| CycleValue::new(Grain::Month)
    );
    b.rule_1_terminal("trimestre (cycle)",
                      b.reg(r#"trimestres?"#)?,
                      |_| CycleValue::new(Grain::Quarter)
    );
    b.rule_1_terminal("año (cycle)",
                      b.reg(r#"a(?:n|ñ)os?"#)?,
                      |_| CycleValue::new(Grain::Year)
    );
    b.rule_2("este|en un <cycle>",
             b.reg(r#"(?:est(?:e|a|os)|en (?:el|los|la|las) ?)"#)?,
             cycle_check!(),
             |_, cycle| helpers::cycle_nth(cycle.value().grain, 0)
    );
    b.rule_3("la <cycle> pasado",
             b.reg(r#"(?:el|los|la|las) ?"#)?,
             cycle_check!(),
             b.reg(r#"pasad(?:a|o)s?|[u|ú]ltim[a|o]s?"#)?,
             |_, cycle, _| helpers::cycle_nth(cycle.value().grain, -1)
    );
    b.rule_3("la pasado <cycle>",
             b.reg(r#"(?:el|los|la|las) ?"#)?,
             b.reg(r#"pasad(?:a|o)s?|[u|ú]ltim[a|o]s?"#)?,
             cycle_check!(),
             |_, _, cycle| helpers::cycle_nth(cycle.value().grain, -1)
    );
    b.rule_3("el <cycle> (proximo|que viene)",
             b.reg(r#"(?:el|los|la|las) ?"#)?,
             cycle_check!(),
             b.reg(r#"(?:pr(?:ó|o)xim(?:o|a)s?|que vienen?|siguientes?)"#)?,
             |_, cycle, _| helpers::cycle_nth(cycle.value().grain, 1)
    );
    b.rule_3("el proximo <cycle>",
             b.reg(r#"(?:el|los|la|las) ?"#)?,
             b.reg(r#"pr(?:ó|o)xim(?:o|a)s?|siguientes?"#)?,
             cycle_check!(),
             |_, _, cycle| helpers::cycle_nth(cycle.value().grain, 1)
    );
    b.rule_4("el <cycle> proximo|que viene <time>",
             b.reg(r#"(?:el|los|la|las)"#)?,
             cycle_check!(),
             b.reg(r#"(?:pr(?:ó|o)xim(?:o|a)s?|que vienen?|siguientes?)"#)?,
             time_check!(),
             |_, cycle, _, time| helpers::cycle_nth_after(cycle.value().grain, 1, time.value())
    );
    b.rule_4("el <cycle> antes <time>",
             b.reg(r#"l[ea']? ?"#)?,
             cycle_check!(),
             b.reg(r#"antes de"#)?,
             time_check!(),
             |_, cycle, _, time| helpers::cycle_nth_after(cycle.value().grain, -1, time.value())
    );
    b.rule_3("pasados n <cycle>",
             b.reg(r#"pasad(?:a|o)s?"#)?,
             integer_check_by_range!(2, 9999),
             cycle_check!(),
             |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    b.rule_3("n pasados <cycle>",
             integer_check_by_range!(2, 9999),
             b.reg(r#"pasad(?:a|o)s?"#)?,
             cycle_check!(),
             |integer, _, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    b.rule_3("proximas n <cycle>",
             b.reg(r#"pr(?:ó|o)xim(?:o|a)s?"#)?,
             integer_check_by_range!(2, 9999),
             cycle_check!(),
             |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    b.rule_3("n proximas <cycle>",
             integer_check_by_range!(2, 9999),
             b.reg(r#"pr(?:ó|o)xim(?:o|a)s?"#)?,
             cycle_check!(),
             |integer, _, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    b.rule_3("n <cycle> (proximo|que viene)",
             integer_check_by_range!(2, 9999),
             cycle_check!(),
             b.reg(r#"(?:pr(?:ó|o)xim(?:o|a)s?|que vienen?|siguientes?)"#)?,
             |integer, cycle, _| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    b.rule_2("<ordinal> quarter",
             ordinal_check!(),
             cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Quarter),
             |ordinal, _| helpers::cycle_nth_after(
                 Grain::Quarter,
                 ordinal.value().value - 1,
                 &helpers::cycle_nth(Grain::Year, 0)?
             )
    );
    b.rule_4("<ordinal> quarter <year>",
             ordinal_check!(),
             cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Quarter),
             b.reg(r#"del? ?"#)?,
             time_check!(),
             |ordinal, _, _, time| helpers::cycle_nth_after(
                 Grain::Quarter,
                 ordinal.value().value - 1,
                 time.value()
             )
    );
    Ok(())
}

pub fn rules_time(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("intersect",
             time_check!(|time: &TimeValue| !time.latent),
             time_check!(|time: &TimeValue| !time.latent),
             |a, b| a.value().intersect(b.value())
    );
    b.rule_3("intersect by `de`",
             time_check!(|time: &TimeValue| !time.latent),
             b.reg(r#"de"#)?,
             time_check!(|time: &TimeValue| !time.latent),
             |a, _, b| a.value().intersect(b.value())
    );
    b.rule_3("two time tokens separated by \",\"",
             time_check!(|time: &TimeValue| !time.latent),
             b.reg(r#","#)?,
             time_check!(|time: &TimeValue| !time.latent),
             |a, _, b| a.value().intersect(b.value())
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
                      b.reg(r#"mi(?:e|é)\.?(?:rcoles)?|mx|mier?\."#)?,
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
    b.rule_1_terminal("Navidad",
                      b.reg(r#"(?:la )?navidad"#)?,
                      |_| helpers::month_day(12, 25)
    );
    b.rule_1_terminal("Nochevieja",
                      b.reg(r#"nochevieja"#)?,
                      |_| helpers::month_day(12, 31)
    );
    b.rule_1_terminal("ano nuevo",
                      b.reg(r#"a[nñ]o nuevo"#)?,
                      |_| helpers::month_day(1, 1)
    );
    b.rule_1_terminal("right now",
                      b.reg(r#"ahor(?:it)?a|ya|en\s?seguida|cuanto antes"#)?,
                      |_| helpers::cycle_nth(Grain::Second, 0)
    );
    b.rule_1_terminal("now",
                      b.reg(r#"(?:hoy)|(?:en este momento)"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 0)
    );
    b.rule_1_terminal("tomorrow",
                      b.reg(r#"ma(?:n|ñ)ana"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 1)
    );
    b.rule_1_terminal("yesterday",
                      b.reg(r#"ayer"#)?,
                      |_| helpers::cycle_nth(Grain::Day, -1)
    );
    b.rule_1_terminal("the day after tomorrow",
                      b.reg(r#"pasado\s?ma(?:n|ñ)ana"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 2)
    );
    b.rule_1_terminal("the day before yesterday",
                      b.reg(r#"anteayer|antes de (?:ayer|anoche)|antier"#)?,
                      |_| helpers::cycle_nth(Grain::Day, -2)
    );
    b.rule_2("this <day-of-week>", //assumed to be in the future
             b.reg(r#"este"#)?,
             time_check!(form!(Form::DayOfWeek{..})),
             |_, time| time.value().the_nth_not_immediate(0)
    );
    b.rule_2("this <time>",
             b.reg(r#"este"#)?,
             time_check!(),
             |_, time| time.value().the_nth(0)
    );
    b.rule_2("<named-month|named-day> next",
             time_check!(),
             b.reg(r#"que vienen?"#)?,
             |time, _| time.value().the_nth(1)
    );
    b.rule_2("<named-month|named-day> past",
             time_check!(),
             b.reg(r#"pasad(?:o|a)"#)?,
             |time, _| time.value().the_nth(-1)
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
             b.reg(r#"del(?: a[ñn]o)?"#)?,
             integer_check_by_range!(1000, 2100),
             |_, integer| helpers::year(integer.value().value as i32)
    );
    b.rule_1_terminal("day of month (1st)",
                      b.reg(r#"primero|uno|prem\.?|1o"#)?,
                      |_| helpers::day_of_month(1)
    );
    b.rule_2("el <day-of-month> (non ordinal)",
             b.reg(r#"el"#)?,
             integer_check_by_range!(1, 31),
             |_, integer| Ok(helpers::day_of_month(integer.value().value as u32)?.latent())
    );
    b.rule_3("<day-of-month> de <named-month>",
             integer_check_by_range!(1, 31),
             b.reg(r#"de"#)?,
             time_check!(form!(Form::Month(_))),
             |integer, _, month| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_4("el <day-of-month> de <named-month>",
             b.reg(r#"el"#)?,
             integer_check_by_range!(1, 31),
             b.reg(r#"de"#)?,
             time_check!(form!(Form::Month(_))),
             |_, integer, _, month| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_4("ultimo <day-of-week> de <time>",
             b.reg(r#"[ú|u]ltimo"#)?,
             time_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"de|en"#)?,
             time_check!(),
             |_, dow, _, time| dow.value().last_of(time.value())
    );
    b.rule_4("nth <time> de <time>",
             ordinal_check!(),
             time_check!(),
             b.reg(r#"de|en"#)?,
             time_check!(),
             |ordinal, a, _, b| b.value().intersect(a.value())?.the_nth(ordinal.value().value - 1)
    );
    b.rule_5("nth <time> de <time>",
             b.reg(r#"el"#)?, // TODO to be checked
             ordinal_check!(),
             time_check!(),
             b.reg(r#"de|en"#)?,
             time_check!(),
             |_, ordinal, a, _, b| b.value().intersect(a.value())?.the_nth(ordinal.value().value - 1)
    );
    b.rule_2("<named-month> <day-of-month>",
             time_check!(form!(Form::Month(_))),
             integer_check_by_range!(1, 31),
             |month, integer| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_2("<day-of-week> <day-of-month>",
             time_check!(form!(Form::DayOfWeek{..})),
             integer_check_by_range!(1, 31),
             |dow, integer| dow.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_1("time-of-day (latent)",
             integer_check_by_range!(0, 23),
             |integer| Ok(helpers::hour(integer.value().value as u32, true)?.latent())
    );
    b.rule_1_terminal("noon",
                      b.reg(r#"mediod(?:í|i)a"#)?,
                      |_| helpers::hour(12, false)
    );
    b.rule_1_terminal("midnight",
                      b.reg(r#"medianoche"#)?,
                      |_| helpers::hour(0, false)
    );
    b.rule_2("<time-of-day> horas",
             time_check!(form!(Form::TimeOfDay(Some(_)))),
             b.reg(r#"h\.?(?:ora)?s?"#)?,
             |time, _| Ok(time.value().clone().not_latent())
    );
    b.rule_2("a las <time-of-day>",
             b.reg(r#"(?:al?)(?: las?)?|las?"#)?,
             time_check!(form!(Form::TimeOfDay(_))),
             |_, tod| Ok(tod.value().clone().not_latent())
    );
    b.rule_3("a las <hour-min>(time-of-day)",
             b.reg(r#"(?:(?:al?)(?: las?)?|las?)"#)?,
             time_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"horas?"#)?,
             |_, tod, _| Ok(tod.value().clone())
    );
    b.rule_1_terminal("hh(:|.|h)mm (time-of-day)",
                      b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:h\.]([0-5]\d)"#)?,
                      |text_match| helpers::hour_minute(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?,
                          true
                      )
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
             time_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"([ap])\.?m?\.?"#)?,
             |a, text_match| {
                 let day_period = if text_match.group(1) == "a" {
                     helpers::hour(0, false)?.span_to(&helpers::hour(12, false)?, false)?
                 } else {
                     helpers::hour(12, false)?.span_to(&helpers::hour(0, false)?, false)?
                 };
                 Ok(a.value().intersect(&day_period)?.form(Form::TimeOfDay(None)))
             }
    );
    b.rule_1_terminal("quarter (relative minutes)",
                      b.reg(r#"cuarto"#)?,
                      |_| Ok(RelativeMinuteValue(15))
    );
    b.rule_1_terminal("half (relative minutes)",
                      b.reg(r#"y media"#)?,
                      |_| Ok(RelativeMinuteValue(30))
    );
    b.rule_1_terminal("3 quarter (relative minutes)",
                      b.reg(r#"(3|tres) cuartos?"#)?,
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
    b.rule_2("<hour-of-day> <integer> (as relative minutes)",
             time_check!(form!(Form::TimeOfDay(Some(_)))),
             relative_minute_check!(),
             |time, relative_minute| helpers::hour_relative_minute(
                 time.value().form_time_of_day()?.full_hour,
                 relative_minute.value().0,
                 time.value().form_time_of_day()?.is_12_clock)
    );
    b.rule_3("<hour-of-day> minus <integer> (as relative minutes)",
             time_check!(form!(Form::TimeOfDay(Some(_)))),
             b.reg(r#"menos\s?"#)?,
             relative_minute_check!(),
             |time, _, relative_minute| helpers::hour_relative_minute(
                 time.value().form_time_of_day()?.full_hour,
                 -1 * relative_minute.value().0,
                 time.value().form_time_of_day()?.is_12_clock)
    );
    b.rule_3("<hour-of-day> and <relative minutes>",
             time_check!(form!(Form::TimeOfDay(Some(_)))),
             b.reg(r#"y"#)?,
             relative_minute_check!(),
             |time, _, relative_minute| helpers::hour_relative_minute(
                 time.value().form_time_of_day()?.full_hour,
                 relative_minute.value().0,
                 time.value().form_time_of_day()?.is_12_clock)
    );
    b.rule_1_terminal("dd[/-.]mm[/-.]yyyy",
                      b.reg(r#"(3[01]|[12]\d|0?[1-9])[-/.](0?[1-9]|1[0-2])[-/.](\d{2,4})"#)?,
                      |text_match| helpers::ymd(
                          text_match.group(3).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(1).parse()?
                      )
    );
    b.rule_1_terminal("yyyy-mm-dd",
                      b.reg(r#"(\d{2,4})-(0?[1-9]|1[0-2])-(3[01]|[12]\d|0?[1-9])"#)?,
                      |text_match| helpers::ymd(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(3).parse()?
                      )
    );
    b.rule_1_terminal("dd[/-]mm",
                      b.reg(r#"(3[01]|[12]\d|0?[1-9])[/-](0?[1-9]|1[0-2])"#)?,
                      |text_match| helpers::month_day(
                          text_match.group(2).parse()?,
                          text_match.group(1).parse()?
                      )
    );
    b.rule_1_terminal("morning",
                      b.reg(r#"ma(?:ñ|n)ana"#)?,
                      |_| Ok(helpers::hour(4, false)?.span_to(&helpers::hour(12, false)?, false)?
                          .form(Form::PartOfDay)
                          .latent())
    );
    b.rule_1_terminal("afternoon",
                      b.reg(r#"tarde"#)?,
                      |_| Ok(helpers::hour(12, false)?.span_to(&helpers::hour(19, false)?, false)?
                          .form(Form::PartOfDay)
                          .latent())
    );
    b.rule_1_terminal("del mediodía",
                      b.reg(r#"del mediod[ií]a"#)?,
                      |_| Ok(helpers::hour(12, false)?.span_to(&helpers::hour(17, false)?, false)?
                          .form(Form::PartOfDay)
                          .latent())
    );
    b.rule_1_terminal("evening",
                      b.reg(r#"noche"#)?,
                      |_| Ok(helpers::hour(18, false)?.span_to(&helpers::hour(0, false)?, false)?
                          .form(Form::PartOfDay)
                          .latent())
    );
    b.rule_2("in the <part-of-day>",
             b.reg(r#"(?:a|en|de|por) la"#)?,
             time_check!(form!(Form::PartOfDay)),
             |_, pod| Ok(pod.value().clone().not_latent())
    );
    b.rule_2("this <part-of-day>",
             b.reg(r#"est(?:e|a)"#)?,
             time_check!(form!(Form::PartOfDay)),
             |_, pod| Ok(helpers::cycle_nth(Grain::Day, 0)?
                 .intersect(pod.value())?
                 .form(Form::PartOfDay))
    );
    b.rule_2("<time-of-day> <part-of-day>",
             time_check!(),
             time_check!(form!(Form::PartOfDay)),
             |time, pod| time.value().intersect(pod.value())
    );
    b.rule_2("<dim time> de la tarde",
             time_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:a|en|de) la tarde"#)?,
             |time, _| {
                 let period = helpers::hour(12, false)?
                     .span_to(&helpers::hour(21, false)?, false)?
                     .form(Form::PartOfDay)
                     .latent();
                 time.value().intersect(&period)
             }
    );
    b.rule_2("<dim time> de la manana",
             time_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:a|en|de) la ma(?:ñ|n)ana"#)?,
             |time, _| {
                 let period = helpers::hour(0, false)?
                     .span_to(&helpers::hour(12, false)?, false)?
                     .form(Form::PartOfDay)
                     .latent();
                 time.value().intersect(&period)
             }
    );
    b.rule_3("<integer> in the <part-of-day>",
             time_check!(form!(Form::PartOfDay)),
             b.reg(r#"(?:a|en|de|por) la"#)?,
             time_check!(),
             |pod, _, time| time.value().intersect(pod.value())
    );
    b.rule_1_terminal("week-end",
                      b.reg(r#"week[ -]?end|fin de semana"#)?,
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
    b.rule_2("el <time>",
             b.reg(r#"d?el"#)?,
             time_check!(|time: &TimeValue| !time.latent),
             |_, time| Ok(time.value().clone())
    );
    b.rule_5("dd-dd <month>(interval)",
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             b.reg(r#"\-|al?"#)?,
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             b.reg(r#"de"#)?,
             time_check!(form!(Form::Month(_))),
             |a, _, b, _, month| {
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
             b.reg(r#"de"#)?,
             time_check!(form!(Form::Month(_))),
             |_, a, _, b, _, month| {
                 let start = month.value().intersect(&helpers::day_of_month(a.group(1).parse()?)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(b.group(1).parse()?)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_3("<datetime> - <datetime> (interval)",
             time_check!(|time: &TimeValue| !time.latent),
             b.reg(r#"\-|al?"#)?,
             time_check!(|time: &TimeValue| !time.latent),
             |a, _, b| a.value().span_to(b.value(), false)
    );

    b.rule_4("<datetime> - <datetime> (interval)",
             b.reg(r#"del?"#)?,
             time_check!(),
             b.reg(r#"\-|al?"#)?,
             time_check!(),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );
    b.rule_4("entre <datetime> et <datetime> (interval)",
             b.reg(r#"entre"#)?,
             time_check!(),
             b.reg(r#"y"#)?,
             time_check!(),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );
    b.rule_2("dentro de <duration>",
             b.reg(r#"dentro de"#)?,
             duration_check!(),
             |_, duration| helpers::cycle_nth(Grain::Second, 0)?.span_to(&duration.value().in_present()?, false)
    );
    Ok(())
}

pub fn rules_temperature(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1("number as temp", number_check!(), |a| {
        Ok(TemperatureValue {
            value: a.value().value(),
            unit: None,
            latent: true,
        })
    });
    b.rule_2("<latent temp> temp",
             temperature_check!(),
             b.reg(r#"(?:grados?)|°"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("degree"),
                     latent: false,
                 })
             });
    b.rule_2("<temp> Celcius",
             temperature_check!(),
             b.reg(r#"(?:cent(?:i|í)grados?|c(?:el[cs]?(?:ius)?)?\.?)"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("celsius"),
                     latent: false,
                 })
             });
    b.rule_2("<temp> Fahrenheit",
             temperature_check!(),
             b.reg(r#"f(?:ah?reh?n(?:h?eit)?)?\.?"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("fahrenheit"),
                     latent: false,
                 })
             });
    b.rule_2("<latent temp> temp bajo cero",
             temperature_check!(),
             b.reg(r#"(?:(?:grados?)|°)?(?: bajo cero)"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: -1.0 * a.value().value,
                     latent: false,
                     ..*a.value()
                 })
             });
    Ok(())
}

pub fn rules_numbers(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("number (0..15)",
                      b.reg(r#"((?:c|z)ero|un(?:o|a)?|dos|tr(?:é|e)s|cuatro|cinco|s(?:e|é)is|siete|ocho|nueve|die(?:z|s)|once|doce|trece|catorce|quince)"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "cero" => 0,
                              "zero" => 0,
                              "un" => 1,
                              "uno" => 1,
                              "una" => 1,
                              "dos" => 2,
                              "tres" => 3,
                              "trés" => 3,
                              "cuatro" => 4,
                              "cinco" => 5,
                              "seis" => 6,
                              "séis" => 6,
                              "siete" => 7,
                              "ocho" => 8,
                              "nueve" => 9,
                              "diez" => 10,
                              "dies" => 10,
                              "once" => 11,
                              "doce" => 12,
                              "trece" => 13,
                              "catorce" => 14,
                              "quince" => 15,
                              _ => return Err(RuleErrorKind::Invalid.into()),
                          };
                          IntegerValue::new(value)
                      }
    );
    b.rule_1_terminal("number (20..90)",
                      b.reg(r#"(veinte|treinta|cuarenta|cincuenta|sesenta|setenta|ochenta|noventa)"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "veinte" => 20,
                              "treinta" => 30,
                              "cuarenta" => 40,
                              "cincuenta" => 50,
                              "sesenta" => 60,
                              "setenta" => 70,
                              "ochenta" => 80,
                              "noventa" => 90,
                              _ => return Err(RuleErrorKind::Invalid.into()),
                          };
                          IntegerValue::new(value)
                      });
    b.rule_3("number (16..19)",
             integer_check_by_range!(0, 10),
             b.reg(r#"y"#)?,
             integer_check_by_range!(6, 9),
             |_, _, a| IntegerValue::new(a.value().value + 10));
    b.rule_3("number (21..29 31..39 41..49 51..59 61..69 71..79 81..89 91..99)",
             integer_check_by_range!(20, 90, |integer: &IntegerValue| integer.value % 10 == 0),
             b.reg(r#"y"#)?,
             integer_check_by_range!(1, 9),
             |a, _, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_1_terminal("number (16..19 21..29)",
                      b.reg(r#"(die(?:c|s)is(?:é|e)is|diecisiete|dieciocho|diecinueve|veintiun(?:o|a)|veintidos|veintitr(?:é|e)s|veinticuatro|veinticinco|veintis(?:é|e)is|veintisiete|veintiocho|veintinueve)"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "dieciseis" => 16,
                              "diesiseis" => 16,
                              "diesiséis" => 16,
                              "dieciséis" => 16,
                              "diecisiete" => 17,
                              "dieciocho" => 18,
                              "diecinueve" => 19,
                              "veintiuno" => 21,
                              "veintiuna" => 21,
                              "veintidos" => 22,
                              "veintitres" => 23,
                              "veintitrés" => 23,
                              "veinticuatro" => 24,
                              "veinticinco" => 25,
                              "veintiseis" => 26,
                              "veintiséis" => 26,
                              "veintisiete" => 27,
                              "veintiocho" => 28,
                              "veintinueve" => 29,
                              _ => return Err(RuleErrorKind::Invalid.into())
                          };
                          IntegerValue::new(value)
                      });
    b.rule_1_terminal("number 100..1000",
                      b.reg(r#"(cien(?:to)?s?|doscientos|trescientos|cuatrocientos|quinientos|seiscientos|setecientos|ochocientos|novecientos|mil)"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "cien" => 100,
                              "cientos" => 100,
                              "ciento" => 100,
                              "doscientos" => 200,
                              "trescientos" => 300,
                              "cuatrocientos" => 400,
                              "quinientos" => 500,
                              "seiscientos" => 600,
                              "setecientos" => 700,
                              "ochocientos" => 800,
                              "novecientos" => 900,
                              "mil" => 1000,
                              _ => return Err(RuleErrorKind::Invalid.into())
                          };
                          IntegerValue::new(value)
                      });
    b.rule_3("numbers 200..999",
             integer_check_by_range!(2, 9),
             integer_check_by_range!(100, 100),
             integer_check_by_range!(0, 99),
             |a, b, c| IntegerValue::new(a.value().value * b.value().value + c.value().value));
    b.rule_1_terminal("integer (numeric)",
                      b.reg(r#"(\d{1,18})"#)?,
                      |text_match| IntegerValue::new(text_match.group(0).parse()?));
    b.rule_1_terminal("integer with thousands separator .",
                      b.reg(r#"(\d{1,3}(\.\d\d\d){1,5})"#)?,
                      |text_match| {
                          let reformatted_string = text_match.group(1).replace(".", "");
                          let value: i64 = reformatted_string.parse()?;
                          IntegerValue::new(value)
                      });
    b.rule_1_terminal("decimal number",
                      b.reg(r#"(\d*,\d+)"#)?,
                      |text_match| {
                          let reformatted_string = text_match.group(1).replace(",", ".");
                          let value: f32 = reformatted_string.parse()?;
                          FloatValue::new(value)
                      });
    b.rule_3("number dot number",
             number_check!(|number: &NumberValue| !number.prefixed()),
             b.reg(r#"punto"#)?,
             number_check!(|number: &NumberValue| !number.suffixed()),
             |a, _, b| {
                 Ok(FloatValue {
                     value: b.value().value() * 0.1 + a.value().value(),
                     ..FloatValue::default()
                 })
             });
    b.rule_1_terminal("decimal with thousands separator",
                      b.reg(r#"(\d+(\.\d\d\d)+,\d+)"#)?,
                      |text_match| {
                          let reformatted_string = text_match.group(1).replace(".", "").replace(",", ".");
                          let value: f32 = reformatted_string.parse()?;
                          FloatValue::new(value)
                      });
    b.rule_2("numbers prefix with -, negative or minus",
             b.reg(r#"-|menos"#)?,
             number_check!(|number: &NumberValue| !number.prefixed()),
             |_, a| -> RuleResult<NumberValue> {
                 Ok(match a.value().clone() {
                     // checked
                     NumberValue::Integer(integer) => {
                         IntegerValue {
                             value: integer.value * -1,
                             prefixed: true,
                             ..integer
                         }
                             .into()
                     }
                     NumberValue::Float(float) => {
                         FloatValue {
                             value: float.value * -1.0,
                             prefixed: true,
                             ..float
                         }
                             .into()
                     }
                 })
             });
    b.rule_2("numbers suffixes (K, M, G)",
             number_check!(|number: &NumberValue| !number.suffixed()),
             b.reg_neg_lh(r#"([kmg])"#, r#"^[\W\$€]"#)?,
             |a, text_match| -> RuleResult<NumberValue> {
                 let multiplier = match text_match.group(0).as_ref() {
                     "k" => 1000,
                     "m" => 1000000,
                     "g" => 1000000000,
                     _ => return Err(RuleErrorKind::Invalid.into()),
                 };
                 Ok(match a.value().clone() {
                     // checked
                     NumberValue::Integer(integer) => {
                         IntegerValue {
                             value: integer.value * multiplier,
                             suffixed: true,
                             ..integer
                         }
                             .into()
                     }
                     NumberValue::Float(float) => {
                         let product = float.value * (multiplier as f32);
                         if product.floor() == product {
                             IntegerValue {
                                 value: product as i64,
                                 suffixed: true,
                                 ..IntegerValue::default()
                             }
                                 .into()
                         } else {
                             FloatValue {
                                 value: product,
                                 suffixed: true,
                                 ..float
                             }
                                 .into()
                         }
                     }
                 })
             });
    b.rule_1_terminal("ordinals (primero..10)",
                      b.reg(r#"(primer|tercer(?:os?|as?)?|(?:primer|segund|cuart|quint|sext|s[eé]ptim|octav|noven|d[eé]cim)(?:os?|as?))"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "primer" => 1,
                              "primero" => 1,
                              "segundo" => 2,
                              "tercero" => 3,
                              "tercer" => 3,
                              "cuarto" => 4,
                              "quinto" => 5,
                              "sexto" => 6,
                              "séptimo" => 7,
                              "septimo" => 7,
                              "octavo" => 8,
                              "noveno" => 9,
                              "décimo" => 10,
                              "decimo" => 10,
                              "primeros" => 1,
                              "segundos" => 2,
                              "terceros" => 3,
                              "cuartos" => 4,
                              "quintos" => 5,
                              "sextos" => 6,
                              "séptimos" => 7,
                              "septimos" => 7,
                              "octavos" => 8,
                              "novenos" => 9,
                              "décimos" => 10,
                              "decimos" => 10,
                              "primera" => 1,
                              "segunda" => 2,
                              "tercera" => 3,
                              "cuarta" => 4,
                              "quinta" => 5,
                              "sexta" => 6,
                              "séptima" => 7,
                              "septima" => 7,
                              "octava" => 8,
                              "novena" => 9,
                              "décima" => 10,
                              "decima" => 10,
                              "primeras" => 1,
                              "segundas" => 2,
                              "terceras" => 3,
                              "cuartas" => 4,
                              "quintas" => 5,
                              "sextas" => 6,
                              "séptimas" => 7,
                              "septimas" => 7,
                              "octavas" => 8,
                              "novenas" => 9,
                              "décimas" => 10,
                              "decimas" => 10,
                              _ => return Err(RuleErrorKind::Invalid.into())
                          };
                          Ok(OrdinalValue::new(value))
                      });
    Ok(())
}
