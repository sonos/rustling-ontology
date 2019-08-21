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
    Ok(())
}
