extern crate rmp_serde;
extern crate rustling;
extern crate rustling_ontology_grammar as grammar;
extern crate rustling_ontology_values;
extern crate serde;
#[macro_use]
extern crate serde_derive;

#[path="src/parser.rs"]
mod parser;

use std::thread::{self, JoinHandle};
use std::{path, env, fs};
use grammar::Lang;

pub fn train_async(lang: Lang) -> JoinHandle<()> {
    println!("cargo:rerun-if-changed=grammar/{}/src/", lang.to_string().to_lowercase());
    thread::spawn(move || {
        let out_dir = path::PathBuf::from(env::var("OUT_DIR").unwrap());
        let mut file = fs::File::create(out_dir.join(format!("{}{}", lang.to_string().to_lowercase(), ".rmp"))).unwrap(); 
        let rules = grammar::rules(lang).unwrap();
        let examples =  grammar::examples(lang);
        let model = ::rustling::train::train(&rules, examples, ::parser::FeatureExtractor()).unwrap();
        ::rmp_serde::encode::write(&mut file, &model).unwrap();
    })
}

pub fn train_sync(lang: Lang) {
    println!("cargo:rerun-if-changed=grammar/{}/src/", lang.to_string().to_lowercase());
    let out_dir = path::PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut file = fs::File::create(out_dir.join(format!("{}{}", lang.to_string().to_lowercase(), ".rmp"))).unwrap(); 
    let rules = grammar::rules(lang).unwrap();
    let examples =  grammar::examples(lang);
    let model = ::rustling::train::train(&rules, examples, ::parser::FeatureExtractor()).unwrap();
    ::rmp_serde::encode::write(&mut file, &model).unwrap();
}

pub fn train_all_async() {
    let join_handlers: Vec<_> = Lang::all().into_iter().map(|lang| {
        train_async(lang)
    }).collect();

    for join in join_handlers {
        join.join().unwrap();
    }
}

pub fn train_all_sync() {
    for lang in Lang::all() {
        train_sync(lang);
    }
}

fn main() {
    train_all_sync();
}
