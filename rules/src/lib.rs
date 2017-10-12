//#[macro_use]
extern crate rustling;
extern crate rustling_ontology_moment as moment;
extern crate rustling_ontology_values as values;
extern crate regex;

use std::result;

#[macro_use]
mod macros;

// pub mod de;
// pub mod en;
// pub mod es;
pub mod fr;
// pub mod ko;
// pub mod zh;

macro_rules! lang_enum {
    ([$($lang:ident),*]) => {
        /// Enumerates all language supported for the general purpose ontology.
        #[derive(Copy,Clone,Debug,PartialEq, Eq)]
        pub enum Lang {
            $( $lang, )*
        }

        impl Lang {
            pub fn all() -> Vec<Lang> {
                vec![
                    $( Lang::$lang, )*
                ]
            }
        }

        impl std::str::FromStr for Lang {
            type Err = String;
            fn from_str(it: &str) -> result::Result<Lang, Self::Err> {
                match &*it.to_uppercase() {
                    $( stringify!($lang) => Ok(Lang::$lang),  )*
                    _ => Err(format!("Unknown language {}", it)),
                }
            }
        }

        impl ::std::string::ToString for Lang {
            fn to_string(&self) -> String {
                match self {
                    $( &Lang::$lang => stringify!($lang).to_string(),)*
                }
            }
        }

    }
}

// lang_enum!([DE, EN, ES, FR, KO, ZH]);
lang_enum!([FR]);


macro_rules! lang {
    ($lang:ident, $config:ident, $word_boundaries:ident, $match_boundaries:ident, [$($rule:ident),*], [$($dim:ident),*]) => {
        pub mod $config {
            use values;
            use $lang;
            pub fn rule_set() -> ::rustling::RustlingResult<::rustling::RuleSet<values::Dimension>> {
                let mut b = ::rustling::RuleSetBuilder::new(
                                ::rustling::BoundariesChecker::$word_boundaries(),
                                ::rustling::BoundariesChecker::$match_boundaries());
                $( $lang::$rule(&mut b)?; )*
                Ok(b.build())
            }

            pub fn dims() -> Vec<values::DimensionKind> {
                let mut dims = vec![];
                $( dims.push(values::DimensionKind::$dim); )*
                dims
            }
        }
    }
}

/// Obtain rules for a given language.
pub fn rules(lang: Lang) -> ::rustling::RustlingResult<::rustling::RuleSet<values::Dimension>> {
    match lang {
        // Lang::DE => de_config::rule_set(),
        // Lang::EN => en_config::rule_set(),
        // Lang::ES => es_config::rule_set(),
        Lang::FR => fr_config::rule_set(),
        // Lang::KO => ko_config::rule_set(),
        // Lang::ZH => zh_config::rule_set()
    }
}

/// Obtain dimensions for a given language.
pub fn dims(lang: Lang) -> Vec<values::DimensionKind> {
    match lang {
        // Lang::DE => de_config::dims(),
        // Lang::EN => en_config::dims(),
        // Lang::ES => es_config::dims(),
        Lang::FR => fr_config::dims(),
        // Lang::KO => ko_config::dims(),
        // Lang::ZH => zh_config::dims()
    }
}


// lang!(de, de_config, composed_word_or_detailed, separated_alphanumeric_word, [rules_numbers, rules_time, rules_cycle, rules_duration, rules_temperature, rules_finance], 
//           [Number, Ordinal, Time, Duration, Temperature, AmountOfMoney]);
// lang!(en, en_config, detailed, separated_alphanumeric_word, [rules_numbers, rules_time, rules_cycle, rules_duration, rules_temperature, rules_finance], 
//           [Number, Ordinal, Time, Duration, Temperature, AmountOfMoney]);
// lang!(es, es_config, detailed, separated_alphanumeric_word, [rules_numbers, rules_temperature, rules_cycle, rules_duration, rules_time],
//           [Number, Ordinal, Time, Duration, Temperature]);
lang!(fr, fr_config, detailed, separated_alphanumeric_word, [rules_numbers, rules_time, rules_temperature, rules_cycle, rules_duration, rules_finance],
          [Number, Ordinal, Time, Duration, Temperature, AmountOfMoney]);
// lang!(ko, ko_config, detailed, separated_alphanumeric_word, [rules_numbers, rule_time, rule_temperature, rules_finance, rules_cycle, rules_duration], 
//           [Number, Ordinal, Time, Duration, Temperature, AmountOfMoney]);
// lang!(zh, zh_config, no_check, no_check, [rules_numbers, rules_time, rules_temperature, rules_cycle, rules_duration], [Number, Ordinal, Time, Duration, Temperature]);

