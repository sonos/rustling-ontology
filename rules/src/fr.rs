use rustling::*;
use dimension::*;
use helpers;
use moment::{Weekday, Grain, PeriodComp};

pub fn rules_duration(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1("seconde (unit-of-duration)",
        b.reg(r#"sec(?:onde)?s?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Second))
    );
    b.rule_1("minute (unit-of-duration)",
        b.reg(r#"min(?:ute)?s?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Minute))
    );
    b.rule_1("heure (unit-of-duration)",
        b.reg(r#"h(?:eure)?s?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Hour))
    );
    b.rule_1("jour (unit-of-duration)",
        b.reg(r#"jour(?:n[ée]e?)?s?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Day))
    );
    b.rule_1("semaine (unit-of-duration)",
        b.reg(r#"semaines?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Week))
    );
    b.rule_1("mois (unit-of-duration)",
        b.reg(r#"mois?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Month))
    );
    b.rule_1("année (unit-of-duration)",
        b.reg(r#"an(?:n[ée]e?)?s?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Year))
    );
    b.rule_1("un quart heure",
        b.reg(r#"(1/4\s?h(?:eure)?|(?:un|1) quart d'heure)"#)?,
        |_| Ok(DurationValue::new(PeriodComp::minutes(15).into()))
    );
    b.rule_1("une demi heure",
        b.reg(r#"(?:1/2\s?h(?:eure)?|(?:1|une) demi(?:e)?(?:\s|-)heure)"#)?,
        |_| Ok(DurationValue::new(PeriodComp::minutes(30).into()))
    );
    b.rule_1("trois quarts d'heure",
        b.reg(r#"(?:3/4\s?h(?:eure)?|(?:3|trois) quart(?:s)? d'heure)"#)?,
        |_| Ok(DurationValue::new(PeriodComp::minutes(45).into()))
    );
    b.rule_2("<integer> <unit-of-duration>",
        integer_check!(0),
        unit_of_duration_check!(),
        |integer, unit| Ok(DurationValue::new(PeriodComp::new(unit.value().grain, integer.value().value).into()))
    );
    b.rule_2("une <unit-of-duration>",
        b.reg(r#"une|la|le?"#)?,
        unit_of_duration_check!(),
        |_, unit| Ok(DurationValue::new(PeriodComp::new(unit.value().grain, 0).into()))
    );
    b.rule_2("dans <duration>",
        b.reg(r#"dans"#)?,
        duration_check!(),
        |_, duration| duration.value().in_present()
    );
    b.rule_2("environ <duration>",
        b.reg(r#"environ"#)?,
        duration_check!(),
        |_, duration| duration.value().in_present()
    );
    b.rule_2("il y a <duration>",
        b.reg(r#"il y a"#)?,
        duration_check!(),
        |_, duration| duration.value().ago()
    );
    b.rule_3("<duration> apres <time>",
        duration_check!(),
        b.reg(r#"apr[eè]s"#)?,
        time_check!(),
        |duration, _ , time| duration.value().after(time.value())
    );
    b.rule_3("<duration> apres <time>",
        duration_check!(),
        b.reg(r#"apr[eè]s"#)?,
        time_check!(),
        |duration, _ , time| duration.value().after(time.value())
    );
    b.rule_3("<duration> avant <time>",
        duration_check!(),
        b.reg(r#"avant"#)?,
        time_check!(),
        |duration, _, time| duration.value().before(time.value())
    );
    Ok(())
}

pub fn rules_cycle(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1("seconde (cycle)",
        b.reg(r#"secondes?"#)?,
        |_| CycleValue::new(Grain::Second)
    );
    b.rule_1("minute (cycle)",
        b.reg(r#"minutes?"#)?,
        |_| CycleValue::new(Grain::Minute)
    );
    b.rule_1("heure (cycle)",
        b.reg(r#"heures?"#)?,
        |_| CycleValue::new(Grain::Hour)
    );
    b.rule_1("jour (cycle)",
        b.reg(r#"jour(n[ée]e?)?s?"#)?,
        |_| CycleValue::new(Grain::Day)
    );
    b.rule_1("semaine (cycle)",
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
        integer_check!(2, 9999),
        cycle_check!(),
        b.reg(r#"(?:d')? ?avant|plus t[oô]t"#)?,
        |integer, cycle, _| helpers::cycle_nth(cycle.value().grain, -1 * integer.value().value)
    );
    b.rule_3("n <cycle> après",
        integer_check!(2, 9999),
        cycle_check!(),
        b.reg(r#"(?:d')? ?apr[eèé]s|qui sui(?:t|ves?)|plus tard"#)?,
        |integer, cycle, _| helpers::cycle_nth(cycle.value().grain, integer.value().value)
    );
    b.rule_4("le <cycle> après|suivant <time>",
        b.reg(r#"l[ea']? ?"#)?,
        cycle_check!(),
        b.reg(r#"suivante?|apr[eèé]s"#)?,
        time_check!(),
        |_, cycle, _, time| helpers::cycle_nth_after(cycle.value().grain, 1, time.value())
    );
    b.rule_4("le <cycle> avant|précédent <time>",
        b.reg(r#"l[ea']? ?"#)?,
        cycle_check!(),
        b.reg(r#"avant|pr[ée]c[ée]dent"#)?,
        time_check!(),
        |_, cycle, _, time| helpers::cycle_nth_after(cycle.value().grain, -1, time.value())
    );
    b.rule_3("n derniers <cycle>",
        integer_check!(2, 9999),
        b.reg(r#"derni.re?s?"#)?,
        cycle_check!(),
        |integer,  _, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    b.rule_3("n prochains <cycle>",
        integer_check!(2, 9999),
        b.reg(r#"prochaine?s?|suivante?s?|apr[eèé]s"#)?,
        cycle_check!(),
        |integer, _, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    b.rule_3("n <cycle> passes|precedents",
        integer_check!(2, 9999),
        cycle_check!(),
        b.reg(r#"pass[eèé][eèé]?s?|pr[eé]c[eé]dente?s?|(?:d')? ?avant|plus t[oô]t"#)?,
        |integer, cycle, _| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    b.rule_3("n <cycle> suivants",
        integer_check!(2, 9999),
        cycle_check!(),
        b.reg(r#"prochaine?s?|suivante?s?|apr[eèé]s|qui sui(?:t|ves?)|plus tard"#)?,
        |integer, cycle, _| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    b.rule_4("<ordinal> <cycle> de <time>",
        ordinal_check!(),
        cycle_check!(),
        b.reg(r#"d['eu]|en"#)?,
        time_check!(),
        |ordinal, cycle, _, time| helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value, time.value())
    );
    b.rule_5("le <ordinal> <cycle> de <time>",
        b.reg(r#"l[ea]"#)?,
        ordinal_check!(),
        cycle_check!(),
        b.reg(r#"d['eu]|en"#)?,
        time_check!(),
        |_, ordinal, cycle, _, time| helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value, time.value())
    );
    b.rule_4("le <cycle> de <time>",
        b.reg(r#"l[ea]"#)?,
        cycle_check!(),
        b.reg(r#"d['eu]|en"#)?,
        time_check!(),
        |_, cycle, _, time| helpers::cycle_nth_after_not_immediate(cycle.value().grain, 0, time.value())
    );
    b.rule_2("le lendemain du <time>",
        b.reg(r#"(?:le|au)? ?lendemain du"#)?,
        time_check!(),
        |_, time| helpers::cycle_nth_after_not_immediate(Grain::Day, 1, time.value())
    );
    b.rule_2("le veille du <time>",
        b.reg(r#"(la )?veille du"#)?,
        time_check!(),
        |_, time| helpers::cycle_nth_after_not_immediate(Grain::Day, -1, time.value())
    );
    Ok(())
}

pub fn rules_time(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("intersect",
        time_check!(|time: &TimeValue| !time.latent),
        time_check!(|time: &TimeValue| !time.latent),
        |a, b| a.value().intersect(b.value())
    );
    b.rule_3("intersect by 'de' or ','",
        time_check!(|time: &TimeValue| !time.latent),
        b.reg(r#"de|,"#)?,
        time_check!(|time: &TimeValue| !time.latent),
        |a, _, b| a.value().intersect(b.value())
    );
    b.rule_3("intersect by 'mais/par exemple/plutôt'",
        time_check!(|time: &TimeValue| !time.latent),
        b.reg(r#"mais|par exemple|plutôt|plutot"#)?,
        time_check!(|time: &TimeValue| !time.latent),
        |a, _, b| a.value().intersect(b.value())
    );
    b.rule_2("en <named-month>",
        b.reg(r#"en|au mois de?'?"#)?,
        time_check!(form!(Form::Month(_))),
        |_, a| Ok(a.value().clone())
    );
    b.rule_1("named-day",
        b.reg(r#"lun\.?(di)?"#)?,
        |_| helpers::day_of_week(Weekday::Mon)
    );
    b.rule_1("named-day",
        b.reg(r#"mar\.?(di)?"#)?,
        |_| helpers::day_of_week(Weekday::Tue)
    );
    b.rule_1("named-day",
        b.reg(r#"mer\.?(credi)?"#)?,
        |_| helpers::day_of_week(Weekday::Wed)
    );
    b.rule_1("named-day",
        b.reg(r#"jeu\.?(di)?"#)?,
        |_| helpers::day_of_week(Weekday::Thu)
    );
    b.rule_1("named-day",
        b.reg(r#"ven\.?(dredi)?"#)?,
        |_| helpers::day_of_week(Weekday::Fri)
    );
    b.rule_1("named-day",
        b.reg(r#"sam\.?(edi)?"#)?,
        |_| helpers::day_of_week(Weekday::Sat)
    );
    b.rule_1("named-day",
        b.reg(r#"dim\.?(anche)?"#)?,
        |_| helpers::day_of_week(Weekday::Sun)
    );
    b.rule_1("named-month",
        b.reg(r#"janvier|janv\.?"#)?,
        |_| helpers::month(1)
    );
    b.rule_1("named-month",
        b.reg(r#"fevrier|février|fev|fév\.?"#)?,
        |_| helpers::month(2)
    );
    b.rule_1("named-month",
        b.reg(r#"mars|mar\.?"#)?,
        |_| helpers::month(3)
    );
    b.rule_1("named-month",
        b.reg(r#"avril|avr\.?"#)?,
        |_| helpers::month(4)
    );
    b.rule_1("named-month",
        b.reg(r#"mai"#)?,
        |_| helpers::month(5)
    );
    b.rule_1("named-month",
        b.reg(r#"juin|jun\.?"#)?,
        |_| helpers::month(6)
    );
    b.rule_1("named-month",
        b.reg(r#"juillet|juil?\."#)?,
        |_| helpers::month(7)
    );
    b.rule_1("named-month",
        b.reg(r#"aout|août|aou\.?"#)?,
        |_| helpers::month(8)
    );
    b.rule_1("named-month",
        b.reg(r#"septembre|sept?\.?"#)?,
        |_| helpers::month(9)
    );
    b.rule_1("named-month",
        b.reg(r#"octobre|oct\.?"#)?,
        |_| helpers::month(10)
    );
    b.rule_1("named-month",
        b.reg(r#"novembre|nov\.?"#)?,
        |_| helpers::month(11)
    );
    b.rule_1("named-month",
        b.reg(r#"décembre|decembre|déc\.?|dec\.?"#)?,
        |_| helpers::month(12)
    );
    b.rule_1("noel",
        b.reg(r#"(?:jour de )?no[eë]l"#)?,
        |_| helpers::month_day(12, 25)
    );
    b.rule_1("soir de noël",
        b.reg(r#"soir(?:ée)? de no[eë]l"#)?,
        |_| {
            let start = helpers::month_day(12, 24)?.intersect(&helpers::hour(18, false)?)?;
            let end = helpers::month_day(12, 25)?.intersect(&helpers::hour(0, false)?)?;
            start.span_to(&end, false)
        }
    );
    b.rule_1("jour de l'an",
        b.reg(r#"(?:jour de l'|nouvel )an"#)?,
        |_| helpers::month_day(1, 1)
    );
    b.rule_1("toussaint",
        b.reg(r#"(?:(?:la |la journée de la |jour de la )?toussaint|jour des morts)"#)?,
        |_| helpers::month_day(11, 1)
    );
    b.rule_1("1er mai",
        b.reg(r#"f(e|ê)te du travail"#)?,
        |_| helpers::month_day(5, 1)
    );
    b.rule_1("maintenant",
        b.reg(r#"maintenant|(?:tout de suite)"#)?,
        |_| helpers::cycle_nth(Grain::Second, 0)
    );
    b.rule_1("aujourd'hui",
        b.reg(r#"(?:aujourd'? ?hui)|(?:ce jour)|(?:dans la journ[ée]e?)|(?:en ce moment)"#)?,
        |_| helpers::cycle_nth(Grain::Day, 0)
    );
    b.rule_1("demain",
        b.reg(r#"(?:demain)|(?:le lendemain)"#)?,
        |_| helpers::cycle_nth(Grain::Day, 1)
    );
    b.rule_1("hier",
        b.reg(r#"hier|la veille"#)?,
        |_| helpers::cycle_nth(Grain::Day, -1)
    );
    b.rule_1("fin du mois",
        b.reg(r#"(?:(?:[aà] )?la )?fin (?:du|de) mois"#)?,
        |_| {
            let month = helpers::cycle_nth(Grain::Month, 1)?;
            Ok(helpers::cycle_nth_after(Grain::Day, -10, &month)?
                .span_to(&month, false)?
                .latent()
                .form(Form::PartOfDay)) // Weird form
        } 
    );
    b.rule_1("après-demain",
        b.reg(r#"apr(?:e|è)s[- ]?demain"#)?,
        |_| helpers::cycle_nth(Grain::Day, 2)
    );
    b.rule_1("avant-hier",
        b.reg(r#"avant[- ]?hier"#)?,
        |_| helpers::cycle_nth(Grain::Day, -2)
    );
    b.rule_2("ce <day-of-week>",
        b.reg(r#"ce"#)?,
        time_check!(form!(Form::DayOfWeek{..})),
        |_, time| time.value().the_nth_not_immediate(0)
    );
    b.rule_2("ce <time>",
        b.reg(r#"ce"#)?,
        time_check!(),
        |_, time| time.value().the_nth(0)
    );
    b.rule_2("<day-of-week> prochain",
        time_check!(form!(Form::DayOfWeek{..})),
        b.reg(r#"prochain"#)?,
        |time, _| time.value().the_nth_not_immediate(0)
    );
    b.rule_2("<named-month> prochain",
        time_check!(),
        b.reg(r#"prochain"#)?,
        |time, _| time.value().the_nth(1)
    );
    b.rule_2("<named-month|named-day> suivant|d'après",
        time_check!(),
        b.reg(r#"suivante?s?|d'apr[eéè]s"#)?,
        |time, _| time.value().the_nth(1)
    );
    b.rule_2("<named-month|named-day> dernier|passé",
        time_check!(),
        b.reg(r#"derni[eéè]re?|pass[ée]e?"#)?,
        |time, _| time.value().the_nth(-1)
    );
    b.rule_2("<named-day> en huit",
        time_check!(form!(Form::DayOfWeek{..})),
        b.reg(r#"en (?:huit|8)"#)?,
        |time, _| time.value().the_nth(1)
    );
    b.rule_2("<named-day> en quinze",
        time_check!(form!(Form::DayOfWeek{..})),
        b.reg(r#"en (quinze|15)"#)?,
        |time, _| time.value().the_nth(2)
    );
    b.rule_4("dernier <day-of-week> de <time> (latent)",
        b.reg(r#"derni[eéè]re?"#)?,
        time_check!(form!(Form::DayOfWeek{..})),
        b.reg(r#"d['e]"#)?,
        time_check!(),
        |_, dow, _, time| dow.value().last_of(time.value())
    );
    b.rule_4("dernier <day-of-week> de <time> (latent)",
        b.reg(r#"derni[eéè]re?"#)?,
        cycle_check!(),
        b.reg(r#"d['e]"#)?,
        time_check!(),
        |_, cycle, _, time| cycle.value().last_of(time.value())
    );
    b.rule_3("<ordinal> week-end de <time>",
        ordinal_check!(),
        b.reg(r#"week(?:\s|-)?end (?:d['eu]|en|du mois de)"#)?,
        time_check!(form!(Form::Month(_))),
        |ordinal, _, time| { 
            let week_day_start = helpers::day_of_week(Weekday::Fri)?.intersect(&helpers::hour(18, false)?)?;
            let week_day_end = helpers::day_of_week(Weekday::Mon)?.intersect(&helpers::hour(0, false)?)?;
            let week_day = week_day_start.span_to(&week_day_end, false)?;
            let week_ends_of_time = time.value().intersect(&week_day)?;
            week_ends_of_time.the_nth(ordinal.value().value - 1)
        }
    );
    b.rule_2("dernier week-end de <time>",
        b.reg(r#"dernier week(?:\s|-)?end (?:d['eu]|en|du mois de)"#)?,
        time_check!(form!(Form::Month(_))),
        |_, time| { 
            let week_day_start = helpers::day_of_week(Weekday::Fri)?.intersect(&helpers::hour(18, false)?)?;
            let week_day_end = helpers::day_of_week(Weekday::Mon)?.intersect(&helpers::hour(0, false)?)?;
            let week_day = week_day_start.span_to(&week_day_end, false)?;
            week_day.last_of(time.value())
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
        integer_check!(2101, 3000),
        |integer| {
            Ok(helpers::year(integer.value().value as i32)?.latent())
        }
    );
    b.rule_1("day of month (premier)",
        b.reg(r#"premier|prem\.?|1er|1 er"#)?,
        |_| helpers::day_of_month(1)
    );
    b.rule_2("le <day-of-month> (non ordinal)",
        b.reg(r#"le"#)?,
        integer_check!(1, 31),
        |_, integer| helpers::day_of_month(integer.value().value as u32)
    );
    b.rule_4("le <day-of-month> à <datetime>",
        b.reg(r#"le"#)?,
        integer_check!(1, 31),
        b.reg(r#"[aà]"#)?,
        time_check!(),
        |_, integer, _, time| {
            let day_of_month = helpers::day_of_month(integer.value().value as u32)?;
            day_of_month.intersect(&time.value())
        }
    );
    b.rule_2("<day-of-month> <named-month>",
        integer_check!(1, 31),
        time_check!(form!(Form::Month(_))),
        |integer, month| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_2("<day-of-week> <day-of-month>",
        time_check!(form!(Form::DayOfWeek{..})), // Weird it is not used in the production of the rule
        integer_check!(1, 31),
        |_, integer| helpers::day_of_month(integer.value().value as u32)
    );
    b.rule_3("<day-of-week> <day-of-month> à <time-of-day>)",
        time_check!(form!(Form::DayOfWeek{..})), // Weird it is not used in the production of the rule
        integer_check!(1, 31),
        time_check!(form!(Form::TimeOfDay(_))),
        |_, integer, tod| helpers::day_of_month(integer.value().value as u32)
            ?.intersect(tod.value())
    );
    b.rule_1("time-of-day (latent)",
        integer_check!(0, 23),
        |integer| Ok(helpers::hour(integer.value().value as u32, integer.value().value < 12)?.latent())
    );
    b.rule_1("midi",
        b.reg(r#"midi"#)?,
        |_| helpers::hour(12, false)
    );
    b.rule_1("minuit",
        b.reg(r#"minuit"#)?,
        |_| helpers::hour(0, false)
    );
    b.rule_2("<time-of-day> heures",
        time_check!(form!(Form::TimeOfDay(Some(_)))),
        b.reg(r#"h\.?(?:eure)?s?"#)?,
        |a, _| Ok(a.value().clone().not_latent())
    );
    b.rule_2("à|vers <time-of-day>",
        b.reg(r#"(?:vers|autour de|[aà] environ|aux alentours de|[aà])"#)?,
        time_check!(form!(Form::TimeOfDay(_))),
        |_, a| Ok(a.value().clone().not_latent())
    );
    b.rule_1("hh(:|h)mm (time-of-day)",
        b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:h]([0-5]\d)"#)?,
        |text_match| {
            let hour: u32 = text_match.group(1).parse()?;
            let minute: u32 = text_match.group(2).parse()?;
            helpers::hour_minute(hour, minute, hour < 12)
        }
    );
    b.rule_1("hh:mm:ss",
        b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:.]([0-5]\d)[:.]([0-5]\d)"#)?,
        |text_match| helpers::hour_minute_second(
                text_match.group(1).parse()?, 
                text_match.group(2).parse()?,
                text_match.group(3).parse()?,
                false 
                )

    );
    b.rule_1("hhmm (military time-of-day)",
        b.reg(r#"((?:[01]?\d)|(?:2[0-3]))([0-5]\d)"#)?,
        |text_match| Ok(helpers::hour_minute(
            text_match.group(1).parse()?,
            text_match.group(2).parse()?,
            false
            )?.latent())
    );
    b.rule_1("quart (relative minutes)",
        b.reg(r#"quart"#)?,
        |_| Ok(RelativeMinuteValue(15))
    );
    b.rule_1("demi (relative minutes)",
        b.reg(r#"(?:3|trois) quarts?"#)?,
        |_| Ok(RelativeMinuteValue(30))
    );
    b.rule_1("trois quarts (relative minutes)",
        b.reg(r#"(?:3|trois) quarts?"#)?,
        |_| Ok(RelativeMinuteValue(45))
    );
    b.rule_1("number (as relative minutes)",
        integer_check!(1, 59),
        |a| Ok(RelativeMinuteValue(a.value().value as i32))
    );
    b.rule_2("number minutes (as relative minutes)",
        integer_check!(1, 59),
        b.reg(r#"min\.?(?:ute)?s?"#)?,
        |a, _| Ok(RelativeMinuteValue(a.value().value as i32))
    );
    b.rule_2("<hour-of-day> <integer> (as relative minutes)",
        time_check!(form!(Form::TimeOfDay(Some(_)))),
        relative_minute_check!(),
        |time, minutes| helpers::hour_relative_minute(
            time.value().form_time_of_day()?.full_hour,
            minutes.value().0,
            time.value().form_time_of_day()?.is_12_clock
        )
    );
    b.rule_3("<hour-of-day> moins <integer> (as relative minutes)",
        time_check!(form!(Form::TimeOfDay(Some(_)))),
        b.reg(r#"moins(?: le)?"#)?,
        relative_minute_check!(),
        |time, _, minutes| helpers::hour_relative_minute(
            time.value().form_time_of_day()?.full_hour,
            -1 * minutes.value().0,
            time.value().form_time_of_day()?.is_12_clock
        )
    );
    b.rule_3("<hour-of-day> et|passé de <relative minutes>",
        time_check!(form!(Form::TimeOfDay(Some(_)))),
        b.reg(r#"et|(?:pass[ée]e? de)"#)?,
        relative_minute_check!(),
        |time, _, minutes| helpers::hour_relative_minute(
            time.value().form_time_of_day()?.full_hour,
            -1 * minutes.value().0,
            time.value().form_time_of_day()?.is_12_clock
        )
    );
    b.rule_1("dd/-.mm/-.yyyy",
        b.reg(r#"(3[01]|[12]\d|0?[1-9])[-/.](1[0-2]|0?[1-9])[-/.](\d{2,4})"#)?,
        |text_match| helpers::ymd(
            text_match.group(3).parse()?,
            text_match.group(2).parse()?,
            text_match.group(1).parse()?
        )
    );
    b.rule_1("yyyy-mm-dd",
        b.reg(r#"(\d{2,4})-(1[0-2]|0?[1-9])-(3[01]|[12]\d|0?[1-9])"#)?,
        |text_match| helpers::ymd(
            text_match.group(1).parse()?,
            text_match.group(2).parse()?,
            text_match.group(3).parse()?
        )
    );
    b.rule_1("dd/-mm",
        b.reg(r#"(3[01]|[12]\d|0?[1-9])[/-](1[0-2]|0?[1-9])"#)?,
        |text_match| helpers::month_day(
            text_match.group(2).parse()?,
            text_match.group(1).parse()?,
        )
    );
    b.rule_1("dd mm yyyy",
        b.reg(r#"(3[01]|[12]\d|0?[1-9]) (1[0-2]|0?[1-9]) (\d{2,4})"#)?,
        |text_match| helpers::ymd(
            text_match.group(3).parse()?,
            text_match.group(2).parse()?,
            text_match.group(1).parse()?
        )
    );
    b.rule_1("dd mm",
        b.reg(r#"(3[01]|[12]\d|0?[1-9]) (1[0-2]|0?[1-9])"#)?,
        |text_match| helpers::month_day(
            text_match.group(2).parse()?,
            text_match.group(1).parse()?,
        )
    );
    b.rule_1("matin",
        b.reg(r#"mat(?:in[ée]?e?)?"#)?,
        |_| Ok(helpers::hour(4, false)?
                .span_to(&helpers::hour(12, false)?, false)?
                .latent()
                .form(Form::PartOfDay))
    );
    b.rule_1("début de matinée",
        b.reg(r#"(?:le matin (?:tr[eè]s )?t[ôo]t|(?:tr[eè]s )?t[ôo]t le matin|d[ée]but de matin[ée]e)"#)?,
        |_| Ok(helpers::hour(4, false)?
                .span_to(&helpers::hour(9, false)?, false)?
                .latent()
                .form(Form::PartOfDay))
    );
    b.rule_1("milieu de matinée",
        b.reg(r#"milieu de matin[ée]e"#)?,
        |_| Ok(helpers::hour(9, false)?
                .span_to(&helpers::hour(11, false)?, false)?
                .latent()
                .form(Form::PartOfDay))
    );
    b.rule_1("fin de matinée",
        b.reg(r#"fin de matin[ée]e"#)?,
        |_| Ok(helpers::hour(10, false)?
                .span_to(&helpers::hour(12, false)?, false)?
                .latent()
                .form(Form::PartOfDay))
    );
    b.rule_1("au déjeuner",
        b.reg(r#"(?:[àa] l(?:')?heure du|pendant(?: le)?|au)? d[eéè]jeuner"#)?,
        |_| Ok(helpers::hour(12, false)?
                .span_to(&helpers::hour(14, false)?, false)?
                .latent()
                .form(Form::PartOfDay))
    );
    b.rule_1("après le déjeuner",
        b.reg(r#"apr[eè]s (?:le )?d[eéè]jeuner"#)?,
        |_| {
            let period = helpers::hour(13, false)?
                    .span_to(&helpers::hour(17, false)?, false)?;
            Ok(helpers::cycle_nth(Grain::Day, 0)?.intersect(&period)?.form(Form::PartOfDay))
        }
    );
    b.rule_1("avant le déjeuner",
        b.reg(r#"avant (?:le )?d[eéè]jeuner"#)?,
        |_| {
            let period = helpers::hour(10, false)?
                    .span_to(&helpers::hour(12, false)?, false)?;
            Ok(helpers::cycle_nth(Grain::Day, 0)?.intersect(&period)?.form(Form::PartOfDay))
        }
    );
    b.rule_1("après le travail",
        b.reg(r#"apr[eè]s (?:le )?travail"#)?,
        |_| {
            let period = helpers::hour(17, false)?
                    .span_to(&helpers::hour(21, false)?, false)?;
            Ok(helpers::cycle_nth(Grain::Day, 0)?.intersect(&period)?.form(Form::PartOfDay))
        }
    );
    b.rule_1("après-midi",
        b.reg(r#"apr[eéè]s?[ \-]?midi"#)?,
        |_| {
            Ok(helpers::hour(12, false)?
                    .span_to(&helpers::hour(19, false)?, false)?
                    .latent()
                    .form(Form::PartOfDay))
        }
    );
    b.rule_1("début d'après-midi",
        b.reg(r#"d[ée]but d'apr[eéè]s?[ \-]?midi"#)?,
        |_| {
            Ok(helpers::hour(12, false)?
                    .span_to(&helpers::hour(14, false)?, false)?
                    .latent()
                    .form(Form::PartOfDay))
        }
    );
    b.rule_1("milieu d'après-midi",
        b.reg(r#"milieu d'apr[eéè]s?[ \-]?midi"#)?,
        |_| {
            Ok(helpers::hour(15, false)?
                    .span_to(&helpers::hour(17, false)?, false)?
                    .latent()
                    .form(Form::PartOfDay))
        }
    );
    b.rule_1("fin d'après-midi",
        b.reg(r#"fin d'apr[eéè]s?[ \-]?midi"#)?,
        |_| {
            Ok(helpers::hour(17, false)?
                    .span_to(&helpers::hour(19, false)?, false)?
                    .latent()
                    .form(Form::PartOfDay))
        }
    );
    b.rule_1("début de journée",
        b.reg(r#"d[ée]but de journ[ée]e"#)?,
        |_| {
            Ok(helpers::hour(6, false)?
                    .span_to(&helpers::hour(10, false)?, false)?
                    .latent()
                    .form(Form::PartOfDay))
        }
    );
    b.rule_1("milieu de journée",
        b.reg(r#"milieu de journ[ée]e"#)?,
        |_| {
            Ok(helpers::hour(11, false)?
                    .span_to(&helpers::hour(16, false)?, false)?
                    .latent()
                    .form(Form::PartOfDay))
        }
    );
    b.rule_1("fin de journée",
        b.reg(r#"fin de journ[ée]e"#)?,
        |_| {
            Ok(helpers::hour(17, false)?
                    .span_to(&helpers::hour(21, false)?, false)?
                    .latent()
                    .form(Form::PartOfDay))
        }
    );
    b.rule_1("soir",
        b.reg(r#"soir[ée]?e?"#)?,
        |_| {
            Ok(helpers::hour(18, false)?
                    .span_to(&helpers::hour(0, false)?, false)?
                    .latent()
                    .form(Form::PartOfDay))
        }
    );
    b.rule_1("début de soirée",
        b.reg(r#"d[ée]but de soir[ée]e?"#)?,
        |_| {
            Ok(helpers::hour(18, false)?
                    .span_to(&helpers::hour(21, false)?, false)?
                    .latent()
                    .form(Form::PartOfDay))
        }
    );
    b.rule_1("fin de soirée",
        b.reg(r#"fin de soir[ée]e?"#)?,
        |_| {
            Ok(helpers::hour(21, false)?
                    .span_to(&helpers::hour(0, false)?, false)?
                    .latent()
                    .form(Form::PartOfDay))
        }
    );
    b.rule_2("du|dans le <part-of-day>",
        b.reg(r#"du|dans l[ae']? ?|au|en|l[ae' ]|d[èe]s l?[ae']? ?"#)?,
        time_check!(form!(Form::PartOfDay)),
        |_, a| Ok(a.value().clone().not_latent())
    );
    b.rule_2("ce <part-of-day>",
        b.reg(r#"cet?t?e?"#)?,
        time_check!(form!(Form::PartOfDay)),
        |_, a| Ok(helpers::cycle_nth(Grain::Day, 0)?.intersect(a.value())?.form(Form::PartOfDay))
    );
    b.rule_2("<dim time> <part-of-day>",
        time_check!(),
        time_check!(form!(Form::PartOfDay)),
        |a, b| a.value().intersect(b.value())
    );
    b.rule_2("<dim time> du matin",
        time_check!(form!(Form::TimeOfDay(_))),
        b.reg(r#"(?:(?:du|dans|de) )?(?:(?:au|le|la) )?mat(?:in[ée]?e?)?"#)?,
        |a, _| { 
            let period = helpers::hour(0, false)?
                    .span_to(&helpers::hour(12, false)?, false)?
                    .latent().form(Form::PartOfDay);
            a.value().intersect(&period)
        }
    );
    b.rule_2("<dim time> du soir",
        time_check!(form!(Form::TimeOfDay(_))),
        b.reg(r#"(?:(?:du|dans|de) )?(?:(?:au|le|la) )?soir[ée]?e?"#)?,
        |a, _| { 
            let period = helpers::hour(16, false)?
                    .span_to(&helpers::hour(0, false)?, false)?
                    .latent().form(Form::PartOfDay);
            a.value().intersect(&period)
        }
    );
    b.rule_3("<part-of-day> du <dim time>",
        time_check!(form!(Form::TimeOfDay(_))),
        b.reg(r#"du"#)?,
        time_check!(),
        |a, _, b| b.value().intersect(a.value())
    );
    b.rule_1("week-end",
        b.reg(r#"week(?:\s|-)?end"#)?,
        |_| {
            let friday = helpers::day_of_week(Weekday::Fri)?
                                .intersect(&helpers::hour(18, false)?)?;
            let monday = helpers::day_of_week(Weekday::Mon)?
                                .intersect(&helpers::hour(0, false)?)?;
            friday.span_to(&monday, false)
        }
    );
    b.rule_1("début de semaine",
        b.reg(r#"(?:en |au )?d[ée]but de (?:cette |la )?semaine"#)?,
        |_| helpers::day_of_week(Weekday::Mon)
                    ?.span_to(&helpers::day_of_week(Weekday::Tue)?, false)
    );
    b.rule_1("milieu de semaine",
        b.reg(r#"(?:en |au )?milieu de (?:cette |la )?semaine"#)?,
        |_| helpers::day_of_week(Weekday::Wed)
                    ?.span_to(&helpers::day_of_week(Weekday::Thu)?, false)
    );
    b.rule_1("fin de semaine",
        b.reg(r#"(?:en |à la )?fin de (?:cette |la )?semaine"#)?,
        |_| helpers::day_of_week(Weekday::Thu)
                    ?.span_to(&helpers::day_of_week(Weekday::Sun)?, false)
    );
    b.rule_1("en semaine",
        b.reg(r#"(pendant la |en )?semaine"#)?,
        |_| helpers::day_of_week(Weekday::Mon)
                    ?.span_to(&helpers::day_of_week(Weekday::Fri)?, false)
    );
    b.rule_1("season",
        b.reg(r#"(?:cet )?(?:été|ete)"#)?,
        |_| helpers::month_day(6, 21)?.span_to(&helpers::month_day(9, 23)?, false)
    );
    b.rule_1("season",
        b.reg(r#"(?:cet )?automne"#)?,
        |_| helpers::month_day(9, 23)?.span_to(&helpers::month_day(12, 21)?, false)
    );
    b.rule_1("season",
        b.reg(r#"(?:cet )?hiver"#)?,
        |_| helpers::month_day(12, 21)?.span_to(&helpers::month_day(3, 20)?, false)
    );
    b.rule_1("season",
        b.reg(r#"(?:ce )?printemps"#)?,
        |_| helpers::month_day(3, 20)?.span_to(&helpers::month_day(6, 21)?, false)
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
             b.reg(r#"f(?:ah?reh?n(?:h?eit)?)?\.?"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                        value: a.value().value,
                        unit: Some("fahrenheit"),
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
    b.rule_1(
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
                        _ => panic!("Unknow match"),
                    };
                    IntegerValue::new(value) 
            });
    b.rule_1("number (20..60)",
             b.reg(r#"(vingt|trente|quarante|cinquante|soixante)"#)?,
             |text_match| {
        let value = match text_match.group(1).as_ref() {
            "vingt" => 20,
            "trente" => 30,
            "quarante" => 40,
            "cinquante" => 50,
            "soixante" => 60,
            _ => panic!("Unknow match"),
        };
        IntegerValue::new(value)
    });
    b.rule_2("number (17..19)",
             integer_check!(10, 10),
             integer_check!(7, 9),
             |_, b| IntegerValue::new(b.value().value + 10));
    b.rule_2("number 80", //
             b.reg(r#"quatre"#)?,
             b.reg(r#"vingts?"#)?,
             |_, _| IntegerValue::new(80));
    b.rule_3("numbers 21 31 41 51",
             integer_check!(20, 50, |integer: &IntegerValue| integer.value % 10 == 0),
             b.reg(r#"et"#)?,
             integer_check!(1, 1),
             |a, _, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_2("numbers 22..29 32..39 .. 52..59",
             integer_check!(20, 50, |integer: &IntegerValue| integer.value % 10 == 0),
             integer_check!(2, 9),
             |a, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_3("numbers 61 71",
             integer_check!(60, 60),
             b.reg(r#"-?et-?"#)?,
             integer_check!(1,
                            11,
                            |integer: &IntegerValue| integer.value == 1 || integer.value == 11),
             |a, _, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_2("numbers 81 91",
             integer_check!(80, 80),
             integer_check!(1,
                            11,
                            |integer: &IntegerValue| integer.value == 1 || integer.value == 11),
             |a, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_2("numbers 62..69 .. 92..99",
             integer_check!(60,
                            80,
                            |integer: &IntegerValue| integer.value == 60 || integer.value == 80),
             integer_check!(2, 19),
             |a, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_1("integer (numeric)", b.reg(r#"(\d{1,18})"#)?, |text_match| {
        let value: i64 = text_match.group(1).parse()?;
        IntegerValue::new(value)
    });
    b.rule_1("integer with thousands separator .",
             b.reg(r#"(\d{1,3}(\.\d\d\d){1,5})"#)?,
             |text_match| {
                 let reformatted_string = text_match.group(1).replace(".", "");
                 let value: i64 = reformatted_string.parse()?;
                 IntegerValue::new(value)
             });
    b.rule_1("decimal number", b.reg(r#"(\d*,\d+)"#)?, |text_match| {
        let reformatted_string = text_match.group(1).replace(",", ".");
        let value: f32 = reformatted_string.parse()?;
        FloatValue::new(value)
    });
    b.rule_1("decimal with thousands separator",
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
            "ordinals (premier..seizieme)",
            b.reg(r#"(premi(?:ere?|ère)|(?:deux|trois|quatr|cinqu|six|sept|huit|neuv|dix|onz|douz|treiz|quatorz|quinz|seiz)i[eè]me)"#)?,
            |text_match| {
                let value = match text_match.group(1).as_ref() {
                    "premier" => 1,
                    "premiere" => 1,
                    "première" => 1, 
                    "deuxieme" => 2,
                    "troisieme" => 3,
                    "quatrieme" => 4, 
                    "cinquieme" => 5, 
                    "sixieme" => 6,
                    "septieme" => 7, 
                    "huitieme" => 8, 
                    "neuvieme" => 9,
                    "dixieme" => 10,
                    "onzieme" => 11,
                    "douzieme" => 12,
                    "treizieme" => 13,
                    "quatorzieme" => 14,
                    "quinzieme" => 15,
                    "seizieme" => 16,
                    "deuxième" => 2,
                    "troisième" => 3,
                    "quatrième" => 4, 
                    "cinquième" => 5, 
                    "sixième" => 6, 
                    "septième" => 7,
                    "huitième" => 8, 
                    "neuvième" => 9, 
                    "dixième" => 10, 
                    "onzième" => 11, 
                    "douzième" => 12, 
                    "treizième" => 13,
                    "quatorzième" => 14,
                    "quinzième" => 15,
                    "seizième" => 16,
                     _ => panic!("Unknow match"),
                 };
                 Ok(OrdinalValue { value: value })
            });
    b.rule_1("ordinal (digits)",
             b.reg(r#"0*(\d+) ?(ere?|ère|ème|eme|ieme|ième)"#)?,
             |text_match| {
                 let value: i64 = text_match.group(1).parse()?;
                 Ok(OrdinalValue { value: value })
             });
    b.rule_2("le <ordinal>",
             b.reg(r#"le"#)?,
             ordinal_check!(),
             |_, a| Ok(*a.value()));
    Ok(())
}
