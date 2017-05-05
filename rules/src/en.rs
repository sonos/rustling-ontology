use rustling::*;
use dimension::*;
use dimension::Precision::*;
use helpers;
use moment::{Weekday, Grain};

pub fn rules_time(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("intersect",
        time_check!(|time: &TimeValue| !time.latent),
        time_check!(|time: &TimeValue| !time.latent),
        |a, b| a.value().intersect(b.value())
    );
    b.rule_3("intersect by \"of\", \"from\", \"'s\"",
        time_check!(|time: &TimeValue| !time.latent),
        b.reg(r#"of|from|for|'s"#)?,
        time_check!(|time: &TimeValue| !time.latent),
        |a, _, b| a.value().intersect(b.value())
    );
    b.rule_3("intersect by \",\"",
        time_check!(|time: &TimeValue| !time.latent),
        b.reg(r#","#)?,
        time_check!(|time: &TimeValue| !time.latent),
        |a, _, b| a.value().intersect(b.value())
    );
    b.rule_2("on <date>",
        b.reg(r#"on"#)?,
        time_check!(),
        |_, a| Ok(a.value().clone())
    );
    b.rule_2("on a <named-day>",
        b.reg(r#"on a"#)?,
        time_check_form!(Form::DayOfWeek{..}),
        |_, a| Ok(a.value().clone())
    );
    b.rule_2("in <named-month>",
        b.reg(r#"in"#)?,
        time_check_form!(Form::Month(_)),
        |_, a| Ok(a.value().clone())
    );
    b.rule_1("named-day", b.reg(r#"monday|mon\.?"#)?, |_| {
        helpers::day_of_week(Weekday::Mon)
    });
    b.rule_1("named-day", b.reg(r#"tuesday|tues?\.?"#)?, |_| {
        helpers::day_of_week(Weekday::Tue)
    });
    b.rule_1("named-day", b.reg(r#"wed?nesday|wed\.?"#)?, |_| {
        helpers::day_of_week(Weekday::Wed)
    });
    b.rule_1("named-day", b.reg(r#"thursday|thu(rs?)?\.?"#)?, |_| {
        helpers::day_of_week(Weekday::Thu)
    });
    b.rule_1("named-day", b.reg(r#"friday|fri\.?"#)?, |_| {
        helpers::day_of_week(Weekday::Fri)
    });
    b.rule_1("named-day", b.reg(r#"saturday|sat\.?"#)?, |_| {
        helpers::day_of_week(Weekday::Sat)
    });
    b.rule_1("named-day", b.reg(r#"sunday|sun\.?"#)?, |_| {
        helpers::day_of_week(Weekday::Sun)
    });
    b.rule_1("named-month", b.reg(r#"january|jan\.?"#)?, |_| {
        helpers::month(1)
    });
    b.rule_1("named-month", b.reg(r#"february|feb\.?"#)?, |_| {
        helpers::month(2)
    });
    b.rule_1("named-month", b.reg(r#"march|mar\.?"#)?, |_| {
        helpers::month(3)
    });
    b.rule_1("named-month", b.reg(r#"april|apr\.?"#)?, |_| {
        helpers::month(4)
    });
    b.rule_1("named-month", b.reg(r#"may"#)?, |_| {
        helpers::month(5)
    });
    b.rule_1("named-month", b.reg(r#"june|jun\.?"#)?, |_| {
        helpers::month(6)
    });
    b.rule_1("named-month", b.reg(r#"july|jul\.?"#)?, |_| {
        helpers::month(7)
    });
    b.rule_1("named-month", b.reg(r#"august|aug\.?"#)?, |_| {
        helpers::month(8)
    });
    b.rule_1("named-month", b.reg(r#"september|sept?\.?"#)?, |_| {
        helpers::month(9)
    });
    b.rule_1("named-month", b.reg(r#"october|oct\.?"#)?, |_| {
        helpers::month(10)
    });
    b.rule_1("named-month", b.reg(r#"november|nov\.?"#)?, |_| {
        helpers::month(11)
    });
    b.rule_1("named-month", b.reg(r#"december|dec\.?"#)?, |_| {
        helpers::month(12)
    });
    b.rule_1("christmas", b.reg(r#"(xmas|christmas)( day)?"#)?, |_| {
        helpers::month_day(12, 25)
    });
    b.rule_1("christmas eve",
             b.reg(r#"(xmas|christmas)( day)?('s)? eve"#)?,
             |_| {
                 helpers::month_day(12, 24)
             });
    b.rule_1("new year's eve", b.reg(r#"new year'?s? eve"#)?, |_| {
        helpers::month_day(12, 31)
    });
    b.rule_1("new year's day", b.reg(r#"new year'?s?( day)?"#)?, |_| {
        helpers::month_day(1, 1)
    });
    b.rule_1("valentine's day", b.reg(r#"valentine'?s?( day)?"#)?, |_| {
        helpers::month_day(2, 14)
    });
    b.rule_1("MLK Day",
             b.reg(r#"(MLK|Martin Luther King,?)( Jr.?| Junior)? day"#)?,
             |_| {
                 let third_week_january =
                     helpers::cycle_nth_after(Grain::Week, 3, &helpers::month_day(1, 1)?)?;
                 let january = helpers::month(1)?;
                 let monday = helpers::day_of_week(Weekday::Mon)?;
                 january.intersect(&third_week_january)?.intersect(&monday)
             });

    b.rule_1("memorial day", b.reg(r#"memorial day"#)?, |_| {
        let monday = helpers::day_of_week(Weekday::Mon)?;
        let may = helpers::month(5)?;
        monday.last_of(&may)
    });
    b.rule_1("memorial day weekend",
             b.reg(r#"memorial day week(\s|-)?end"#)?,
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
    b.rule_1("independence day",
            b.reg(r#"independence day"#)?,
            |_| {
                helpers::month_day(7, 4)
            }
    );
    b.rule_1("labor day",
            b.reg(r#"labor day"#)?,
            |_| {
                helpers::month(9)?.intersect(&helpers::day_of_week(Weekday::Mon)?)
            }
    );
    b.rule_1("labor day weekend",
            b.reg(r#"labor day week(\s|-)?end"#)?,
            |_| {
                let start = helpers::cycle_nth_after(Grain::Day, -3, &helpers::month(9)?.intersect(&helpers::day_of_week(Weekday::Mon)?)?)?
                            .intersect(&helpers::hour(18, false)?)?;
                let end = helpers::month(9)?.intersect(&helpers::day_of_week(Weekday::Tue)?)?
                            .intersect(&helpers::hour(0, false)?)?;
                start.span_to(&end, false)
            }
    );
    b.rule_1("Father's Day",
            b.reg(r#"father'?s?'? day"#)?,
            |_| {
                let sundays_of_june = helpers::month(6)?.intersect(&helpers::day_of_week(Weekday::Sun)?)?;
                let second_week_of_june = helpers::cycle_nth_after(Grain::Week, 2, &helpers::month_day(6, 1)?)?;
                sundays_of_june.intersect(&second_week_of_june) // third sunday of June
            }
    );
    b.rule_1("Mother's Day",
            b.reg(r#"mother'?s? day"#)?,
            |_| {
                let sundays_of_may = helpers::month(5)?.intersect(&helpers::day_of_week(Weekday::Sun)?)?;
                let first_week_of_may = helpers::cycle_nth_after(Grain::Week, 1, &helpers::month_day(5, 1)?)?;
                sundays_of_may.intersect(&first_week_of_may) // second sunday of May
            }
    );
    b.rule_1("halloween day",
            b.reg(r#"hall?owe?en( day)?"#)?,
            |_| {
                helpers::month_day(10, 31)
            }
    );
    b.rule_1("thanksgiving day",
        b.reg(r#"thanks?giving( day)?"#)?,
        |_| {
            let thursday_november = helpers::month(11)?.intersect(&helpers::day_of_week(Weekday::Thu)?)?;
            let fourth_week_of_november = helpers::cycle_nth_after(Grain::Week, 4, &helpers::month_day(11, 1)?)?;
            thursday_november.intersect(&fourth_week_of_november) // fourth thursday of november
        }
    );
    b.rule_1("black friday",
        b.reg(r#"black frid?day"#)?,
        |_| {
            let thursday_november = helpers::month(11)?.intersect(&helpers::day_of_week(Weekday::Fri)?)?;
            let fourth_week_of_november = helpers::cycle_nth_after(Grain::Week, 4, &helpers::month_day(11, 1)?)?;
            thursday_november.intersect(&fourth_week_of_november) // fourth friday of november
        }
    );
    b.rule_2("absorption of , after named day",
        time_check_form!(Form::DayOfWeek{..}),
        b.reg(r#","#)?,
        |a, _| Ok(a.value().clone())
    );

    b.rule_1("now",
        b.reg(r#"(just|right)? ?now|immediately"#)?,
        |_| {
            helpers::cycle_nth(Grain::Second, 0)
        }
    );
    b.rule_1("today",
        b.reg(r#"todays?|(at this time)"#)?,
        |_| {
            helpers::cycle_nth(Grain::Day, 0)
        }
    );
    b.rule_1("tomorrow",
        b.reg(r#"(tmrw?|tomm?or?rows?)"#)?,
        |_| {
            helpers::cycle_nth(Grain::Day, 1)
        }
    );
    b.rule_1("yesterday",
        b.reg(r#"yesterdays?"#)?,
        |_| {
            helpers::cycle_nth(Grain::Day, -1)
        }
    );
    b.rule_1("EOM|End of month",
        b.reg(r#"(the )?(eom|end of (the )?month)"#)?,
        |_| {
            helpers::cycle_nth(Grain::Month, 1)
        }
    );
    b.rule_1("EOY|End of year",
        b.reg(r#"(the )?(eoy|end of (the )?year)"#)?,
        |_| {
            helpers::cycle_nth(Grain::Year, 1)
        }
    );

    b.rule_2("this|next <day-of-week>",
        b.reg(r#"this|next"#)?,
        time_check_form!(Form::DayOfWeek{..}),
        |_, a| {
            a.value().the_nth_not_immediate(0)
        }

    );
    b.rule_2("this <time>",
        b.reg(r#"this|current|coming"#)?,
        time_check!(),
        |_, a| {
            a.value().the_nth(0)
        }
    );
    b.rule_2("next <time>",
        b.reg(r#"next"#)?,
        time_check!(|time: &TimeValue| !time.latent),
        |_, a| {
            a.value().the_nth(0)
        }
    );
    b.rule_2("last <time>",
        b.reg(r#"(this past|last)"#)?,
        time_check!(),
        |_, a| {
            a.value().the_nth(-1)
        }
    );
    b.rule_2("<time> after next",
        time_check!(), b.reg(r#"after next"#)?,
        |a, _| {
            a.value().the_nth_not_immediate(1)
        }
    );
    b.rule_2("<time> before last",
        time_check!(), b.reg(r#"before last"#)?,
        |a, _| {
            a.value().the_nth(-2)
        }
    );
    b.rule_4("last <day-of-week> of <time>",
        b.reg(r#"last"#)?, 
        time_check_form!(Form::DayOfWeek{..}),
        b.reg(r#"of"#)?,
        time_check!(),
        |_, a, _, b| {
            a.value().last_of(b.value())
        } 
    );
    b.rule_4("nth <time> of <time>",
        ordinal_check!(),
        time_check!(),
        b.reg(r#"of|in"#)?,
        time_check!(),
        |ordinal, a, _, b| {
            b.value().intersect(a.value())?.the_nth(ordinal.value().value - 1)
        }
    );
    b.rule_5("nth <time> of <time>",
        b.reg(r#"the"#)?,
        ordinal_check!(),
        time_check!(),
        b.reg(r#"of|in"#)?,
        time_check!(),
        |_, ordinal, a, _, b| {
            b.value().intersect(a.value())?.the_nth(ordinal.value().value - 1)
        }
    );
    b.rule_4("nth <time> after <time>",
        ordinal_check!(),
        time_check!(),
        b.reg(r#"after"#)?,
        time_check!(),
        |ordinal, a, _, b| {
            a.value().the_nth_after(ordinal.value().value - 1, b.value())
        }
    );
    b.rule_5("nth <time> after <time>",
        b.reg(r#"the"#)?,
        ordinal_check!(),
        time_check!(),
        b.reg(r#"after"#)?,
        time_check!(),
        |_, ordinal, a, _, b| {
            a.value().the_nth_after(ordinal.value().value - 1, b.value())
        }
    );
    b.rule_1("year",
        integer_check!(1000, 2100),
        |integer| {
            helpers::year(integer.value().value as i32)
        }
    );  
    b.rule_1("year (latent)",
        integer_check!(-1000, 999),
        |integer| {
            Ok(helpers::year(integer.value().value as i32)?.latent())
        }
    );
    b.rule_1("year (latent)",
        integer_check!(2101, 2200),
        |integer| {
            Ok(helpers::year(integer.value().value as i32)?.latent())
        }
    );
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
    b.rule_2("the <day-of-month> (non ordinal)",
        b.reg(r#"the"#)?,
        integer_check!(1, 31),
        |_, integer| {
            Ok(helpers::day_of_month(integer.value().value as u32)?.latent())
        }
    );
    b.rule_2("<named-day> <day-of-month> (ordinal)",
        time_check_form!(Form::DayOfWeek{..}),
        ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
        |a, ordinal| {
            a.value().intersect(&helpers::day_of_month(ordinal.value().value as u32)?)
        }
    );
    b.rule_2("<named-month> <day-of-month> (ordinal)", // march 12th
        time_check_form!(Form::Month{..}),
        ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
        |a, ordinal| {
            a.value().intersect(&helpers::day_of_month(ordinal.value().value as u32)?)
        }
    );
    b.rule_2("<named-month> <day-of-month> (non ordinal)",
        time_check_form!(Form::Month(_)),
        integer_check!(1, 31),
        |a, integer| {
            a.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
        }
    );
    b.rule_3("<day-of-month> (ordinal) of <named-month>",
        ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
        b.reg(r#""of|in""#)?,
        time_check_form!(Form::Month(_)),
        |ordinal, _, a| {
            a.value().intersect(&helpers::day_of_month(ordinal.value().value as u32)?)
        }
    );
    b.rule_3("<day-of-month> (non ordinal) of <named-month>",
        integer_check!(1, 31),
        b.reg(r#"of|in"#)?,
        time_check_form!(Form::Month(_)),
        |integer, _, a| {
            a.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
        }
    );
    b.rule_2("<day-of-month> (non ordinal) <named-month>",
        integer_check!(1, 31),
        time_check_form!(Form::Month(_)),
        |integer, a| {
            a.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
        }
    );
    b.rule_2("<day-of-month>(ordinal) <named-month>", //12nd march
        ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
        time_check_form!(Form::Month(_)),
        |ordinal, a| {
            a.value().intersect(&helpers::day_of_month(ordinal.value().value as u32)?)
        }
    );
    b.rule_3("<day-of-month>(ordinal) <named-month> year",
        ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
        time_check_form!(Form::Month(_)),
        b.reg(r#"(\d{2,4})"#)?,
        |ordinal, a, text_match| {
            let year: i32 = text_match.group(1).parse()?;
            a.value().intersect(&helpers::day_of_month(ordinal.value().value as u32)?)?.intersect(&helpers::year(year)?)
        }
    );
    Ok(())
}

pub fn rules_finance(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("intersect (X cents)",
             amount_of_money_check!(),
             amount_of_money_check!(|money: &AmountOfMoneyValue| money.unit == Some("cent")),
             |a, b| helpers::compose_money(a.value(), b.value()));
    b.rule_3("intersect (and X cents)",
             amount_of_money_check!(),
             b.reg(r#"and"#)?,
             amount_of_money_check!(|money: &AmountOfMoneyValue| money.unit == Some("cent")),
             |a, _, b| helpers::compose_money(&a.value(), &b.value()));
    b.rule_2("intersect",
             amount_of_money_check!(),
             number_check!(),
             |a, b| helpers::compose_money_number(&a.value(), &b.value()));
    b.rule_3("intersect (and number)",
             amount_of_money_check!(),
             b.reg(r#"and"#)?,
             number_check!(),
             |a, _, b| helpers::compose_money_number(&a.value(), &b.value()));
    b.rule_1("$",
             b.reg(r#"\$|dollars?"#)?,
             |_| Ok(MoneyUnitValue { unit: Some("$") }));
    b.rule_1("€",
             b.reg(r#"€|([e€]uro?s?)"#)?,
             |_| Ok(MoneyUnitValue { unit: Some("€") }));
    b.rule_1("£",
             b.reg(r#"£|pounds?"#)?,
             |_| Ok(MoneyUnitValue { unit: Some("£") }));
    b.rule_1("USD",
             b.reg(r#"us[d\$]"#)?,
             |_| Ok(MoneyUnitValue { unit: Some("USD") }));
    b.rule_1("GBP",
             b.reg(r#"gbp"#)?,
             |_| Ok(MoneyUnitValue { unit: Some("GBP") }));
    b.rule_1("PTS",
             b.reg(r#"pta?s?"#)?,
             |_| Ok(MoneyUnitValue { unit: Some("PTS") }));
    b.rule_1("cent",
             b.reg(r#"cents?|penn(y|ies)|c|¢"#)?,
             |_| Ok(MoneyUnitValue { unit: Some("cent") }));
    b.rule_1("INR",
             b.reg(r#""#)?,
             |_| Ok(MoneyUnitValue { unit: Some("INR") }));
    b.rule_1("unnamed currency",
             b.reg(r#"(buck|balle|pouloute)s?"#)?,
             |_| Ok(MoneyUnitValue { unit: None }));
    b.rule_2("<unit> <amount>", money_unit!(), number_check!(), |a, b| {
        Ok(AmountOfMoneyValue {
               value: b.value().value(),
               unit: a.value().unit,
               ..AmountOfMoneyValue::default()
           })
    });
    b.rule_2("<amount> <unit>", number_check!(), money_unit!(), |a, b| {
        Ok(AmountOfMoneyValue {
               value: a.value().value(),
               unit: b.value().unit,
               ..AmountOfMoneyValue::default()
           })
    });
    b.rule_2("about <amount-of-money>",
             b.reg(r#"about|approx(\.|imately)?|close to|near( to)?|around|almost"#)?,
             amount_of_money_check!(),
             |_, a| {
                 Ok(AmountOfMoneyValue {
                        precision: Approximate,
                        ..a.value().clone()
                    })
             });
    b.rule_2("exactly <amount-of-money>",
             b.reg(r#"exactly|precisely"#)?,
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
    b.rule_1("number as temp", number_check!(), |a| {
        Ok(TemperatureValue {
               value: a.value().value(),
               unit: None,
               latent: true,
           })
    });
    b.rule_2("<latent temp> degrees",
             temperature_check!(),
             b.reg(r#"(?:deg(?:ree?)?s?\.?)|°"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                        value: a.value().value,
                        unit: Some("degree"),
                        latent: false,
                    })
             });
    b.rule_2("<temp> Celcius",
             temperature_check!(),
             b.reg(r#"c(?:el[cs]?(?:ius)?)?\.?"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                        value: a.value().value,
                        unit: Some("celsius"),
                        latent: false,
                    })
             });
    b.rule_2("<temp> Fahrenheit",
             temperature_check!(),
             b.reg(r#"f(?:ah?rh?eh?n(?:h?eit)?)?\.?"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                        value: a.value().value,
                        unit: Some("fahrenheit"),
                        latent: false,
                    })
             });
    Ok(())
}

pub fn rules_numbers(b:&mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_3("intersect (with and)",
             number_check!(|number: &NumberValue| number.grain().unwrap_or(0) > 1),
             b.reg(r#"and"#)?,
             number_check!(),
             |a, _, b| helpers::compose_numbers(&a.value(), &b.value()));
    b.rule_2("intersect",
             number_check!(|number: &NumberValue| number.grain().unwrap_or(0) > 1),
             number_check!(),
             |a, b| helpers::compose_numbers(&a.value(), &b.value()));
    b.rule_1(
            "integer (0..19)", 
            b.reg(r#"(none|zilch|naught|nought|nil|zero|one|two|three|fourteen|four|five|sixteen|six|seventeen|seven|eighteen|eight|nineteen|nine|eleven|twelve|thirteen|fifteen)"#)?,
            |text_match| {
                let value = match text_match.group(1).as_ref()  {
                    "none" => 0, 
                    "zilch" => 0, 
                    "naught" => 0, 
                    "nought" => 0, 
                    "nil" => 0, 
                    "zero" => 0,
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
                    "thirteen" => 13,
                    "fourteen" => 14,
                    "fifteen" => 15,
                    "sixteen" => 16,
                    "seventeen" => 17, 
                    "eighteen" => 18, 
                    "nineteen" => 19,
                    _ => panic!("Unknow match"),
                };
                IntegerValue::new_with_grain(value, 1) 
            });
    b.rule_1("ten",
             b.reg(r#"ten"#)?,
             |_| IntegerValue::new_with_grain(10, 1));
    b.rule_1("single",
             b.reg(r#"single"#)?,
             |_| IntegerValue::new_with_grain(1, 1));
    b.rule_1("a pair",
             b.reg(r#"a pair(?: of)?"#)?,
             |_| IntegerValue::new_with_grain(2, 1));
    b.rule_1("dozen",
             b.reg(r#"dozen"#)?,
             |_| IntegerValue::new_with_grain(12, 1));
    b.rule_1("hundred",
             b.reg(r#"hundreds?"#)?,
             |_| IntegerValue::new_with_grain(100, 2));
    b.rule_1("thousand",
             b.reg(r#"thousands?"#)?,
             |_| IntegerValue::new_with_grain(1000, 3));
    b.rule_1("million",
             b.reg(r#"millions?"#)?,
             |_| IntegerValue::new_with_grain(1000000, 6));
    b.rule_1("couple",
             b.reg(r#"(a )?couple( of)?"#)?,
             |_| IntegerValue::new_with_grain(2, 1));
    b.rule_1("few", b.reg(r#"(a )?few"#)?, |_| {
        Ok(IntegerValue {
               value: 3,
               grain: Some(1),
               precision: Approximate,
               ..IntegerValue::default()
           })
    });
    b.rule_1("integer (20..90)",
             b.reg(r#"(twenty|thirty|fou?rty|fifty|sixty|seventy|eighty|ninety)"#)?,
             |text_match| {
        let value = match text_match.group(1).as_ref() {
            "twenty" => 20,
            "thirty" => 30,
            "fourty" => 40,
            "forty" => 40,
            "fifty" => 50,
            "sixty" => 60,
            "seventy" => 70,
            "eighty" => 80,
            "ninety" => 90,
            _ => panic!("Unknow match"),
        };
        IntegerValue::new_with_grain(value, 1)
    });
    b.rule_2("integer 21..99",
             integer_check!(10, 90, |integer: &IntegerValue| integer.value % 10 == 0),
             integer_check!(1, 9),
             |a, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_1("integer (numeric)",
             b.reg(r#"(\d{1,18})"#)?,
             |text_match| IntegerValue::new(text_match.group(0).parse()?));
    b.rule_1("integer with thousands separator ,",
             b.reg(r#"(\d{1,3}(,\d\d\d){1,5})"#)?,
             |text_match| {
                 let reformatted_string = text_match.group(1).replace(",", "");
                 let value: i64 = reformatted_string.parse()?;
                 IntegerValue::new(value)
             });
    b.rule_2("special composition for missing hundreds like in one twenty two",
             integer_check!(1, 9),
             integer_check!(10, 99),
             |a, b| {
                 let value = a.value().value * 100 + b.value().value;
                 IntegerValue::new_with_grain(value, 1)
             });
    b.rule_2("number dozen",
             integer_check!(1, 10),
             integer_filter!(|integer: &IntegerValue| integer.group),
             |a, b| {
                 Ok(IntegerValue {
                        value: a.value().value * b.value().value,
                        grain: b.value().grain,
                        ..IntegerValue::default()
                    })
             });
    b.rule_2("number hundreds",
             integer_check!(1, 99),
             integer_check!(100, 100),
             |a, b| {
                 Ok(IntegerValue {
                        value: a.value().value * b.value().value,
                        grain: b.value().grain,
                        ..IntegerValue::default()
                    })
             });
    b.rule_2("number thousands",
             integer_check!(1, 999),
             integer_check!(1000, 1000),
             |a, b| {
                 Ok(IntegerValue {
                        value: a.value().value * b.value().value,
                        grain: b.value().grain,
                        ..IntegerValue::default()
                    })
             });
    b.rule_2("number millions",
             integer_check!(1, 99),
             integer_check!(1000000, 1000000),
             |a, b| {
                 Ok(IntegerValue {
                        value: a.value().value * b.value().value,
                        grain: b.value().grain,
                        ..IntegerValue::default()
                    })
             });
    b.rule_1("decimal number", b.reg(r#"(\d*\.\d+)"#)?, |text_match| {
        let value: f32 = text_match.group(0).parse()?;
        Ok(FloatValue {
               value: value,
               ..FloatValue::default()
           })
    });
    b.rule_3("number dot number",
             number_check!(|number: &NumberValue| !number.prefixed()),
             b.reg(r#"dot|point"#)?,
             number_check!(|number: &NumberValue| !number.suffixed()),
             |a, _, b| {
                 Ok(FloatValue {
                        value: b.value().value() * 0.1 + a.value().value(),
                        ..FloatValue::default()
                    })
             });
    b.rule_1("decimal with thousands separator",
             b.reg(r#"(\d+(,\d\d\d)+\.\d+)"#)?,
             |text_match| {
                 let reformatted_string = text_match.group(1).replace(",", "");
                 let value: f32 = reformatted_string.parse()?;
                 Ok(FloatValue {
                        value: value,
                        ..FloatValue::default()
                    })
             });
    b.rule_2("numbers prefix with -, negative or minus",
             b.reg(r#"-|minus\s?|negative\s?"#)?,
             number_check!(|number: &NumberValue| !number.prefixed()),
             |_, a| -> RuleResult<NumberValue> {
        Ok(match a.value().clone() { // checked
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
            _ => panic!("Unknown match"),
        };
        Ok(match a.value().clone() { // checked
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
    b.rule_1(
             "ordinals (first..31st)",
            b.reg(r#"(first|second|third|fourth|fifth|sixth|seventh|eighth|ninth|tenth|eleventh|twelfth|thirteenth|fourteenth|fifteenth|sixteenth|seventeenth|eighteenth|nineteenth|twentieth|twenty-first|twenty-second|twenty-third|twenty-fourth|twenty-fifth|twenty-sixth|twenty-seventh|twenty-eighth|twenty-ninth|thirtieth|thirty-first)"#)?,
             |text_match| {
                 let value = match text_match.group(1).as_ref() {
                     "first" => 1,
                     "second" => 2,
                     "third" => 3,
                     "fourth" => 4,
                     "fifth" => 5,
                     "sixth" => 6,
                     "seventh" => 7,
                     "eighth" => 8,
                     "ninth" => 9,
                     "tenth" => 10,
                     "eleventh" => 11,
                     "twelfth" => 12,
                     "thirteenth" => 13,
                     "fourteenth" => 14,
                     "fifteenth" => 15,
                     "sixteenth" => 16,
                     "seventeenth" => 17,
                     "eighteenth" => 18,
                     "nineteenth" => 19,
                     "twentieth" => 20,
                     "twenty-first" => 21,
                     "twenty-second" => 22,
                     "twenty-third" => 23,
                     "twenty-fourth" => 24,
                     "twenty-fifth" => 25,
                     "twenty-sixth" => 26,
                     "twenty-seventh" => 27,
                     "twenty-eighth" => 28,
                     "twenty-ninth" => 29,
                     "thirtieth" => 30,
                     "thirty-first" => 31,
                     _ => panic!("Unknow match"),
                 };
                 Ok(OrdinalValue { value: value })
             });
    b.rule_2("<number> <ordinal>",
             integer_check!(10, 90, |integer: &IntegerValue| integer.value % 10 == 0),
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 9),
             |integer, ordinal| {
                 Ok(OrdinalValue { value: integer.value().value + ordinal.value().value })
             });
    b.rule_1("ordinal (digits)",
             b.reg(r#"0*(\d+) ?(st|nd|rd|th)"#)?,
             |text_match| {
                 let value: i64 = text_match.group(1).parse()?;
                 Ok(OrdinalValue { value: value })
             });
    b.rule_2("the <ordinal>",
             b.reg(r#"the"#)?,
             ordinal_check!(),
             |_, ordinal| Ok(*ordinal.value()));
    Ok(())
}
