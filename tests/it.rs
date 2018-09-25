extern crate rustling_ontology;
extern crate rustling_ontology_moment as moment;
extern crate rustling_ontology_json_utils as json_utils;
extern crate serde_json;

mod utils;

use rustling_ontology::Lang;

#[test]
#[ignore]
fn test_it_numbers() {
    utils::run_json_test(Lang::IT, utils::build_resources_path("it", "number.json"));
}

#[test]
#[ignore]
fn test_it_ordinal() {
    utils::run_json_test(Lang::IT, utils::build_resources_path("it", "ordinal.json"));
}

#[test]
#[ignore]
fn test_it_percentage() {
    utils::run_json_test(Lang::IT, utils::build_resources_path("it", "percentage.json"));
}

#[test]
#[ignore]
fn test_it_duration() {
    utils::run_json_test(Lang::IT, utils::build_resources_path("it", "duration.json"));
}

#[test]
#[ignore]
fn test_it_temperature() {
    utils::run_json_test(Lang::IT, utils::build_resources_path("it", "temperature.json"));
}

#[test]
#[ignore]
fn test_it_amount_of_money() {
    utils::run_json_test(Lang::IT, utils::build_resources_path("it", "amount_of_money.json"));
}

#[test]
#[ignore]
fn test_it_datetime() {
    utils::run_json_test(Lang::FR, utils::build_resources_path("it", "datetime.json"));
}