use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::helpers;
//use rustling_ontology_moment::{Weekday, Grain};


pub fn rules_celebration(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("christmas",
                      b.reg(r#"natale"#)?,
                      |_| Ok(helpers::month_day(12, 25)?
                          .form(Form::Celebration))
    );
    b.rule_1_terminal("New year's eve",
                          b.reg(r#"capodanno"#)?,
                          |_| Ok(helpers::month_day(1, 1)?
                              .form(Form::Celebration))
    );
    b.rule_1_terminal("Labor's day",
                          b.reg(r#"festa del lavoro"#)?,
                          |_| Ok(helpers::month_day(5, 1)?
                              .form(Form::Celebration))
    );
    b.rule_1_terminal("Festa della liberazione",
                          b.reg(r#"festa della liberazione"#)?,
                          |_| Ok(helpers::month_day(4, 25)?
                              .form(Form::Celebration))
    );
    b.rule_1_terminal("Ferragosto",
                          b.reg(r#"Assumption"#)?,
                          |_| Ok(helpers::month_day(8, 15)?
                              .form(Form::Celebration))
    );

    b.rule_1_terminal("all soul's day",
                          b.reg(r#"giorno dei morti"#)?,
                          |_| Ok(helpers::month_day(11, 2)?
                              .form(Form::Celebration))
    );
    b.rule_1_terminal("Father day",
                          b.reg(r#"festa del papà"#)?,
                          |_| Ok(helpers::month_day(3, 19)?
                              .form(Form::Celebration))
    );

    Ok(())
}