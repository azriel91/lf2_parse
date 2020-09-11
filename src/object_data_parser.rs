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
            let pairs = pair.into_inner();
            pairs
                .zip(subrule_fns.into_iter())
                .try_fold(builder, |builder, (pair, subrule_fn)| {
                    subrule_fn(builder, pair)
                })
        } else {
            Err(Error::Grammar {
                rule_expected,
                pair_found: Some(pair),
            })
        }
    }

    pub fn parse_value<'i, TBuilder>(
        builder: TBuilder,
        tag_pair: Pair<'i, Rule>,
        subrule_fn: SubRuleFn<TBuilder>,
    ) -> Result<TBuilder, Error<'i>>
    where
        TBuilder: 'static,
    {
        if let Some(value_pair) = tag_pair.into_inner().next() {
            subrule_fn(builder, value_pair)
        } else {
            Err(Error::ValueExpected { tag_pair })
        }
    }
}
