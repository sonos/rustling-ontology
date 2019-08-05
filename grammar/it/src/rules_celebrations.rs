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
    Ok(())
}