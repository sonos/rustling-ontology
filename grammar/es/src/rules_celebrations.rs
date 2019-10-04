use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::helpers;


pub fn rules_celebration(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("Navidad",
                      b.reg(r#"navidad"#)?,
                      |_| Ok(helpers::month_day(12, 25)?
                       .form(Form::Celebration))
    );
    b.rule_1_terminal("Nochevieja",
                      b.reg(r#"nochevieja"#)?,
                      |_| Ok(helpers::month_day(12, 31)?
                       .form(Form::Celebration))
    );
    b.rule_1_terminal("ano nuevo",
                      b.reg(r#"a[nñ]o nuevo"#)?,
                      |_| Ok(helpers::month_day(1, 1)?
                       .form(Form::Celebration))
    );
    b.rule_1_terminal("Father's day",
                      b.reg(r#"(?:el )?d[íi]a del padre"#)?,
                      |_| Ok(helpers::month_day(6, 18)?
                       .form(Form::Celebration))
    );
    b.rule_1_terminal("National day",
                      b.reg(r#"(?:el )?d[íi]a de la hispanidad"#)?,
                      |_| Ok(helpers::month_day(7, 4)?
                       .form(Form::Celebration))
    );
    b.rule_1_terminal("All saints days",
                      b.reg(r#"(?:el )?d[íi]a de todos los santos"#)?,
                      |_| Ok(helpers::month_day(9, 11)?
                       .form(Form::Celebration))
    );
    b.rule_1_terminal("día de la constitucíon",
                      b.reg(r#"(?:el )?d[íi]a de la constitucíon"#)?,
                      |_| Ok(helpers::month_day(6, 14)?
                       .form(Form::Celebration))
    );
    b.rule_1_terminal("Women's day",
                      b.reg(r#"(?:el )?d[íi]a de la mujer"#)?,
                      |_| Ok(helpers::month_day(8, 26)?
                       .form(Form::Celebration))
    );
    Ok(())
}
