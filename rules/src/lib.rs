//#[macro_use]
extern crate rustling;
extern crate rustling_ontology_moment as moment;
extern crate rustling_ontology_values as values;

use std::result;

#[macro_use]
mod macros;

pub mod en;
pub mod fr;
pub mod es;

/// Enumerates all language supported for the general purpose ontology.
#[derive(Copy,Clone,Debug)]
pub enum Lang {
    /// English
    EN,
    /// French
    FR,
    /// Spanish
    ES,
}

impl std::str::FromStr for Lang {
    type Err = String;
    fn from_str(it: &str) -> result::Result<Lang, Self::Err> {
        match &*it.to_lowercase() {
            "en" => Ok(Lang::EN),
            "fr" => Ok(Lang::FR),
            "es" => Ok(Lang::ES),
            _ => Err(format!("Unknown language {}", it)),
        }
    }
}

impl ::std::string::ToString for Lang {
    fn to_string(&self) -> String {
        match self {
            &Lang::EN => "en".to_string(),
            &Lang::FR => "fr".to_string(),
            &Lang::ES => "es".to_string(),
        }
    }
}

macro_rules! lang {
    ($lang:ident, [$($rule:ident),*]) => {
        pub fn $lang() -> ::rustling::RustlingResult<::rustling::RuleSet<values::Dimension>> {
            let mut b = ::rustling::RuleSetBuilder::default();
            $( $lang::$rule(&mut b)?; )*
            Ok(b.build())
        }
    }
}

/// Obtain rules for a given language.
pub fn rules(lang: Lang) -> ::rustling::RustlingResult<::rustling::RuleSet<values::Dimension>> {
    match lang {
        Lang::EN => en(),
        Lang::FR => fr(),
        Lang::ES => es(),
    }
}

lang!(en, [rules_numbers, rules_time, rules_cycle, rules_duration, rules_temperature]);
lang!(es, [rules_numbers, rules_temperature, rules_cycle, rules_duration, rules_time]);
lang!(fr, [rules_numbers, rules_time, rules_temperature, rules_cycle, rules_duration]);
