extern crate rustling;
extern crate rustling_ontology_rules as rules;
extern crate rustling_ontology_values as values;

use values::Dimension;
use values::check::*;

use rustling::Check;

fn run(rules: &rustling::RuleSet<Dimension>, sentence: &str) -> Vec<rustling::ParsedNode<Dimension>> {
    rules.apply_all(&sentence.to_lowercase()).unwrap()
}

#[test]
fn test_ordinal() {
    let fr = rules::fr_config::rule_set().unwrap();
    let ordinals_0_99 = ordinals_0_99();
    for ordinal in ordinals_0_99.into_iter() {
        assert!(run(&fr, ordinal.0).into_iter().any(|it| check_ordinal(ordinal.1 as i64).check(&it)), format!("{} doesn't match ordinal {}", ordinal.0, ordinal.1));
    }
}

#[test]
fn test_lang() {
    use std::str::FromStr;
    assert_eq!(rules::Lang::FR.to_string(), "FR");
    assert_eq!(rules::Lang::from_str("fr").unwrap(), rules::Lang::FR);
}

fn ordinals_0_99() -> Vec<(&'static str, usize)> {
    vec![
        ("zéroième", 0),
        ("premier", 1),
        ("deuxième", 2),
        ("troisième", 3),
        ("quatrième", 4),
        ("cinquième", 5),
        ("sixième", 6),
        ("septième", 7),
        ("huitième", 8),
        ("neuvième", 9),
        ("dixième", 10),
        ("onzième", 11),
        ("douzième", 12),
        ("treizième", 13),
        ("quatorzième", 14),
        ("quinzième", 15),
        ("seizième", 16),
        ("dix-septième", 17),
        ("dix-huitième", 18),
        ("dix-neuvième", 19),
        ("vingtième", 20),
        ("vingt et unième", 21),
        ("vingt-deuxième", 22),
        ("vingt-troisième", 23),
        ("vingt-quatrième", 24),
        ("vingt-cinquième", 25),
        ("vingt-sixième", 26),
        ("vingt-septième", 27),
        ("vingt-huitième", 28),
        ("vingt-neuvième", 29),
        ("trentième", 30),
        ("trente et unième", 31),
        ("trente-deuxième", 32),
        ("trente-troisième", 33),
        ("trente-quatrième", 34),
        ("trente-cinquième", 35),
        ("trente-sixième", 36),
        ("trente-septième", 37),
        ("trente-huitième", 38),
        ("trente-neuvième", 39),
        ("quarantième", 40),
        ("quarante et unième", 41),
        ("quarante-deuxième", 42),
        ("quarante-troisième", 43),
        ("quarante-quatrième", 44),
        ("quarante-cinquième", 45),
        ("quarante-sixième", 46),
        ("quarante-septième", 47),
        ("quarante-huitième", 48),
        ("quarante-neuvième", 49),
        ("cinquantième", 50),
        ("cinquante et unième", 51),
        ("cinquante-deuxième", 52),
        ("cinquante-troisième", 53),
        ("cinquante-quatrième", 54),
        ("cinquante-cinquième", 55),
        ("cinquante-sixième", 56),
        ("cinquante-septième", 57),
        ("cinquante-huitième", 58),
        ("cinquante-neuvième", 59),
        ("soixantième", 60),
        ("soixante et unième", 61),
        ("soixante-deuxième", 62),
        ("soixante-troisième", 63),
        ("soixante-quatrième", 64),
        ("soixante-cinquième", 65),
        ("soixante-sixième", 66),
        ("soixante-septième", 67),
        ("soixante-huitième", 68),
        ("soixante-neuvième", 69),
        ("soixante-dixième", 70),
        ("soixante et onzième", 71),
        ("soixante-douzième", 72),
        ("soixante-treizième", 73),
        ("soixante-quatorzième", 74),
        ("soixante-quinzième", 75),
        ("soixante-seizième", 76),
        ("soixante-dix-septième", 77),
        ("soixante-dix-huitième", 78),
        ("soixante-dix-neuvième", 79),
        ("quatre-vingtsième", 80),
        ("quatre-vingtième", 80),
        ("quatre-vingt-unième", 81),
        ("quatre-vingt-deuxième", 82),
        ("quatre-vingt-troisième", 83),
        ("quatre-vingt-quatrième", 84),
        ("quatre-vingt-cinquième", 85),
        ("quatre-vingt-sixième", 86),
        ("quatre-vingt-septième", 87),
        ("quatre-vingt-huitième", 88),
        ("quatre-vingt-neuvième", 89),
        ("quatre-vingt-dixième", 90),
        ("quatre-vingt-onzième", 91),
        ("quatre-vingt-douzième", 92),
        ("quatre-vingt-treizième", 93),
        ("quatre-vingt-quatorzième", 94),
        ("quatre-vingt-quinzième", 95),
        ("quatre-vingt-seizième", 96),
        ("quatre-vingt-dix-septième", 97),
        ("quatre-vingt-dix-huitième", 98),
        ("quatre-vingt-dix-neuvième", 99),
    ]
}