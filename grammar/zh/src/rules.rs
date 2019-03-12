use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::helpers;
use rustling_ontology_moment::{Weekday, Grain, PeriodComp};


pub fn rules_duration(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("second (unit-of-duration)",
                      b.reg(r#"秒(?:钟|鐘)?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Second))
    );

    b.rule_1_terminal("minute (unit-of-duration)",
                      b.reg(r#"分(?:钟|鐘)?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Minute))
    );

    b.rule_1_terminal("hour (unit-of-duration)",
                      b.reg(r#"小时|小時|鐘頭?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Hour))
    );

    b.rule_1_terminal("day (unit-of-duration)",
                      b.reg(r#"天|日"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Day))
    );

    b.rule_1_terminal("week (unit-of-duration)",
                      b.reg(r#"周|週|礼拜|禮拜|星期"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Week))
    );

    b.rule_1_terminal("month (unit-of-duration)",
                      b.reg(r#"月"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Month))
    );

    b.rule_1_terminal("year (unit-of-duration)",
                      b.reg(r#"年"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Year))
    );

    b.rule_2("<integer> <unit-of-duration>",
             integer_check_by_range!(0),
             unit_of_duration_check!(),
             |integer, uod| Ok(DurationValue::new(PeriodComp::new(uod.value().grain, integer.value().value).into()))
    );

    Ok(())
}

pub fn rules_cycle(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("second (cycle)",
                      b.reg(r#"秒(?:钟|鐘)?"#)?,
                      |_| CycleValue::new(Grain::Second)
    );

    b.rule_1_terminal("minute (cycle)",
                      b.reg(r#"分(?:钟|鐘)?"#)?,
                      |_| CycleValue::new(Grain::Minute)
    );

    b.rule_1_terminal("hour (cycle)",
                      b.reg(r#"小时|小時|鐘頭?"#)?,
                      |_| CycleValue::new(Grain::Hour)
    );

    b.rule_1_terminal("day (cycle)",
                      b.reg(r#"天|日"#)?,
                      |_| CycleValue::new(Grain::Day)
    );

    b.rule_1_terminal("week (cycle)",
                      b.reg(r#"周|週|礼拜|禮拜|星期"#)?,
                      |_| CycleValue::new(Grain::Week)
    );
    b.rule_1_terminal("month (cycle)",
                      b.reg(r#"月"#)?,
                      |_| CycleValue::new(Grain::Month)
    );

    b.rule_1_terminal("year (cycle)",
                      b.reg(r#"年"#)?,
                      |_| CycleValue::new(Grain::Year)
    );

    Ok(())
}


pub fn rules_datetime(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("named-day",
                      b.reg(r#"(?:星期|周|(?:礼|禮)拜|週)一"#)?,
                      |_| helpers::day_of_week(Weekday::Mon)
    );

    b.rule_1_terminal("named-day",
                      b.reg(r#"(?:星期|周|(?:礼|禮)拜|週)二"#)?,
                      |_| helpers::day_of_week(Weekday::Tue)
    );

    b.rule_1_terminal("named-day",
                      b.reg(r#"(?:星期|周|(?:礼|禮)拜|週)三"#)?,
                      |_| helpers::day_of_week(Weekday::Wed)
    );

    b.rule_1_terminal("named-day",
                      b.reg(r#"(?:星期|周|(?:礼|禮)拜|週)四"#)?,
                      |_| helpers::day_of_week(Weekday::Thu)
    );

    b.rule_1_terminal("named-day",
                      b.reg(r#"(?:星期|周|(?:礼|禮)拜|週)五"#)?,
                      |_| helpers::day_of_week(Weekday::Fri)
    );

    b.rule_1_terminal("named-day",
                      b.reg(r#"(?:星期|周|(?:礼|禮)拜|週)六"#)?,
                      |_| helpers::day_of_week(Weekday::Sat)
    );

    b.rule_1_terminal("named-day",
                      b.reg(r#"星期日|星期天|礼拜天|周日|禮拜天|週日|禮拜日"#)?,
                      |_| helpers::day_of_week(Weekday::Sun)
    );

    b.rule_1_terminal("named-month",
                      b.reg(r#"一月份?"#)?,
                      |_| helpers::month(1)
    );

    b.rule_1_terminal("named-month",
                      b.reg(r#"二月份?"#)?,
                      |_| helpers::month(2)
    );

    b.rule_1_terminal("named-month",
                      b.reg(r#"三月份?"#)?,
                      |_| helpers::month(3)
    );

    b.rule_1_terminal("named-month",
                      b.reg(r#"四月份?"#)?,
                      |_| helpers::month(4)
    );

    b.rule_1_terminal("named-month",
                      b.reg(r#"五月份?"#)?,
                      |_| helpers::month(5)
    );

    b.rule_1_terminal("named-month",
                      b.reg(r#"六月份?"#)?,
                      |_| helpers::month(6)
    );

    b.rule_1_terminal("named-month",
                      b.reg(r#"七月份?"#)?,
                      |_| helpers::month(7)
    );

    b.rule_1_terminal("named-month",
                      b.reg(r#"八月份?"#)?,
                      |_| helpers::month(8)
    );

    b.rule_1_terminal("named-month",
                      b.reg(r#"九月份?"#)?,
                      |_| helpers::month(9)
    );

    b.rule_1_terminal("named-month",
                      b.reg(r#"十月份?"#)?,
                      |_| helpers::month(10)
    );

    b.rule_1_terminal("named-month",
                      b.reg(r#"十一月份?"#)?,
                      |_| helpers::month(11)
    );

    b.rule_1_terminal("named-month",
                      b.reg(r#"十二月份?"#)?,
                      |_| helpers::month(12)
    );

    b.rule_1_terminal("the day after tomorrow",
                      b.reg(r#"后天|後天|後日"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 2)
    );

    b.rule_1_terminal("valentine's day",
                      b.reg(r#"情人(?:节|節)"#)?,
                      |_| helpers::month_day(2, 14)
    );

    b.rule_1_terminal("hh:.mm (time-of-day)",
                      b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:.]([0-5]\d)"#)?,
                      |text_match| {
                          let hour: u32 = text_match.group(1).parse()?;
                          let minute: u32 = text_match.group(2).parse()?;
                          helpers::hour_minute(hour, minute, hour < 12)
                      }
    );

    b.rule_1_terminal("new year's day",
                      b.reg(r#"元旦(?:节|節)?"#)?,
                      |_| helpers::month_day(1, 1)
    );

    b.rule_1_terminal("christmas",
                      b.reg(r#"(?:圣诞|聖誕)(?:节|節)?"#)?,
                      |_| helpers::month_day(12, 25)
    );

    b.rule_1_terminal("now",
                      b.reg(r#"现在|此时|此刻|当前|現在|此時|當前|宜家|而家|依家"#)?,
                      |_| helpers::cycle_nth(Grain::Second, 0)
    );

    b.rule_1_terminal("mm/dd",
                      b.reg(r#"(0?[1-9]|1[0-2])/(3[01]|[12]\d|0?[1-9])"#)?,
                      |text_match| {
                          helpers::month_day(text_match.group(1).parse()?,
                                             text_match.group(2).parse()?)
                      }
    );

    b.rule_1("hhmm (military time-of-day)",
             b.reg(r#"((?:[01]?\d)|(?:2[0-3]))([0-5]\d)"#)?,
             |text_match| Ok(helpers::hour_minute(
                 text_match.group(1).parse()?,
                 text_match.group(2).parse()?,
                 false
             )?.latent())
    );

    b.rule_1_terminal("week-end",
                      b.reg(r#"(周|週)末"#)?,
                      |_| {
                          let friday = helpers::day_of_week(Weekday::Fri)?
                              .intersect(&helpers::hour(18, false)?)?;
                          let monday = helpers::day_of_week(Weekday::Mon)?
                              .intersect(&helpers::hour(0, false)?)?;
                          friday.span_to(&monday, false)
                      }
    );

    b.rule_1_terminal("last year",
                      b.reg(r#"(?:去|上)年"#)?,
                      |_| {
                          helpers::cycle_nth(Grain::Year, -1)
                      }
    );

    b.rule_1_terminal("next year",
                      b.reg(r#"明年|下年"#)?,
                      |_| {
                          helpers::cycle_nth(Grain::Year, 1)
                      }
    );

    b.rule_1_terminal("yesterday",
                      b.reg(r#"昨天|昨日|尋日"#)?,
                      |_| {
                          helpers::cycle_nth(Grain::Day, -1)
                      }
    );

    b.rule_1_terminal("yyyy-mm-dd",
                      b.reg(r#"(\d{2,4})-(0?[1-9]|1[0-2])-(3[01]|[12]\d|0?[1-9])"#)?,
                      |text_match| helpers::year_month_day(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(3).parse()?)
    );

    b.rule_1_terminal("morning",
                      b.reg(r#"早上|早晨|朝頭?早"#)?,
                      |_| {
                          Ok(helpers::hour(4, false)?
                              .span_to(&helpers::hour(12, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Morning)))
                      }
    );

    b.rule_1_terminal("last night",
                      b.reg(r#"昨晚|昨天晚上|尋晚"#)?,
                      |_| {
                          let yesterday = helpers::cycle_nth(Grain::Day, -1)?;
                          let night = helpers::hour(18, false)?
                              .span_to(&helpers::hour(0, false)?, false)?;
                          Ok(yesterday.intersect(&night)?.form(Form::PartOfDay(PartOfDayForm::Night)))
                      }
    );

    b.rule_1_terminal("army's day",
                      b.reg(r#"建(?:军节|軍節)"#)?,
                      |_| helpers::month_day(8, 1)
    );

    b.rule_1_terminal("tonight",
                      b.reg(r#"今晚|今天晚上"#)?,
                      |_| {
                          let period = helpers::hour(18, false)?.span_to(&helpers::hour(0, false)?, false)?;
                          Ok(helpers::cycle_nth(Grain::Day, 0)?
                              .intersect(&period)?
                              .form(Form::PartOfDay(PartOfDayForm::Night)))
                      }
    );

    b.rule_1_terminal("tomorrow night",
                      b.reg(r#"明晚|明天晚上|聽晚"#)?,
                      |_| {
                          let tomorrow = helpers::cycle_nth(Grain::Day, 1)?;
                          let night = helpers::hour(18, false)?
                              .span_to(&helpers::hour(0, false)?, false)?;
                          Ok(tomorrow.intersect(&night)?.form(Form::PartOfDay(PartOfDayForm::Night)))
                      }
    );

    b.rule_1_terminal("army's day",
                      b.reg(r#"(?:儿|兒)童(?:节|節)"#)?,
                      |_| helpers::month_day(6, 1)
    );


    b.rule_1_terminal("this year",
                      b.reg(r#"今年"#)?,
                      |_| helpers::cycle_nth(Grain::Year, 0)
    );

    b.rule_1_terminal("women's day",
                      b.reg(r#"(?:妇|婦)女(?:节|節)"#)?,
                      |_| helpers::month_day(3, 8)
    );

    b.rule_1_terminal("evening|night",
                      b.reg(r#"晚上|晚间"#)?,
                      |_| {
                          Ok(helpers::hour(18, false)?
                              .span_to(&helpers::hour(0, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );

    b.rule_1_terminal("mm/dd/yyyy",
                      b.reg(r#"(0?[1-9]|1[0-2])/(3[01]|[12]\d|0?[1-9])/(\d{2,4})"#)?,
                      |text_match| helpers::year_month_day(
                          text_match.group(3).parse()?,
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?)
    );

    b.rule_1_terminal("tomorrow",
                      b.reg(r#"明天|明日|聽日"#)?,
                      |_| {
                          helpers::cycle_nth(Grain::Day, 1)
                      }
    );

    b.rule_1("time-of-day (latent)",
             integer_check_by_range!(0, 23),
             |integer| Ok(helpers::hour(integer.value().value as u32, integer.value().value < 12)?.latent())
    );

    b.rule_2("<time-of-day> o'clock",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"點|点|時"#)?,
             |a, _| Ok(a.value().clone().not_latent())
    );

    b.rule_1("number (as relative minutes)",
             integer_check_by_range!(1, 59),
             |a| Ok(RelativeMinuteValue(a.value().value as i32))
    );

    b.rule_2("number minutes (as relative minutes)",
             integer_check_by_range!(1, 59),
             b.reg(r#"分钟?"#)?,
             |a, _| Ok(RelativeMinuteValue(a.value().value as i32))
    );

    b.rule_3("relative minutes to|till|before <integer> (hour-of-day)",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:点|點)?差"#)?,
             relative_minute_check!(),
             |datetime, _, relative_minute| helpers::hour_relative_minute(
                 datetime.value().form_time_of_day()?.full_hour(),
                 -1 * relative_minute.value().0,
                 datetime.value().form_time_of_day()?.is_12_clock())
    );

    b.rule_3("relative minutes after|past  <integer> (hour-of-day)",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"点|點|过|過"#)?,
             relative_minute_check!(),
             |datetime, _, relative_minute| helpers::hour_relative_minute(
                 datetime.value().form_time_of_day()?.full_hour(),
                 relative_minute.value().0,
                 datetime.value().form_time_of_day()?.is_12_clock())
    );

    b.rule_1_terminal("quarter (relative minutes)",
                      b.reg(r#"一刻"#)?,
                      |_| Ok(RelativeMinuteValue(15))
    );

    b.rule_1_terminal("half (relative minutes)",
                      b.reg(r#"半"#)?,
                      |_| Ok(RelativeMinuteValue(30))
    );


    b.rule_2("this <day-of-week>",
             b.reg(r#"这|這|今(?:个|個)"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             |_, a| a.value().the_nth_not_immediate(0)
    );

    b.rule_4("nth <datetime> of <datetime>",
             datetime_check!(),
             b.reg(r#"的"#)?,
             ordinal_check!(),
             datetime_check!(),
             |a, _, ordinal, b| {
                 a.value().intersect(b.value())?.the_nth(ordinal.value().value - 1)
             }
    );

    b.rule_2("last <datetime>",
             b.reg(r#"去|上(?:个|個)?"#)?,
             datetime_check!(),
             |_, a| {
                 a.value().the_nth(-1)
             }
    );

    b.rule_2("in <duration>",
             b.reg(r#"再"#)?,
             duration_check!(),
             |_, duration| duration.value().in_present()
    );


    b.rule_1_terminal("national day",
                      b.reg(r#"(?:国庆|國慶)(?:节|節)?"#)?,
                      |_| helpers::month_day(10, 1)
    );

    b.rule_4("the <cycle> after <datetime>",
             b.reg(r#"那"#)?,
             cycle_check!(),
             b.reg(r#"之?(?:后|後)"#)?,
             datetime_check!(),
             |_, cycle, _, datetime| helpers::cycle_nth_after(cycle.value().grain, 1, datetime.value())
    );

    b.rule_4("<cycle> before <datetime>",
             b.reg(r#"那"#)?,
             cycle_check!(),
             b.reg(r#"之?前"#)?,
             datetime_check!(),
             |_, cycle, _, datetime| helpers::cycle_nth_after(cycle.value().grain, -1, datetime.value())
    );

    b.rule_1_terminal("noon",
                      b.reg(r#"中午"#)?,
                      |_| helpers::hour(12, false)
    );

    b.rule_1_terminal("today",
                      b.reg(r#"今天|今日"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 0)
    );

    b.rule_2("this|next <day-of-week>",
             b.reg(r#"今(?:个|個)?|明|下(?:个|個)?"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             |_, a| {
                 a.value().the_nth_not_immediate(0)
             }
    );

    b.rule_1_terminal("the day before yesterday",
                      b.reg(r#"前天|前日"#)?,
                      |_| helpers::cycle_nth(Grain::Day, -2)
    );

    b.rule_1_terminal("labor day",
                      b.reg(r#"劳动节|勞動節"#)?,
                      |_| helpers::month_day(5, 1)
    );

    b.rule_2("next <cycle>",
             b.reg(r#"下(?:个|個)?"#)?,
             cycle_check!(),
             |_, a| helpers::cycle_nth(a.value().grain, 1)
    );

    b.rule_2("<duration> from now",
             duration_check!(),
             b.reg(r#"后|後|之後"#)?,
             |a, _| a.value().in_present()
    );

    b.rule_2("last <cycle>",
             b.reg(r#"上(?:个|個)?"#)?,
             cycle_check!(),
             |_, a| helpers::cycle_nth(a.value().grain, -1)
    );

    b.rule_1_terminal("afternoon",
                      b.reg(r#"下午|中午|晏晝"#)?,
                      |_| {
                          Ok(helpers::hour(12, false)?
                              .span_to(&helpers::hour(19, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
                      }
    );

    b.rule_1_terminal("midnight",
                      b.reg(r#"午夜|凌晨|半夜"#)?,
                      |_| helpers::hour(0, false)
    );

    b.rule_2("in|during the <part-of-day>",
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             b.reg(r#"点|點"#)?,
             |datetime, _| Ok(datetime.value().clone().not_latent())
    );

    b.rule_3("intersect by \",\"",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             b.reg(r#","#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |a, _, b| a.value().intersect(b.value())
    );

    b.rule_2("year (numeric with year symbol)",
             integer_check_by_range!(1000, 9999),
             b.reg(r#"年"#)?,
             |integer, _| helpers::year(integer.value().value as i32)
    );

    b.rule_2("<duration> ago",
             duration_check!(),
             b.reg(r#"之?前"#)?,
             |a, _| a.value().ago()
    );

    b.rule_3("last n <cycle>",
             b.reg(r#"上|前"#)?,
             integer_check_by_range!(1, 9999),
             cycle_check!(),
             |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );

    b.rule_3("n <cycle> last",
             integer_check_by_range!(1, 9999),
             cycle_check!(),
             b.reg(r#"之?前"#)?,
             |integer, cycle, _| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );

    b.rule_2("intersect",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |a, b| a.value().intersect(b.value())
    );

    b.rule_3("nth <datetime> of <datetime>",
             datetime_check!(),
             ordinal_check!(),
             datetime_check!(),
             |a, ordinal, b| {
                 b.value().intersect(a.value())?.the_nth(ordinal.value().value - 1)
             }
    );

    b.rule_2("<datetime> <part-of-day>",
             datetime_check!(),
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             |datetime, part_of_day| part_of_day.value().intersect(datetime.value())
    );

    b.rule_2("next <datetime>",
             b.reg(r#"明|下(?:个|個)?"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |_, a| {
                 a.value().the_nth(0)
             }
    );

    b.rule_3("next n <cycle>",
             b.reg(r#"下|后|後"#)?,
             integer_check_by_range!(1, 9999),
             cycle_check!(),
             |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );

    b.rule_3("next n <cycle>",
             integer_check_by_range!(1, 9999),
             cycle_check!(),
             b.reg(r#"下|之?后|之?後"#)?,
             |integer, cycle, _| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );

    b.rule_2("this <cycle>",
             b.reg(r#"(?:这|這)一?|今個"#)?,
             cycle_check!(),
             |_, a| helpers::cycle_nth(a.value().grain, 0)
    );

    b.rule_2("this <datetime>",
             b.reg(r#"今(?:个|個)?|这个?|這個?"#)?,
             datetime_check!(),
             |_, a| {
                 a.value().the_nth(0)
             }
    );

    b.rule_2("<time-of-day> am|pm",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"([ap])(?:\s|\.)?m?\.?"#)?,
             |a, text_match| {
                 let day_period = if text_match.group(1) == "a" {
                     helpers::hour(0, false)?.span_to(&helpers::hour(12, false)?, false)?
                 } else {
                     helpers::hour(12, false)?.span_to(&helpers::hour(0, false)?, false)?
                 };
                 Ok(a.value().intersect(&day_period)?.form(a.value().form.clone()))
             }
    );

    b.rule_3("<named-month> <day-of-month> (non ordinal)",
             datetime_check!(form!(Form::Month(_))),
             integer_check_by_range!(1, 31),
             b.reg(r#"号|號|日"#)?,
             |a, integer, _| {
                 a.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
             }
    );

    b.rule_2("last <datetime>",
             b.reg(r#"上"#)?,
             datetime_check!(),
             |_, a| {
                 a.value().the_nth(-1)
             }
    );

    b.rule_2("<part-of-day> <datetime>",
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             datetime_check!(),
             |part_of_day, datetime| part_of_day.value().intersect(datetime.value())
    );

    b.rule_2("month (numeric with month symbol)",
             integer_check_by_range!(1, 12),
             b.reg(r#"月"#)?,
             |integer, _| Ok(helpers::month(integer.value().value as u32)?.latent())
    );

    b.rule_2("absorption of , after named day",
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#","#)?,
             |a, _| Ok(a.value().clone())
    );

    Ok(())
}


pub fn rules_temperature(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1("number as temp",
             number_check!(),
             |a| {
                 Ok(TemperatureValue {
                     value: a.value().value(),
                     unit: None,
                     latent: true,
                 })
             });

    b.rule_2("<latent temp> degrees",
             temperature_check!(),
             b.reg(r#"度|°"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("degree"),
                     latent: false,
                 })
             });

    b.rule_2("<temp> Celcius",
             temperature_check!(),
             b.reg(r#"(?:摄|攝)氏(?:°|度)|°c"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("celsius"),
                     latent: false,
                 })
             });

    b.rule_3("Celcius <temp>",
             b.reg(r#"(?:摄|攝)氏"#)?,
             temperature_check!(),
             b.reg(r#"度|°"#)?,
             |_, b, _| {
                 Ok(TemperatureValue {
                     value: b.value().value,
                     unit: Some("celsius"),
                     latent: false,
                 })
             }
    );

    b.rule_2("<temp> Fahrenheit",
             temperature_check!(),
             b.reg(r#"(?:华|華)氏(?:°|度)|°f"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("fahrenheit"),
                     latent: false,
                 })
             });

    b.rule_3("Fahrenheit <temp>",
             b.reg(r#"(?:华|華)氏"#)?,
             temperature_check!(),
             b.reg(r#"度|°"#)?,
             |_, b, _| {
                 Ok(TemperatureValue {
                     value: b.value().value,
                     unit: Some("fahrenheit"),
                     latent: false,
                 })
             }
    );

    Ok(())
}


pub fn rules_numbers(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("integer (0..10)",
                      b.reg(r#"(〇|零|一|二|两|兩|三|四|五|六|七|八|九|十)(?:个|個)?"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "〇" => 0,
                              "零" => 0,
                              "一" => 1,
                              "二" => 2,
                              "两" => 2,
                              "兩" => 2,
                              "三" => 3,
                              "四" => 4,
                              "五" => 5,
                              "六" => 6,
                              "七" => 7,
                              "八" => 8,
                              "九" => 9,
                              "十" => 10,
                              _ => return Err(RuleError::Invalid.into())
                          };
                          IntegerValue::new_with_grain(value, 1)
                      });

    b.rule_1_terminal(
        "integer (numeric)",
        b.reg(r#"(\d{1,18})"#)?,
        |text_match| IntegerValue::new(text_match.group(0).parse()?));

    b.rule_1("decimal number", b.reg(r#"(\d*\.\d+)"#)?, |text_match| {
        let value: f32 = text_match.group(0).parse()?;
        Ok(FloatValue {
            value: value,
            ..FloatValue::default()
        })
    });

    b.rule_2("numbers prefix with -, negative or minus",
             b.reg(r#"-|负\s?|負\s?"#)?,
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

    b.rule_1_terminal("decimal with thousands separator",
                      b.reg(r#"(\d+(,\d\d\d)+\.\d+)"#)?,
                      |text_match| {
                          let reformatted_string = text_match.group(1).replace(",", "");
                          let value: f32 = reformatted_string.parse()?;
                          Ok(FloatValue {
                              value: value,
                              ..FloatValue::default()
                          })
                      });

    b.rule_2("<number>个",
             number_check!(),
             b.reg(r#"个"#)?,
             |number, _| Ok(number.value().clone()));

    b.rule_2("integer (20..90)",
             integer_check_by_range!(2, 9),
             b.reg(r#"十"#)?,
             |a, _| {
                 Ok(IntegerValue {
                     value: a.value().value * 10,
                     ..a.value().clone()
                 })
             });

    b.rule_2("numbers suffixes (K, M, G)",
             number_check!(|number: &NumberValue| !number.suffixed()),
             b.reg_neg_lh(r#"([kmg])"#, r#"^[^\W\$€元¥(?:人民币)]"#)?,
             |a, text_match| -> RuleResult<NumberValue> {
                 let multiplier = match text_match.group(0).as_ref() {
                     "k" => 1000,
                     "m" => 1000000,
                     "g" => 1000000000,
                     _ => return Err(RuleError::Invalid.into()),
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

    b.rule_2("integer 21..99",
             integer_check_by_range!(10, 90, |integer: &IntegerValue| integer.value % 10 == 0),
             integer_check_by_range!(1, 9),
             |a, b| IntegerValue::new(a.value().value + b.value().value));

    b.rule_2("integer (11..19)",
             b.reg(r#"十"#)?,
             integer_check_by_range!(1, 9),
             |_, b| IntegerValue::new(10 + b.value().value));

    b.rule_1("integer with thousands separator, ",
             b.reg(r#"(\d{1,3}(,\d\d\d){1,5})"#)?,
             |text_match| {
                 let reformatted_string = text_match.group(1).replace(",", "");
                 let value: i64 = reformatted_string.parse()?;
                 Ok(IntegerValue {
                     value: value,
                     ..IntegerValue::default()
                 })
             });

    b.rule_2("ordinal (digits)",
             b.reg(r#"第"#)?,
             integer_check!(),
             |_, b| {
                 Ok(OrdinalValue::new(b.value().value).prefixed())
             }
    );

    Ok(())
}
