use std::marker::PhantomData;

use pest::iterators::Pair;
use pest_derive::Parser;

use crate::Error;

#[derive(Parser)]
#[grammar = "lf2_object.pest"]
pub struct ObjectDataParser;

/// Function that processes a sub grammar rule.
pub trait SubRuleFnTrait<'f, 'i: 'f> {
    type T: 'f;

    fn call(&self, t: Self::T, pair: Pair<'i, Rule>) -> Result<Self::T, Error<'i>>;
}

/// Function that processes a sub grammar rule.
pub type SubRuleFn<TBuilder> = for<'i> fn(TBuilder, Pair<'i, Rule>) -> Result<TBuilder, Error<'i>>;

impl<'f, 'i: 'f, T> SubRuleFnTrait<'f, 'i> for SubRuleFn<T>
where
    T: 'f,
{
    type T = T;

    fn call(&self, t: T, pair: Pair<'i, Rule>) -> Result<T, Error<'i>> {
        (self)(t, pair)
    }
}

#[derive(Debug)]
pub struct SubRuleWrapper<F, T> {
    f: F,
    marker: PhantomData<T>,
}

impl<F, T> SubRuleWrapper<F, T> {
    pub fn new(f: F) -> Self {
        Self {
            f,
            marker: PhantomData,
        }
    }
}

impl<'f, 'i: 'f, F, T> SubRuleFnTrait<'f, 'i> for SubRuleWrapper<F, T>
where
    F: Fn(T, Pair<'i, Rule>) -> Result<T, Error<'i>>,
    T: 'f,
{
    type T = T;

    fn call(&self, t: T, pair: Pair<'i, Rule>) -> Result<T, Error<'i>> {
        (self.f)(t, pair)
    }
}

impl ObjectDataParser {
    pub fn parse_as_type<'f, 'i: 'f, TBuilder, SubRule>(
        builder: TBuilder,
        pair: Pair<'i, Rule>,
        rule_expected: Rule,
        subrule_fns: impl IntoIterator<Item = &'f SubRule>,
    ) -> Result<TBuilder, Error<'i>>
    where
        TBuilder: 'i,
        SubRule: SubRuleFnTrait<'f, 'i, T = TBuilder> + 'f,
    {
        if pair.as_rule() == rule_expected {
            let pairs = pair.into_inner();
            pairs
                .zip(subrule_fns.into_iter())
                .try_fold(builder, |builder, (pair, subrule_fn)| {
                    subrule_fn.call(builder, pair)
                })
        } else {
            Err(Error::GrammarSingle {
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
        TBuilder: 'i,
    {
        if let Some(value_pair) = tag_pair.clone().into_inner().next() {
            subrule_fn(builder, value_pair)
        } else {
            Err(Error::ValueExpected { tag_pair })
        }
    }
}
