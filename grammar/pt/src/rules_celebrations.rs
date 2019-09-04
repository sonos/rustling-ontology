use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::helpers;
use rustling_ontology_moment::{Weekday, Grain};

pub fn rules_celebration(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    // Date HOLIDAY
    b.rule_1_terminal("Christmas day",
         b.reg(r#"(?:dia de )?natal"#)?,
         |_| Ok(helpers::month_day(12, 25)?.form(Form::Celebration))
    );
    // Date HOLIDAY
    b.rule_1_terminal("New year's eve",
        b.reg(r#"ano novo|primeiro dia do ano"#)?,
        |_| Ok(helpers::month_day(1, 1)?.form(Form::Celebration))
    );
    // Date HOLIDAY
    b.rule_1_terminal("Christmas'eve",
        b.reg(r#"(?:véspera |(?:a )?noite )de natal"#)?,
        |_| {
            let start = helpers::month_day(12, 24)?.intersect(&helpers::hour(18, false)?)?;
            let end = helpers::month_day(12, 25)?.intersect(&helpers::hour(0, false)?)?;
            Ok(start.span_to(&end, false)?
                 .form(Form::Celebration))
        }
    );
    // Date HOLIDAY
    b.rule_1_terminal("All saint's day",
        b.reg(r#"(?:(?:no )?dia de )?(?:todos os santos|finados)"#)?,
        |_| Ok(helpers::month_day(11, 1)?.form(Form::Celebration))
    );
    // Date HOLIDAY
    b.rule_1_terminal("1st of May",
        b.reg(r#"dia do (?:trabalho|trabalhador)"#)?,
        |_| Ok(helpers::month_day(5, 1)?.form(Form::Celebration))
    );
    Ok(())
}
