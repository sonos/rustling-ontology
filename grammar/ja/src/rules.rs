use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::dimension::Precision::*;
use rustling_ontology_values::helpers;
use rustling_ontology_moment::{Weekday, Grain, PeriodComp, Period};

pub fn rules_numbers(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {

    b.rule_2("intersect",
             number_check!(|number: &NumberValue| number.grain().unwrap_or(0) > 1),
             number_check!(),
             |a, b| helpers::compose_numbers(&a.value(), &b.value()));

    // b.rule_1_terminal("number as digits",
    //     b.reg(r#"(\d+)"#)?,
    //     |digit| { 
    //         let value = digit.group(1).parse()?;
    //         IntegerValue::new(value) 
    //     }
    // );

    b.rule_1_terminal("0..9",
        b.reg(r#"(零|一|二|三|四|五|六|七|八|九)"#)?,
        |text_match| {
            let value = match text_match.group(1).as_ref() {
                "零" => 0,
                "一" => 1,
                "二" => 2,
                "三" => 3,
                "四" => 4,
                "五" => 5,
                "六" => 6,
                "七" => 7,
                "八" => 8,
                "九" => 9,
                _ => return Err(RuleErrorKind::Invalid.into()),
            };
            IntegerValue::new(value) 
        }
    );

    b.rule_1_terminal("10",
        b.reg(r#"十"#)?,
        |_| IntegerValue::new(10)
    );

    b.rule_2("10..19",
        b.reg(r#"十"#)?,
        integer_check_by_range!(1, 9),
        |_, a| IntegerValue::new(a.value().value + 10)
    );

    b.rule_2("20..90",
        integer_check_by_range!(2, 9),
        b.reg(r#"十"#)?,
        |a, _| IntegerValue::new(a.value().value * 10)
    );

    // b.rule_2("21..99",
    //     integer_check_by_range!(2, 9),
    //     integer_check_by_range!(1, 9),
    //     |a, b| IntegerValue::new(a.value().value * 10 + b.value().value)
    // );

    b.rule_3("21..99",
        integer_check_by_range!(2, 9),
        b.reg(r#"十"#)?,
        integer_check_by_range!(1, 9),
        |a, _, b| IntegerValue::new(a.value().value * 10 + b.value().value)
    );

    b.rule_1_terminal("100",
        b.reg(r#"百"#)?,
        |_| IntegerValue::new_with_grain(100, 2)
    );

    b.rule_2("200..900",
        integer_check_by_range!(2, 9),
        b.reg(r#"百"#)?,
        |a, _| {
            Ok(IntegerValue {
                value: a.value().value * 100,
                grain: Some(2),
                ..IntegerValue::default()
            })
        }
    );

    b.rule_1_terminal("1000",
        b.reg(r#"千"#)?,
        |_| IntegerValue::new_with_grain(1000, 3)
    );

    b.rule_2("2000..9000",
        integer_check_by_range!(2, 9),
        b.reg(r#"千"#)?,
        |a, _| {
            Ok(IntegerValue {
                value: a.value().value * 1000,
                grain: Some(3),
                ..IntegerValue::default()
            })
        }
    );

    b.rule_1_terminal("10000",
        b.reg(r#"万"#)?,
        |_| IntegerValue::new_with_grain(10000, 4)
    );

    b.rule_2("10000..99990000",
        integer_check_by_range!(1, 9999),
        b.reg(r#"万"#)?,
        |a, _| {
            Ok(IntegerValue {
                value: a.value().value * 10000,
                grain: Some(4),
                ..IntegerValue::default()
            })
        }
    );

    b.rule_1_terminal("100000000",
        b.reg(r#"億"#)?,
        |_| IntegerValue::new_with_grain(100000000, 8)
    );

    b.rule_2("100000000..999900000000",
        integer_check_by_range!(1, 9999),
        b.reg(r#"億"#)?,
        |a, _| {
            Ok(IntegerValue {
                value: a.value().value * 100000000,
                grain: Some(8),
                ..IntegerValue::default()
            })
        }
    );
    b.rule_2("ordinal number",
            integer_check!(|integer: &IntegerValue| integer.value >= 0),
            b.reg(r#"番目"#)?,
            |integer, _| Ok(OrdinalValue::new(integer.value().value))
    );
    // b.rule_1("float number", b.reg(r#"(\d*\.\d+)"#)?, |text_match| {
        // let value: f32 = text_match.group(1).parse()?;
        // Ok(FloatValue {
            // value: value,
            // ..FloatValue::default()
        // })
    // });
    Ok(())
}

pub fn rules_percentage(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("<number> per cent",
        number_check!(),
        b.reg(r#"パーセント|%"#)?,
        |number, _| Ok(PercentageValue(number.value().value()))
    );
    b.rule_1_terminal("ten per cent",
        b.reg(r#"割"#)?,
        |_| Ok(PercentageValue(10.0))
    );
        b.rule_1_terminal("one per cent",
        b.reg(r#"分"#)?,
        |_| Ok(PercentageValue(1.0))
    );
    b.rule_1_terminal("zero dot one per cent",
        b.reg(r#"厘"#)?,
        |_| Ok(PercentageValue(0.1))
    );
    Ok(())
}

pub fn rules_finance(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("intersect (X cents)",
             amount_of_money_check!(),
             amount_of_money_check!(|money: &AmountOfMoneyValue| money.unit == Some("cent")),
             |a, b| helpers::compose_money(a.value(), b.value()));
    b.rule_2("intersect",
             amount_of_money_check!(|money: &AmountOfMoneyValue| money.unit != Some("cent")),
             number_check!(),
             |a, b| helpers::compose_money_number(&a.value(), &b.value()));
    b.rule_1_terminal("USD",
        b.reg(r#"ドル|\$|＄"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("$") })
    );
    b.rule_1_terminal("EUR",
        b.reg(r#"ユーロ|€"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("€") })
    );
    b.rule_1_terminal("GBP",
        b.reg(r#"ポンド|£"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("£") })
    );
    b.rule_1_terminal("JPY",
        b.reg(r#"円|¥"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("JPY") })
    );
    b.rule_1_terminal("CNY",
        b.reg(r#"(?:人民)?元|¥"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("¥") })
    );
    b.rule_1_terminal("KRW",
        b.reg(r#"ウォン"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("₩") })
    );
    b.rule_1_terminal("INR",
        b.reg(r#"ルピー"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("INR") })
    );
    b.rule_1_terminal("RUB",
        b.reg(r#"ルーブル"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("RUB") })
    );
    b.rule_1_terminal("cent",
                      b.reg(r#"銭"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("cent") })
    );
    b.rule_2("<amount> <unit>",
             number_check!(),
             money_unit!(),
             |a, b| {
                 Ok(AmountOfMoneyValue {
                     value: a.value().value(),
                     unit: b.value().unit,
                     ..AmountOfMoneyValue::default()
                 })
             });
    b.rule_2("about <amount-of-money>",
             b.reg(r#"約|だいたい|およそ"#)?,
             amount_of_money_check!(),
             |_, a| {
                 Ok(AmountOfMoneyValue {
                     precision: Approximate,
                     ..a.value().clone()
                 })
             });
        b.rule_2("about <amount-of-money>",
             amount_of_money_check!(),
             b.reg(r#"ほど|くらい|程|位"#)?,
             |a, _| {
                 Ok(AmountOfMoneyValue {
                     precision: Approximate,
                     ..a.value().clone()
                 })
             });

    b.rule_2("exactly <amount-of-money>",
             b.reg(r#"ちょうど|まさに"#)?,
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
        integer_check!(),
        |a| {
            Ok(TemperatureValue {
                value: a.value().value as f32,
                unit: None,
                latent: true,
            })
    });
    b.rule_2("<temp> Celcius",
             temperature_check!(),
             b.reg(r#"度|°C|℃"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("celsius"),
                     latent: false,
                 })
             });
    b.rule_2("<latent temp> en dessous de zero",
             b.reg(r#"マイナス|零下"#)?,
             temperature_check!(),
             |_, a| {
                 Ok(TemperatureValue {
                     value: -1.0 * a.value().value,
                     latent: false,
                     ..*a.value()
                 })
             });
    Ok(())
}

pub fn rules_time(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("intersect",
             time_check!(|time: &TimeValue| !time.latent),
             time_check!(|time: &TimeValue| !time.latent),
             |a, b| a.value().intersect(b.value())
    );
    b.rule_3("intersect",
             time_check!(|time: &TimeValue| !time.latent),
             b.reg(r#"の|と"#)?,
             time_check!(|time: &TimeValue| !time.latent),
             |a, _, b| a.value().intersect(b.value())
    );
    b.rule_2("on <date>",
             time_check!(),
             b.reg(r#"に"#)?,
             |a, _| Ok(a.value().clone().not_latent())
    );
    b.rule_2("for <date>",
             time_check!(|time: &TimeValue| !time.latent),
             b.reg(r#"に合わせて"#)?,
             |a, _| Ok(a.value().clone().not_latent())
    );
    b.rule_2("in <named-month>",
             time_check!(form!(Form::Month(_))),
             b.reg(r#"に"#)?,
             |a, _| Ok(a.value().clone())
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"(月曜|火曜|水曜|木曜|金曜|土曜|日曜)日?"#)?,
                      |text_match| {
                        let value = match text_match.group(1).as_ref() {
                            "月曜" => Weekday::Mon,
                            "火曜" => Weekday::Tue,
                            "水曜" => Weekday::Wed,
                            "木曜" => Weekday::Thu,
                            "金曜" => Weekday::Fri,
                            "土曜" => Weekday::Sat,
                            "日曜" => Weekday::Sun,
                            _ => return Err(RuleErrorKind::Invalid.into()),
                        };
                        helpers::day_of_week(value)
                      }
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"(一|二|三|四|五|六|七|八|九|十|十一|十二)月"#)?,
                      |text_match| {
                        let value = match text_match.group(1).as_ref() {
                            "一" => 1,
                            "二" => 2,
                            "三" => 3,
                            "四" => 4,
                            "五" => 5,
                            "六" => 6,
                            "七" => 7,
                            "八" => 8,
                            "九" => 9,
                            "十" => 10,
                            "十一" => 11,
                            "十二" => 12,
                            _ => return Err(RuleErrorKind::Invalid.into()),
                        };
                        helpers::month(value)
                      }
    );
    b.rule_1_terminal("new year's day",
                      b.reg(r#"元(?:日|旦)"#)?,
                      |_| helpers::month_day(1, 1)
    );
    b.rule_1_terminal("coming of age day",
                      b.reg(r#"成人(?:式|の(?:日の)?)"#)?,
                      |_| {
                            let monday_january = helpers::month(1)?.intersect(&helpers::day_of_week(Weekday::Mon)?)?;
                            let second_week_of_january = helpers::cycle_nth_after(Grain::Week, 2, &helpers::month_day(1, 1)?)?;
                            monday_january.intersect(&second_week_of_january) // second monday of january
                      }
    );
    b.rule_1_terminal("setsubun",
                      b.reg(r#"節分の日"#)?,
                      |_| helpers::month_day(2, 3)
    );
    //b.rule_1_terminal("vernal equinox day",
                      //b.reg(r#"春分の日"#)?,
                      //|_| helpers::month_day(3, 20) // around 20th of March (between 19th and 22nd)
    //);
    b.rule_1_terminal("national foundation day",
                      b.reg(r#"建国記念日"#)?,
                      |_| helpers::month_day(2, 11)
    );
    b.rule_1_terminal("the emperor's birthday",
                      b.reg(r#"天皇誕生日"#)?,
                      |_| helpers::month_day(12, 23) // To be deleted for 2019 as there will be no emperor's birthday in 2019 and a new date for 2020 as there will be a new emperor
    );
    b.rule_1_terminal("the emperor's birthday",
                      b.reg(r#"天皇誕生日"#)?,
                      |_| helpers::month_day(2, 23) // New date from 2020 to be uncomment for 2020
    );
    b.rule_1_terminal("girls day",
                      b.reg(r#"ひな(?:まつり|祭り)(?:の日)?"#)?,
                      |_| helpers::month_day(3, 3)
    );
    b.rule_1_terminal("womens day",
                      b.reg(r#"女性(?:の日)?"#)?,
                      |_| helpers::month_day(3, 8)
    );
    b.rule_1_terminal("showa day",
                      b.reg(r#"昭和の日"#)?,
                      |_| helpers::month_day(4, 29)
    );
    b.rule_1_terminal("constitution memorial day",
                      b.reg(r#"憲法記念日"#)?,
                      |_| helpers::month_day(5, 3)
    );
    b.rule_1_terminal("greenery day",
                      b.reg(r#"みどりの日"#)?,
                      |_| helpers::month_day(5, 4)
    );
    b.rule_1_terminal("children's day",
                      b.reg(r#"こどもの日"#)?,
                      |_| helpers::month_day(5, 5)
    );
    b.rule_1_terminal("marine day",
                      b.reg(r#"海の日"#)?,
                      |_| {
                            let monday_july = helpers::month(7)?.intersect(&helpers::day_of_week(Weekday::Mon)?)?;
                            let third_week_of_july = helpers::cycle_nth_after(Grain::Week, 3, &helpers::month_day(7, 1)?)?;
                            monday_july.intersect(&third_week_of_july) // third monday of july
                      }
    );
    b.rule_1_terminal("mountain day",
                      b.reg(r#"山の日"#)?,
                      |_| helpers::month_day(8, 11)
    );
    b.rule_1_terminal("respect for the aged day",
                      b.reg(r#"敬老の日"#)?,
                      |_| {
                            let monday_september = helpers::month(9)?.intersect(&helpers::day_of_week(Weekday::Mon)?)?;
                            let third_week_of_september = helpers::cycle_nth_after(Grain::Week, 3, &helpers::month_day(9, 1)?)?;
                            monday_september.intersect(&third_week_of_september) // third monday of september
                      }
    );
    //b.rule_1_terminal("autumnal equinox day",
                      //b.reg(r#"秋分の日"#)?,
                      //|_| helpers::month_day(9, 21) // around 22th of September
    //);
    b.rule_1_terminal("culture day",
                      b.reg(r#"文化の日"#)?,
                      |_| helpers::month_day(11, 3)
    );
    b.rule_1_terminal("health and sports day",
                      b.reg(r#"体育の日"#)?,
                      |_| {
                            let monday_october = helpers::month(10)?.intersect(&helpers::day_of_week(Weekday::Mon)?)?;
                            let second_week_of_october = helpers::cycle_nth_after(Grain::Week, 2, &helpers::month_day(10, 1)?)?;
                            monday_october.intersect(&second_week_of_october) // second monday of october
                      }
    );
    b.rule_1_terminal("labor thanksgiving day",
                      b.reg(r#"勤労感謝の日"#)?,
                      |_| helpers::month_day(11, 23)
    );
    b.rule_1_terminal("christmas eve",
                      b.reg(r#"クリスマスイブ(?:の?日に?)?"#)?,
                      |_| helpers::month_day(12, 24)
    );
    b.rule_1_terminal("christmas",
                      b.reg(r#"クリスマス(?:の?日に?)?"#)?,
                      |_| helpers::month_day(12, 25)
    );
    b.rule_1_terminal("halloween",
                      b.reg(r#"ハロウィン(?:の?日に?)?"#)?,
                      |_| helpers::month_day(10, 31)
    );
    b.rule_1_terminal("valentines's day",
                      b.reg(r#"バレンタインデー(?:の?日に?)?"#)?,
                      |_| helpers::month_day(2, 14)
    );
    b.rule_1_terminal("now",
                      b.reg(r#"今(?:すぐに?)?|現在|只今|ただいま"#)?,
                      |_| helpers::cycle_nth(Grain::Second, 0)
    );
    b.rule_1_terminal("today",
                      b.reg(r#"今日|本日"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 0)
    );
    b.rule_1_terminal("tomorrow",
                      b.reg(r#"明日?|翌日"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 1)
    );
    b.rule_1_terminal("the day after tomorrow",
                      b.reg(r#"明後日"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 2)
    );
    b.rule_1_terminal("yesterday",
                      b.reg(r#"昨日"#)?,
                      |_| helpers::cycle_nth(Grain::Day, -1)
    );
    b.rule_1_terminal("the day before yesterday",
                      b.reg(r#"一昨日"#)?,
                      |_| helpers::cycle_nth(Grain::Day, -2)
    );
    b.rule_1_terminal("the day before chirstmas eve",
        b.reg(r#"クリスマスイブの前の日"#)?,
        |_| helpers::month_day(12, 23)
    );
    b.rule_2("the day before <time>",
        time_check!(),
        b.reg(r#"の?前の?日"#)?,
        |a, _| helpers::cycle_nth_after_not_immediate(Grain::Day, -1, a.value())
    );
    b.rule_2("the day after <time>",
        time_check!(),
        b.reg(r#"の?次の日"#)?,
        |a, _| helpers::cycle_nth_after_not_immediate(Grain::Day, 1, a.value())
    );
    b.rule_1_terminal("end of week",
        b.reg(r#"週の終わりに"#)?,
        |_| helpers::day_of_week(Weekday::Thu)?
                    .span_to(&helpers::day_of_week(Weekday::Sun)?, false)
    );
    b.rule_1_terminal("by the end of week",
        b.reg(r#"週の終わり(?:までに|の前に)"#)?,
        |_| helpers::cycle_nth(Grain::Second, 0)?
                    .span_to(&helpers::day_of_week(Weekday::Sun)?, true)
    );
    b.rule_2("first ten days of <month>",
        time_check!(form!(Form::Month{..})),
        b.reg(r#"の?上旬"#)?,
        |month, _| {
            Ok(helpers::month_day(month.value().form_month()?, 1)?
                .span_to(&helpers::month_day(month.value().form_month()?,10)?, false)?
                .latent()
                .form(Form::PartOfMonth))
        } 
    );
    b.rule_1_terminal("first ten days of last month",
        b.reg(r#"先月の?上旬"#)?,
        |_| {
            let month = helpers::cycle_nth(Grain::Month, 0)?;
            Ok(helpers::cycle_nth_after(Grain::Day, -10, &month)?
                .span_to(&month, false)?
                .latent()
                .form(Form::PartOfMonth))
        } 
    );
    b.rule_1_terminal("first ten days of next month",
        b.reg(r#"来月の?上旬"#)?,
        |_| {
            let month = helpers::cycle_nth(Grain::Month, 2)?;
            Ok(helpers::cycle_nth_after(Grain::Day, -10, &month)?
                .span_to(&month, false)?
                .latent()
                .form(Form::PartOfMonth))
        } 
    );
    b.rule_2("first three days of <month>",
        time_check!(form!(Form::Month{..})),
        b.reg(r#"の?(?:頭|初め|始め)"#)?,
        |month, _| {
            Ok(helpers::month_day(month.value().form_month()?,0)?
                .span_to(&helpers::month_day(month.value().form_month()?,3)?, true)?
                .latent()
                .form(Form::PartOfMonth))
        } 
    );
    b.rule_1_terminal("first three days of next month",
        b.reg(r#"来月の?(?:頭|初め|始め)"#)?,
        |_| {
           let month = helpers::cycle_nth(Grain::Month, 1)?;
            Ok(helpers::cycle_nth_after(Grain::Day, 0, &month)?
                .span_to(&helpers::cycle_nth_after(Grain::Day, 3, &month)?, true)?
                .latent()
                .form(Form::PartOfMonth))
        } 
    );
    b.rule_1_terminal("first three days of last month",
        b.reg(r#"先月の?(?:頭|初め|始め)"#)?,
        |_| {
            let month = helpers::cycle_nth(Grain::Month, -1)?;
            Ok(helpers::cycle_nth_after(Grain::Day, 0, &month)?
                .span_to(&helpers::cycle_nth_after(Grain::Day, 3, &month)?, true)?
                .latent()
                .form(Form::PartOfMonth))
        } 
    );
    b.rule_2("middle ten days of <month>",
        time_check!(form!(Form::Month{..})),
        b.reg(r#"中旬|の?半ば"#)?,
        |month, _| {
            Ok(helpers::month_day(month.value().form_month()?, 10)?
                .span_to(&helpers::month_day(month.value().form_month()?,20)?, false)?
                .latent()
                .form(Form::PartOfMonth))
        }
    );
    b.rule_1_terminal("middle ten days of last month",
        b.reg(r#"先月(?:中旬|の?半ば)"#)?,
        |_| {
            let month = helpers::cycle_nth(Grain::Month, -1)?;
            Ok(helpers::cycle_nth_after(Grain::Day, 10, &month)?
                .span_to(&helpers::cycle_nth_after(Grain::Day, 20, &month)?, false)?
                .latent()
                .form(Form::PartOfMonth))
        } 
    );
    b.rule_1_terminal("middle ten days of next month",
        b.reg(r#"来月(?:中旬|の?半ば)"#)?,
        |_| {
            let month = helpers::cycle_nth(Grain::Month, 1)?;
            Ok(helpers::cycle_nth_after(Grain::Day, 10, &month)?
                .span_to(&helpers::cycle_nth_after(Grain::Day, 20, &month)?, false)?
                .latent()
                .form(Form::PartOfMonth))
        } 
    );
    b.rule_2("by the end of <time>",
             time_check!(),
             b.reg(r#"の終わりまでに|末の前に"#)?,
             |a, _| helpers::cycle_nth(Grain::Second, 0)?.span_to(a.value(), true)
    );
    b.rule_2("last ten days of the month",
        time_check!(form!(Form::Month{..})),
        b.reg(r#"下旬|の終わりに"#)?,
        |month, _| {
            let next_month = helpers::month(month.value().form_month()? + 1)?;
            Ok(helpers::cycle_nth_after(Grain::Day, -10, &next_month)?
                .span_to(&next_month, false)?
                .latent()
                .form(Form::PartOfMonth))
        }
    );
    b.rule_2("last three days of <month>",
        time_check!(form!(Form::Month{..})),
        b.reg(r#"末"#)?,
        |month, _| {
            let next_month = helpers::month(month.value().form_month()? + 1)?;
            Ok(helpers::cycle_nth_after(Grain::Day, -3, &next_month)?
                .span_to(&next_month, false)?
                .latent()
                .form(Form::PartOfMonth))
        }
    );
    b.rule_1_terminal("last three days of last month",
        b.reg(r#"先月末"#)?,
        |_| {
            let month = helpers::cycle_nth(Grain::Month, 0)?;
            Ok(helpers::cycle_nth_after(Grain::Day, -3, &month)?
                .span_to(&month, false)?
                .latent()
                .form(Form::PartOfMonth))
        } 
    );
    b.rule_1_terminal("last three days of next month",
        b.reg(r#"来月末"#)?,
        |_| {
            let month = helpers::cycle_nth(Grain::Month, 2)?;
            Ok(helpers::cycle_nth_after(Grain::Day, -3, &month)?
                .span_to(&month, false)?
                .latent()
                .form(Form::PartOfMonth))
        } 
    );
    b.rule_1_terminal("by the end of month",
        b.reg(r#"月?の終わりまでに|月?末の前に"#)?,
        |_| helpers::cycle_nth(Grain::Second, 0)?
                    .span_to(&helpers::cycle_nth(Grain::Month, 0)?, true)
    );
    b.rule_1_terminal("start of year",
        b.reg(r#"年?始|年の初め"#)?,
        |_| {
            let current_year = helpers::cycle_nth(Grain::Year, 0)?;
            let start = current_year.intersect(&helpers::month(1)?)?;
            let end = current_year.intersect(&helpers::month(1)?)?;
            start.span_to(&end, true)
        } 
    );
    b.rule_1_terminal("end of year",
        b.reg(r#"年?の終わりに"#)?,
        |_| {
            let current_year = helpers::cycle_nth(Grain::Year, 0)?;
            let start = current_year.intersect(&helpers::month(10)?)?;
            let end = current_year.intersect(&helpers::month(12)?)?;
            start.span_to(&end, true)
        } 
    );
    b.rule_2("end of year",
        integer_check!(),
        b.reg(r#"年?の終わりに"#)?,
        |integer, _| {
            let wished_year = helpers::year(integer.value().value as i32)?;
            let start = wished_year.intersect(&helpers::month(10)?)?;
            let end = wished_year.intersect(&helpers::month(12)?)?;
            start.span_to(&end, true)
        } 
    );
    b.rule_1_terminal("end of year",
        b.reg(r#"年?末|年?の暮れ?"#)?,
        |_| {
            let current_year = helpers::cycle_nth(Grain::Year, 0)?;
            let start = current_year.intersect(&helpers::month(12)?)?;
            let end = current_year.intersect(&helpers::month(12)?)?;
            start.span_to(&end, true)
        } 
    );
    b.rule_1_terminal("by the start of year",
        b.reg(r#"(?:年?始|年?の初め)までに"#)?,
        |_| {
          let current_year = helpers::cycle_nth(Grain::Year, 0)?;
          let end = current_year.intersect(&helpers::month(12)?)?;
          helpers::cycle_nth(Grain::Second, 0)?
                    .span_to(&end, true)
        }
    );
    b.rule_1_terminal("by the end of year",
        b.reg(r#"年の終わりまでに"#)?,
        |_| {
          let current_year = helpers::cycle_nth(Grain::Year, 0)?;
          let end = current_year.intersect(&helpers::month(12)?)?;
          helpers::cycle_nth(Grain::Second, 0)?
                    .span_to(&end, true)
        }
    );
    b.rule_2("next <day-of-week>",
             b.reg(r#"次の"#)?,
             time_check!(form!(Form::DayOfWeek{..})),
             |_, a| {
                 a.value().the_nth(1)
             }
    );
    b.rule_2("<day-of-week> of next week",
             b.reg(r#"来週の"#)?,
             time_check!(form!(Form::DayOfWeek{..})),
             |_, a| a.value().intersect(&helpers::cycle_nth(Grain::Week, 1)?)
    );
    
    b.rule_2("<day-of-week> of this week",
             b.reg(r#"今週の"#)?,
             time_check!(form!(Form::DayOfWeek{..})),
             |_, a| a.value().intersect(&helpers::cycle_nth(Grain::Week, 0)?)
    );
    b.rule_2("last <time>",
             b.reg(r#"(?:先|前)の?"#)?,
             time_check!(),
             |_, a| {
                 a.value().the_nth(-1)
             }
    );
    b.rule_2("next <time>",
             b.reg(r#"次の?|翌"#)?,
             time_check!(),
             |_, a| {
                 a.value().the_nth(0)
             }
    );
    b.rule_2("last <day-of-week>",
             b.reg(r#"前の"#)?,
             time_check!(form!(Form::DayOfWeek{..})),
             |_, a| {
                 a.value().the_nth(-1)
             }
    );
    b.rule_2("<day-of-week> of last week",
             b.reg(r#"先週の"#)?,
             time_check!(form!(Form::DayOfWeek{..})),
             |_, a| a.value().intersect(&helpers::cycle_nth(Grain::Week, -1)?)
    );
    b.rule_3("last <day-of-week> of <time>",
             time_check!(),
             b.reg(r#"の最後の"#)?,
             time_check!(form!(Form::DayOfWeek{..})),
             |a, _, b| {
                 b.value().last_of(a.value())
             }
    );
    b.rule_3("last <cycle> of <time>",
             time_check!(),
             b.reg(r#"の最終"#)?,
             cycle_check!(),          
             |time, _, cycle| {
                 cycle.value().last_of(time.value())
             }
    );
    b.rule_4("nth <day-of-week> of <time>",
             time_check!(|time: &TimeValue| form!(Form::Month(_))(time) ||  form!(Form::Year(_))(time)),
             b.reg(r#"の?第"#)?,
             integer_check!(),
             time_check!(),
             |anchor, _, number, weekday| {
                 if let Ok(month) = anchor.value().form_month() {
                    let weekdays_of_month = helpers::month(month)?.intersect(weekday.value())?;
                    let nth_week_of_month = helpers::cycle_nth_after(Grain::Week, number.value().value - 1, &helpers::month_day(month, 1)?)?;
                    weekdays_of_month.intersect(&nth_week_of_month)
                 } else if let Ok(year) = anchor.value().form_year() {
                    let weekdays_of_year = helpers::year(year)?.intersect(weekday.value())?;
                    let nth_week_of_year = helpers::cycle_nth_after(Grain::Week, number.value().value - 1, &helpers::year_month_day(year, 1, 1)?)?;
                    weekdays_of_year.intersect(&nth_week_of_year)
                 } else {
                    Err(RuleErrorKind::Invalid.into())
                 }
             }
    );
    b.rule_3("nth <day> of (last/this/next) month",
             b.reg(r#"(先|今|来)月"#)?,
             integer_check_by_range!(1, 31),
             b.reg(r#"日"#)?,
             |text_match, integer, _| {
                let value = match text_match.group(1).as_ref() {
                    "先" => -1,
                    "今" => 0,
                    "来" => 1,
                    _ => return Err(RuleErrorKind::Invalid.into()),
                };
                helpers::cycle_nth(Grain::Month, value)?.intersect(&helpers::day_of_month(integer.value().value as u32)?)
            }
    );
    b.rule_4("nth <time> of <time>",
             time_check!(),
             b.reg(r#"の?第"#)?,
             integer_check!(),
             cycle_check!(),
             |a, _, number, b| {
                helpers::cycle_nth_after_not_immediate(b.value().grain, number.value().value - 1, a.value())
             }
    );
    b.rule_5("nth <time> of <time>",
             time_check!(),
             b.reg(r#"の?第"#)?,
             integer_check!(),
             cycle_check!(),
             b.reg(r#"目"#)?,
             |a, _, number, b, _| {
                 helpers::cycle_nth_after_not_immediate(b.value().grain, number.value().value - 1, a.value())
             }
    );
    b.rule_3("first <time> of <time>",
             time_check!(|time: &TimeValue| form!(Form::Month(_))(time) ||  form!(Form::Year(_))(time)),
             b.reg(r#"の?最初の"#)?,
             time_check!(),
             |anchor, _, time| {
                   if let Ok(month) = anchor.value().form_month() {
                    let time = helpers::month(month)?.intersect(time.value())?;
                    let nth_week_of_month = helpers::cycle_nth_after(Grain::Week, 0, &helpers::month_day(month, 1)?)?;
                    time.intersect(&nth_week_of_month)
                 } else if let Ok(year) = anchor.value().form_year() {
                    let time = helpers::year(year)?.intersect(time.value())?;
                    let nth_week_of_year = helpers::cycle_nth_after(Grain::Week, 0, &helpers::year_month_day(year, 1, 1)?)?;
                    time.intersect(&nth_week_of_year)
                 } else {
                    Err(RuleErrorKind::Invalid.into())
                 }
             }
    );
    b.rule_3("last <time> of <time>",
             time_check!(|time: &TimeValue| form!(Form::Month(_))(time) ||  form!(Form::Year(_))(time)),
             b.reg(r#"の?最後の"#)?,
             time_check!(),
             |anchor, _, time| {
                   if let Ok(month) = anchor.value().form_month() {
                    let time = helpers::month(month)?.intersect(time.value())?;
                    let nth_week_of_month = helpers::cycle_nth_after(Grain::Week, -1, &helpers::month_day(month + 1, 1)?)?;
                    time.intersect(&nth_week_of_month)
                 } else if let Ok(year) = anchor.value().form_year() {
                    let time = helpers::year(year)?.intersect(time.value())?;
                    let nth_week_of_year = helpers::cycle_nth_after(Grain::Week, -1, &helpers::year_month_day(year + 1, 1, 1)?)?;
                    time.intersect(&nth_week_of_year)
                 } else {
                    Err(RuleErrorKind::Invalid.into())
                 }
             }
    );
    b.rule_4("nth <time> after <time>",
             time_check!(),
             b.reg(r#"の?(?:後の|以降)第"#)?,
             integer_check!(),
             cycle_check!(),
             |a, _, number, b| {
                 a.value().intersect(&helpers::cycle_nth(b.value().grain, number.value().value)?)
             }
    );
    b.rule_5("nth <time> after <time>",
             time_check!(),
             b.reg(r#"の?"#)?,
             integer_check!(),
             time_check!(),
             b.reg(r#"後"#)?,
             |a, _, number, b, _| {
                 a.value().the_nth_after(number.value().value - 1, b.value())
             }
    );
    b.rule_1_terminal("first half of the week (monday/tuesday)",
             b.reg(r#"先週の前半|今週の頭"#)?,
             |_| helpers::cycle_nth(Grain::Week, 0)?.intersect(&helpers::day_of_week(Weekday::Mon)?.span_to(&helpers::day_of_week(Weekday::Tue)?, true)?)
    );
    b.rule_1_terminal("second half of the week (thursday/friday)",
             b.reg(r#"来週の後半"#)?,
             |_| helpers::cycle_nth(Grain::Week, 0)?.intersect(&helpers::day_of_week(Weekday::Thu)?.span_to(&helpers::day_of_week(Weekday::Fri)?, true)?)
    );

    b.rule_1("current second",
             b.reg(r#"秒"#)?,
             |_| helpers::cycle_nth(Grain::Second, 0)
    );
    b.rule_1("current minute",
             b.reg(r#"分"#)?,
             |_| helpers::cycle_nth(Grain::Minute, 0)
    );
    b.rule_1("current hour",
             b.reg(r#"時間?"#)?,
             |_| helpers::cycle_nth(Grain::Hour, 0)
    );
    b.rule_1("current day",
             b.reg(r#"日"#)?,
             |_| helpers::cycle_nth(Grain::Day, 0)
    );
    b.rule_1("current week",
             b.reg(r#"週間?"#)?,
             |_| helpers::cycle_nth(Grain::Week, 0)
    );
    b.rule_1("current month",
             b.reg(r#"月"#)?,
             |_| helpers::cycle_nth(Grain::Month, 0)
    );
    b.rule_1("current year",
             b.reg(r#"年"#)?,
             |_| helpers::cycle_nth(Grain::Year, 0)
    );
    b.rule_1_terminal("half a year",
        b.reg(r#"半年"#)?,
        |_| helpers::cycle_nth(Grain::Month, 6)
    );
    b.rule_1_terminal("half a month",
        b.reg(r#"半月"#)?,
        |_| helpers::cycle_nth(Grain::Day, 15)
    );
    b.rule_1_terminal("half a day",
        b.reg(r#"半日"#)?,
        |_| helpers::cycle_nth(Grain::Hour, 12)
    );
    b.rule_2("year",
             integer_check_by_range!(1000, 2100),
             b.reg(r#"年"#)?,
             |integer, _| {
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

    b.rule_1("the <day-of-month> (non ordinal)",
             b.reg(r#"初日"#)?,
             |_| {
                 Ok(helpers::day_of_month(1 as u32)?.latent())
             }
    );
    b.rule_2("the <day-of-month>",
             integer_check_by_range!(1, 31),
             b.reg(r#"日"#)?,
             |integer, _| {
                 Ok(helpers::day_of_month(integer.value().value as u32)?.latent())
             }
    );
    b.rule_3("<named-day> <day-of-month>",
             integer_check_by_range!(1, 31),
             b.reg(r#"日の?"#)?,
             time_check!(form!(Form::DayOfWeek{..})),
             |integer, _, a| {
                 a.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
             }
    );
    b.rule_3("<named-month> <day-of-month>",
             time_check!(form!(Form::Month(_))),
             integer_check_by_range!(1, 31),
             b.reg(r#"日"#)?,
             |month, integer, _| {
                let m = month.value().form_month()?;
                let form = Form::MonthDay(Some(MonthDayForm { month: m,  day_of_month: integer.value().value as u32}));
                Ok(month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)?.form(form))
             }
    );
    b.rule_2("<month-day> <year>",
        time_check!(form!(Form::Year(_))),
        time_check!(form!(Form::MonthDay(_))),
        |year, month_day| {
            year.value().intersect(&month_day.value())
        }
    );
    b.rule_2("hour",
                      integer_check_by_range!(0, 24),
                      b.reg(r#"時"#)?,
                      |a, _| helpers::hour(a.value().value as u32, true)
    );
    b.rule_4("<hour and minutes",
                      integer_check_by_range!(0, 24),
                      b.reg(r#"時"#)?,
                      integer_check_by_range!(0, 59),
                      b.reg(r#"分"#)?,
                      |a, _, b, _| helpers::hour_minute(a.value().value as u32, b.value().value as u32, true)
    );
    // b.rule_1_terminal("hh:mm",
                      // b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:.]([0-5]\d)"#)?,
                      // |text_match| helpers::hour_minute(
                          // text_match.group(1).parse()?,
                          // text_match.group(2).parse()?,
                          // true)
    // );
    b.rule_6("hour and minutes and seconds",
                      integer_check_by_range!(0, 24),
                      b.reg(r#"時"#)?,
                      integer_check_by_range!(0, 59),
                      b.reg(r#"分"#)?,
                      integer_check_by_range!(0, 59),
                      b.reg(r#"秒"#)?,
                      |a, _, b, _, c, _| helpers::hour_minute_second(a.value().value as u32, b.value().value as u32, c.value().value as u32, true)
    );
    // b.rule_1_terminal("hh:mm:ss",
                      // b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:.]([0-5]\d)[:.]([0-5]\d)"#)?,
                      // |text_match| helpers::hour_minute_second(
                          // text_match.group(1).parse()?,
                          // text_match.group(2).parse()?,
                          // text_match.group(3).parse()?,
                          // true)
    // );
    b.rule_2("<time-of-day> am|pm",
             b.reg(r#"午(前|後)"#)?,
             time_check!(form!(Form::TimeOfDay(_))),
             |text_match, a| {
                 let day_period = if text_match.group(1) == "前" {
                     helpers::hour(0, false)?.span_to(&helpers::hour(12, false)?, false)?
                 } else {
                     helpers::hour(12, false)?.span_to(&helpers::hour(0, false)?, false)?
                 };
                 Ok(a.value().intersect(&day_period)?.form(a.value().form.clone()))
             }
    );
    b.rule_2("<time-of-day> am",
             b.reg(r#"夜中の?"#)?,
             time_check!(form!(Form::TimeOfDay(_))),
             |_, a| {
                 let day_period = helpers::hour(0, false)?.span_to(&helpers::hour(12, false)?, false)?;
                 Ok(a.value().intersect(&day_period)?.form(a.value().form.clone()))
             }
    );
    b.rule_1_terminal("noon",
                      b.reg(r#"昼|正午"#)?,
                      |_| helpers::hour(12, false)
    );
    b.rule_1_terminal("midnight",
                      b.reg(r#"真?夜中"#)?,
                      |_| helpers::hour(0, false)
    );
    b.rule_2("<hour> and a half",
                      integer_check_by_range!(0, 24),
                      b.reg(r#"時半"#)?,
                      |a, _| helpers::hour_minute(a.value().value as u32, 30, true)
    );
    b.rule_1("number (as relative minutes)",
             integer_check_by_range!(1, 59),
             |a| Ok(RelativeMinuteValue(a.value().value as i32))
    );
    b.rule_2("number <minutes> (as relative minutes)",
             integer_check_by_range!(1, 59),
             b.reg(r#"分"#)?,
             |a, _| Ok(RelativeMinuteValue(a.value().value as i32))
    );

    b.rule_3("relative minutes to|till|before <integer> (hour-of-day)",
             time_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour {.. }))),
             relative_minute_check!(),
             b.reg(r#"前"#)?,
             |time, relative_minute, _| helpers::hour_relative_minute(
                 time.value().form_time_of_day()?.full_hour(),
                 -1 * relative_minute.value().0,
                 true)
    );

    b.rule_2("relative minutes after|past <integer> (hour-of-day)",
             time_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour {.. }))),
             relative_minute_check!(),
             |time, relative_minute| helpers::hour_relative_minute(
                 time.value().form_time_of_day()?.full_hour(),
                 relative_minute.value().0,
                 true)
    );
    // b.rule_1_terminal("yyyy-mm-dd",
    //                   b.reg(r#"(\d{2,4})-(0?[1-9]|1[0-2])-(3[01]|[12]\d|0?[1-9])"#)?,
    //                   |text_match| helpers::ymd(
    //                       text_match.group(1).parse()?,
    //                       text_match.group(2).parse()?,
    //                       text_match.group(3).parse()?)
    // );

    // b.rule_1_terminal("mm/dd",
    //                   b.reg(r#"(0?[1-9]|1[0-2])/(3[01]|[12]\d|0?[1-9])"#)?,
    //                   |text_match| helpers::month_day(
    //                       text_match.group(1).parse()?,
    //                       text_match.group(2).parse()?)
    // );
    b.rule_1_terminal("morning",
                      b.reg(r#"朝の?|午前中?|今朝"#)?,
                      |_| {
                          Ok(helpers::hour(4, false)?
                              .span_to(&helpers::hour(12, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Morning)))
                      }
    );
    b.rule_1_terminal("breakfast",
        b.reg(r#"朝(?:食|ごはん|ご飯)"#)?,
        |_| Ok(helpers::hour(5, false)?
                .span_to(&helpers::hour(10, false)?, false)?
                .latent()
                .form(Form::Meal))
    );
    b.rule_1_terminal("early morning",
                      b.reg(r#"明け方|早朝|朝早く"#)?,
                      |_| {
                          Ok(helpers::hour(4, false)?
                              .span_to(&helpers::hour(8, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Morning)))
                      }
    );
    b.rule_1_terminal("before work",
        b.reg(r#"仕事の?前"#)?,
        |_| {
            let period = helpers::hour(7, false)?
                    .span_to(&helpers::hour(10, false)?, false)?;
            Ok(helpers::cycle_nth(Grain::Day, 0)?.intersect(&period)?.form(Form::PartOfDay(PartOfDayForm::Morning)))
        }
    );
    b.rule_1_terminal("work",
        b.reg(r#"仕事中|勤務時間の間"#)?,
        |_| {
            let period = helpers::hour(9, false)?
                    .span_to(&helpers::hour(19, false)?, false)?;
            Ok(helpers::cycle_nth(Grain::Day, 0)?.intersect(&period)?.form(Form::PartOfDay(PartOfDayForm::None)))
        }
    );
    b.rule_1_terminal("afternoon",
                      b.reg(r#"午後"#)?,
                      |_| {
                          Ok(helpers::hour(12, false)?
                              .span_to(&helpers::hour(17, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
                      }
    );
    b.rule_1_terminal("evening",
                      b.reg(r#"夕方"#)?,
                      |_| {
                          Ok(helpers::hour(16, false)?
                              .span_to(&helpers::hour(19, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );

    b.rule_1_terminal("night",
                      b.reg(r#"夜|晩|晚"#)?,
                      |_| {
                          Ok(helpers::hour(18, false)?
                              .span_to(&helpers::hour(0, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Night)))
                      }
    );
    b.rule_1_terminal("brunch",
        b.reg(r#"ブランチ"#)?,
        |_| Ok(helpers::hour(10, false)?
                .span_to(&helpers::hour(15, false)?, false)?
                .latent()
                .form(Form::Meal))
    );
    b.rule_1_terminal("lunch",
                      b.reg(r#"昼食|お昼ご飯|昼ごはん"#)?,
                      |_| {
                          Ok(helpers::hour(12, false)?
                              .span_to(&helpers::hour(14, false)?, false)?
                              .latent()
                              .form(Form::Meal))
                      }
    );

    b.rule_1_terminal("dinner",
        b.reg(r#"夕食|夕ご飯|夕ごはん"#)?,
        |_| Ok(helpers::hour(18, false)?
                .span_to(&helpers::hour(23, false)?, false)?
                .latent()
                .form(Form::Meal))
    );

    b.rule_1_terminal("second dinner",
        b.reg(r#"夜食"#)?,
        |_| Ok(helpers::hour(22, false)?
                .span_to(&helpers::hour(0, false)?, false)? //should be until 2am the next day
                .latent()
                .form(Form::Meal))
    );

    b.rule_1_terminal("goûter :)",
        b.reg(r#"おやつ"#)?,
        |_| Ok(helpers::hour(15, false)?
                .span_to(&helpers::hour(17, false)?, false)?
                .form(Form::Meal))
    );
    b.rule_2("at <meal>",
        time_check!(form!(Form::Meal)),
        b.reg(r#"の?時に?|の?時間に"#)?,
        |a, _| Ok(a.value().clone().not_latent())
    );
    b.rule_2("after <meal>",
        time_check!(form!(Form::Meal)),
        b.reg(r#"の?後に|以降"#)?,
        |a, _| Ok(a.value().clone().not_latent())
    );
    b.rule_2("around <meal>",
        time_check!(form!(Form::Meal)),
        b.reg(r#"(?:ぐ|く)らいに"#)?,
        |a, _| Ok(a.value().clone().not_latent().precision(Approximate))
    );
    b.rule_3("around <meal>",
        b.reg(r#"だいたい"#)?,
        time_check!(form!(Form::Meal)),
        b.reg(r#"の時間に"#)?,
        |_, a, _| Ok(a.value().clone().not_latent().precision(Approximate))
    );
    b.rule_1_terminal("tonight",
                      b.reg(r#"今夜"#)?,
                      |_| {
                          let period = helpers::hour(18, false)?.span_to(&helpers::hour(0, false)?, false)?;
                          Ok(helpers::cycle_nth(Grain::Day, 0)?
                              .intersect(&period)?
                              .form(Form::PartOfDay(PartOfDayForm::Night)))
                      }
    );
    b.rule_1_terminal("after lunch",
                      b.reg(r#"(?:昼食後|ランチタイム後)に?"#)?,
                      |_| {
                          let period = helpers::hour(13, false)?.span_to(&helpers::hour(17, false)?, false)?;
                          Ok(helpers::cycle_nth(Grain::Day, 0)?
                              .intersect(&period)?
                              .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
                      }
    );
    b.rule_1_terminal("after work",
                      b.reg(r#"仕事の?後に?"#)?,
                      |_| {
                          let period = helpers::hour(17, false)?.span_to(&helpers::hour(0, false)?, false)?;
                          Ok(helpers::cycle_nth(Grain::Day, 0)?
                              .intersect(&period)?
                              .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
                      }
    );
    b.rule_2("last <part-of-day>",
        b.reg(r#"昨"#)?,
        time_check!(|time: &TimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
        |_, part_of_day| part_of_day.value().the_nth(-1)
    );
     b.rule_2("this <part-of-day>",
        b.reg(r#"今"#)?,
        time_check!(|time: &TimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
        |_, part_of_day| part_of_day.value().the_nth(0)
    );
    b.rule_3("<time> <part-of-day>",
             time_check!(|time: &TimeValue| !time.has_direction()),
             b.reg(r#"の"#)?,
             time_check!(|time: &TimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
             |time, _, part_of_day| part_of_day.value().intersect(time.value())
    );
    b.rule_2("<time> <part-of-day>",
             time_check!(),
             time_check!(|time: &TimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
             |time, part_of_day| part_of_day.value().intersect(time.value())
    );
    b.rule_1_terminal("week-end",
                      b.reg(r#"週末"#)?,
                      |_| {
                          let saturday = helpers::day_of_week(Weekday::Sat)?
                              .intersect(&helpers::hour(0, false)?)?;
                          let monday = helpers::day_of_week(Weekday::Mon)?
                              .intersect(&helpers::hour(0, false)?)?;
                          saturday.span_to(&monday, false)
                      }
    );
    b.rule_1_terminal("this week-end",
                      b.reg(r#"今週末"#)?,
                      |_| {
                          let saturday = helpers::day_of_week(Weekday::Sat)?
                              .intersect(&helpers::hour(0, false)?)?;
                          let monday = helpers::day_of_week(Weekday::Mon)?
                              .intersect(&helpers::hour(0, false)?)?;
                          saturday.the_nth(0)?.span_to(&monday.the_nth(0)?, false)
                      }
    );
    b.rule_1_terminal("season",
                      b.reg(r#"夏"#)?,
                      |_| helpers::month_day(6, 21)?.span_to(&helpers::month_day(9, 23)?, false)
    );
    b.rule_1_terminal("season",
                      b.reg(r#"秋"#)?,
                      |_| helpers::month_day(9, 23)?.span_to(&helpers::month_day(12, 21)?, false)
    );
    b.rule_1_terminal("season",
                      b.reg(r#"冬"#)?,
                      |_| helpers::month_day(12, 21)?.span_to(&helpers::month_day(3, 20)?, false)
    );
    b.rule_1_terminal("season",
                      b.reg(r#"春"#)?,
                      |_| helpers::month_day(3, 20)?.span_to(&helpers::month_day(6, 21)?, false)
    );
    b.rule_2("<time-of-day> approximately",
             b.reg(r#"だいたい"#)?,
             time_check!(form!(Form::TimeOfDay(_))),
             |_, time| Ok(time.value().clone().not_latent().precision(Precision::Approximate))
    );
    b.rule_2("<time-of-day> approximately",
             time_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"くらいに|頃"#)?,
             |time, _| Ok(time.value().clone().not_latent().precision(Precision::Approximate))
    );
    b.rule_2("<time-of-day> sharp",
             b.reg(r#"きっかり|ちょうど|丁度|ぴったり"#)?,
             time_check!(),
             |_, time| Ok(time.value().clone().not_latent().precision(Precision::Exact))
    );
    b.rule_2("<time-of-day> sharp",
             time_check!(),
             b.reg(r#"きっかり|ちょうど|丁度|ぴったり"#)?,
             |time, _| Ok(time.value().clone().not_latent().precision(Precision::Exact))
    );
    b.rule_5("<month> dd-dd (interval)",
             time_check!(form!(Form::Month(_))),
             integer_check_by_range!(1, 31),
             b.reg(r#"日から"#)?,
             integer_check_by_range!(1, 31),
             b.reg(r#"日まで"#)?,
             |time, a, _, b, _| {
                 let start = time.value()
                     .intersect(&helpers::day_of_month(a.value().value as u32)?)?;
                 let end = time.value()
                     .intersect(&helpers::day_of_month(b.value().value as u32)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_4("dd-dd (interval)",
             integer_check_by_range!(1, 31),
             b.reg(r#"日?ー"#)?,
             integer_check_by_range!(1, 31),
             b.reg(r#"日"#)?,
             |a, _, b, _| {
                 let start = &helpers::cycle_nth(Grain::Month, 0)?
                     .intersect(&helpers::day_of_month(a.value().value as u32)?)?;
                 let end = &helpers::cycle_nth(Grain::Month, 0)?
                     .intersect(&helpers::day_of_month(b.value().value as u32)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_4("dd-dd (interval)",
             integer_check_by_range!(1, 31),
             b.reg(r#"日から"#)?,
             integer_check_by_range!(1, 31),
             b.reg(r#"日まで"#)?,
             |a, _, b, _| {
                 let start = &helpers::cycle_nth(Grain::Month, 0)?
                     .intersect(&helpers::day_of_month(a.value().value as u32)?)?;
                 let end = &helpers::cycle_nth(Grain::Month, 0)?
                     .intersect(&helpers::day_of_month(b.value().value as u32)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_5("<month> dd-dd (interval)",
             time_check!(form!(Form::Month(_))),
             integer_check_by_range!(1, 31),
             b.reg(r#"日?ー"#)?,
             integer_check_by_range!(1, 31),
             b.reg(r#"日"#)?,
             |time, a, _, b, _| {
                 let start = time.value()
                     .intersect(&helpers::day_of_month(a.value().value as u32)?)?;
                 let end = time.value()
                     .intersect(&helpers::day_of_month(b.value().value as u32)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_4("<datetime> - <datetime> (interval)",
             time_check!(),
             b.reg(r#"から"#)?,
             time_check!(),
             b.reg(r#"まで"#)?,
             |a, _, b, _| a.value().span_to(b.value(), true)
    );
    b.rule_4("between <datetime> and <datetime> (interval)",
             time_check!(),
             b.reg(r#"と|から"#)?,
             time_check!(),
             b.reg(r#"の間に?"#)?,
             |a, _, b, _| a.value().span_to(b.value(), false)
    );
    b.rule_4("between <datetime> and <datetime> (interval)",
             time_check!(|time: &TimeValue| !time.latent && excluding_form!(Form::TimeOfDay(_))(time)),
             b.reg(r#"から"#)?,
             time_check!(|time: &TimeValue| !time.latent && excluding_form!(Form::TimeOfDay(_))(time)),
             b.reg(r#"にかけて"#)?,
             |a, _, b, _| a.value().span_to(b.value(), true)
    );
    b.rule_4("<time-of-day> - <time-of-day> (interval)",
             time_check!(|time: &TimeValue|  !time.latent && form!(Form::TimeOfDay(_))(time)),
             b.reg(r#"から"#)?,
             time_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"まで"#)?,
             |a, _, b, _| a.value().span_to(b.value(), false)
    );
    b.rule_5("between <time-of-day> am and <time-of-day> pm (interval)",
             b.reg(r#"(朝|夜)の"#)?,
             time_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"から(朝|夜)の"#)?,
             time_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"まで"#)?,
             |pod_marker1, tod1, pod_marker2, tod2, _| {
                let morning = helpers::hour(0, false)?.span_to(&helpers::hour(12, false)?, false)?;
                let afternoon =  helpers::hour(12, false)?.span_to(&helpers::hour(0, false)?, false)?;

                let start = tod1.value().intersect(
                        if pod_marker1.group(1) == "朝" {
                            &morning
                        } else {
                            &afternoon
                        })?;
                let end = tod2.value().intersect(
                        if pod_marker2.group(1) == "朝" {
                            &morning
                        } else {
                            &afternoon
                        })?;
                start.span_to(&end, false)
             }
    );
    b.rule_3("<datetime> before <time-of-day> (interval)",
             time_check!(form!(Form::TimeOfDay(_))),
             time_check!(|time: &TimeValue| !time.latent && excluding_form!(Form::TimeOfDay(_))(time)),
             b.reg(r#"(?:前|まで)に?"#)?,
             |a, b, _| b.value().span_to(a.value(), false)
    );
    b.rule_2("within <duration>",
             duration_check!(),
             b.reg(r#"以内に"#)?,
             |a, _| helpers::cycle_nth(Grain::Second, 0)?.span_to(&a.value().in_present()?, false)
    );
    b.rule_2("by <time>",
             time_check!(),
             b.reg(r#"までに"#)?,
             |a, _| helpers::cycle_nth(Grain::Second, 0)?.span_to(a.value(), false)
    );
    b.rule_3("by <time>",
             b.reg(r#"向こう|今から"#)?,
             time_check!(),
             b.reg(r#"間"#)?,
             |_, a, _| helpers::cycle_nth(Grain::Second, 0)?.span_to(a.value(), false)
    );

    b.rule_2("by the end of <time>",
             time_check!(),
             b.reg(r#"の?(?:終わり|末)までに"#)?,
             |a, _| helpers::cycle_nth(Grain::Second, 0)?.span_to(a.value(), true)
    );
    b.rule_1_terminal("until the begining of the evening",
        b.reg(r#"昼間に"#)?,
        |_| helpers::cycle_nth(Grain::Second, 0)?.span_to(&helpers::hour(18, false)?, false)
    );
    b.rule_3("before the end of <time>",
        b.reg(r#"今"#)?,
        time_check!(),
        b.reg(r#"中に"#)?,
        |_, cycle, _| helpers::cycle_nth(Grain::Second, 0)?.span_to(cycle.value(), true)
    );
    b.rule_1_terminal("before the end of the year",
        b.reg(r#"年内に"#)?,
        |_| helpers::cycle_nth(Grain::Second, 0)?.span_to(&helpers::cycle_nth(Grain::Year, 1)?, false)
    );
    b.rule_2("until <time>",
             time_check!(),
             b.reg(r#"後まで"#)?,
             |a, _| Ok(a.value().clone().mark_before_end())
    );
    b.rule_2("before <time>",
             time_check!(),
             b.reg(r#"の?(?:前に|以前)"#)?,
             |a, _| Ok(a.value().clone().mark_before_start())
    );
    b.rule_2("after <time>",
             time_check!(),
             b.reg(r#"の?後に?|以降|すぎ"#)?,
             |a, _| Ok(a.value().clone().mark_after_end())
    );
    b.rule_2("from <time-of-day>",
             time_check!(),
             b.reg(r#"から"#)?,
             |a, _| Ok(a.value().the_nth(0)?.mark_after_start())
    );
    b.rule_2("since <time-of-day>",
             time_check!(),
             b.reg(r#"から|以来"#)?,
             |a, _| Ok(a.value().the_nth(-1)?.mark_after_start())
    );
    b.rule_5("since <time> and during <time>",
             time_check!(),
             b.reg(r#"から|以来"#)?,
             integer_check!(),
             cycle_check!(),
             b.reg(r#"間"#)?,
             |a, _, b, c, _| a.value().the_nth(0)?.mark_after_start().span_to(&helpers::cycle_nth(c.value().grain, b.value().value)?, true)
    );
    b.rule_2("around <time-of-day>",
        time_check!(),
        b.reg(r#"くらいに"#)?,
        |a, _| Ok(a.value().clone().not_latent().precision(Approximate))
    );

    b.rule_4("date using emperor years",
        b.reg(r#"(平成|昭和)"#)?,
        integer_check!(),
        b.reg(r#"年"#)?,
        time_check!(form!(Form::MonthDay(_))),
        |emperor, integer, _, month_day| {
            let start_year = match emperor.group(1).as_ref() {
                "平成" => 1989,
                "昭和" => 1926,
                _ => return Err(RuleErrorKind::Invalid.into()),
            };

            let year = integer.value().value + &start_year - 1;
            month_day.value().intersect(&helpers::year(year as i32)?)
        }
    );
    Ok(())
}

pub fn rules_duration(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("second (unit-of-duration)",
                      b.reg(r#"秒間?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Second))
    );
    b.rule_1_terminal("minute (unit-of-duration)",
                      b.reg(r#"分間?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Minute))
    );
    b.rule_1_terminal("hour (unit-of-duration)",
                      b.reg(r#"時間"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Hour))
    );
    b.rule_1_terminal("day (unit-of-duration)",
                      b.reg(r#"日間?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Day))
    );
    b.rule_1_terminal("week (unit-of-duration)",
                      b.reg(r#"週間"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Week))
    );
    b.rule_1_terminal("month (unit-of-duration)",
                      b.reg(r#"(?:カ|ヶ|か)月間?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Month))
    );
    b.rule_1_terminal("year (unit-of-duration)",
                      b.reg(r#"年間?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Year))
    );
    b.rule_1_terminal("half a year",
        b.reg(r#"半年"#)?,
        |_| Ok(DurationValue::new(PeriodComp::new(Grain::Month, 6).into()))
    );
     b.rule_1_terminal("half a month",
        b.reg(r#"半月"#)?,
        |_| Ok(DurationValue::new(PeriodComp::new(Grain::Day, 15).into()))
    );
    b.rule_1_terminal("half a day",
        b.reg(r#"半日"#)?,
        |_| Ok(DurationValue::new(PeriodComp::new(Grain::Hour, 12).into()))
    );
    b.rule_2("<integer> <unit-of-duration>",
             integer_check_by_range!(0),
             unit_of_duration_check!(),
             |integer, uod| Ok(DurationValue::new(PeriodComp::new(uod.value().grain, integer.value().value).into()))
    );
    b.rule_3("<integer> more <unit-of-duration>",
             b.reg(r#"もう|後|あと"#)?,
             integer_check_by_range!(0),
             unit_of_duration_check!(),
             |_, integer, uod| Ok(DurationValue::new(PeriodComp::new(uod.value().grain, integer.value().value).into()))
    );
    b.rule_3("<integer> less <unit-of-duration>",
             integer_check_by_range!(0),
             unit_of_duration_check!(),
             b.reg(r#"短く"#)?,
             |integer, uod, _| Ok(DurationValue::new(PeriodComp::new(uod.value().grain, integer.value().value).into()))
    );
    b.rule_2_terminal("number.number hours",
                      b.reg(r#"(\d+)\.(\d+)"#)?,
                      b.reg(r#"時間"#)?,
                      |text_match, _| {
                          Ok(DurationValue::new(
                              PeriodComp::minutes(
                                  helpers::decimal_hour_in_minute(text_match.group(1), text_match.group(2))?
                              ).into()
                          )
                          )
                      }
    );
    b.rule_3("<integer> <unit-of-duration> and a half",
             integer_check_by_range!(0),
             unit_of_duration_check!(),
             b.reg(r#"半"#)?,
             |integer, uod, _| {
                let half_period: Period = uod.value().grain.half_period().map(|a| a.into()).unwrap_or_else(|| Period::default());
                Ok(DurationValue::new(half_period + PeriodComp::new(uod.value().grain, integer.value().value)))
            }
    );
    b.rule_2("in <duration>",
             duration_check!(),
             b.reg(r#"で|後"#)?,
             |duration, _| duration.value().in_present()
    );
    b.rule_3("before <duration>",
             duration_check!(),
             b.reg(r#"前"#)?,
             time_check!(),
             |duration, _, time| duration.value().before(time.value())
    );
    b.rule_3("in <duration> <part-of-day>",
             duration_check!(),
             b.reg(r#"後の?"#)?,
             time_check!(),
             |duration, _, time| {
                let this_time = time.value().the_nth(0)?;
                duration.value().after(&this_time)
            }
    );
    // b.rule_2("after <duration>",
    //          duration_check!(),
    //          b.reg(r#"後"#)?,
    //          |duration, _| Ok(duration
    //              .value()
    //              .in_present()?
    //              .mark_after_start())
    // );
    b.rule_3("<duration> and <duration-after-addition>",
             duration_check!(|duration: &DurationValue| !duration.suffixed && !duration.is_from_addition()),
             b.reg(r#"と"#)?,
             duration_check!(|duration: &DurationValue| !duration.prefixed && duration.is_from_addition()),
             |a, _, b| Ok(a.value() + b.value())
    );

    b.rule_2("<duration> <duration-after-addition>",
             duration_check!(|duration: &DurationValue| !duration.suffixed && !duration.is_from_addition()),
             duration_check!(|duration: &DurationValue| !duration.prefixed && duration.is_from_addition()),
             |a, b| Ok(a.value() + b.value())
    );

    b.rule_3("<duration> and <duration>",
             duration_check!(|duration: &DurationValue| !duration.suffixed && !duration.is_from_addition()),
             b.reg(r#"と"#)?,
             duration_check!(|duration: &DurationValue| !duration.prefixed && !duration.is_from_addition()),
             |a, _, b| Ok(a.value() + b.value())
    );

    b.rule_2("<duration> <duration>",
             duration_check!(|duration: &DurationValue| !duration.suffixed && !duration.is_from_addition()),
             duration_check!(|duration: &DurationValue| !duration.prefixed && !duration.is_from_addition()),
             |a, b| Ok(a.value() + b.value())
    );

    b.rule_2("<duration> from now",
             b.reg(r#"今から"#)?,
             duration_check!(),
             |_, a| a.value().in_present()
    );

    b.rule_2("<duration> ago",
             duration_check!(),
             b.reg(r#"前(?:の|に)?"#)?,
             |a, _| a.value().ago()
    );

    b.rule_4("<duration> after <time>",
             time_check!(),
             b.reg(r#"の"#)?,
             duration_check!(),
             b.reg(r#"後に?"#)?,
             |time, _, duration, _| duration.value().after(time.value())
    );

    b.rule_4("<duration> before <time>",
             time_check!(),
             b.reg(r#"の"#)?,
             duration_check!(),
             b.reg(r#"前に?"#)?,
             |time, _, duration, _| duration.value().before(time.value())
    );
    b.rule_4("<duration> before <time>",
             time_check!(),
             b.reg(r#"の"#)?,
             b.reg(r#"前の"#)?,
             duration_check!(),
             |time, _, _, duration| duration.value().before(time.value())
    );
    b.rule_2("about <duration>",
             b.reg(r#"だいたい|約"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().precision(Precision::Approximate))
    );
    b.rule_2("about <duration>",
             duration_check!(),
             b.reg(r#"くらい|ほど"#)?,
             |duration, _| Ok(duration.value().clone().precision(Precision::Approximate))
    );
    b.rule_2("<duration> (approximate)",
             b.reg(r#"数"#)?,
             cycle_check!(),
             |_, cycle| Ok(DurationValue::new(PeriodComp::new(cycle.value().grain, 3).into()).precision(Precision::Approximate))
    );

    b.rule_2("exactly <duration>",
             b.reg(r#"きっちり|ぴったり|ちょうど|丁度"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().precision(Precision::Exact))
    );
    b.rule_2("exactly <duration>",
             duration_check!(),
             b.reg(r#"きっちり|ぴったり|ちょうど|丁度"#)?,
             |duration, _| Ok(duration.value().clone().precision(Precision::Exact))
    );

    b.rule_3("exactly <duration>",
             b.reg(r#"きっちり|ぴったり|ちょうど|丁度"#)?,
             duration_check!(),
             b.reg(r#"に"#)?,
             |_, duration, _| Ok(duration.value().clone().precision(Precision::Exact))
    );
    b.rule_3("exactly <duration>",
             duration_check!(),
             b.reg(r#"きっちり|ぴったり|ちょうど|丁度"#)?,
             b.reg(r#"に"#)?,
             |duration, _, _| Ok(duration.value().clone().precision(Precision::Exact))
    );
    b.rule_2("exactly <duration>",
             duration_check!(),
             b.reg(r#"まるまる"#)?,
             |duration, _| Ok(duration.value().clone().precision(Precision::Exact))
    );

    Ok(())
}

pub fn rules_cycle(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("second (cycle)",
                      b.reg(r#"秒"#)?,
                      |_| CycleValue::new(Grain::Second)
    );
    b.rule_1_terminal("minute (cycle)",
                      b.reg(r#"分"#)?,
                      |_| CycleValue::new(Grain::Minute)
    );
    b.rule_1_terminal("hour (cycle)",
                      b.reg(r#"時"#)?,
                      |_| CycleValue::new(Grain::Hour)
    );
    b.rule_1_terminal("day (cycle)",
                      b.reg(r#"日"#)?,
                      |_| CycleValue::new(Grain::Day)
    );
    b.rule_1_terminal("week (cycle)",
                      b.reg(r#"週"#)?,
                      |_| CycleValue::new(Grain::Week)
    );
    b.rule_1_terminal("month (cycle)",
                      b.reg(r#"(?:カ|か|ヶ)?月"#)?,
                      |_| CycleValue::new(Grain::Month)
    );
    b.rule_1_terminal("quarter (cycle)",
                      b.reg(r#"四半期"#)?,
                      |_| CycleValue::new(Grain::Quarter)
    );
    b.rule_1_terminal("year (cycle)",
                      b.reg(r#"年"#)?,
                      |_| CycleValue::new(Grain::Year)
    );
    b.rule_2("this <cycle>",
             b.reg(r#"今|当"#)?,
             cycle_check!(),
             |_, a| helpers::cycle_nth(a.value().grain, 0)
    );
    b.rule_2("last <cycle>",
             b.reg(r#"去|先|昨|前の?"#)?,
             cycle_check!(),
             |_, a| helpers::cycle_nth(a.value().grain, -1)
    );
    b.rule_1_terminal("new year",
             b.reg(r#"新年"#)?,
             |_| helpers::cycle_nth(Grain::Year, 1)
    );
    b.rule_2("next <cycle>",
             b.reg(r#"来|次の"#)?,
             cycle_check!(),
             |_, a| helpers::cycle_nth(a.value().grain, 1)
    );
    b.rule_2("<cycle> after next <cycle>",
             b.reg(r#"再来"#)?,
             cycle_check!(),
             |_, a| helpers::cycle_nth(a.value().grain, 2)
    );
    b.rule_4("last n <cycle>",
             b.reg(r#"過去"#)?,
             integer_check!(),
             cycle_check!(),
             b.reg(r#"間で?"#)?,
             |_, integer, cycle, _| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    b.rule_4("last n <cycle>",
             b.reg(r#"過去"#)?,
             integer_check!(),
             cycle_check!(),
             b.reg(r#"間で?"#)?,
             |_, integer, cycle, _| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    b.rule_4("last n <cycle>",
             b.reg(r#"ここ"#)?,
             integer_check!(),
             cycle_check!(),
             b.reg(r#"間?で"#)?,
             |_, integer, cycle, _| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    b.rule_4("next n <cycle>",
             b.reg(r#"次の|これから|今から|今後"#)?,
             integer_check!(),
             cycle_check!(),
             b.reg(r#"間"#)?,
             |_, integer, cycle, _| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    b.rule_3("<1..4> quarter",
             b.reg(r#"第"#)?,
             integer_check_by_range!(1, 4),
             cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Quarter),
             |_, integer, _cycle| helpers::cycle_nth_after(Grain::Quarter, integer.value().value - 1, &helpers::cycle_nth(Grain::Year, 0)?)
    );
    b.rule_4("<year> <1..4>quarter",
             time_check!(),
             b.reg(r#"第"#)?,
             integer_check_by_range!(1, 4),
             cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Quarter),
             |time, _, integer, _| helpers::cycle_nth_after(Grain::Quarter, integer.value().value - 1, time.value())
    );
    Ok(())
}