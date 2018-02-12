extern crate rustling;
extern crate rustling_ontology_values;
extern crate rustling_ontology_moment;

pub fn rule_set() -> ::rustling::RustlingResult<::rustling::RuleSet<rustling_ontology_values::Dimension>> {
    let b = ::rustling::RuleSetBuilder::new(
        ::rustling::BoundariesChecker::no_check(),
        ::rustling::BoundariesChecker::no_check());
    Ok(b.build())
}

pub fn dims() -> Vec<rustling_ontology_values::DimensionKind> {
    return vec![];
}

pub fn examples() -> Vec<::rustling::train::Example<rustling_ontology_values::Dimension>> {
    let v = vec![];
    v
}

#[cfg(test)]
mod test {
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