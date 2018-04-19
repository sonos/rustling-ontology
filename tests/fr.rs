extern crate rustling_ontology;
extern crate rustling_ontology_moment as moment;
extern crate rustling_ontology_json_utils as json_utils;
extern crate serde_json;

mod utils;

use rustling_ontology::Lang;

#[test]
#[ignore]
fn test_fr_numbers() {
    utils::run_json_test(Lang::FR, utils::build_resources_path("fr", "number.json"));
}

#[test]
#[ignore]
fn test_fr_ordinal() {
    utils::run_json_test(Lang::FR, utils::build_resources_path("fr", "ordinal.json"));
}

#[test]
#[ignore]
fn test_fr_percentage() {
    utils::run_json_test(Lang::FR, utils::build_resources_path("fr", "percentage.json"));
}

#[test]
#[ignore]
fn test_fr_duration() {
    utils::run_json_test(Lang::FR, utils::build_resources_path("fr", "duration.json"));
}

#[test]
#[ignore]
fn test_fr_temperature() {
    utils::run_json_test(Lang::FR, utils::build_resources_path("fr", "temperature.json"));
}

#[test]
#[ignore]
fn test_fr_amount_of_money() {
    utils::run_json_test(Lang::FR, utils::build_resources_path("fr", "amount_of_money.json"));
}

#[test]
#[ignore]
fn test_fr_datetime() {
    utils::run_json_test(Lang::FR, utils::build_resources_path("fr", "datetime.json"));
}