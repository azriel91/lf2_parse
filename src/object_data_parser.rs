use pest::iterators::Pair;
use pest_derive::Parser;

use crate::Error;

#[derive(Parser)]
#[grammar = "lf2_object.pest"]
pub struct ObjectDataParser;

/// Function that processes a sub grammar rule.
pub type SubRuleFn<TBuilder> = for<'i> fn(TBuilder, Pair<'i, Rule>) -> Result<TBuilder, Error<'i>>;

impl ObjectDataParser {
    pub fn parse_as_type<'f, 'i: 'f, TBuilder>(
        builder: TBuilder,
        pair: Pair<'i, Rule>,
        rule_expected: Rule,
        subrule_fns: impl IntoIterator<Item = &'f SubRuleFn<TBuilder>>,
    ) -> Result<TBuilder, Error<'i>>
    where
        TBuilder: 'static,
    {
        if pair.as_rule() == rule_expected {
            let mut pairs = pair.into_inner();
            subrule_fns
                .into_iter()
                .try_fold(builder, |builder, subrule_fn| {
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
