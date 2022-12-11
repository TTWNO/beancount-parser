//! Types for representing an [`Account`]

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::char,
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

/// Account
///
/// An account has a type (`Assets`, `Liabilities`, `Equity`, `Income` or `Expenses`)
/// and components.
///
/// # Examples
///
/// * `Assets:Liquidity:Cash` (type: `Assets`, components: ["Liquidity", "Cash"]
/// * `Expenses:Groceries` (type: `Assets`, components: ["Groceries"]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Account<'a> {
    type_: Type,
    components: Vec<&'a str>,
}

/// Type of account
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Type {
    /// The assets
    Assets,
    /// The liabilities
    Liabilities,
    /// The equity
    Equity,
    /// Income
    Income,
    /// Expenses
    Expenses,
}

impl<'a> Account<'a> {
    pub(crate) fn new(type_: Type, path: impl IntoIterator<Item = &'a str>) -> Self {
        Self {
            type_,
            components: path.into_iter().collect(),
        }
    }

    /// Returns the type of account
    #[must_use]
    pub fn type_(&self) -> Type {
        self.type_
    }

    /// Returns the components
    #[must_use]
    pub fn components(&self) -> &[&str] {
        self.components.as_ref()
    }
}

pub(crate) fn account(input: &str) -> IResult<&str, Account<'_>> {
    map(
        separated_pair(
            type_,
            char(':'),
            separated_list1(
                char(':'),
                take_while1(|c: char| c.is_alphanumeric() || c == '-'),
            ),
        ),
        |(t, p)| Account::new(t, p),
    )(input)
}

fn type_(input: &str) -> IResult<&str, Type> {
    alt((
        map(tag("Assets"), |_| Type::Assets),
        map(tag("Liabilities"), |_| Type::Liabilities),
        map(tag("Income"), |_| Type::Income),
        map(tag("Expenses"), |_| Type::Expenses),
        map(tag("Equity"), |_| Type::Equity),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest]
    #[case("Assets:MyAccount", Account::new(Type::Assets, ["MyAccount"]))]
    #[case("Liabilities:A:B:C", Account::new(Type::Liabilities, ["A", "B", "C"]))]
    #[case("Income:Foo:Bar12", Account::new(Type::Income, ["Foo", "Bar12"]))]
    #[case("Expenses:3Foo", Account::new(Type::Expenses, ["3Foo"]))]
    #[case("Equity:Foo-Bar", Account::new(Type::Equity, ["Foo-Bar"]))]
    fn valid_account(#[case] input: &str, #[case] expected: Account<'_>) {
        assert_eq!(account(input), Ok(("", expected)));
    }
}
