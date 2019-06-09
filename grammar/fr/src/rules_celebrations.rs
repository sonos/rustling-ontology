use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::helpers;
use rustling_ontology_moment::{Weekday, Grain};

pub fn rules_celebration(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {

    b.rule_1_terminal("noel",
        b.reg(r#"(?:(?:le )?jour de )?no[eë]l"#)?,
        |_| Ok(helpers::month_day(12, 25)?.form(Form::Celebration))
    );
    b.rule_1_terminal("soir de noël",
        b.reg(r#"(?:l[ea] )?(?:soir(?:ée)?|veille|r[eé]veillon) de no[eë]l"#)?,
        |_| {
            let start = helpers::month_day(12, 24)?.intersect(&helpers::hour(18, false)?)?;
            let end = helpers::month_day(12, 25)?.intersect(&helpers::hour(0, false)?)?;
            Ok(start.span_to(&end, false)?
                 .form(Form::Celebration))
        }
    );
    b.rule_1_terminal("saint sylvestre",
                      b.reg(r#"(?:l[ea] )?(?:saint[- ]sylvestre|r[eé]veillon)"#)?,
                      |_| Ok(helpers::month_day(12, 31)?.form(Form::Celebration))
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
    b.rule_1_terminal("pentecôte",
        b.reg(r#"(?:la f[eê]te de |(?:le )?lundi de )?(?:la )?pentec[oô]te"#)?,
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
    b.rule_2("à <celebration>",
             b.reg(r#"au|[aà](?:l['a])?"#)?,
             datetime_check!(form!(Form::Celebration)),
             |_, a| Ok(a.value().clone())
    );

    Ok(())

}