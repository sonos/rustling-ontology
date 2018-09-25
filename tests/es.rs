extern crate rustling_ontology;
extern crate rustling_ontology_moment as moment;
extern crate rustling_ontology_json_utils as json_utils;
extern crate serde_json;

mod utils;

use rustling_ontology::Lang;

#[test]
#[ignore]
fn test_es_numbers() {
    utils::run_json_test(Lang::ES, utils::build_resources_path("es", "number.json"));
}

#[test]
#[ignore]
fn test_es_ordinal() {
    utils::run_json_test(Lang::ES, utils::build_resources_path("es", "ordinal.json"));
}

#[test]
#[ignore]
fn test_es_percentage() {
    utils::run_json_test(Lang::ES, utils::build_resources_path("es", "percentage.json"));
}

#[test]
#[ignore]
fn test_es_duration() {
    utils::run_json_test(Lang::ES, utils::build_resources_path("es", "duration.json"));
}

#[test]
#[ignore]
fn test_es_temperature() {
    utils::run_json_test(Lang::ES, utils::build_resources_path("es", "temperature.json"));
}

#[test]
#[ignore]
fn test_es_amount_of_money() {
    utils::run_json_test(Lang::ES, utils::build_resources_path("es", "amount_of_money.json"));
}

#[test]
#[ignore]
fn test_es_datetime() {
    utils::run_json_test(Lang::ES, utils::build_resources_path("es", "datetime.json"));
}