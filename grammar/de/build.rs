extern crate rmp_serde;
extern crate rustling;
extern crate rustling_ontology_values;
extern crate rustling_ontology_de_data as lang_data;
extern crate serde;

#[path = "../../src/parser.rs"]
mod parser;

use std::{env, fs, path};

pub fn train_sync(lang: &str) {
    let out_dir = path::PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut file =
        fs::File::create(out_dir.join(format!("{}{}", lang.to_string().to_lowercase(), ".rmp")))
            .unwrap();
    let rules = lang_data::rule_set().unwrap();
    let examples = lang_data::examples();
    let model = rustling::train::train(&rules, examples, parser::FeatureExtractor()).unwrap();
    rmp_serde::encode::write(&mut file, &model).unwrap();
}

fn main() {
    train_sync("de");
}
