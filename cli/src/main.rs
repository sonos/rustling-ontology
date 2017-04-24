#[macro_use]
extern crate clap;
extern crate rustling_ontology;
#[macro_use]
extern crate prettytable;

use rustling_ontology::*;
use prettytable::Table;
use ::std::cmp::min;

fn main() {
    let matches = clap_app!(rustling_cli =>
        (@arg lang: -l --lang default_value[en] "2-letter language code (default to \"en\")")
        (@subcommand parse =>
             (@arg sentence: +required "Sentence to test")
        )
    )
        .get_matches();

    let lang = value_t!(matches.value_of("lang"), Lang).unwrap_or_else(|e| e.exit());
    match matches.subcommand() {
        ("parse", Some(matches)) => {
            let sentence = matches.value_of("sentence").unwrap().to_lowercase();
            let parser = build_parser(lang).unwrap();
            let candidates = parser.candidates(&*sentence, |_| Some(12)).unwrap();
            let mut table = Table::new();
            table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            table.set_titles(row!["ix", "best", "log(p)", "p", "text", "dim", "rule", "childs"]);
            for (ix, c) in candidates.iter().enumerate().rev() {
                let mut hilite = String::new();
                for _ in 0..c.1.range.0 {
                    hilite.push('_');
                }
                hilite.push_str(&sentence[c.1.range.0..c.1.range.1]);
                for _ in c.1.range.1..sentence.len() {
                    hilite.push('_');
                }
                table.add_row(row![ix,
                                   if c.3 { "*" } else { " " },
                                   c.1.probalog,
                                   f32::exp(c.1.probalog),
                                   hilite,
                                   format!("{}", c.1.value),
                                   c.0.root_node.rule_name,
                                   c.0
                                       .root_node
                                       .children
                                       .iter()
                                       .map(|n| {
                                           let max_length = min(20, n.rule_name.len());
                                           &n.rule_name[..max_length]
                                       })
                                       .collect::<Vec<_>>()
                                       .join(" + ")]);
            }
            table.printstd();
        }
        (cmd, _) => panic!("Unknown command {}", cmd),
    }
}
