extern crate rustling_ontology;
extern crate rustling_ontology_moment as moment;
extern crate rustling_ontology_json_utils as json_utils;
extern crate serde_json;

mod utils;

use rustling_ontology::Lang;

#[test]
// #[ignore]
fn test_en_numbers() {
    utils::run_json_test(Lang::EN, utils::build_resources_path("en", "number.json"));
}

#[test]
// #[ignore]
fn test_en_ordinal() {
    utils::run_json_test(Lang::EN, utils::build_resources_path("en", "ordinal.json"));
}

#[test]
// #[ignore]
fn test_en_percentage() {
    utils::run_json_test(Lang::EN, utils::build_resources_path("en", "percentage.json"));
}

#[test]
// #[ignore]
fn test_en_duration() {
    utils::run_json_test(Lang::EN, utils::build_resources_path("en", "duration.json"));
}

#[test]
// #[ignore]
fn test_en_temperature() {
    utils::run_json_test(Lang::EN, utils::build_resources_path("en", "temperature.json"));
}

#[test]
// #[ignore]
fn test_en_amount_of_money() {
    utils::run_json_test(Lang::EN, utils::build_resources_path("en", "amount_of_money.json"));
}

#[test]
// #[ignore]
fn test_en_datetime() {
    utils::run_json_test(Lang::EN, utils::build_resources_path("en", "datetime.json"));
}