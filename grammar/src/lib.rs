extern crate rustling;
extern crate rustling_ontology_values;
extern crate rustling_ontology_de as de;
extern crate rustling_ontology_en as en;
extern crate rustling_ontology_es as es;
extern crate rustling_ontology_fr as fr;
extern crate rustling_ontology_ja as ja;
extern crate rustling_ontology_ko as ko;
extern crate rustling_ontology_zh as zh;

use std::result;

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

lang_enum!([DE, EN, ES, FR, JA, KO, ZH]);


/// Obtain rules for a given language.
pub fn rules(lang: Lang) -> ::rustling::RustlingResult<::rustling::RuleSet<rustling_ontology_values::Dimension>> {
    match lang {
        Lang::DE => de::rule_set(),
        Lang::EN => en::rule_set(),
        Lang::ES => es::rule_set(),
        Lang::FR => fr::rule_set(),
        Lang::JA => ja::rule_set(),
        Lang::KO => ko::rule_set(),
        Lang::ZH => zh::rule_set(),
    }
}

/// Obtain dimensions for a given language.
pub fn dims(lang: Lang) -> Vec<rustling_ontology_values::DimensionKind> {
    match lang {
        Lang::DE => de::dims(),
        Lang::EN => en::dims(),
        Lang::ES => es::dims(),
        Lang::FR => fr::dims(),
        Lang::JA => ja::dims(),
        Lang::KO => ko::dims(),
        Lang::ZH => zh::dims(),
    }
}

/// Obtain examples for a given language.
pub fn examples(lang: Lang) -> Vec<::rustling::train::Example<rustling_ontology_values::Dimension>> {
    match lang {
        Lang::DE => de::examples(),
        Lang::EN => en::examples(),
        Lang::ES => es::examples(),
        Lang::FR => fr::examples(),
        Lang::JA => ja::examples(),
        Lang::KO => ko::examples(),
        Lang::ZH => zh::examples(),
    }
}