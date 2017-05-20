use rustling_ontology_values::dimension::*;
use rustling;

#[derive(Debug, Hash, Clone, Eq, PartialEq,Serialize,Deserialize)]
pub enum Feat {
  Rules(Vec<rustling::Sym>),
  Grains(Vec<usize>),
}

impl rustling::Feature for Feat {}

pub struct FeatureExtractor();

impl rustling::FeatureExtractor<Dimension, Feat> for FeatureExtractor {
    fn for_parsed_node(&self,
                       node: &rustling::ParsedNode<Dimension>)
                       -> rustling::Input<rustling::RuleId, Feat> {
        self.for_node(&node.root_node)
    }
    fn for_node(&self, node: &rustling::Node<Payload>) -> rustling::Input<rustling::RuleId, Feat> {
        extract_node_features(&node)
    }
}

pub fn extract_node_features(node: &rustling::Node<Payload>) -> rustling::Input<rustling::RuleId, Feat> {
    let grains_feat = node.children.iter().filter_map(|c| c.payload.map(|p| p.0 as usize)).collect::<Vec<_>>();
    let rules_feat = node.children.iter().map({ |child| child.rule_sym }).collect::<Vec<_>>();
    
    let mut features = vec![Feat::Rules(rules_feat)];
    if grains_feat.is_empty() { features.push(Feat::Grains(grains_feat)); }

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
    use ::*;
    use rustling::ParserMatch;
    use rustling_ontology_values::dimension::*;

    #[test]
    fn test_twenty() {
        let parser = build_raw_parser(Lang::EN).unwrap();
        let result = parser
            .parse_with_kind_order("twenty", &[DimensionKind::Number])
            .unwrap();
        assert_eq!(ParserMatch {
                       byte_range: Range(0, 6),
                       char_range: Range(0, 6),
                       value: IntegerValue::new_with_grain(20, 1).unwrap().into(),
                       probalog: 0.0,
                       latent: false,
                   },
                   result[0]);
    }

    #[test]
    fn test_21() {
        let parser = build_raw_parser(Lang::EN).unwrap();
        let result = parser
            .parse_with_kind_order("twenty-one", &[DimensionKind::Number])
            .unwrap();
        assert_eq!(21,
                   IntegerValue::attempt_from(result[0].value.clone())
                       .unwrap()
                       .value);
    }

    #[test]
    fn test_2_1000() {
        let parser = build_raw_parser(Lang::EN).unwrap();
        let result = parser.parse("twenty-one thousands").unwrap();
        assert_eq!(21000,
                   IntegerValue::attempt_from(result[0].value.clone())
                       .unwrap()
                       .value);
    }

    #[test]
    fn test_foobar() {
        let parser = build_raw_parser(Lang::EN).unwrap();
        let result = parser.parse("foobar twenty thousands").unwrap();
        assert_eq!(20000,
                   IntegerValue::attempt_from(result[0].value.clone())
                       .unwrap()
                       .value);
    }
}
