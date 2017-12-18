extern crate rmp_serde;
extern crate rustling;
extern crate rustling_ontology_grammar as grammar;
extern crate rustling_ontology_values;
extern crate serde;
#[macro_use]
extern crate serde_derive;

#[path="src/parser.rs"]
mod parser;

use std::{path, env, fs};
use grammar::Lang;

pub fn train(lang: Lang) {
    println!("cargo:rerun-if-changed=grammar/{}/src/rules.rs", lang.to_string().to_lowercase());
    let out_dir = path::PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut file = fs::File::create(out_dir.join(format!("{}{}", lang.to_string().to_lowercase(), ".rmp"))).unwrap(); 
    let rules = grammar::rules(lang).unwrap();
    let examples =  grammar::examples(lang);
    let model = ::rustling::train::train(&rules, examples, ::parser::FeatureExtractor()).unwrap();
    ::rmp_serde::encode::write(&mut file, &model).unwrap();
}

fn main() {
    for lang in Lang::all() {
        train(lang);
    }
}
