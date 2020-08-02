use pest::iterators::Pair;
use pest_derive::Parser;

use crate::Error;

#[derive(Parser)]
#[grammar = "lf2_object.pest"]
pub struct ObjectDataParser;

impl ObjectDataParser {
    pub fn parse_as_type<'f, 'i: 'f, TBuilder>(
        builder: TBuilder,
        pair: Pair<'i, Rule>,
        rule_expected: Rule,
        subrule_fns: &'f [for<'sub_fn> fn(
            TBuilder,
            Pair<'sub_fn, Rule>,
        ) -> Result<TBuilder, Error<'sub_fn>>],
    ) -> Result<TBuilder, Error<'i>> {
        if pair.as_rule() == rule_expected {
            let mut pairs = pair.into_inner();
            subrule_fns.iter().try_fold(builder, |builder, subrule_fn| {
                pairs
                    .next()
                    .ok_or(Error::Grammar {
                        rule_expected: Rule::Header,
                        pair_found: None,
                    })
                    .and_then(|pair| subrule_fn(builder, pair))
            })
        } else {
            Err(Error::Grammar {
                rule_expected,
                pair_found: Some(pair),
            })
        }
    }
}
