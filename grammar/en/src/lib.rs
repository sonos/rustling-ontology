extern crate rustling;
#[macro_use]
extern crate rustling_ontology_values;
extern crate rustling_ontology_moment;

mod rules_datetime;
mod rules_celebrations;
mod rules_duration;
mod rules_number;
mod rules_amount;
mod training;

use rustling_ontology_values::DimensionKind::*;

pub fn rule_set() -> ::rustling::RustlingResult<::rustling::RuleSet<rustling_ontology_values::Dimension>> {
    let mut b = ::rustling::RuleSetBuilder::new(
                    ::rustling::BoundariesChecker::detailed(),
                    ::rustling::BoundariesChecker::separated_alphanumeric_word());
    rules_number::rules_numbers(&mut b)?;
    rules_amount::rules_temperature(&mut b)?;
    rules_amount::rules_finance(&mut b)?;
    rules_amount::rules_percentage(&mut b)?;
    rules_datetime::rules_cycle(&mut b)?;
    rules_datetime::rules_datetime(&mut b)?;
    rules_datetime::rules_datetime_with_duration(&mut b)?;
    rules_datetime::rules_datetime_with_cycle(&mut b)?;
    rules_celebrations::rules_celebration(&mut b)?;
    rules_duration::rules_duration(&mut b)?;
    Ok(b.build())
}

pub fn dims() -> Vec<rustling_ontology_values::DimensionKind> {
    return vec![Number, Ordinal, Datetime, Duration, Temperature, AmountOfMoney, Percentage];
}

pub fn examples() -> Vec<::rustling::train::Example<rustling_ontology_values::Dimension>> {
    let mut v = vec![];
    training::examples_numbers(&mut v);
    training::examples_datetime(&mut v);
    training::examples_durations(&mut v);
    training::examples_temperature(&mut v);
    training::examples_finance(&mut v);
    training::examples_percentage(&mut v);
    v
}

#[cfg(test)]
mod test {
    use rustling::*;
    use rustling_ontology_values::dimension::Dimension;
    
    use super::*;
    fn assert_examples(rules: &RuleSet<Dimension>, examples: Vec<Example<Dimension>>) {
        for ex in examples.iter() {
            let stash = rules.apply_all(&ex.text.to_lowercase()).unwrap();
            let correct_results = stash
                        .into_iter()
                        .filter(|candidate| candidate.root_node.byte_range == Range(0, ex.text.len()) && ex.predicate.check(&candidate))
                        .collect::<Vec<_>>();
            assert!(!correct_results.is_empty(), format!("No full match found for: {:?}", ex.text));
        }
    }
    #[test]
    fn test_examples() {
        let rules = rule_set().unwrap();
        let examples = examples();
        assert_examples(&rules, examples);
    }
}
