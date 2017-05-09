#[macro_use]
extern crate rustling;
extern crate rustling_ontology_moment as moment;

use std::result;

#[macro_use]
mod macros;
#[allow(dead_code)]
mod helpers;
pub mod dimension;
pub mod output;
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
        pub fn $lang() -> ::rustling::RustlingResult<::rustling::RuleSet<dimension::Dimension>> {
            let mut b = ::rustling::RuleSetBuilder::default();
            $( $lang::$rule(&mut b)?; )*
            $lang::rules_numbers(&mut b)?;
            $lang::rules_time(&mut b)?;
            $lang::rules_temperature(&mut b)?;
            /*
            $lang::rules_finance(&mut b)?;
            */
            Ok(b.build())
        }
    }
}

lang!(en, [rules_numbers, rules_time, rules_temperature, rules_cycle]);
lang!(es, [rules_numbers, rules_time, rules_temperature]);
lang!(fr, [rules_numbers, rules_time, rules_temperature]);
