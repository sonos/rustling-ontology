#[macro_use]
extern crate clap;
extern crate rustling_ontology_json_utils as json_utils;
extern crate rustling_ontology_moment;
extern crate rustling_ontology;
extern crate serde_json;
#[macro_use]
extern crate prettytable;

use std::str::FromStr;
use std::io::Write;

use rustling_ontology::*;
use rustling_ontology_moment::*;
use prettytable::Table;
use json_utils::{PartialUtterance, Utterance, TestOutput, TestAssertion};

fn main() {
    let matches = clap_app!(rustling_cli =>
        (@arg lang: -l --lang default_value[en] "2-letter language code (default to \"en\")")
        (@subcommand parse =>
             (@arg kinds: -k --kinds +takes_value +use_delimiter "kinds, last one wins, coma separated")
             (@arg sentence: +required "Sentence to test")
        )
        (@subcommand play =>
             (@arg kinds: -k --kinds +takes_value +use_delimiter "kinds, last one wins, coma separated")
             (@arg sentence: +required "Sentence to test")
        )
        (@subcommand utterance =>
             (@arg path: -p --path +takes_value "Path to utterances file")
        )
        (@subcommand utterance =>
             (@arg input: -i --input +takes_value "Path to utterances file")
             (@arg output: -o --output +takes_value "Path to test output file")
        )
    ).get_matches();

    let lang = value_t!(matches.value_of("lang"), Lang).unwrap_or_else(|e| e.exit());
    match matches.subcommand() {
        ("parse", Some(matches)) => {
            let kinds = matches
                  .values_of("kinds")
                  .map(|values| {
                           values
                               .map(|s| OutputKind::from_str(s).unwrap())
                               .collect::<Vec<_>>()
                  });
            let sentence = matches.value_of("sentence").unwrap().to_lowercase();
            let parser = build_parser(lang).unwrap();
            let decoder = ResolverContext::new(Interval::starting_at(Moment(Local.ymd(2017, 6, 1).and_hms(5, 00, 0)), Grain::Second));

            let context = ResolverContext::default();
            let entities = if let Some(kinds) = kinds {
                parser.parse_with_kind_order(&*sentence, &decoder, &kinds).unwrap()
            } else {
                parser.parse(&*sentence, &decoder).unwrap()
            };
            let mut table = Table::new();
            table.set_titles(row!["ix", "log(p)", "p", "text", "value"]);
            for (ix, c) in entities.iter().enumerate().rev() {
                let mut hilite = String::new();
                for _ in 0..c.byte_range.0 {
                    hilite.push('_');
                }
                hilite.push_str(&sentence[c.byte_range.0..c.byte_range.1]);
                for _ in c.byte_range.1..sentence.len() {
                    hilite.push('_');
                }
                table.add_row(row![ix,
                                   c.probalog,
                                   f32::exp(c.probalog),
                                   hilite,
                                   format!("{:?}", c.value)]);
            }
            table.printstd();
        }
        ("play", Some(matches)) => {
            let kinds = matches
                .values_of("kinds")
                .map(|values| {
                         values
                             .map(|s| OutputKind::from_str(s).unwrap())
                             .collect()
                     })
                .unwrap_or(OutputKind::all());
            let sentence = matches.value_of("sentence").unwrap().to_lowercase();
            let parser = build_raw_parser(lang).unwrap();
            let decoder = ResolverContext::new(Interval::starting_at(Moment(Local.ymd(2017, 6, 1).and_hms(5, 00, 0)), Grain::Second));

            let context = ResolverContext::default();
            let tagger = CandidateTagger {
                order: &kinds,
                context: &decoder,
                resolve_all_candidates: true,
            };
            let candidates = parser.candidates(&*sentence, &tagger).unwrap();
            let mut table = Table::new();
            table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            table.set_titles(row!["ix", "best", "log(p)", "p", "text", "value", "latent", "rule", "childs"]);

            for (ix, c) in candidates.iter().enumerate().rev() {
                let mut hilite = String::new();
                for _ in 0..c.match_.byte_range.0 {
                    hilite.push('_');
                }
                hilite.push_str(&sentence[c.match_.byte_range.0..c.match_.byte_range.1]);
                for _ in c.match_.byte_range.1..sentence.len() {
                    hilite.push('_');
                }
                table.add_row(row![ix,
                                   if c.tagged { "*" } else { " " },
                                   c.match_.probalog,
                                   f32::exp(c.match_.probalog),
                                   hilite,
                                   c.match_.value.as_ref().map(|v| format!("{:?}", v)).unwrap_or("".into()),
                                   c.node.value.latent(),
                                   parser.resolve_sym(&c.node.root_node.rule_sym).unwrap_or(""),
                                   c.node
                                       .root_node
                                       .children
                                       .iter()
                                       .map(|n| {
                                           let name = parser.resolve_sym(&n.rule_sym).unwrap_or("");
                                                name.chars().take(20).collect::<String>()
                                            })
                                       .collect::<Vec<_>>()
                                       .join(" + ")]);
            }
            table.printstd();
        }
        ("utterance", Some(matches)) => {
            let input = matches.value_of("input").unwrap();
            let utterances: Vec<Utterance> = {
              let file = ::std::fs::File::open(input).unwrap();
              serde_json::from_reader(&file).unwrap()
            };
            let parser = build_parser(lang).unwrap();
            let default_context = Moment(Local.ymd(2017, 6, 1).and_hms(5, 00, 0));
            let output: Vec<TestOutput> = utterances.into_iter()
                .map(|it| {
                  if it.in_grammar {
                      let context = ResolverContext::new(Interval::starting_at(default_context, Grain::Second));
                      let entities = parser.parse(it.phrase.to_lowercase().as_str(), &context).unwrap();
                      let full_match = entities
                        .into_iter()
                        .filter(|entity| entity.byte_range.len() == it.phrase.len())
                        .next();
                      Utterance {
                          phrase: it.phrase,
                          in_grammar: it.in_grammar,
                          context: default_context.clone(),
                          value: full_match.map(|it| it.value.into()),
                      }   
                  } else {
                    Utterance {
                      phrase: it.phrase,
                      in_grammar: it.in_grammar,
                      context: default_context.clone(),
                      value: None,
                    }
                  }
                })
                .collect();
            let mut file = ::std::fs::File::create(path).unwrap();
            serde_json::to_writer_pretty(&file, &utterances).unwrap();
        }
        ("test", Some(matches)) => {
            let path = matches.value_of("path").unwrap();
            let partial_utterances: Vec<PartialUtterance> = {
              let file = ::std::fs::File::open(path).unwrap();
              serde_json::from_reader(&file).unwrap()
            };
            let parser = build_parser(lang).unwrap();
            let default_context = Moment(Local.ymd(2017, 6, 1).and_hms(5, 00, 0));
            let utterances: Vec<Utterance> = partial_utterances.into_iter()
                .map(|it| {
                  if it.in_grammar {
                      let context = ResolverContext::new(Interval::starting_at(default_context, Grain::Second));
                      let entities = parser.parse(it.phrase.to_lowercase().as_str(), &context).unwrap();
                      let full_match = entities
                        .into_iter()
                        .filter(|entity| entity.byte_range.len() == it.phrase.len())
                        .next();
                      Utterance {
                          phrase: it.phrase,
                          in_grammar: it.in_grammar,
                          context: default_context.clone(),
                          value: full_match.map(|it| it.value.into()),
                      }   
                  } else {
                    Utterance {
                      phrase: it.phrase,
                      in_grammar: it.in_grammar,
                      context: default_context.clone(),
                      value: None,
                    }
                  }
                })
                .collect();
            let mut file = ::std::fs::File::create(path).unwrap();
            serde_json::to_writer_pretty(&file, &utterances).unwrap();
        }
        (cmd, _) => panic!("Unknown command {}", cmd),
    }
}
