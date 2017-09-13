use std::cmp::{PartialOrd, Ordering};
use rustling::{ParserMatch, ParsedNode, Candidate, MaxElementTagger, Value, Range};
use rustling_ontology_values::ParsingContext;
use rustling_ontology_values::dimension::{Dimension};
use rustling_ontology_values::output::OutputKind;

pub struct CandidateTagger<'a, C: ParsingContext<Dimension> + 'a> {
    pub order: &'a [OutputKind],
    pub context: &'a C,
    pub resolve_all_candidates: bool,
}


impl<'a, C: ParsingContext<Dimension>> MaxElementTagger<Dimension> for CandidateTagger<'a, C> {
    type O = Option<C::O>;
    fn tag(&self, 
            candidates: Vec<(ParsedNode<Dimension>, ParserMatch<Dimension>)>) -> Vec<Candidate<Dimension, Option<C::O>>> {
        let order = self.order.iter().map(|o| o.to_dim()).collect::<Vec<_>>();

        let mut candidates = candidates.into_iter()
            .filter_map(|(pn, pm)| {
                order
                    .iter()
                    .rev()
                    .position(|k| *k == pn.value.kind())
                    .map(|prio| (pn, pm, prio))
            })
            .collect::<Vec<_>>();

        candidates.sort_by(|a, b|{
            a.1.byte_range.len().cmp(&b.1.byte_range.len())
                .then_with(|| {
                    a.1.byte_range.0.cmp(&b.1.byte_range.0)
                })
                .then_with(|| {
                    a.2.cmp(&b.2)
                })
                .then_with(|| {
                    if a.1.value.kind() == b.1.value.kind() {
                        a.1.probalog
                            .partial_cmp(&b.1.probalog)
                            .unwrap_or(Ordering::Equal)
                    } else {
                        Ordering::Equal
                    }
                })
                .then_with(|| {
                    b.1.parsing_tree_height.cmp(&a.1.parsing_tree_height)
                })
                .then_with(|| {
                    b.1.parsing_tree_num_nodes.cmp(&a.1.parsing_tree_num_nodes)
                })
        });

        let mut selected_ranges: Vec<Range> = vec![];

        candidates.into_iter().rev().map(|c| {
            if selected_ranges.iter().all(|a| a.is_disjoint(&c.1.byte_range)) {
                let resolved_value = self.context.resolve(&c.1.value);
                if resolved_value.is_some() {
                    selected_ranges.push(c.1.byte_range);
                    
                    return Candidate {
                        node: c.0,
                        match_:  ParserMatch { 
                            byte_range: c.1.byte_range, 
                            char_range: c.1.char_range,
                            parsing_tree_height: c.1.parsing_tree_height,
                            parsing_tree_num_nodes: c.1.parsing_tree_num_nodes,
                            value: resolved_value, 
                            probalog: c.1.probalog, 
                            latent: c.1.latent 
                        }, 
                        tagged: true 
                    }
                }
            }
            let resolved_value = if self.resolve_all_candidates {
                self.context.resolve(&c.1.value)
            } else {
                None
            };
            Candidate {
                node: c.0,
                match_:  ParserMatch { 
                    byte_range: c.1.byte_range, 
                    char_range: c.1.char_range,
                    parsing_tree_height: c.1.parsing_tree_height,
                    parsing_tree_num_nodes: c.1.parsing_tree_num_nodes,
                    value: resolved_value, 
                    probalog: c.1.probalog, 
                    latent: c.1.latent 
                }, 
                tagged: false 
            }
        })
        .collect()
    }
}