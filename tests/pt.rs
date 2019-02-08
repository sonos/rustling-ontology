extern crate rustling_ontology;
extern crate rustling_ontology_moment as moment;
extern crate rustling_ontology_json_utils as json_utils;
extern crate serde_json;

mod utils;

use rustling_ontology::Lang;

#[test]
#[ignore]
fn test_pt_numbers() {
    utils::run_json_test(Lang::PT, utils::build_resources_path("pt", "number.json"));
}

#[test]
#[ignore]
fn test_pt_ordinal() {
    utils::run_json_test(Lang::PT, utils::build_resources_path("pt", "ordinal.json"));
}

#[test]
#[ignore]
fn test_pt_percentage() {
    utils::run_json_test(Lang::PT, utils::build_resources_path("pt", "percentage.json"));
}

#[test]
#[ignore]
fn test_pt_duration() {
    utils::run_json_test(Lang::PT, utils::build_resources_path("pt", "duration.json"));
}

#[test]
#[ignore]
fn test_pt_temperature() {
    utils::run_json_test(Lang::PT, utils::build_resources_path("pt", "temperature.json"));
}

#[test]
#[ignore]
fn test_pt_amount_of_money() {
    utils::run_json_test(Lang::PT, utils::build_resources_path("pt", "amount_of_money.json"));
}

#[test]
#[ignore]
fn test_pt_datetime() {
    utils::run_json_test(Lang::PT, utils::build_resources_path("pt", "datetime.json"));
}