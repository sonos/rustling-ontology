#[macro_use]
extern crate clap;
extern crate rustling;
extern crate rustling_ontology_grammar as grammar;
extern crate rustling_ontology_values as values;
extern crate rustling_ontology_moment;
#[macro_use]
extern crate prettytable;

use rustling_ontology_moment::*;
use prettytable::Table;
use values::{ParsingContext, ResolverContext};

fn main() {
    let matches = clap_app!(rustling_cli =>
        (@arg lang: -l --lang default_value[en] "2-letter language code (default to \"en\")")
        (@subcommand parse =>
             (@arg kinds: -k --kinds +takes_value +use_delimiter "kinds, last one wins, coma separated")
             (@arg sentence: +required "Sentence to test")
        )
    ).get_matches();
    let lang = value_t!(matches.value_of("lang"), grammar::Lang).unwrap_or_else(|e| e.exit());
    match matches.subcommand() {
        ("parse", Some(matches)) => {
            let sentence = matches.value_of("sentence").unwrap().to_lowercase();
            let decoder = ResolverContext::new(Interval::starting_at(Moment(Local.ymd(2013, 2, 12).and_hms(4, 30, 0)), Grain::Second));
            let rules = grammar::rules(lang).unwrap();
            let matches = rules.apply_all(&*sentence).unwrap();
            let mut table = Table::new();
            table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            table.set_titles(row!["ix", "text", "Dimension", "Output(OutputValue)", "rule", "children"]);
            for (ix, m) in matches.iter().enumerate().rev() {
                let mut hilite = String::new();
                let byte_range = m.root_node.byte_range;
                for _ in 0..byte_range.0 {
                    hilite.push('_');
                }
                hilite.push_str(&sentence[byte_range.0..byte_range.1]);
                for _ in byte_range.1..sentence.len() {
                    hilite.push('_');
                }
                // TODO: Manage to display the XyzValue of a Dimension (e.g. DatetimeValue for
                // Dimension::Datetime) and its fields (e.g. datetime_type for DatetimeValue)
                table.add_row(row![ix,
                                   hilite,
                                   &m.value,
                                   decoder.resolve(&m.value).map(|v| format!("{:?}", v)).unwrap_or("".into()),
                                   rules.resolve_sym(&m.root_node.rule_sym).unwrap_or(""),
                                   m.root_node
                                      .children
                                      .iter()
                                      .map(|n| {
                                          let name = rules.resolve_sym(&n.rule_sym).unwrap_or("");
                                               name.chars().take(20).collect::<String>()
                                           })
                                      .collect::<Vec<_>>()
                                      .join(" + ")
                                   ]);
            }
    table.printstd();
        }
        (cmd, _) => panic!("Unknown command {}", cmd),
    }
}

