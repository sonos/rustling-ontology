#[macro_use]
extern crate clap;
extern crate rustling_ontology_json_utils as json_utils;
extern crate rustling_ontology_moment;
extern crate rustling_ontology;
extern crate serde_json;
#[macro_use]
extern crate prettytable;

use std::str::FromStr;

use rustling_ontology::*;
use prettytable::Table;
use json_utils::{PartialUtterance, Utterance, TestOutput, TestAssertion, SlotValue};

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
            (@arg force: -f --force "if the value should be recomputed")
            (@arg path: -p --path +takes_value "Path to utterances file")
        )
        (@subcommand test =>
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
            
            let context = ResolverContext::default();
            let entities = if let Some(kinds) = kinds {
                parser.parse_with_kind_order(&*sentence, &context, &kinds).unwrap()
            } else {
                parser.parse(&*sentence, &context).unwrap()
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

            let context = ResolverContext::default();
            
            let tagger = CandidateTagger {
                order: &kinds,
                context: &context,
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
            let path = matches.value_of("path").unwrap();
            let force_resolution = matches.is_present("force");

            let partial_utterances: Vec<PartialUtterance> = {
              let file = ::std::fs::File::open(path).unwrap();
              serde_json::from_reader(&file).unwrap()
            };
            let parser = build_parser(lang).unwrap();
            let default_context = Moment(Local.ymd(2017, 6, 1).and_hms(5, 00, 0));
            let utterances: Vec<Utterance> = partial_utterances.into_iter()
                .map(|it| {
                  if it.keep() && (it.value.is_none() || force_resolution) {
                      let context = ResolverContext::new(Interval::starting_at(default_context, Grain::Second));
                      let entities = parser.parse(it.phrase.to_lowercase().as_str(), &context).unwrap();
                      let full_match = entities
                        .into_iter()
                        .filter(|entity| entity.byte_range.len() == it.phrase.len())
                        .next();
                      Utterance {
                          phrase: it.phrase,
                          in_grammar: it.in_grammar,
                          skip_rustling: it.skip_rustling,
                          translation: it.translation,
                          context: default_context.clone(),
                          value: full_match.map(|it| it.value.into()),
                      }   
                  } else {
                    Utterance {
                      phrase: it.phrase,
                      in_grammar: it.in_grammar,
                      skip_rustling: it.skip_rustling,
                      translation: it.translation,
                      context: default_context.clone(),
                      value: it.value,
                    }
                  }
                })
                .collect();
            let mut file = ::std::fs::File::create(path).unwrap();
            serde_json::to_writer_pretty(&file, &utterances).unwrap();
        }
        ("test", Some(matches)) => {
            let input_path = matches.value_of("input").unwrap();
            let output_path = matches.value_of("output").unwrap();
            let utterances: Vec<Utterance> = {
              let file = ::std::fs::File::open(input_path).map_err(|e| format!("Could not open input file at path: {}, with error {}", input_path, e)).unwrap();;
              serde_json::from_reader(&file).unwrap()
            };
            let parser = build_parser(lang).unwrap();
            let default_context = Moment(Local.ymd(2017, 6, 1).and_hms(5, 00, 0));
            
            let output: Vec<TestOutput> = utterances.into_iter()
                .map(|utterance| {
                  if utterance.keep() {
                      let context = ResolverContext::new(Interval::starting_at(default_context, Grain::Second));
                      let entities = parser.parse(utterance.phrase.to_lowercase().as_str(), &context).unwrap();
                      let assertion = if entities.len() == 1 {
                         let entity = entities.first();
                         match (entity, utterance.value) {
                            (Some(entity), Some(ref expected_value)) => {
                              let entity_value = SlotValue::from(entity.value.clone());
                              if entity.byte_range.len() == utterance.phrase.len() {
                                  if entity_value == expected_value.clone() {
                                      TestAssertion::Success(Some(expected_value.clone()))
                                  } else {
                                      TestAssertion::Failed {
                                          expected: vec![expected_value.clone()],
                                          found: vec![entity_value],
                                          reason: "Entities are not equal".to_string(),
                                      }
                                  }
                              } else {
                                TestAssertion::Failed {
                                    expected: vec![expected_value.clone()],
                                    found: vec![entity_value],
                                    reason: "An entity was found but it doesn't match the full utterance".to_string(),
                                }
                              }
                            }
                            (None, None) => TestAssertion::Failed { 
                                expected: vec![],
                                found: vec![], 
                                reason: "At least one entity should be found".to_string()
                            },
                          (entity, utterance_value) => {
                            let entity: Vec<SlotValue> = entity.into_iter().map(|it| it.clone().value.into()).collect();
                            let value: Vec<_> = utterance_value.into_iter().collect();
                            TestAssertion::Failed {
                              expected: value, 
                              found: entity,
                              reason: "No entity or more than one entity is found".to_string(),
                            }
                          }
                        }
                      } else {
                          let entities: Vec<SlotValue> = entities.into_iter().map(|it| it.value.into()).collect();
                          let value: Vec<_> = utterance.value.into_iter().collect();
                          TestAssertion::Failed {
                            expected: value,
                            found: entities,
                            reason: "No entity or more than one entity is found".to_string(),
                          }
                      };
                      TestOutput {
                          phrase: utterance.phrase,
                          in_grammar: utterance.in_grammar,
                          skip_rustling: utterance.skip_rustling,
                          context: utterance.context,
                          translation: utterance.translation,
                          output: assertion,
                      }   
                  } else {
                    TestOutput {
                      phrase: utterance.phrase,
                      in_grammar: utterance.in_grammar,
                      skip_rustling: utterance.skip_rustling,
                      context: utterance.context,
                      translation: utterance.translation,
                      output: TestAssertion::Success(None),
                    }
                  }
                })
                .collect();
            let total_test = output.len();
            let failed_test = output.iter().filter(|it| it.output.is_failed()).collect::<Vec<_>>().len();
            let mut file = ::std::fs::File::create(output_path).map_err(|e| format!("Could not create output file at path: {} with error {}", output_path, e)).unwrap();
            serde_json::to_writer_pretty(&file, &output).unwrap();
            println!("Total: {:?} | {:?} tests fail", total_test, failed_test);
        }
        (cmd, _) => panic!("Unknown command {}", cmd),
    }
}
