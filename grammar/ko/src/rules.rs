use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::dimension::Precision::*;
use rustling_ontology_values::helpers;
use regex::Regex;
use rustling_ontology_moment::{Weekday, Grain, PeriodComp};

pub fn rules_finance(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("intersect (X cents)",
             amount_of_money_check!(),
             amount_of_money_check!(|money: &AmountOfMoneyValue| money.unit == Some("cent")),
             |a, b| helpers::compose_money(a.value(), b.value())
    );
    b.rule_1_terminal("KRW",
                      b.reg(r#"₩|원|krw"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("KRW") })
    );
    b.rule_1_terminal("$",
                      b.reg(r#"\$|달러|불"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("$") })
    );
    b.rule_1_terminal("cent",
                      b.reg(r#"cents?|센[트|츠]|c|¢"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("cent") })
    );
    b.rule_1_terminal("EUR",
                      b.reg(r#"€|유로|euro?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("EUR") })
    );
    b.rule_1_terminal("£",
                      b.reg(r#"£|파운드|영국파운드"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("£") })
    );
    b.rule_1_terminal("GBP",
                      b.reg(r#"gbp"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("GBP") })
    );
    b.rule_1_terminal("AUD",
                      b.reg(r#"aud|호주달러"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("AUD") })
    );
    b.rule_1_terminal("USD",
                      b.reg(r#"us[d\$]"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("USD") })
    );
    b.rule_1_terminal("PTS",
                      b.reg(r#"pta?s?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("PTS") })
    );
    b.rule_1_terminal("INR",
                      b.reg(r#"inr|rs(?:. )?|(?:R|r)upees?|루피|인도루피"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("INR") })
    );
    b.rule_1_terminal("AED", //  Emirates Currency
                      b.reg(r#"디르함|aed|dirhams?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("AED") })
    );
    b.rule_2("<unit> <amount>",
             money_unit!(),
             number_check!(),
             |a, b| {
                 Ok(AmountOfMoneyValue {
                     value: b.value().value(),
                     unit: a.value().unit,
                     ..AmountOfMoneyValue::default()
                 })
             });
    b.rule_2("<amount> <unit>",
             number_check!(),
             money_unit!(),
             |a, b| Ok(AmountOfMoneyValue {
                 value: a.value().value(),
                 unit: b.value().unit,
                 ..AmountOfMoneyValue::default()
             })
    );
    b.rule_2("about <amount-of-money>",
             b.reg(r#"대략|약|대충|얼추"#)?,
             amount_of_money_check!(),
             |_, a| {
                 Ok(AmountOfMoneyValue {
                     precision: Approximate,
                     ..a.value().clone()
                 })
             });
    b.rule_2("<amount-of-money> about",
             amount_of_money_check!(),
             b.reg(r#"정도|쯤"#)?,
             |a, _| {
                 Ok(AmountOfMoneyValue {
                     precision: Approximate,
                     ..a.value().clone()
                 })
             });
    b.rule_2("exactly <amount-of-money>",
             b.reg(r#"딱|정확히"#)?,
             amount_of_money_check!(),
             |_, a| {
                 Ok(AmountOfMoneyValue {
                     precision: Exact,
                     ..a.value().clone()
                 })
             });
    Ok(())
}

pub fn rules_temperature(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1("number as temp",
             number_check!(),
             |a| Ok(TemperatureValue {
                 value: a.value().value(),
                 unit: None,
                 latent: true,
             })
    );
    b.rule_2("below <temp>",
             b.reg(r#"영하"#)?,
             temperature_check!(|temp: &TemperatureValue| !temp.latent),
             |_, temp| {
                 if temp.value().value >= 0.0 {
                     Ok(TemperatureValue {
                         value: -1.0 * temp.value().value,
                         unit: temp.value().unit,
                         latent: false,
                     })
                 } else {
                     Ok(temp.value().clone())
                 }
             }
    );
    b.rule_2("above <temp>",
             b.reg(r#"영상"#)?,
             temperature_check!(|temp: &TemperatureValue| !temp.latent),
             |_, temp| {
                 if temp.value().value <= 0.0 {
                     Ok(TemperatureValue {
                         value: -1.0 * temp.value().value,
                         unit: temp.value().unit,
                         latent: false,
                     })
                 } else {
                     Ok(temp.value().clone())
                 }
             }
    );

    b.rule_2("<latent temp> degrees",
             temperature_check!(),
             b.reg(r#"도|°"#)?,
             |a, _| Ok(TemperatureValue {
                 value: a.value().value,
                 unit: Some("degree"),
                 latent: false,
             })
    );
    b.rule_2("섭씨 <temp> (celsius)",
             b.reg(r#"섭씨"#)?,
             temperature_check!(),
             |_, a| Ok(TemperatureValue {
                 value: a.value().value,
                 unit: Some("celsius"),
                 latent: false,
             })
    );
    b.rule_2("<temp> °C",
             temperature_check!(),
             b.reg(r#"c"#)?,
             |a, _| Ok(TemperatureValue {
                 value: a.value().value,
                 unit: Some("celsius"),
                 latent: false,
             })
    );
    b.rule_2("화씨 <temp>",
             b.reg(r#"화씨"#)?,
             temperature_check!(),
             |_, a| Ok(TemperatureValue {
                 value: a.value().value,
                 unit: Some("fahrenheit"),
                 latent: false,
             })
    );
    b.rule_2("<temp> °F",
             temperature_check!(),
             b.reg(r#"f"#)?,
             |a, _| Ok(TemperatureValue {
                 value: a.value().value,
                 unit: Some("fahrenheit"),
                 latent: false,
             })
    );
    Ok(())
}

pub fn rules_datetime(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("intersect",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |a, b| a.value().intersect(b.value())
    );
    b.rule_3("intersect by \",\"",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             b.reg(r#","#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |a, _, b| a.value().intersect(b.value())
    );
    b.rule_3("intersect by \"의\"",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             b.reg(r#"의"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |a, _, b| a.value().intersect(b.value())
    );
    b.rule_2("<date>에",
             datetime_check!(),
             b.reg(r#"에|때"#)?,
             |datetime, _| Ok(datetime.value().clone())
    );
    b.rule_2("<date>동안",
             datetime_check!(),
             b.reg(r#"동안"#)?,
             |datetime, _| Ok(datetime.value().clone().not_latent())
    );
    b.rule_2("<named-day>에", // on Wed, March 23
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"에"#)?,
             |datetime, _| Ok(datetime.value().clone())
    );
    b.rule_2("<named-month>에", //in September
             datetime_check!(form!(Form::Month(_))),
             b.reg(r#"에"#)?,
             |datetime, _| Ok(datetime.value().clone())
    );
    b.rule_1_terminal("day-of-week",
                      b.reg(r#"(월|화|수|목|금|토|일)(요일|욜)"#)?,
                      |text_match| {
                          let dow = match text_match.group(1).as_ref() {
                              "월" => Weekday::Mon,
                              "화" => Weekday::Tue,
                              "수" => Weekday::Wed,
                              "목" => Weekday::Thu,
                              "금" => Weekday::Fri,
                              "토" => Weekday::Sat,
                              "일" => Weekday::Sun,
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          helpers::day_of_week(dow)
                      }
    );
    b.rule_2("month",
             integer_check_by_range!(1, 12),
             b.reg(r#"월"#)?,
             |integer, _| helpers::month(integer.value().value as u32)
    );
    b.rule_2("day",
             integer_check_by_range!(1, 31),
             b.reg(r#"일"#)?,
             |integer, _| helpers::day_of_month(integer.value().value as u32)
    );
    b.rule_1_terminal("day with korean number - 십일..삼십일일",
                      b.reg(r#"([이|삼]?십[일|이|삼|사|오|육|칠|팔|구]?)일"#)?,
                      |text_match| {
                          fn map_number(s: char) -> i64 {
                              match s {
                                  '일' => 1,
                                  '이' => 2,
                                  '삼' => 3,
                                  '사' => 4,
                                  '오' => 5,
                                  '육' => 6,
                                  '칠' => 7,
                                  '팔' => 8,
                                  '구' => 9,
                                  '십' => 1,
                                  _ => 0,
                              }
                          }

                          fn get_number(s: &str) -> RuleResult<i64> {
                              let regex = Regex::new(r#"(.*십)?(.*)?"#)?;
                              let groups = helpers::find_regex_group(&regex, s)?
                                  .into_iter()
                                  .nth(0)
                                  .ok_or_else(|| format_err!("Regex {:?} has no match for {:?}", regex, s))?
                                  .groups;
                              let number = 10 * groups.get(1).and_then(|g| *g)
                                  .and_then(|g| g.chars().nth(0))
                                  .map(|g| map_number(g))
                                  .unwrap_or(0)
                                  + groups.get(2).and_then(|g| *g)
                                  .and_then(|g| g.chars().nth(0))
                                  .map(|g| map_number(g))
                                  .unwrap_or(0);
                              Ok(number)
                          }
                          let number = get_number(text_match.group(1));
                          helpers::day_of_month(number? as u32)
                      }
    );
    b.rule_1_terminal("day with korean number - 일일..구일",
                      b.reg(r#"([일|이|삼|사|오|육|칠|팔|구])일"#)?,
                      |text_match| {
                          let dom = match text_match.group(1).as_ref() {
                              "일" => 1,
                              "이" => 2,
                              "삼" => 3,
                              "사" => 4,
                              "오" => 5,
                              "육" => 6,
                              "칠" => 7,
                              "팔" => 8,
                              "구" => 9,
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          helpers::day_of_month(dom)
                      }
    );
    b.rule_1_terminal("New Year's Day",
                      b.reg(r#"신정|새해 첫 날"#)?,
                      |_| helpers::month_day(1, 1)
    );

    // b.rule_1("Korean New Year",
    //     b.reg(r#"설날|설|원일|구정"#)?,
    //     based on the lunear calendar which is not supported yet
    // );

    // b.rule_1("Buddha’s Birthday",
    //     b.reg(r#"부처님 오신 날|석존성탄절|석가탄신일|석탄일|석탄절"#)?,
    //     based on the lunear calendar which is not supported yet
    // );

    // b.rule_1("Chuseok",
    //     b.reg(r#"추석|한가위"#)?,
    //     based on the lunear calendar which is not supported yet
    // );

    // b.rule_1("Chuseok Holidays",
    //     b.reg(r#"추석연휴"#),
    //     based on the lunear calendar which is not supported yet
    // );

    b.rule_1_terminal("Independence Movement Day",
                      b.reg(r#"삼일절"#)?,
                      |_| helpers::month_day(3, 1)
    );
    b.rule_1_terminal("Children's Day",
                      b.reg(r#"어린이날"#)?,
                      |_| helpers::month_day(5, 5)
    );
    b.rule_1_terminal("Memorial Day",
                      b.reg(r#"현충일"#)?,
                      |_| helpers::month_day(6, 6)
    );
    b.rule_1_terminal("Constitution Day",
                      b.reg(r#"제헌절"#)?,
                      |_| helpers::month_day(6, 17)
    );
    b.rule_1_terminal("Liberation Day",
                      b.reg(r#"광복절"#)?,
                      |_| helpers::month_day(8, 15)
    );
    b.rule_1_terminal("National Foundation Day",
                      b.reg(r#"개천절"#)?,
                      |_| helpers::month_day(10, 3)
    );
    b.rule_1_terminal("Hangul Day",
                      b.reg(r#"한글날"#)?,
                      |_| helpers::month_day(10, 9)
    );
    b.rule_1_terminal("christmas eve",
                      b.reg(r#"(크리스마스)?이브"#)?,
                      |_| helpers::month_day(12, 24)
    );
    b.rule_1_terminal("christmas",
                      b.reg(r#"크리스마스|성탄절"#)?,
                      |_| helpers::month_day(12, 25)
    );
    b.rule_2("absorption of , after named day",
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#","#)?,
             |dow, _| Ok(dow.value().clone())
    );
    b.rule_1_terminal("Father's day",
                      b.reg(r#"아버지\s?날"#)?,
                      |_| {
                          let sundays_of_june = helpers::month(6)?.intersect(&helpers::day_of_week(Weekday::Sun)?)?;
                          let second_week_of_june = helpers::cycle_nth_after(Grain::Week, 2, &helpers::month_day(6, 1)?)?;
                          sundays_of_june.intersect(&second_week_of_june) // third sunday of June
                      }
    );
    b.rule_1_terminal("Mother's day",
                      b.reg(r#"어머니\s?날"#)?,
                      |_| {
                          let sundays_of_may = helpers::month(5)?.intersect(&helpers::day_of_week(Weekday::Sun)?)?;
                          let first_week_of_may = helpers::cycle_nth_after(Grain::Week, 1, &helpers::month_day(5, 1)?)?;
                          sundays_of_may.intersect(&first_week_of_may) // second sunday of May
                      }
    );
    b.rule_1_terminal("Parents day",
                      b.reg(r"어버이\s?날")?,
                      |_| helpers::month_day(5, 8)
    );
    b.rule_1_terminal("Teachers' day",
                      b.reg(r#"스승의\s?날"#)?,
                      |_| helpers::month_day(5, 15)
    );
    b.rule_1_terminal("Labor Day",
                      b.reg(r#"노동절|노동일|근로자의\s?날"#)?,
                      |_| helpers::month_day(5, 1)
    );
    b.rule_1_terminal("Valentine’s Day",
                      b.reg(r#"발렌타인\s?데이"#)?,
                      |_| helpers::month_day(2, 14)
    );
    b.rule_1_terminal("White Day",
                      b.reg(r#"화이트\s?데이"#)?,
                      |_| helpers::month_day(3, 14)
    );
    b.rule_1_terminal("Coming-of-Age Day",
                      b.reg(r#"성년의\s?날"#)?,
                      |_| {
                          let mondays_of_may = helpers::month(5)?.intersect(&helpers::day_of_week(Weekday::Mon)?)?;
                          let third_week_of_may = helpers::cycle_nth_after(Grain::Week, 3, &helpers::month_day(5, 1)?)?;
                          mondays_of_may.intersect(&third_week_of_may) // third monday of May
                      }
    );
    b.rule_1_terminal("First Dog Days",
                      b.reg(r#"초복"#)?,
                      |_| helpers::month_day(7, 13)
    );
    b.rule_1_terminal("Second Dog Days",
                      b.reg(r#"중복"#)?,
                      |_| helpers::month_day(7, 23)
    );
    b.rule_1_terminal("Last Dog Days",
                      b.reg(r#"말복"#)?,
                      |_| helpers::month_day(8, 12)
    );
    b.rule_1_terminal("Halloween",
                      b.reg(r#"핼러윈\s?데이|핼러윈"#)?,
                      |_| helpers::month_day(10, 31)
    );
    b.rule_1_terminal("Armed Forces Day",
                      b.reg(r#"국군의\s?날"#)?,
                      |_| helpers::month_day(10, 1)
    );
    b.rule_1_terminal("Couple’s Day",
                      b.reg(r#"부부의\s?날"#)?,
                      |_| helpers::month_day(5, 21)
    );
    b.rule_1_terminal("Elderly Day",
                      b.reg(r#"노인의\s?날"#)?,
                      |_| helpers::month_day(10, 2)
    );
    b.rule_1_terminal("Dokdo Day",
                      b.reg(r#"독도의\s?날"#)?,
                      |_| helpers::month_day(10, 25)
    );
    b.rule_1_terminal("now",
                      b.reg(r#"방금|지금|방금|막|이제"#)?,
                      |_| helpers::cycle_nth(Grain::Second, 0)
    );
    b.rule_1_terminal("today",
                      b.reg(r#"오늘|당일|금일"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 0)
    );
    b.rule_1_terminal("tomorrow",
                      b.reg(r#"내일|명일|낼"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 1)
    );
    b.rule_1_terminal("yesterday",
                      b.reg(r#"어제|작일|어저께"#)?,
                      |_| helpers::cycle_nth(Grain::Day, -1)
    );
    b.rule_1_terminal("in two years",
                      b.reg(r#"후년|재명년|내명년"#)?,
                      |_| helpers::cycle_nth(Grain::Year, 2),
    );
    b.rule_1_terminal("in three years",
                      b.reg(r#"내후년|명후년|후후년"#)?,
                      |_| helpers::cycle_nth(Grain::Year, 3),
    );
    b.rule_1_terminal("two years ago",
                      b.reg(r#"재작년"#)?,
                      |_| helpers::cycle_nth(Grain::Year, -2)
    );
    b.rule_1_terminal("three years ago",
                      b.reg(r#"재재작년"#)?,
                      |_| helpers::cycle_nth(Grain::Year, -3)
    );
    b.rule_2("start of week",
             datetime_check!(form!(Form::Cycle(Grain::Week))),
             b.reg(r#"초"#)?,
             |week, _| {
                 let start = week.value().intersect(&helpers::day_of_week(Weekday::Mon)?)?;
                 let end = week.value().intersect(&helpers::day_of_week(Weekday::Tue)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_2("end of week",
             datetime_check!(form!(Form::Cycle(Grain::Week))),
             b.reg(r#"말"#)?,
             |week, _| {
                 let start = week.value().intersect(&helpers::day_of_week(Weekday::Fri)?)?;
                 let end = week.value().intersect(&helpers::day_of_week(Weekday::Sun)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_2("beginning of year",
             datetime_check!(|datetime: &DatetimeValue| {
            match datetime.form {
                Form::Year(_) | Form::Cycle(Grain::Year) => true,
                _ => false
            }
        }),
             b.reg(r#"초"#)?,
             |year, _| {
                 let start = year.value().intersect(&helpers::month(1)?)?;
                 let end = year.value().intersect(&helpers::month(3)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_2("end of year",
             datetime_check!(|datetime: &DatetimeValue| {
            match datetime.form {
                Form::Year(_) | Form::Cycle(Grain::Year) => true,
                _ => false
            }
        }),
             b.reg(r#"말"#)?,
             |year, _| {
                 let start = year.value().intersect(&helpers::month(10)?)?;
                 let end = year.value().intersect(&helpers::month(12)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_2("beginning of month",
             datetime_check!(|datetime: &DatetimeValue| {
            match datetime.form {
                Form::Month(_) | Form::Cycle(Grain::Month) => true,
                _ => false
            }
        }),
             b.reg(r#"초"#)?,
             |month, _| {
                 let start = month.value().intersect(&helpers::day_of_month(1)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(5)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_2("end of month",
             datetime_check!(|datetime: &DatetimeValue| {
            match datetime.form {
                Form::Month(_) | Form::Cycle(Grain::Month) => true,
                _ => false
            }
        }),
             b.reg(r#"말"#)?,
             |month, _| {
                 let start = month.value().intersect(&helpers::day_of_month(25)?)?;
                 let end = helpers::cycle(Grain::Day)?.last_of(month.value())?;
                 start.span_to(&end, true)
             }
    );
    b.rule_2("this <day-of-week>",
             b.reg(r#"이번\s*주?|돌아오는|금주"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             |_, datetime| datetime.value().the_nth(0)
    );
    b.rule_2("this <datetime>",
             b.reg(r#"이번|이|금|올|돌아오는"#)?,
             datetime_check!(),
             |_, datetime| datetime.value().the_nth(0)
    );
    b.rule_2("next <datetime>",
             b.reg(r#"다음|오는"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |_, datetime| datetime.value().the_nth(1)
    );
    b.rule_2("last <datetime>",
             b.reg(r#"전|저번|지난|거"#)?,
             datetime_check!(),
             |_, datetime| datetime.value().the_nth(-1)
    );
    b.rule_3("<datetime> 마지막 <day-of-week>",
             datetime_check!(),
             b.reg(r#"마지막"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             |a, _, b| b.value().last_of(a.value())
    );
    b.rule_3("<datetime> 마지막 <cycle>",
             datetime_check!(),
             b.reg(r#"마지막"#)?,
             cycle_check!(),
             |datetime, _, cycle| cycle.value().last_of(datetime.value())
    );
    b.rule_3("<datetime> nth <datetime> - 3월 첫째 화요일",
             datetime_check!(),
             ordinal_check!(),
             datetime_check!(),
             |a, ordinal, b| a.value()
                 .intersect(b.value())?
                 .the_nth(ordinal.value().value - 1)
    );
    b.rule_4("nth <datetime> - 3월 첫째 화요일",
             datetime_check!(),
             b.reg(r#"의"#)?,
             ordinal_check!(),
             datetime_check!(),
             |a, _, ordinal, b| a.value()
                 .intersect(b.value())?
                 .the_nth(ordinal.value().value - 1)
    );
    b.rule_3("<datetime> nth <cycle> - 3월 첫째 화요일",
             datetime_check!(),
             ordinal_check!(),
             cycle_check!(),
             |datetime, ordinal, cycle| helpers::cycle_nth_after_not_immediate(
                 cycle.value().grain,
                 ordinal.value().value - 1,
                 datetime.value())
    );

    b.rule_4("<datetime> nth of <cycle> - 3월 첫째 화요일",
             datetime_check!(),
             b.reg(r#"의"#)?,
             ordinal_check!(),
             cycle_check!(),
             |datetime, _, ordinal, cycle| helpers::cycle_nth_after_not_immediate(
                 cycle.value().grain,
                 ordinal.value().value - 1,
                 datetime.value())
    );
    b.rule_2("year",
             integer_check_by_range!(1),
             b.reg(r#"년"#)?,
             |integer, _| helpers::year(integer.value().value as i32)
    );
    b.rule_1("time-of-day (latent)",
             integer_check_by_range!(0, 23),
             |integer| Ok(helpers::hour(integer.value().value as u32, true)?.latent())
    );
    b.rule_2("time-of-day",
             integer_check_by_range!(0, 24),
             b.reg(r#"시"#)?,
             |integer, _| helpers::hour(integer.value().value as u32, true)
    );
    b.rule_2("<time-of-day>에",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"에"#)?,
             |datetime, _| Ok(datetime.value().clone().not_latent())
    );
    b.rule_2("<time-of-day> 정각",
             b.reg(r#"정각"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, datetime| Ok(datetime.value().clone().not_latent())
    );
    b.rule_1_terminal("hh:mm",
                      b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:.]([0-5]\d)"#)?,
                      |text_match| helpers::hour_minute(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?,
                          true)
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
    // From "am|pm <time-of-day>" rules in the original grammar version
    b.rule_2("<time-of-day> am",
             b.reg(r#"오전|아침|새벽"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, tod| {
                 let day_period = helpers::hour(0, false)?.span_to(&helpers::hour(12, false)?, false)?;
                 Ok(tod.value().intersect(&day_period)?.form(tod.value().form.clone()))
             }
    );
    // From "am|pm <time-of-day>" rules in the original grammar version
    b.rule_2("<time-of-day> pm",
             b.reg(r#"오후|저녁|밤"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, tod| {
                 let day_period = helpers::hour(12, false)?.span_to(&helpers::hour(0, false)?, false)?;
                 Ok(tod.value().intersect(&day_period)?.form(tod.value().form.clone()))
             }
    );
    b.rule_1_terminal("noon",
                      b.reg(r#"정오|오정|한낮"#)?,
                      |_| helpers::hour(12, false)
    );
    b.rule_1_terminal("midnight|EOD|end of day",
                      b.reg(r#"자정|영시"#)?,
                      |_| helpers::hour(0, false)
    );
    b.rule_1_terminal("half (relative minutes)",
                      b.reg(r#"반"#)?,
                      |_| Ok(RelativeMinuteValue(30))
    );
    b.rule_2("number (as relative minutes)",
             integer_check_by_range!(1, 59),
             b.reg(r#"분"#)?,
             |integer, _| Ok(RelativeMinuteValue(integer.value().value as i32))
    );
    b.rule_2("<hour-of-day> <integer> (as relative minutes)",
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))),
             relative_minute_check!(),
             |tod, relative_minutes| helpers::hour_relative_minute(
                 tod.value().form_time_of_day()?.full_hour(),
                 relative_minutes.value().0,
                 true
             )
    );
    b.rule_2("<hour-of-day> <integer>",
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))),
             integer_check_by_range!(0, 59),
             |tod, integer| helpers::hour_minute(
                 tod.value().form_time_of_day()?.full_hour(),
                 integer.value().value as u32,
                 true
             )
    );
    b.rule_3("<integer> (hour-of-day) relative minutes 전",
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))),
             relative_minute_check!(),
             b.reg(r#"전"#)?,
             |tod, relative_minutes, _| helpers::hour_relative_minute(
                 tod.value().form_time_of_day()?.full_hour(),
                 -1 * relative_minutes.value().0,
                 true
             )
    );
    b.rule_2("seconds",
             integer_check_by_range!(1, 59),
             b.reg(r#"초"#)?,
             |integer, _| helpers::second(integer.value().value as u32)
    );
    // Written dates in numeric formats
    b.rule_1_terminal("yyyy-mm-dd - ISO",
                      b.reg(r#"(\d{4})[-/](0?[1-9]|1[0-2])[-/](3[01]|[12]\d|0?[1-9])"#)?,
                      |text_match| helpers::year_month_day(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(3).parse()?)
    );
    // FIXME: Check if this format should really be supported
    b.rule_1_terminal("yy/mm/dd",
                      b.reg(r#"(\d{2})[-/](0?[1-9]|1[0-2])[-/](3[01]|[12]\d|0?[1-9])"#)?,
                      |text_match| helpers::year_month_day(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(3).parse()?)
    );
    // FIXME: Check if this format should really be supported
    b.rule_1_terminal("dd/mm/yyyy",
                      b.reg(r#"(0?[1-9]|[12]\d|3[01])[-\./](0?[1-9]|1[0-2])[-\./](\d{4})"#)?,
                      |text_match| helpers::year_month_day(
                          text_match.group(3).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(1).parse()?,
                      )
    );
    // FIXME: Check if this format should really be supported
    // Warning:
    // this pattern matches for days: 3[0-2]|[12]\d|0?[1-9]
    // but not 0?[1-9]|[12]\d|3[0-2]
    b.rule_1_terminal("mm/dd",
                      b.reg(r#"(0?[1-9]|1[0-2])/(3[0-2]|[12]\d|0?[1-9])"#)?,
                      |text_match| helpers::month_day(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?)
    );
    // Something like "1975년 7월 14일" would be abbreviated "1975. 7. 14."
    b.rule_1_terminal("yyyy. mm. dd.",
                      b.reg(r#"(\d{4})\. ?(0?[1-9]|1[0-2])\. ?(3[01]|[12]\d|0?[1-9])\."#)?,
                      |text_match| helpers::year_month_day(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(3).parse()?)
    );
    // End of Written dates in numeric formats
    b.rule_1_terminal("early morning",
                      b.reg(r#"이른 아침|조조|아침 일찍"#)?,
                      |_| Ok(helpers::hour(4, false)?
                          .span_to(&helpers::hour(9, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Morning)))
    );
    b.rule_1_terminal("morning",
                      b.reg(r#"아침"#)?,
                      |_| Ok(helpers::hour(4, false)?
                          .span_to(&helpers::hour(12, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Morning)))
    );
    b.rule_1_terminal("morning (latent)",
                      b.reg(r#"오전"#)?,
                      |_| Ok(helpers::hour(4, false)?
                          .span_to(&helpers::hour(12, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Morning)))
    );
    b.rule_1_terminal("late morning (latent)",
                      b.reg(r#"늦은 아침|오전 늦게|아침 늦게|아침 느지막이"#)?,
                      |_| Ok(helpers::hour(11, false)?
                          .span_to(&helpers::hour(12, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Morning)))
    );
    b.rule_1_terminal("early afternoon (latent)",
                      b.reg(r#"이른 오후|낮곁|오후 들어|오후 일찍"#)?,
                      |_| Ok(helpers::hour(12, false)?
                          .span_to(&helpers::hour(16, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
    );
    b.rule_1_terminal("afternoon",
                      b.reg(r#"오후"#)?,
                      |_| Ok(helpers::hour(12, false)?
                          .span_to(&helpers::hour(19, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
    );
    b.rule_1_terminal("late afternoon (latent)",
                      b.reg(r#"늦은 오후|오후 늦게"#)?,
                      |_| Ok(helpers::hour(17, false)?
                          .span_to(&helpers::hour(19, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
    );
    b.rule_1_terminal("early evening (latent)",
                      b.reg(r#"이른 저녁|초저녁|저녁 일찍"#)?,
                      |_| Ok(helpers::hour(18, false)?
                          .span_to(&helpers::hour(21, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Evening)))
    );
    b.rule_1_terminal("evening",
                      b.reg(r#"저녁"#)?,
                      |_| Ok(helpers::hour(18, false)?
                          .span_to(&helpers::hour(0, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Evening)))
    );
    b.rule_1_terminal("late evening (latent)",
                      b.reg(r#"늦은 저녁|저녁 늦게"#)?,
                      |_| Ok(helpers::hour(21, false)?
                          .span_to(&helpers::hour(0, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Evening)))
    );
    b.rule_1_terminal("early night (latent)",
                      b.reg(r#"이른 밤|밤에 일찍"#)?,
                      |_| Ok(helpers::hour(21, false)?
                          .span_to(&helpers::hour(0, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Night)))
    );
    b.rule_1_terminal("night",
                      b.reg(r#"밤"#)?,
                      |_| Ok(helpers::hour(19, false)?
                          .span_to(&helpers::hour(0, false)?, false)?
                          .form(Form::PartOfDay(PartOfDayForm::Night)))
    );
    b.rule_1_terminal("late night (latent)",
                      b.reg(r#"늦은 밤|밤 늦게|깊은 밤"#)?,
                      |_| Ok(helpers::hour(1, false)?
                          .span_to(&helpers::hour(4, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Night)))
    );
    b.rule_1_terminal("breakfast (latent)",
                      b.reg(r#"아침(?: ?(?:식사|밥))?|조반"#)?,
                      |_| Ok(helpers::hour(6, false)?
                          .span_to(&helpers::hour(9, false)?, false)?
                          .latent()
                          .form(Form::Meal))
    );
    b.rule_1_terminal("brunch (latent)",
                      b.reg(r#"브런취|브런치|아침 겸 점심|늦은 아침|아점"#)?,
                      |_| Ok(helpers::hour(11, false)?
                          .span_to(&helpers::hour(14, false)?, false)?
                          .latent()
                          .form(Form::Meal))
    );
    b.rule_1_terminal("lunch (latent)",
                      b.reg(r#"점심(?: ?(?:식사|밥))?"#)?,
                      |_| Ok(helpers::hour(12, false)?
                          .span_to(&helpers::hour(14, false)?, false)?
                          .latent()
                          .form(Form::Meal))
    );
    b.rule_1_terminal("dinner (latent)",
                      b.reg(r#"저녁(?: ?(?:식사|밥))?"#)?,
                      |_| Ok(helpers::hour_minute(17, 30, false)?
                          .span_to(&helpers::hour(21, false)?, false)?
                          .latent()
                          .form(Form::Meal))
    );
    b.rule_2("in|during the <part-of-day>",
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             b.reg(r#"에|동안|때"#)?,
             |datetime, _| Ok(datetime.value().clone().not_latent())
    );

    b.rule_2("<datetime> <part-of-day>",
             datetime_check!(),
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             |datetime, pod| pod.value().intersect(datetime.value())
    );

    b.rule_1_terminal("week-end",
                      b.reg(r#"주말"#)?,
                      |_| {
                          let friday = helpers::day_of_week(Weekday::Fri)?
                              .intersect(&helpers::hour(18, false)?)?;
                          let monday = helpers::day_of_week(Weekday::Mon)?
                              .intersect(&helpers::hour(0, false)?)?;
                          friday.span_to(&monday, false)
                      }
    );
    b.rule_1_terminal("season",
                      b.reg(r#"여름"#)?,
                      |_| helpers::month_day(6, 21)?.span_to(&helpers::month_day(9, 23)?, false)
    );
    b.rule_1_terminal("season",
                      b.reg(r#"가을"#)?,
                      |_| helpers::month_day(9, 23)?.span_to(&helpers::month_day(12, 21)?, false)
    );
    b.rule_1_terminal("season",
                      b.reg(r#"겨울"#)?,
                      |_| helpers::month_day(12, 21)?.span_to(&helpers::month_day(3, 20)?, false)
    );
    b.rule_1_terminal("season",
                      b.reg(r#"봄"#)?,
                      |_| helpers::month_day(3, 20)?.span_to(&helpers::month_day(6, 21)?, false)
    );
    b.rule_2("<datetime> approximately",
             datetime_check!(),
             b.reg(r#"경"#)?,
             |datetime, _| Ok(datetime.value().clone().precision(Precision::Approximate))
    );
    b.rule_2("<time-of-day> approximately",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"정도|쯤"#)?,
             |datetime, _| Ok(datetime.value().clone().not_latent().precision(Precision::Approximate))
    );
    b.rule_2("about <time-of-day>",
             b.reg(r#"대충|약"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, datetime| Ok(datetime.value().clone().not_latent().precision(Precision::Approximate))
    );
    b.rule_2("exactly <time-of-day>",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"정각"#)?,
             |datetime, _| Ok(datetime.value().clone().not_latent().precision(Precision::Approximate))
    );
    b.rule_3("<datetime> - <datetime> (interval)",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             b.reg(r#"\-|\~"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |a, _, b| a.value().span_to(b.value(), true)
    );
    b.rule_3("<time-of-day> - <time-of-day> (interval)",
             datetime_check!(|datetime: &DatetimeValue| if let Form::TimeOfDay(_) = datetime.form { !datetime.latent } else { false }),
             b.reg(r#"\-|\~"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |a, _, b| a.value().span_to(b.value(), true)
    );
    b.rule_2("within <duration>",
             duration_check!(),
             b.reg(r#"이내에?"#)?,
             |duration, _| helpers::cycle_nth(Grain::Second, 0)?
                 .span_to(&duration.value().in_present()?, false)
    );

    b.rule_2("within <duration>",
             duration_check!(),
             b.reg(r#"(?:안|내)에?"#)?,
             |duration, _| helpers::cycle_nth(Grain::Second, 0)?
                 .span_to(&duration.value().in_present()?, false)
    );

    b.rule_2("by <datetime> - 까지",
             datetime_check!(),
             b.reg(r#"까지"#)?,
             |datetime, _| helpers::cycle_nth(Grain::Second, 0)?.span_to(datetime.value(), false)
    );
    b.rule_2("<time-of-day>이전",
             datetime_check!(),
             b.reg(r#"이?전"#)?,
             |datetime, _| Ok(datetime.value().clone().mark_before_start())
    );
    b.rule_2("after <datetime>",
             datetime_check!(),
             b.reg(r#"지나(?:서|고)|되면|이?후에?|뒤에?"#)?,
             |datetime, _| Ok(datetime.value().clone().mark_after_end())
    );
    b.rule_2("since <datetime>",
             datetime_check!(),
             b.reg(r#"이래로?|이후로"#)?,
             |datetime, _| Ok(datetime.value().clone().mark_after_start())
    );
    b.rule_4("from <datetime> to <datetime>",
             datetime_check!(),
             b.reg(r#"부터"#)?,
             datetime_check!(),
             b.reg(r#"까지"#)?,
             |a, _, b, _| a.value().span_to(b.value(), true)
    );
    b.rule_3("during the last n cycle",
             b.reg(r#"과거"#)?,
             integer_check_by_range!(0),
             cycle_check!(),
             |_, integer, cycle| {
                 let end = helpers::cycle_nth(cycle.value().grain, 0)?;
                 let start = helpers::cycle_nth(cycle.value().grain, -1 * integer.value().value)?;
                 start.span_to(&end, false)
             }
    );
    b.rule_3("during the next n cycle",
             b.reg(r#"앞으로"#)?,
             integer_check_by_range!(1),
             cycle_check!(),
             |_, integer, cycle| {
                 let start = helpers::cycle_nth(cycle.value().grain, 1)?;
                 let end = helpers::cycle_nth(cycle.value().grain, integer.value().value)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_4("<duration> from <datetime>",
             datetime_check!(),
             b.reg(r#"보다"#)?,
             duration_check!(),
             b.reg(r#"후에|뒤에"#)?,
             |datetime, _, duration, _| {
                 duration.value().after(datetime.value())
             }
    );
    Ok(())
}

pub fn rules_duration(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("second (unit-of-duration)",
                      b.reg(r#"초"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Second))
    );
    b.rule_1_terminal("minute (unit-of-duration)",
                      b.reg(r#"분"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Minute))
    );
    b.rule_1_terminal("hour (unit-of-duration)",
                      b.reg(r#"시간?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Hour))
    );
    b.rule_1_terminal("day (unit-of-duration)",
                      b.reg(r#"날|일간?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Day))
    );
    b.rule_1_terminal("week (unit-of-duration)",
                      b.reg(r#"주(?:일|간)?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Week))
    );
    b.rule_1_terminal("month (unit-of-duration)",
                      b.reg(r#"달간?|개월"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Month))
    );
    // TODO check if the quarter duration is needed
    b.rule_1_terminal("year (unit-of-duration)",
                      b.reg(r#"해|연간?|년간?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Year))
    );
    b.rule_2("<duration>동안",
             duration_check!(),
             b.reg(r#"동안|사이에"#)?,
             |duration, _| Ok(duration.value().clone())
    );
    // TODO check that a cycle is ncessary for this rule and not a unit of duration (hour)
    b.rule_2("half an hour",
             cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Hour),
             b.reg(r#"반"#)?,
             |_, _| Ok(DurationValue::new(PeriodComp::minutes(30).into()))
    );
    b.rule_1_terminal("a day - 하루",
                      b.reg(r#"하루"#)?,
                      |_| Ok(DurationValue::new(PeriodComp::days(1).into()))
    );
    b.rule_2("<integer> <unit-of-duration>",
             integer_check_by_range!(0),
             unit_of_duration_check!(),
             |integer, uod| Ok(DurationValue::new(PeriodComp::new(uod.value().grain, integer.value().value).into()))
    );
    b.rule_2_terminal("number.number hours",
                      b.reg(r#"(\d+)\.(\d+)"#)?,
                      b.reg(r#"시간"#)?,
                      |text_match, _| {
                          let decimal_hour = helpers::decimal_hour_in_minute(text_match.group(1), text_match.group(2))?;
                          Ok(DurationValue::new(PeriodComp::new(Grain::Minute, decimal_hour).into()))
                      }
    );
    b.rule_2("<integer> and an half hours",
             integer_check_by_range!(0),
             b.reg(r#"시간반"#)?,
             |integer, _| Ok(DurationValue::new(PeriodComp::new(Grain::Minute, integer.value().value * 60 + 30).into()))
    );
    b.rule_2("in <duration>",
             duration_check!(),
             b.reg(r#"후|뒤|되면|지나(?:고|서|면)|있다가"#)?,
             |duration, _| duration.value().in_present()
    );
    b.rule_2("after <duration>",
             duration_check!(),
             b.reg(r#"(?:이 ?)후|부터"#)?,
             |duration, _| Ok(duration
                 .value()
                 .in_present()?
                 .mark_after_start())
    );
    b.rule_3("<duration> from now",
             b.reg(r#"지금부터|현시간부터"#)?,
             duration_check!(),
             b.reg(r#"후|뒤"#)?,
             |_, duration, _| duration.value().in_present()
    );
    b.rule_2("<duration> ago",
             duration_check!(),
             b.reg(r#"이?전"#)?,
             |duration, _| duration.value().ago()
    );
    b.rule_2("about <duration>",
             b.reg(r#"대충|약"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().precision(Precision::Approximate))
    );
    b.rule_2("exactly <duration>",
             b.reg(r#"정확히|딱"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().precision(Precision::Exact))
    );
    b.rule_1_terminal("Specific number of days",
                      b.reg(r#"(하루|이틀|양일|(?:사|나)흘|(?:닷|엿)새|(?:이|여드|아흐)레|열흘|열하루)"#)?,
                      |text_match| {
                          let number_of_days = match text_match.group(1).as_ref() {
                              "하루" => 1,
                              "이틀" | "양일" => 2,
                              "사흘" => 3,
                              "나흘" => 4,
                              "닷새" => 5,
                              "엿새" => 6,
                              "이레" => 7,
                              "여드레" => 8,
                              "아흐레" => 9,
                              "열흘" => 10,
                              "열하루" => 11,
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          Ok(DurationValue::new(PeriodComp::new(Grain::Day, number_of_days).into()))
                      }
    );
    Ok(())
}

pub fn rules_cycle(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("second (cycle)",
                      b.reg(r#"초"#)?,
                      |_| CycleValue::new(Grain::Second)
    );
    b.rule_1_terminal("minute (cycle)",
                      b.reg(r#"분"#)?,
                      |_| CycleValue::new(Grain::Minute)
    );
    b.rule_1_terminal("hour (cycle)",
                      b.reg(r#"시간?"#)?,
                      |_| CycleValue::new(Grain::Hour)
    );
    b.rule_1_terminal("day (cycle)",
                      b.reg(r#"날|일간?"#)?,
                      |_| CycleValue::new(Grain::Day)
    );
    b.rule_1_terminal("week (cycle)",
                      b.reg(r#"주(?:간|일)?"#)?,
                      |_| CycleValue::new(Grain::Week)
    );
    b.rule_1_terminal("month (cycle)",
                      b.reg(r#"(?:달|개?월)"#)?,
                      |_| CycleValue::new(Grain::Month)
    );
    b.rule_1_terminal("quarter (cycle)",
                      b.reg(r#"분기"#)?,
                      |_| CycleValue::new(Grain::Quarter)
    );
    b.rule_1_terminal("year (cycle)",
                      b.reg(r#"해|(?:연|년)간?"#)?,
                      |_| CycleValue::new(Grain::Year)
    );
    b.rule_2("this <cycle>",
             b.reg(r#"이번?|금|올|돌아오는"#)?,
             cycle_check!(),
             |_, a| helpers::cycle_nth(a.value().grain, 0)
    );
    b.rule_2("last <cycle>",
             b.reg(r#"지난|작|전|저번|거"#)?,
             cycle_check!(),
             |_, a| helpers::cycle_nth(a.value().grain, -1)
    );
    b.rule_2("next <cycle>",
             b.reg(r#"다음|차|오는|내|새|훗"#)?,
             cycle_check!(),
             |_, a| helpers::cycle_nth(a.value().grain, 1)
    );
    b.rule_3("<datetime> 마지막 <cycle>",
             datetime_check!(),
             b.reg(r#"다음|오는|차|내"#)?,
             cycle_check!(),
             |datetime, _, cycle| cycle.value().last_of(datetime.value())
    );
    b.rule_3("<datetime> <ordinal> <cycle>",
             datetime_check!(),
             ordinal_check!(),
             cycle_check!(),
             |datetime, ordinal, cycle| helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value - 1, datetime.value())
    );
    b.rule_1_terminal("the day after tomorrow - 내일모래",
                      b.reg(r#"(?:내일)?모레|명후일|다음다음 ?날"#)?,
                      |_| helpers::cycle_nth_after(Grain::Day, 1, &helpers::cycle_nth(Grain::Day, 1)?)
    );
    b.rule_1_terminal("the day before yesterday - 엊그제",
                      b.reg(r#"그(?:제|재)|그저께|전전 ?날|재작일"#)?,
                      |_| helpers::cycle_nth_after(Grain::Day, -1, &helpers::cycle_nth(Grain::Day, -1)?)
    );
    b.rule_3("last n <cycle>",
             b.reg(r#"지난"#)?,
             integer_check_by_range!(1, 9999),
             cycle_check!(),
             |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    b.rule_3("next n <cycle>",
             b.reg(r#"다음"#)?,
             integer_check_by_range!(1, 9999),
             cycle_check!(),
             |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    b.rule_2("<1..4> quarter",
             integer_check_by_range!(1, 4),
             cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Quarter),
             |integer, _| helpers::cycle_nth_after(Grain::Quarter, integer.value().value - 1, &helpers::cycle_nth(Grain::Year, 0)?)
    );
    b.rule_3("<year> <1..4>quarter",
             datetime_check!(),
             integer_check_by_range!(1, 4),
             cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Quarter),
             |datetime, integer, _| helpers::cycle_nth_after(Grain::Quarter, integer.value().value - 1, datetime.value())
    );
    Ok(())
}


pub fn rules_numbers(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("intersect",
             number_check!(|number: &NumberValue| number.grain().unwrap_or(0) > 1),
             number_check!(),
             |a, b| helpers::compose_numbers(&a.value(), &b.value()));
    b.rule_1_terminal("integer (numeric)",
                      b.reg(r#"(\d{1,18})"#)?,
                      |text_match| {
                          let value: i64 = text_match.group(1).parse()?;
                          IntegerValue::new(value)
                      });
    b.rule_1_terminal("integer with thousands separator ,",
                      b.reg(r#"(\d{1,3}(,\d\d\d){1,5})"#)?,
                      |text_match| {
                          let reformatted_string = text_match.group(1).replace(",", "");
                          let value: i64 = reformatted_string.parse()?;
                          IntegerValue::new(value)
                      }
    );
    b.rule_1_terminal("integer 0",
                      b.reg(r#"영|공|빵"#)?,
                      |_| IntegerValue::new(0)
    );
    b.rule_1_terminal("half - 반",
                      b.reg(r#"반"#)?,
                      |_| FloatValue::new(0.5)
    );
    b.rule_1_terminal("few 몇",
                      b.reg(r#"몇"#)?,
                      |_| Ok(IntegerValue {
                          value: 3,
                          precision: Approximate,
                          ..IntegerValue::default()
                      })
    );
    b.rule_1_terminal("integer - TYPE 1",
                      b.reg(r#"[일|이|삼|사|오|육|칠|팔|구|십|백|천|만|억|조]+"#)?,
                      |text_match| {
                          fn map_number(s: char) -> i64 {
                              match s {
                                  '일' => 1,
                                  '이' => 2,
                                  '삼' => 3,
                                  '사' => 4,
                                  '오' => 5,
                                  '육' => 6,
                                  '칠' => 7,
                                  '팔' => 8,
                                  '구' => 9,
                                  '천' => 1,
                                  '백' => 1,
                                  '십' => 1,
                                  _ => 0,
                              }
                          }

                          fn get_number(s: &str) -> RuleResult<(i64, Option<u8>)> {
                              let regex = Regex::new(r#"(.*천)?(.*백)?(.*십)?(.*)?"#)?;
                              let groups = helpers::find_regex_group(&regex, s)?
                                  .into_iter()
                                  .nth(0)
                                  .ok_or_else(|| format_err!("Regex {:?} has no match for {:?}", regex, s))?
                                  .groups;
                              let coef_1 = groups.get(4).and_then(|g| *g)
                                  .and_then(|g| g.chars().nth(0))
                                  .map(|g| map_number(g))
                                  .unwrap_or(0);
                              let coef_10 = groups.get(3).and_then(|g| *g)
                                  .and_then(|g| g.chars().nth(0))
                                  .map(|g| map_number(g))
                                  .unwrap_or(0);
                              let coef_100 = groups.get(2).and_then(|g| *g)
                                  .and_then(|g| g.chars().nth(0))
                                  .map(|g| map_number(g))
                                  .unwrap_or(0);
                              let coef_1000 = groups.get(1).and_then(|g| *g)
                                  .and_then(|g| g.chars().nth(0))
                                  .map(|g| map_number(g))
                                  .unwrap_or(0);
                              let number = 1000 * coef_1000 + 100 * coef_100 + 10 * coef_10 + coef_1;
                              let grain = if coef_1 != 0 || coef_10 != 0 {
                                  Some(1)
                              } else if coef_100 != 0 {
                                  Some(2)
                              } else if coef_1000 != 0 {
                                  Some(3)
                              } else {
                                  None
                              };
                              Ok((number, grain))
                          }

                          let regex = Regex::new(r#"(.*조)?(.*억)?(.*만)?(.*)?"#)?;

                          let groups = helpers::find_regex_group(&regex, text_match.group(0))?
                              .into_iter()
                              .nth(0)
                              .ok_or_else(|| format_err!("Regex {:?} has no match for {:?}", regex, text_match.group(0)))?
                              .groups;

                          let coef_1000000000000 = groups.get(1).and_then(|g| *g)
                              .map(|g| get_number(g))
                              .unwrap_or(Ok((0, None)))?;
                          let coef_100000000 = groups.get(2).and_then(|g| *g)
                              .map(|g| get_number(g))
                              .unwrap_or(Ok((0, None)))?;
                          let coef_10000 = groups.get(3).and_then(|g| *g)
                              .map(|g| if g == "만" { Ok((1, Some(1))) } else { get_number(g) })
                              .unwrap_or(Ok((0, None)))?;
                          let coef_1 = groups.get(4).and_then(|g| *g)
                              .map(|g| get_number(g))
                              .unwrap_or(Ok((0, None)))?;
                          let number = 1000000000000 * coef_1000000000000.0 + 100000000 * coef_100000000.0 + 10000 * coef_10000.0 + coef_1.0;
                          let grain = if coef_1.0 != 0 {
                              coef_1.1
                          } else if coef_10000.0 != 0 {
                              coef_10000.1.map(|g| 4 + g - 1)
                          } else if coef_100000000.0 != 0 {
                              coef_100000000.1.map(|g| 8 + g - 1)
                          } else if coef_1000000000000.0 != 0 {
                              coef_1000000000000.1.map(|g| 12 + g - 1)
                          } else {
                              None
                          };
                          Ok(IntegerValue {
                              value: number,
                              grain: grain,
                              ..IntegerValue::default()
                          })
                      }
    );
    b.rule_1_terminal("integer (1..10) - TYPE 2",
                      b.reg(r#"(하나|둘|셋|넷|다섯|여섯|일곱|여덟|아홉)"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "하나" => 1,
                              "둘" => 2,
                              "셋" => 3,
                              "넷" => 4,
                              "다섯" => 5,
                              "여섯" => 6,
                              "일곱" => 7,
                              "여덟" => 8,
                              "아홉" => 9,
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          IntegerValue::new(value)
                      }
    );
    b.rule_2("number hundreds",
             integer_check_by_range!(1, 99),
             integer_check_by_range!(100, 100),
             |a, b| {
                 Ok(IntegerValue {
                     value: a.value().value * b.value().value,
                     grain: b.value().grain,
                     ..IntegerValue::default()
                 })
             });
    b.rule_2("number thousands",
             integer_check_by_range!(1, 999),
             integer_check_by_range!(1000, 1000),
             |a, b| {
                 Ok(IntegerValue {
                     value: a.value().value * b.value().value,
                     grain: b.value().grain,
                     ..IntegerValue::default()
                 })
             });
    b.rule_2("number millions",
             integer_check_by_range!(1, 99),
             integer_check_by_range!(1000000, 1000000),
             |a, b| {
                 Ok(IntegerValue {
                     value: a.value().value * b.value().value,
                     grain: b.value().grain,
                     ..IntegerValue::default()
                 })
             });
    b.rule_1_terminal("integer (1..4) - for ordinals",
                      b.reg(r#"(한|두|세|네)"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "한" => 1,
                              "두" => 2,
                              "세" => 3,
                              "네" => 4,
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          IntegerValue::new(value)
                      }
    );
    b.rule_1_terminal("first ordinal",
                      b.reg(r#"첫(?:번째|번|째|째번)?"#)?,
                      |_| Ok(OrdinalValue::new(1))
    );
    b.rule_1_terminal("integer (20..90) - TYPE 2 and ordinals",
                      b.reg(r#"(열|스물|서른|마흔|쉰|예순|일흔|여든|아흔)"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "열" => 10,
                              "스물" => 20,
                              "서른" => 30,
                              "마흔" => 40,
                              "쉰" => 50,
                              "예순" => 60,
                              "일흔" => 70,
                              "여든" => 80,
                              "아흔" => 90,
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          IntegerValue::new(value)
                      }
    );
    // previous name "integer (21..99) - TYPE 2"
    b.rule_2("integer (11..99) - TYPE 2",
             integer_check_by_range!(10, 90, |integer: &IntegerValue| integer.value % 10 == 0),
             integer_check_by_range!(1, 9),
             |a, b| IntegerValue::new(a.value().value + b.value().value)
    );

    b.rule_1_terminal("decimal number",
                      b.reg(r#"(\d*\.\d+)"#)?,
                      |text_match| FloatValue::new(text_match.group(1).parse()?)
    );

    b.rule_2("number dot number - 삼점사",
             number_check!(|number: &NumberValue| !number.prefixed()),
             b.reg(r#"(점|쩜)([일|이|삼|사|오|육|칠|팔|구|영]+)"#)?,
             |a, text_match| {
                 fn number_mapping(c: char) -> Option<char> {
                     match c {
                         '일' => Some('1'),
                         '이' => Some('2'),
                         '삼' => Some('3'),
                         '사' => Some('4'),
                         '오' => Some('5'),
                         '육' => Some('6'),
                         '칠' => Some('7'),
                         '팔' => Some('8'),
                         '구' => Some('9'),
                         '영' => Some('0'),
                         _ => None,
                     }
                 }
                 let number_string = format!("0.{}",
                                             text_match.group(2).chars()
                                                 .filter_map(number_mapping)
                                                 .collect::<String>());
                 FloatValue::new(a.value().value() + number_string.parse::<f32>()?)
             }
    );

    b.rule_1_terminal("decimal with thousands separator",
                      b.reg(r#"(\d+(,\d\d\d)+\.\d+)"#)?,
                      |text_match| FloatValue::new(text_match.group(1).replace(",", "").parse()?)
    );
    b.rule_2("numbers prefix with -, 마이너스, or 마이나스",
             b.reg(r#"-|마이너스\s?|마이나스\s?"#)?,
             number_check!(|number: &NumberValue| !number.prefixed()),
             |_, a| -> RuleResult<NumberValue> {
                 Ok(match a.value().clone() {
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
             }
    );
    b.rule_2("ordinals (첫번째)",
             integer_check!(),
             b.reg(r#"번째|째|째번"#)?,
             |a, _| Ok(OrdinalValue::new(a.value().value))
    );
    b.rule_3("fraction",
             number_check!(|number: &NumberValue| !number.prefixed()),
             b.reg(r#"분(의|에)"#)?,
             number_check!(|number: &NumberValue| !number.suffixed()),
             |a, _, b| FloatValue::new(b.value().value() / a.value().value())
    );
    b.rule_3("fraction",
             number_check!(|number: &NumberValue| !number.prefixed()),
             b.reg(r#"/"#)?,
             number_check!(|number: &NumberValue| !number.suffixed()),
             |a, _, b| FloatValue::new(a.value().value() / b.value().value())
    );
    Ok(())
}
