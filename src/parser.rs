use rules::dimension::Dimension;
use rustling;

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct Feat(Vec<rustling::Sym>);
impl rustling::Feature for Feat {}

pub struct FeatureExtractor();

impl rustling::FeatureExtractor<Dimension, Feat> for FeatureExtractor {
    fn for_parsed_node(&self,
                       node: &rustling::ParsedNode<Dimension>)
                       -> rustling::Input<rustling::RuleId, Feat> {
        self.for_node(&node.root_node)
    }
    fn for_node(&self, node: &rustling::Node) -> rustling::Input<rustling::RuleId, Feat> {
        extract_node_features(&node)
    }
}

pub fn extract_node_features(node: &rustling::Node) -> rustling::Input<rustling::RuleId, Feat> {
    let features = vec![Feat(node.children
                                 .iter()
                                 .map({
                                          |child| child.rule_sym
                                      })
                                 .collect())];

    let children_features = node.children
        .iter()
        .map({
                 |child| extract_node_features(child)
             })
        .collect();

    rustling::Input {
        classifier_id: rustling::RuleId(node.rule_sym),
        features: features,
        children: children_features,
    }
}

#[cfg(test)]
mod tests {
    use rustling::*;
    use {build_parser, Lang};
    use dimension::*;
    use rustling::ParserMatch;

    #[test]
    fn test_twenty() {
        let parser = build_parser(Lang::EN).unwrap();
        let result = parser.parse("twenty").unwrap();
        assert_eq!(vec![ParserMatch {
                            range: Range(0, 6),
                            value: IntegerValue::new_with_grain(20, 1).unwrap().into(),
                            probalog: 0.0,
                        }],
                   result);
    }

    #[test]
    fn test_21() {
        let parser = build_parser(Lang::EN).unwrap();
        let result = parser.parse("twenty-one").unwrap();
        assert_eq!(1, result.len());
        assert_eq!(21,
                   IntegerValue::attempt_from(result[0].value.clone())
                       .unwrap()
                       .value);
    }

    #[test]
    fn test_2_1000() {
        let parser = build_parser(Lang::EN).unwrap();
        let result = parser.parse("twenty-one thousands").unwrap();
        assert_eq!(21000,
                   IntegerValue::attempt_from(result[0].value.clone())
                       .unwrap()
                       .value);
    }

    #[test]
    fn test_foobar() {
        let parser = build_parser(Lang::EN).unwrap();
        let result = parser.parse("foobar twenty thousands").unwrap();
        assert_eq!(20000,
                   IntegerValue::attempt_from(result[0].value.clone())
                       .unwrap()
                       .value);
    }
}
