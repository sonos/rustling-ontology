extern crate rustling_ontology;
extern crate rustling_ontology_moment as moment;
extern crate rustling_ontology_json_utils as json_utils;
extern crate serde_json;

mod utils;

use rustling_ontology::Lang;

#[test]
#[ignore]
fn test_de_numbers() {
    utils::run_json_test(Lang::DE, utils::build_resources_path("de", "number.json"));
}

#[test]
#[ignore]
fn test_de_ordinal() {
    utils::run_json_test(Lang::DE, utils::build_resources_path("de", "ordinal.json"));
}

#[test]
#[ignore]
fn test_de_percentage() {
    utils::run_json_test(Lang::DE, utils::build_resources_path("de", "percentage.json"));
}

#[test]
#[ignore]
fn test_de_duration() {
    utils::run_json_test(Lang::DE, utils::build_resources_path("de", "duration.json"));
}

#[test]
#[ignore]
fn test_de_temperature() {
    utils::run_json_test(Lang::DE, utils::build_resources_path("de", "temperature.json"));
}

#[test]
#[ignore]
fn test_de_amount_of_money() {
    utils::run_json_test(Lang::DE, utils::build_resources_path("de", "amount_of_money.json"));
}

#[test]
#[ignore]
fn test_de_datetime() {
    utils::run_json_test(Lang::DE, utils::build_resources_path("de", "datetime.json"));
}