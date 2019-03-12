use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::dimension::Precision::*;
use rustling_ontology_values::helpers;
use rustling_ontology_moment::{Weekday, Grain, PeriodComp, Period};

pub fn rules_percentage(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("<number> per cent",
        number_check!(),
        b.reg(r"(?:%|p\.c\.|p. cents?|pour[ -]?cents?)")?,
        |number, _| Ok(PercentageValue(number.value().value()))
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
             b.reg(r#"et"#)?,
             amount_of_money_check!(|money: &AmountOfMoneyValue| money.unit == Some("cent")),
             |a, _, b| helpers::compose_money(&a.value(), &b.value()));
    b.rule_2("intersect",
             amount_of_money_check!(|money: &AmountOfMoneyValue| money.unit != Some("cent")),
             number_check!(),
             |a, b| helpers::compose_money_number(&a.value(), &b.value()));
    b.rule_1_terminal("$",
        b.reg(r#"\$|dollars?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("$") })
    );
    b.rule_1_terminal("EUR",
        b.reg(r#"€|(?:[e€]uro?s?)"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("EUR") })
    );
    b.rule_1_terminal("£",
        b.reg(r#"£|livres?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("£") })
    );
    b.rule_1_terminal("USD",
        b.reg(r#"us[d\$]|dollars? am[eé]ricains?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("USD") })
    );
    b.rule_1_terminal("AUD",
        b.reg(r#"au[d\$]|dollars? australiens?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("AUD") })
    );
    b.rule_1_terminal("CAD",
        b.reg(r#"cad|dollars? canadiens?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("CAD") })
    );
    b.rule_1_terminal("HKD",
        b.reg(r#"hkd|dollars? de hong[- ]kong"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("HKD") })
    );
    b.rule_1_terminal("KR",
        b.reg(r#"kr|couronnes?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("KR") })
    );
    b.rule_1_terminal("DKK",
        b.reg(r#"dkk|couronnes? danoises?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("DKK") })
    );
    b.rule_1_terminal("NOK",
        b.reg(r#"nok|couronnes? norv[ée]giennes?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("NOK") })
    );
    b.rule_1_terminal("SEK",
        b.reg(r#"sek|couronnes? su[ée]doises?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("SEK") })
    );
    b.rule_1_terminal("CHF",
        b.reg(r#"chf|francs? suisses?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("CHF") })
    );
    b.rule_1_terminal("RUB",
        b.reg(r#"rub|roubles?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("RUB") })
    );
    b.rule_1_terminal("INR",
        b.reg(r#"inr|roupies?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("INR") })
    );
    b.rule_1_terminal("JPY",
        b.reg(r#"jpy|yens?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("JPY") })
    );
    b.rule_1_terminal("RMB|CNH|CNY",
        b.reg(r#"cny|cnh|rmb|yuans?|renmimbis?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("CNY") })
    );
    b.rule_1_terminal("¥",
        b.reg(r#"¥"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("¥") })
    );
    b.rule_1_terminal("KRW",
        b.reg(r#"₩|krw|wons? (?:sud[- ])?cor[ée]ns?|wons?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("KRW") })
    );
    b.rule_1_terminal("Bitcoin",
        b.reg(r#"฿|bitcoins?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("฿") })
    );
    b.rule_1_terminal("GBP",
        b.reg(r#"gbp|livres? sterlings?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("GBP") })
    );
    b.rule_1_terminal("cent",
                      b.reg(r#"centimes?|cents?|penn(?:y|ies)|fens?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("cent") })
    );
    b.rule_1_terminal("unnamed currency",
                      b.reg(r#"(?:balle)s?"#)?,
                      |_| Ok(MoneyUnitValue { unit: None })
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
    b.rule_3("<amount> de <unit>",  // "un million de dollars"
        integer_check!(|integer: &IntegerValue| !integer.group),
        b.reg(r#"d[e']"#)?,
        money_unit!(),
        |a, _, b| {
            Ok(AmountOfMoneyValue {
                value: a.value().value as f32,
                precision: Exact,
                unit: b.value().unit,
                ..AmountOfMoneyValue::default()
            })
    });
    b.rule_3("<amount> de <unit>",  // "une douzaine de dollars"
        integer_check!(|integer: &IntegerValue| integer.group),
        b.reg(r#"d[e']"#)?,
        money_unit!(),
        |a, _, b| {
            Ok(AmountOfMoneyValue {
                value: a.value().value as f32,
                precision: Approximate,
                unit: b.value().unit,
                ..AmountOfMoneyValue::default()
            })
    });
    b.rule_2("about <amount-of-money>",
             b.reg(r#"(?:autour|pas loin|pr[eè]s|aux alentours) d[e']|environ|presque|(?:approximative|quasi)ment"#)?,
             amount_of_money_check!(),
             |_, a| {
                 Ok(AmountOfMoneyValue {
                     precision: Approximate,
                     ..a.value().clone()
                 })
             });
    b.rule_2("exactly <amount-of-money>",
             b.reg(r#"(?:tr[eè]s )?exactement|pr[eé]cis[eé]ment|pile(?: poil)?"#)?,
             amount_of_money_check!(),
             |_, a| {
                 Ok(AmountOfMoneyValue {
                     precision: Exact,
                     ..a.value().clone()
                 })
             });
    b.rule_2("exactly <amount-of-money>",
        amount_of_money_check!(),
        b.reg(r#"pile(?: poil)?|tout rond"#)?,
        |a, _| {
            Ok(AmountOfMoneyValue {
                     precision: Exact,
                     ..a.value().clone()
            })
        }
    );
    Ok(())
}

pub fn rules_duration(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("seconde (unit-of-duration)",
        b.reg(r#"sec(?:onde)?s?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Second))
    );
    b.rule_1_terminal("minute (unit-of-duration)",
        b.reg(r#"min(?:ute)?s?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Minute))
    );
    b.rule_1_terminal("heure (unit-of-duration)",
        b.reg(r#"h(?:eure)?s?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Hour))
    );
    b.rule_1_terminal("jour (unit-of-duration)",
        b.reg(r#"jour(?:n[ée]e?)?s?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Day))
    );
    b.rule_1_terminal("semaine (unit-of-duration)",
        b.reg(r#"semaines?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Week))
    );
    b.rule_1_terminal("mois (unit-of-duration)",
        b.reg(r#"mois?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Month))
    );
    b.rule_1_terminal("année (unit-of-duration)",
        b.reg(r#"an(?:n[ée]e?)?s?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Year))
    );
    b.rule_1_terminal("un quart heure",
        b.reg(r#"(1/4\s?h(?:eure)?|(?:un|1) quart d'heure)"#)?,
        |_| Ok(DurationValue::new(PeriodComp::minutes(15).into()))
    );
    b.rule_1_terminal("une demi heure",
        b.reg(r#"(?:1/2\s?h(?:eure)?|(?:1|une) demi(?:e)?(?:\s|-)heure)"#)?,
        |_| Ok(DurationValue::new(PeriodComp::minutes(30).into()))
    );
    b.rule_1_terminal("trois quarts d'heure",
        b.reg(r#"(?:3/4\s?h(?:eure)?|(?:3|trois) quart(?:s)? d'heure)"#)?,
        |_| Ok(DurationValue::new(PeriodComp::minutes(45).into()))
    );
    b.rule_2("<integer> <unit-of-duration>",
             integer_check_by_range!(0),
             unit_of_duration_check!(),
             |integer, unit| Ok(DurationValue::new(PeriodComp::new(unit.value().grain, integer.value().value).into()))
    );
    b.rule_3("<integer> de <unit-of-duration>",
        integer_check!(|integer: &IntegerValue| integer.value >= 0 && integer.group),
        b.reg(r#"d[e']"#)?,
        unit_of_duration_check!(),
        |integer, _, unit| Ok(DurationValue::new(PeriodComp::new(unit.value().grain, integer.value().value).into()))
    );
    b.rule_3("<number> h <number>",
             integer_check_by_range!(0),
             b.reg(r#"h(?:eures?)?"#)?,
             integer_check_by_range!(0,59),
             |hour, _, minute| {
                 let hour_period = Period::from(PeriodComp::new(Grain::Hour, hour.value().clone().value));
                 let minute_period = Period::from(PeriodComp::new(Grain::Minute, minute.value().clone().value));
                 Ok(DurationValue::new(hour_period + minute_period))
             }
    );
    b.rule_3("<integer> <unit-of-duration> et quart",
        integer_check_by_range!(0),
        unit_of_duration_check!(),
        b.reg(r#"et quart"#)?,
        |integer, uod, _| {
           let quarter_period: Period = uod.value().grain.quarter_period().map(|a| a.into()).ok_or_else(|| RuleError::Invalid)?;
           Ok(DurationValue::new(quarter_period + PeriodComp::new(uod.value().grain, integer.value().value)))
        }
    );
    b.rule_3("<integer> <unit-of-duration> et demie",
        integer_check_by_range!(0),
        unit_of_duration_check!(),
        b.reg(r#"et demie?"#)?,
        |integer, uod, _| {
           let half_period: Period = uod.value().grain.half_period().map(|a| a.into()).ok_or_else(|| RuleError::Invalid)?;
           Ok(DurationValue::new(half_period + PeriodComp::new(uod.value().grain, integer.value().value)))
        }
    );
    b.rule_3("<duration> et <duration>",
             duration_check!(|duration: &DurationValue| !duration.suffixed),
             b.reg(r#"et"#)?,
             duration_check!(|duration: &DurationValue| !duration.prefixed),
             |a, _, b| Ok(a.value() + b.value())
    );
    b.rule_2("<duration> <duration>",
             duration_check!(|duration: &DurationValue| !duration.suffixed),
             duration_check!(|duration: &DurationValue| !duration.prefixed),
             |a, b| Ok(a.value() + b.value())
    );
    b.rule_2("<duration> <integer>",
             duration_check!(|duration: &DurationValue| !duration.prefixed),
             integer_check_by_range!(0),
             |duration, integer| helpers::compose_duration_with_integer(duration.value(), integer.value())
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
    b.rule_2("environ <duration>",
             b.reg(r#"environ"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().precision(Precision::Approximate))
    );
    b.rule_2("pendant <duration>",
             b.reg(r#"pendant|durant|pour(?: une dur[eé]e? d['e])?"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().prefixed())
    );
    b.rule_2("il y a <duration>",
             b.reg(r#"il y a"#)?,
             duration_check!(),
             |_, duration| duration.value().ago()
    );
    b.rule_2("depuis <duration>",
        b.reg(r#"depuis|[cç]a fait"#)?,
        duration_check!(),
        |_, duration| {
            duration.value().ago()?
                .span_to(&helpers::cycle_nth(Grain::Second, 0)?, false)
    });
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
    b.rule_1("année (cycle)",
             b.reg(r#"an(?:n[ée]e?)?s?"#)?,
             |_| CycleValue::new(Grain::Year)
    );
    b.rule_2("ce|dans le <cycle>",
             b.reg(r#"(?:cet?t?e?s?)|(?:dans l[ae']? ?)"#)?,
             cycle_check!(),
             |_, cycle| helpers::cycle_nth(cycle.value().grain, 0)
    );
    b.rule_3("ce <cycle> (la ou ci)",
             b.reg(r#"cet?t?e?s?"#)?,
             cycle_check!(),
             b.reg(r#"-?ci"#)?,
             |_, cycle, _| helpers::cycle_nth(cycle.value().grain, 0)
    );
    
    b.rule_3("le <cycle> dernier",
             b.reg(r#"l[ae']? ?"#)?,
             cycle_check!(),
             b.reg(r#"derni[èe]re?|pass[ée]e?"#)?,
             |_, cycle, _| helpers::cycle_nth(cycle.value().grain, -1)
    );
    b.rule_3("le <cycle> prochain|suivant|d'après",
             b.reg(r#"l[ae']? ?|une? ?"#)?,
             cycle_check!(),
             b.reg(r#"prochaine?|suivante?|qui suit|(?:d'? ?)?apr[eèé]s"#)?,
             |_, cycle, _| helpers::cycle_nth(cycle.value().grain, 1)
    );
    b.rule_2("<cycle> dernier",
             cycle_check!(),
             b.reg(r#"derni[èe]re?|pass[ée]e?|pr[eé]c[eé]dente?|(?:d')? ?avant"#)?,
             |cycle, _| helpers::cycle_nth(cycle.value().grain, -1)
    );
    b.rule_2("<cycle> prochain|suivant|d'après",
             cycle_check!(),
             b.reg(r#"prochaine?|suivante?|qui suit|(?:d')? ?apr[eèé]s"#)?,
             |cycle, _| helpers::cycle_nth(cycle.value().grain, 1)
    );
    b.rule_3("n <cycle> avant",
             integer_check_by_range!(2, 9999),
             cycle_check!(),
             b.reg(r#"(?:d')? ?avant|plus t[oô]t"#)?,
             |integer, cycle, _| helpers::cycle_nth(cycle.value().grain, -1 * integer.value().value)
    );
    b.rule_3("n <cycle> après",
             integer_check_by_range!(2, 9999),
             cycle_check!(),
             b.reg(r#"(?:d')? ?apr[eèé]s|qui sui(?:t|ves?)|plus tard"#)?,
             |integer, cycle, _| helpers::cycle_nth(cycle.value().grain, integer.value().value)
    );
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
    b.rule_4("les n derniers <cycle>",
            b.reg(r#"[cld]es"#)?,
            integer_check_by_range!(2, 9999),
            b.reg(r#"derni.re?s?"#)?,
            cycle_check!(),
            |_, integer, _, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    b.rule_3("n derniers <cycle>",
             integer_check_by_range!(2, 9999),
             b.reg(r#"derni.re?s?"#)?,
             cycle_check!(),
             |integer, _, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );

    b.rule_4("les n prochains <cycle>",
            b.reg(r#"[cld]es"#)?,
            integer_check_by_range!(2, 9999),
            b.reg(r#"prochaine?s?|suivante?s?|apr[eèé]s"#)?,
            cycle_check!(),
            |_, integer, _, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    b.rule_3("n prochains <cycle>",
             integer_check_by_range!(2, 9999),
             b.reg(r#"prochaine?s?|suivante?s?|apr[eèé]s"#)?,
             cycle_check!(),
             |integer, _, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    b.rule_4("les n <cycle> passes|precedents",
            b.reg(r#"[cld]es"#)?,
            integer_check_by_range!(2, 9999),
            cycle_check!(),
            b.reg(r#"pass[eèé][eèé]?s?|pr[eé]c[eé]dente?s?|(?:d')? ?avant|plus t[oô]t"#)?,
            |_, integer, cycle, _| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    b.rule_3("n <cycle> passes|precedents",
             integer_check_by_range!(2, 9999),
             cycle_check!(),
             b.reg(r#"pass[eèé][eèé]?s?|pr[eé]c[eé]dente?s?|(?:d')? ?avant|plus t[oô]t"#)?,
             |integer, cycle, _| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    b.rule_4("les n <cycle> suivants",
            b.reg(r#"[cld]es"#)?,
            integer_check_by_range!(2, 9999),
            cycle_check!(),
            b.reg(r#"prochaine?s?|suivante?s?|apr[eèé]s|qui sui(?:t|ves?)|plus tard"#)?,
            |_, integer, cycle, _| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    b.rule_3("n <cycle> suivants",
             integer_check_by_range!(2, 9999),
             cycle_check!(),
             b.reg(r#"prochaine?s?|suivante?s?|apr[eèé]s|qui sui(?:t|ves?)|plus tard"#)?,
             |integer, cycle, _| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    b.rule_4("<ordinal> <cycle> de <datetime>",
             ordinal_check!(),
             cycle_check!(),
             b.reg(r#"d['eu]|en"#)?,
             datetime_check!(),
             |ordinal, cycle, _, datetime| helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value - 1, datetime.value())
    );
    b.rule_5("le <ordinal> <cycle> de <datetime>",
             b.reg(r#"l[ea]"#)?,
             ordinal_check!(),
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
    b.rule_2("le veille du <datetime>",
             b.reg(r#"(la )?veille du"#)?,
             datetime_check!(),
             |_, datetime| helpers::cycle_nth_after_not_immediate(Grain::Day, -1, datetime.value())
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
        b.reg(r#"septembre|sept?\.?"#)?,
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
    b.rule_1_terminal("noel",
        b.reg(r#"(?:jour de )?no[eë]l"#)?,
        |_| Ok(helpers::month_day(12, 25)?.form(Form::Celebration))
    );
    b.rule_1_terminal("soir de noël",
        b.reg(r#"(soir(?:ée)?|veille) de no[eë]l"#)?,
        |_| {
            let start = helpers::month_day(12, 24)?.intersect(&helpers::hour(18, false)?)?;
            let end = helpers::month_day(12, 25)?.intersect(&helpers::hour(0, false)?)?;
            Ok(start.span_to(&end, false)?
                 .form(Form::Celebration))
        }
    );
    b.rule_1_terminal("jour de l'an",
        b.reg(r#"(?:le )?(?:jour de l'|nouvel )an"#)?,
        |_| Ok(helpers::month_day(1, 1)?.form(Form::Celebration))
    );
    b.rule_1_terminal("toussaint",
        b.reg(r#"(?:(?:la |la journée de la |jour de la )?toussaint|jour des morts)"#)?,
        |_| Ok(helpers::month_day(11, 1)?.form(Form::Celebration))
    );
    b.rule_1_terminal("Armistice",
        b.reg(r#"(?:pour )?l'armistice"#)?,
        |_| Ok(helpers::month_day(11, 11)?.form(Form::Celebration))
    );
    b.rule_1_terminal("Saint Etienne (Alsace)",
        b.reg(r#"(?:(?:le jour|la f[eê]te) de )?la (?:saint|st) [eé]tienne"#)?,
        |_| Ok(helpers::month_day(12, 26)?.form(Form::Celebration))
    );
    b.rule_1_terminal("jeudi saint",
        b.reg(r#"(?:le )?jeudi saint"#)?,
        |_| Ok(helpers::cycle_nth_after(Grain::Day, -3, &helpers::easter()?)?
                .form(Form::Celebration))
    );
    b.rule_1_terminal("vendredi saint",
        b.reg(r#"(?:le )?vendredi saint"#)?,
        |_| Ok(helpers::cycle_nth_after(Grain::Day, -2, &helpers::easter()?)?
                .form(Form::Celebration))
    );
    b.rule_1_terminal("samedi saint",
        b.reg(r#"(?:le )?samedi saint"#)?,
        |_| Ok(helpers::cycle_nth_after(Grain::Day, -1, &helpers::easter()?)?
                .form(Form::Celebration))
    );
    b.rule_1_terminal("pâques",
        b.reg(r#"(?:la f[eê]te de |le jour de |le dimanche de )?p[âa]ques"#)?,
        |_| Ok(helpers::easter()?.form(Form::Celebration))
    );
    b.rule_1_terminal("le lundi de pâques",
        b.reg(r#"le lundi de p[âa]ques"#)?,
        |_| Ok(helpers::cycle_nth_after(Grain::Day, 1, &helpers::easter()?)?
                .form(Form::Celebration))
    );
    b.rule_1_terminal("ascension",
        b.reg(r#"(?:la f[eê]te de l'|le jeudi de l'|l'|le jour de l')ascension"#)?,
        |_| Ok(helpers::cycle_nth_after(Grain::Day, 39, &helpers::easter()?)?
                .form(Form::Celebration))

    );
    b.rule_1_terminal("pencôte",
        b.reg(r#"(?:la f[eê]te de la |la |le lundi de la )?penc[oô]te"#)?,
        |_| Ok(helpers::cycle_nth_after(Grain::Day, 49, &helpers::easter()?)?
                .form(Form::Celebration))
    );
    b.rule_1_terminal("1er mai",
        b.reg(r#"(?:la )?f(e|ê)te du travail"#)?,
        |_| Ok(helpers::month_day(5, 1)?.form(Form::Celebration))
    );
    b.rule_1_terminal("fêtes des pères",
        b.reg(r#"(?:la )?f[eê]te des p[eè]res"#)?,
        |_| {
            let sundays_of_june = helpers::month(6)?.intersect(&helpers::day_of_week(Weekday::Sun)?)?;
            let second_week_of_june = helpers::cycle_nth_after(Grain::Week, 2, &helpers::month_day(6, 1)?)?;
            Ok(sundays_of_june.intersect(&second_week_of_june)? // third sunday of June
                   .form(Form::Celebration))
        }
    );
    b.rule_1_terminal("fêtes des mères",
        b.reg(r#"(?:la )?f[eê]te des m[eè]res"#)?,
        |_| { 
            // It is the last last sunday of may
            // If it is the same day as the Pentecost, it is the first sunday of june
            // This case is not supported for now
            Ok(helpers::day_of_week(Weekday::Sun)?.last_of(&helpers::month(5)?)?
                .form(Form::Celebration))
        }
    );
    b.rule_1_terminal("fête nationale",
        b.reg(r#"(?:la )?f[eê]te (?:nationale|du (?:14|quatorze) juillet)"#)?,
        |_| Ok(helpers::month_day(7, 14)?
                .form(Form::Celebration))
    );
    b.rule_1_terminal("assomption",
        b.reg(r#"(?:la f[eê]te de |le jour de )?l'assomption"#)?,
        |_| Ok(helpers::month_day(8, 15)?
                .form(Form::Celebration))
    );
    b.rule_1_terminal("maintenant",
        b.reg(r#"maintenant|(?:tout de suite)"#)?,
        |_| helpers::cycle_nth(Grain::Second, 0)
    );
    b.rule_1_terminal("aujourd'hui",
        b.reg(r#"(?:aujourd'? ?hui)|(?:ce jour)|(?:dans la journ[ée]e?)|(?:en ce moment)"#)?,
        |_| helpers::cycle_nth(Grain::Day, 0)
    );
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
             |_, datetime| datetime.value().the_nth(0)
    );
    b.rule_2("<day-of-week> prochain",
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"prochain"#)?,
             |datetime, _| datetime.value().the_nth_not_immediate(0)
    );
    b.rule_2("<named-month> prochain",
             datetime_check!(),
             b.reg(r#"prochain"#)?,
             |datetime, _| datetime.value().the_nth(1)
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
                 let week_day_start = helpers::day_of_week(Weekday::Fri)?.intersect(&helpers::hour(18, false)?)?;
                 let week_day_end = helpers::day_of_week(Weekday::Mon)?.intersect(&helpers::hour(0, false)?)?;
                 let week_day = week_day_start.span_to(&week_day_end, false)?;
                 let week_ends_of_time = datetime.value().intersect(&week_day)?;
                 week_ends_of_time.the_nth(ordinal.value().value - 1)
             }
    );
    b.rule_2("dernier week-end de <datetime>",
             b.reg(r#"(?:le )?dernier week(?:\s|-)?end (?:du mois d[e']|d['eu]|en)"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, datetime| {
                 let week_day_start = helpers::day_of_week(Weekday::Fri)?.intersect(&helpers::hour(18, false)?)?;
                 let week_day_end = helpers::day_of_week(Weekday::Mon)?.intersect(&helpers::hour(0, false)?)?;
                 let week_day = week_day_start.span_to(&week_day_end, false)?;
                 week_day.last_of(datetime.value())
             }
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
    b.rule_2("en <year>",
             b.reg(r#"(?:en(?: l'an)?|de l'ann[eé])"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && form!(Form::Year(_))(datetime)),
             |_, year| Ok(year.value().clone())
    );
    b.rule_1("year (latent)",
             integer_check_by_range!(2101, 3000),
             |integer| {
                 Ok(helpers::year(integer.value().value as i32)?.latent())
             }
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
    b.rule_4("le <day-of-month> à <datetime>",
             b.reg(r#"le"#)?,
             integer_check_by_range!(1, 31),
             b.reg(r#"[aà]"#)?,
             datetime_check!(),
             |_, integer, _, datetime| {
                 let day_of_month = helpers::day_of_month(integer.value().value as u32)?;
                 day_of_month.intersect(&datetime.value())
             }
    );
    b.rule_2("<day-of-month> <named-month>",
             integer_check_by_range!(1, 31),
             datetime_check!(form!(Form::Month(_))),
             |integer, month| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_2("<day-of-week> <day-of-month>",
             datetime_check!(form!(Form::DayOfWeek{..})), // Weird it is not used in the production of the rule
             integer_check_by_range!(1, 31),
             |_, integer| helpers::day_of_month(integer.value().value as u32)
    );
    b.rule_3("<day-of-week> <day-of-month> à <time-of-day>)",
             datetime_check!(form!(Form::DayOfWeek{..})), // Weird it is not used in the production of the rule
             integer_check_by_range!(1, 31),
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, integer, tod| helpers::day_of_month(integer.value().value as u32)
                 ?.intersect(tod.value())
    );
    b.rule_1("time-of-day (latent)",
             integer_check_by_range!(1, 23),
             |integer| Ok(helpers::hour(integer.value().value as u32, integer.value().value < 12)?.latent())
    );
    b.rule_1("time-of-day (latent)",
             integer_check_by_range!(0, 0),
             |_| Ok(helpers::hour(0, false)?.latent())
    );
    b.rule_1_terminal("midi",
        b.reg(r#"midi"#)?,
        |_| helpers::hour(12, false)
    );
    b.rule_1_terminal("minuit",
        b.reg(r#"minuit"#)?,
        |_| helpers::hour(0, false)
    );
    b.rule_2("<time-of-day> heures",
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))),
             b.reg(r#"h\.?(?:eure)?s?"#)?,
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
    // Written dates in numeric formats
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
    // End of Written dates in numeric formats
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
    b.rule_1_terminal("petit dejeuner",
        b.reg(r#"petit[- ]d[ée]jeuner"#)?,
        |_| Ok(helpers::hour(5, false)?
                .span_to(&helpers::hour(10, false)?, false)?
                .latent()
                .form(Form::Meal))
    );
    b.rule_1_terminal("milieu de matinée",
        b.reg(r#"milieu de matin[ée]e"#)?,
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
        b.reg(r#"fin de matin[ée]e"#)?,
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
        b.reg(r#"(?:(?:[àa] )?l[' ]heure du|au moment du|pendant le|pour le) go[uû]ter"#)?,
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
        b.reg(r#"milieu de (?:la )?journ[ée]e"#)?,
        |_| {
            Ok(helpers::hour(11, false)?
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
    b.rule_2("a l'heure <meal>",
             b.reg(r#"(?:[àa] )?l[' ]heure du|au moment du|pendant l[ea']|au|pour l[ea']|l[ea']"#)?,
             datetime_check!(|datetime: &DatetimeValue| datetime.latent && form!(Form::Meal)(datetime)),
             |_, a| Ok(a.value().clone().not_latent())
    );
    b.rule_2("<dim time> <meal>",
             datetime_check!(),
             datetime_check!(form!(Form::Meal)),
             |a, b| a.value().intersect(b.value())
    );
    b.rule_2("du|dans le <part-of-day>",
             b.reg(r#"pendant(?: l[ae']?)?|durant(?: l[ae']?)?|du|(?:[aà]|dans) l[ae']?|au|en|l[ae']|d[èe]s(?: l[ae']?)?"#)?,
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             |_, a| Ok(a.value().clone().not_latent())
    );
    b.rule_2("ce <part-of-day>",
             b.reg(r#"cet?t?e?"#)?,
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             |_, a| Ok(helpers::cycle_nth(Grain::Day, 0)?.intersect(a.value())?.form(a.value().form.clone()))
    );
    b.rule_2("<dim time> <part-of-day>",
             datetime_check!(excluding_form!(Form::TimeOfDay(_))),
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             |a, b| a.value().intersect(b.value())
    );
    b.rule_2("<dim time> du matin",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:(?:du|dans|de) )?(?:(?:au|le|la) )?mat(?:in[ée]?e?)?"#)?,
             |a, _| {
                 let period = helpers::hour(0, false)?
                     .span_to(&helpers::hour(12, false)?, false)?;
                 a.value().intersect(&period)
             }
    );
    b.rule_2("<dim time> du soir",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:(?:du|dans|de) )?(?:(?:au|le|la) )?soir[ée]?e?"#)?,
             |a, _| {
                 let period = helpers::hour(16, false)?
                     .span_to(&helpers::hour(0, false)?, false)?;
                 a.value().intersect(&period)
             }
    );
    b.rule_3("<part-of-day> du <dim time>",
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             b.reg(r#"du"#)?,
             datetime_check!(),
             |a, _, b| b.value().intersect(a.value())
    );
    b.rule_1_terminal("week-end",
        b.reg(r#"week(?:\s|-)?end"#)?,
        |_| {
            let friday = helpers::day_of_week(Weekday::Fri)?
                                .intersect(&helpers::hour(18, false)?)?;
            let monday = helpers::day_of_week(Weekday::Mon)?
                                .intersect(&helpers::hour(0, false)?)?;
            friday.span_to(&monday, false)
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
    b.rule_2("le <datetime>",
             b.reg(r#"l[ea]"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |_, a| Ok(a.value().clone())
    );
    b.rule_4("dd-dd <month>(interval)",
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             b.reg(r#"\-|au|jusqu'au"#)?,
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
             b.reg(r#"\-|au|jusqu'au"#)?,
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
             b.reg(r#"\-|au|jusqu'au"#)?,
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
             b.reg(r#"\-|au|jusqu'au"#)?,
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
             b.reg(r#"\-|au|jusqu'au"#)?,
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
             b.reg(r#"\-|au|jusqu'au"#)?,
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
             b.reg(r#"\-|au|jusqu'au"#)?,
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
        b.reg(r#"au|jusqu'au"#)?,
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
    b.rule_3("<datetime> - <datetime> (interval)",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             b.reg(r#"\-|au|[aà]|jusqu'(?:au|[aà])"#)?,
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
             b.reg(r#"\-|au|[aà]|jusqu'(?:au|[aà])"#)?,
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
    // Specific case with years
    b.rule_5("de <datetime> - <datetime> <year> (interval)",
             b.reg(r#"depuis|d[e'u]?"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             b.reg(r#"\-|au|[aà]|jusqu'(?:au|[aà])"#)?,
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
             b.reg(r#"\-|[aà]|au|jusqu'(?:au|[aà])"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && form!(Form::TimeOfDay(_))(datetime)),
             |a, _, b| a.value().smart_span_to(b.value(), false)
    );
    b.rule_4("de <time-of-day> - <time-of-day> (interval)",
             b.reg(r#"(?:midi )?de"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"\-|[aà]|au|jusqu'(?:au|[aà])"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, a, _, b| a.value().smart_span_to(b.value(), false)
    );
    b.rule_4("entre <time-of-day> et <time-of-day> (interval)",
             b.reg(r#"entre"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"et"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, a, _, b| a.value().smart_span_to(b.value(), false)
    );
    b.rule_2("d'ici <duration>",
             b.reg(r#"d'ici|dans l(?:'|es?)"#)?,
             duration_check!(),
             |_, duration| {
                 let start = helpers::cycle_nth(Grain::Second, 0)?;
                 let end = duration.value().in_present()?;
                 start.span_to(&end, false)
             }
    );
    b.rule_2("avant <time-of-day>",
             b.reg(r#"(?:n[ ']importe quand )?jusqu'(?:a|à)"#)?,
             datetime_check!(),
             |_, datetime| Ok(datetime.value().clone().mark_before_end())
    );
    b.rule_2("avant <time-of-day>",
             b.reg(r#"(?:n[ ']importe quand )?avant"#)?,
             datetime_check!(),
             |_, datetime| Ok(datetime.value().clone().mark_before_start())
    );
    b.rule_2("après <time-of-day>",
             b.reg(r#"apr(?:e|è)s"#)?,
             datetime_check!(),
             |_, datetime| Ok(datetime.value().clone().mark_after_end())
    );
    b.rule_2("après <time-of-day>",
             b.reg(r#"(?:a|à) partir de"#)?,
             datetime_check!(),
             |_, datetime| Ok(datetime.value().clone().mark_after_start())
    );
    b.rule_2("après le <day-of-month>",
             b.reg(r#"apr(?:e|è)s le"#)?,
             integer_check_by_range!(1, 31),
             |_, integer| Ok(helpers::day_of_month(integer.value().value as u32)?.mark_after_end())
    );
    b.rule_2("après le <day-of-month>",
             b.reg(r#"(?:a|à) partir du"#)?,
             integer_check_by_range!(1, 31),
             |_, integer| Ok(helpers::day_of_month(integer.value().value as u32)?.mark_after_start())
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
             b.reg(r#"(?:deg(?:r[éeè])?s?\.?)|°"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("degree"),
                     latent: false,
                 })
             });
    b.rule_2("<temp> Celcius",
             temperature_check!(),
             b.reg(r#"centigrades?|c(?:el[cs]?(?:ius)?)?\.?"#)?,
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
    b.rule_2("<temp> Kelvin",
             temperature_check!(),
             b.reg(r#"k(?:elvin)?\.?"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("kelvin"),
                     latent: false,
                 })
             });
    b.rule_2("<latent temp> en dessous de zero",
             temperature_check!(),
             b.reg(r#"en dessous de (?:0|z[ée]ro)"#)?,
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
    b.rule_2("intersect",
             number_check!(|number: &NumberValue| number.grain().unwrap_or(0) > 1),
             number_check!(),
             |a, b| helpers::compose_numbers(&a.value(), &b.value()));
    b.rule_1_terminal(
            "number (0..16)",
            b.reg(r#"(z[eé]ro|une?|deux|trois|quatre|cinq|six|sept|huit|neuf|dix|onze|douze|treize|quatorze|quinze|seize)"#)?,
            |text_match| {
                let value = match text_match.group(1).as_ref()  {
                        "zéro" => 0, 
                        "zero" => 0, 
                        "un" => 1, 
                        "une" => 1, 
                        "deux" => 2, 
                        "trois" => 3,
                        "quatre" => 4,
                        "cinq" => 5,
                        "six" => 6, 
                        "sept" => 7, 
                        "huit" => 8,
                        "neuf" => 9,
                        "dix" => 10,
                        "onze" => 11,
                        "douze" => 12,
                        "treize" => 13,
                        "quatorze" => 14,
                        "quinze" => 15,
                        "seize" => 16,
                        _ => return Err(RuleError::Invalid.into()),
                    };
                    IntegerValue::new(value) 
            });
    b.rule_1_terminal("number (20..60)",
             b.reg(r#"(vingt|trente|quarante|cinquante|soixante)"#)?,
             |text_match| {
                 let value = match text_match.group(1).as_ref() {
                     "vingt" => 20,
                     "trente" => 30,
                     "quarante" => 40,
                     "cinquante" => 50,
                     "soixante" => 60,
                     _ => return Err(RuleError::Invalid.into()),
                 };
                 IntegerValue::new(value)
             });
    b.rule_2("number (17..19)",
             integer_check_by_range!(10, 10),
             integer_check_by_range!(7, 9),
             |_, b| IntegerValue::new(b.value().value + 10));
    b.rule_3("number (17..19)",
             integer_check_by_range!(10, 10),
             b.reg(r"-")?,
             integer_check_by_range!(7, 9),
             |_, _, b| IntegerValue::new(b.value().value + 10));
    b.rule_2_terminal("number 80",
             b.reg(r#"quatre"#)?,
             b.reg(r#"vingts?"#)?,
             |_, _| IntegerValue::new(80));
    b.rule_3_terminal("number 80",
             b.reg(r#"quatre"#)?,
             b.reg(r"-")?,
             b.reg(r#"vingts?"#)?,
             |_, _, _| IntegerValue::new(80));
    b.rule_3("numbers 21 31 41 51",
             integer_check_by_range!(20, 50, |integer: &IntegerValue| integer.value % 10 == 0),
             b.reg(r#"-?et-?"#)?,
             integer_check_by_range!(1, 1),
             |a, _, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_2("numbers 22..29 32..39 .. 52..59",
             integer_check_by_range!(20, 50, |integer: &IntegerValue| integer.value % 10 == 0),
             integer_check_by_range!(2, 9),
             |a, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_3("numbers 22..29 32..39 .. 52..59",
             integer_check_by_range!(20, 50, |integer: &IntegerValue| integer.value % 10 == 0),
             b.reg(r"-")?,
             integer_check_by_range!(2, 9),
             |a, _, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_3("numbers 61 71",
             integer_check_by_range!(60, 60),
             b.reg(r#"-?et-?"#)?,
             integer_check_by_range!(1,
                            11,
                            |integer: &IntegerValue| integer.value == 1 || integer.value == 11),
             |a, _, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_2("numbers 81 91",
             integer_check_by_range!(80, 80),
             integer_check_by_range!(1,
                            11,
                            |integer: &IntegerValue| integer.value == 1 || integer.value == 11),
             |a, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_3("numbers 81 91",
             integer_check_by_range!(80, 80),
             b.reg(r#"-"#)?,
             integer_check_by_range!(1,
                            11,
                            |integer: &IntegerValue| integer.value == 1 || integer.value == 11),
             |a, _, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_2("numbers 62..69 .. 92..99",
             integer_check_by_range!(60,
                            80,
                            |integer: &IntegerValue| integer.value == 60 || integer.value == 80),
             integer_check_by_range!(2, 19),
             |a, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_3("numbers 62..69 .. 92..99",
             integer_check_by_range!(60,
                            80,
                            |integer: &IntegerValue| integer.value == 60 || integer.value == 80),
             b.reg(r"-")?,
             integer_check_by_range!(2, 19),
             |a, _, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_1_terminal("hundred",
        b.reg(r#"cents?"#)?,
        |_| IntegerValue::new_with_grain(100, 2)
    );
    b.rule_1_terminal("thousand",
        b.reg(r#"milles?"#)?,
        |_| IntegerValue::new_with_grain(1000, 3)
    );
    b.rule_1_terminal("million",
        b.reg(r#"millions?"#)?,
        |_| IntegerValue::new_with_grain(1000000, 6)
    );
    b.rule_1_terminal("billion",
        b.reg(r#"milliards?"#)?,
        |_| IntegerValue::new_with_grain(1000000000, 9)
    );
    b.rule_2("number hundreds",
        integer_check_by_range!(1, 99),
        b.reg(r#"cents?"#)?,
        |a, _| {
            Ok(IntegerValue {
                   value: a.value().value * 100,
                   grain: Some(2),
                   ..IntegerValue::default()
               })
        });
    b.rule_2("number thousands",
        integer_check_by_range!(1, 999),
        b.reg(r#"milles?"#)?,
        |a, _| {
            Ok(IntegerValue {
                   value: a.value().value * 1000,
                   grain: Some(3),
                   ..IntegerValue::default()
               })
    });
    b.rule_2("number millions",
        integer_check_by_range!(1, 999),
        b.reg(r#"millions?"#)?,
        |a, _| {
            Ok(IntegerValue {
                   value: a.value().value * 1000000,
                   grain: Some(6),
                   ..IntegerValue::default()
               })
    });
    b.rule_2("number billions",
        integer_check_by_range!(1, 999),
        b.reg(r#"milliards?"#)?,
        |a, _| {
            Ok(IntegerValue {
                   value: a.value().value * 1000000000,
                   grain: Some(9),
                   ..IntegerValue::default()
               })
    });
    b.rule_1_terminal("integer (numeric)", 
        b.reg(r#"(\d{1,18})"#)?, 
        |text_match| {
            let value: i64 = text_match.group(1).parse()?;
            IntegerValue::new(value)
    });
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
        b.reg(r#"virgule|point"#)?,
        number_check!(|number: &NumberValue| !number.suffixed()),
        |a, _, b| {
            let power = b.value().value().to_string().chars().count();
            let coeff = 10.0_f32.powf(-1.0 * power as f32);
            Ok(FloatValue {
                value: b.value().value() * coeff + a.value().value(),
                ..FloatValue::default()
            })
    });
    b.rule_4("number dot zero ... number",
         number_check!(|number: &NumberValue| !number.prefixed()),
         b.reg(r#"virgule|point"#)?,
             b.reg(r#"(?:(?:z[eé]ro )*(?:z[eé]ro))"#)?,
         number_check!(|number: &NumberValue| !number.suffixed()),
         |a, _, zeros, b| {
             let power = zeros.group(0).split_whitespace().count() + b.value().value().to_string().chars().count();
             let coeff = 10.0_f32.powf(-1.0 * power as f32);
             Ok(FloatValue {
                 value: b.value().value() * coeff + a.value().value(),
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
             b.reg(r#"-|moins"#)?,
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
            _ => return Err(RuleError::Invalid.into()),
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
    b.rule_1_terminal("(douzaine ... soixantaine)",
        b.reg(r#"(demi[ -]douz|diz|douz|quinz|vingt|trent|quarant|cinquant|soixant|cent)aines?"#)?,
        |text_match| {
            let value = match text_match.group(1).as_ref() {
                "demi douz" => 6,
                "demi-douz" => 6,
                "diz" => 10,
                "douz" => 12,
                "quinz" => 15,
                "vingt" => 20,
                "trent" => 30,
                "quarant" => 40,
                "cinquant" => 50,
                "soixant" => 60,
                "cent" => 100,
                _ => return Err(RuleError::Invalid.into()),
            };
            Ok(IntegerValue {
                value,
                group: true,
                .. IntegerValue::default()
            })
        }
    );
    b.rule_2("number dozen",
            integer_check_by_range!(1, 9),
            integer_check!(|integer: &IntegerValue| integer.group),
            |a, b| {
                 Ok(IntegerValue {
                     value: a.value().value * b.value().value,
                     grain: b.value().grain,
                     group: true,
                     ..IntegerValue::default()
                 })
    });
    b.rule_1_terminal("ordinal 0",
        b.reg(r#"z[eé]rot?i[eè]me"#)?,
        |_| {
            Ok(OrdinalValue::new(0))
        }
    );
    b.rule_1_terminal("ordinal 1",
        b.reg(r#"premi[eè]re?"#)?,
        |_| {
            Ok(OrdinalValue::new(1))
        }
    );
    b.rule_1_terminal("ordinal 2",
        b.reg(r#"seconde?|deuxi[eè]me"#)?,
        |_| {
            Ok(OrdinalValue::new(2))
        }
    );
    b.rule_1_terminal(
            "ordinals (premier..seizieme)",
            b.reg(r#"(trois|quatr|cinqu|six|sept|huit|neuv|dix|onz|douz|treiz|quatorz|quinz|seiz)i[eè]me"#)?,
            |text_match| {
                let value = match text_match.group(1).as_ref() {
                    "trois" => 3,
                    "quatr" => 4, 
                    "cinqu" => 5, 
                    "six" => 6,
                    "sept" => 7, 
                    "huit" => 8, 
                    "neuv" => 9,
                    "dix" => 10,
                    "onz" => 11,
                    "douz" => 12,
                    "treiz" => 13,
                    "quatorz" => 14,
                    "quinz" => 15,
                    "seiz" => 16,
                     _ => return Err(RuleError::Invalid.into()),
                 };
                 Ok(OrdinalValue::new(value))
            });
    b.rule_2("17ieme, 18ieme, 19ieme",
        b.reg(r#"dix-?"#)?,
        ordinal_check_by_range!(7, 9),
        |_, ordinal| {
            Ok(OrdinalValue::new(10 + ordinal.value().value))
        }
    );
    b.rule_1_terminal("20ieme, 30ieme, 40ieme, 50ieme, 60ieme",
        b.reg(r#"(vingt|trent|quarant|cinquant|soixant)i[èe]me"#)?,
        |text_match| {
            let value = match text_match.group(1).as_ref() {
                "vingt" => 20,
                "trent" => 30,
                "quarant" => 40,
                "cinquant" => 50,
                "soixant" => 60,
                _ => return Err(RuleError::Invalid.into()),
            };
            Ok(OrdinalValue::new(value))
        }
    );
    b.rule_1_terminal("80ieme",
        b.reg(r#"quatre[- ]vingts?i[èe]me"#)?,
        |_| {
            Ok(OrdinalValue::new(80))
        }
    );
    b.rule_2("22ieme...29ieme, 32ieme...39ieme, 42ieme...49ieme, 52ieme...59ieme",
        integer_check_by_range!(20, 50, |integer: &IntegerValue| integer.value % 10 == 0),
        ordinal_check_by_range!(2, 9),
        |integer, ordinal| {
            Ok(OrdinalValue::new(integer.value().value + ordinal.value().value))
        }
    );
    b.rule_3("22ieme...29ieme, 32ieme...39ieme, 42ieme...49ieme, 52ieme...59ieme",
        integer_check_by_range!(20, 50, |integer: &IntegerValue| integer.value % 10 == 0),
        b.reg(r"-")?,
        ordinal_check_by_range!(2, 9),
        |integer, _, ordinal| {
            Ok(OrdinalValue::new(integer.value().value + ordinal.value().value))
        }
    );
    b.rule_2("62ieme...70ieme, 72ieme...79ieme, 90ieme, 92ieme...99ieme",
        integer_check_by_range!(60, 80, |integer: &IntegerValue| integer.value == 60 || integer.value == 80),
        ordinal_check_by_range!(2, 19),
        |integer, ordinal| {
            Ok(OrdinalValue::new(integer.value().value + ordinal.value().value))
        }
    );
    b.rule_3("62ieme...70ieme, 72ieme...79ieme, 90ieme, 92ieme...99ieme",
        integer_check_by_range!(60, 80, |integer: &IntegerValue| integer.value == 60 || integer.value == 80),
        b.reg(r"-")?,
        ordinal_check_by_range!(2, 19),
        |integer, _, ordinal| {
            Ok(OrdinalValue::new(integer.value().value + ordinal.value().value))
        }
    );
    b.rule_2("21, 31, 41, 51, 61",
        integer_check_by_range!(20, 60, |integer: &IntegerValue| integer.value % 10 == 0),
        b.reg(r#"(?:et |-)uni[èe]me"#)?,
        |integer, _| {
            Ok(OrdinalValue::new(integer.value().value + 1))
        }
    );
    b.rule_2("81",
        integer_check_by_range!(80, 80),
        b.reg(r#"(?:et )?uni[èe]me"#)?,
        |integer, _| {
            Ok(OrdinalValue::new(integer.value().value + 1))
        }
    );
    b.rule_2("71, 91",
        integer_check_by_range!(60, 60),
        b.reg(r#"et onzi[eè]me"#)?,
        |integer, _| {
            Ok(OrdinalValue::new(integer.value().value + 11))
        }
    );
    b.rule_2("<number> et demi",
        integer_check_by_range!(0, 99),
        b.reg(r#"et demie?"#)?,
        |integer, _| {
            FloatValue::new(integer.value().value as f32 + 0.5)
        }
    );
    b.rule_1_terminal("70, 80, 90 (Belgium and Switzerland)",
        b.reg(r#"(sept|huit|non)ante"#)?,
        |text_match| {
            let value = match text_match.group(1).as_ref() {
                "sept" => 70,
                "huit" => 80,
                "non" => 90,
                _ => return Err(RuleError::Invalid.into()),
            };
            IntegerValue::new(value)
        }
    );
    b.rule_1_terminal("71, 81, 91 (Belgium and Switzerland)",
        b.reg(r#"(sept|huit|non)ante et une?"#)?,
        |text_match| {
            let value = match text_match.group(1).as_ref() {
                "sept" => 71,
                "huit" => 81,
                "non" => 91,
                _ => return Err(RuleError::Invalid.into()),
            };
            IntegerValue::new(value)
        }
    );

    b.rule_2("72..79, 82..89, 92..99, (Belgium and Switzerland)",
        b.reg(r#"(sept|huit|non)ante"#)?,
        integer_check_by_range!(2, 9),
        |text_match, integer| {
            let value = match text_match.group(1).as_ref() {
                "sept" => 70,
                "huit" => 80,
                "non" => 90,
                _ => return Err(RuleError::Invalid.into()),
            };
            IntegerValue::new(value + integer.value().value)
        }
    );
    b.rule_1_terminal("ordinal (100, 1_000, 1_000_000)",
        b.reg(r#"(cent|mill|million|milliard)i[èe]me"#)?,
        |text_match| {
            let (value, grain) = match text_match.group(1).as_ref() {
                "cent" => (100, 2),
                "mill" => (1_000, 3),
                "million" => (1_000_000, 6),
                "milliard" => (1_000_000_000, 9),
                _ => return Err(RuleError::Invalid.into()),
            };
            Ok(OrdinalValue::new_with_grain(value, grain))
        }
    );

    b.rule_2("ordinal (200..900, 2_000..9_000, 2_000_000..9_000_000_000)",
        integer_check_by_range!(2, 999),
        b.reg(r#"(cent|mill|million|milliard)i[èe]me"#)?,
        |integer, text_match| {
            let (value, grain) = match text_match.group(1).as_ref() {
                "cent" => (100, 2),
                "mill" => (1_000, 3),
                "million" => (1_000_000, 6),
                "milliard" => (1_000_000_000, 9),
                _ => return Err(RuleError::Invalid.into()),
            };
            Ok(OrdinalValue::new_with_grain(integer.value().value * value, grain))
        }
    );
    
    b.rule_2("ordinal (1_1_000..9_999_999_000)",
        integer_check_by_range!(1000, 99_999_999_000),
        ordinal_check!(|ordinal: &OrdinalValue| {
            let grain = ordinal.grain.unwrap_or(0);
            grain == 2 || grain % 3 == 0
        }),
        |integer, ordinal| {
            let grain = ordinal.value().grain.unwrap_or(0);
            let next_grain = (grain / 3) * 3 + 3;
            if integer.value().value % 10i64.pow(next_grain as u32) != 0 { return Err(RuleError::Invalid.into()); }
            Ok(OrdinalValue::new(integer.value().value + ordinal.value().value))
        }
    );

    b.rule_2("ordinal (102...9_999_999)",
        integer_check!(|integer: &IntegerValue| integer.value >= 100 || integer.value % 100 == 0),
        ordinal_check_by_range!(2, 99),
        |integer, ordinal| {
            Ok(OrdinalValue::new(integer.value().value + ordinal.value().value))
        }
    );
    b.rule_2("ordinal (101, 201, 301, ...)",
        integer_check!(|integer: &IntegerValue| integer.value >= 100 || integer.value % 100 == 0),
        b.reg(r#"(?:et |-)?uni[èe]me"#)?,
        |integer, _| {
            Ok(OrdinalValue::new(integer.value().value + 1))
        }
    );
    b.rule_1_terminal("ordinal (digits)",
             b.reg(r#"0*(\d+) ?(ere?|ère|ème|eme|ieme|ième)"#)?,
             |text_match| {
                 let value: i64 = text_match.group(1).parse()?;
                 Ok(OrdinalValue::new(value))
             });
    b.rule_2("le <ordinal>",
             b.reg(r#"l[ea]"#)?,
             ordinal_check!(),
             |_, a| Ok((*a.value()).prefixed())
    );
    Ok(())
}
