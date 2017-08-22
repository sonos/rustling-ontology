extern crate rustling;
extern crate rustling_ontology_rules;
extern crate rustling_ontology_moment;
#[macro_use]
extern crate rustling_ontology_values;

use rustling_ontology_moment::*;
pub use rustling_ontology_values::dimension::*;
pub use rustling_ontology_values::output::*;
pub use rustling_ontology_values::ResolverContext;

macro_rules! example {
    ($v:expr, $check:expr, $($ex:expr),*) => {
        $( $v.push($crate::rustling::Example::new($ex, Box::new($check))); )*
    };
}

pub mod en;
pub mod es;
pub mod fr;
pub mod ko;
pub mod de;
pub mod zh;


macro_rules! lang {
    ($lang:ident, $lang_test:ident, $lang_config:ident, [$($example:ident),*]) => {
        pub fn $lang() -> Vec<::rustling::train::Example<Dimension>> {
            let mut v = vec![];
            $( $lang::$example(&mut v); )*
            v
        }
        #[cfg(test)]
        mod $lang_test {
            use rustling::*;
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
                let rules = ::rustling_ontology_rules::$lang_config::rule_set().unwrap();
                let examples = $lang();
                assert_examples(&rules, examples);
            }
        }
    }
}

lang!(de, de_test, de_config, [examples_numbers, examples_time, examples_temperature, examples_finance]);
lang!(en, en_test, en_config, [examples_numbers, examples_time, examples_durations]);
lang!(fr, fr_test, fr_config, [examples_numbers, examples_time, examples_durations]);
lang!(es, es_test, es_config, [examples_numbers, examples_time]);
lang!(ko, ko_test, ko_config, [examples_numbers, examples_time, examples_temperature, examples_finance]);
lang!(zh, zh_test, zh_config, [examples_numbers]);
