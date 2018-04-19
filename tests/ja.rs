extern crate rustling_ontology;
extern crate rustling_ontology_moment as moment;
extern crate rustling_ontology_json_utils as json_utils;
extern crate serde_json;

mod utils;

use rustling_ontology::Lang;

#[test]
#[ignore]
fn test_ja_numbers() {
    utils::run_json_test(Lang::JA, utils::build_resources_path("ja", "number.json"));
}

#[test]
#[ignore]
fn test_ja_ordinal() {
    utils::run_json_test(Lang::JA, utils::build_resources_path("ja", "ordinal.json"));
}

#[test]
#[ignore]
fn test_ja_percentage() {
    utils::run_json_test(Lang::JA, utils::build_resources_path("ja", "percentage.json"));
}

#[test]
#[ignore]
fn test_ja_duration() {
    utils::run_json_test(Lang::JA, utils::build_resources_path("ja", "duration.json"));
}

#[test]
#[ignore]
fn test_ja_temperature() {
    utils::run_json_test(Lang::JA, utils::build_resources_path("ja", "temperature.json"));
}

#[test]
#[ignore]
fn test_ja_amount_of_money() {
    utils::run_json_test(Lang::JA, utils::build_resources_path("ja", "amount_of_money.json"));
}

#[test]
#[ignore]
fn test_ja_datetime() {
    utils::run_json_test(Lang::JA, utils::build_resources_path("ja", "datetime.json"));
}