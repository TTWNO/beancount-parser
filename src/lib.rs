#![deny(future_incompatible, unsafe_code)]
#![warn(nonstandard_style, rust_2018_idioms, missing_docs, clippy::pedantic)]
#![cfg_attr(test, allow(clippy::needless_pass_by_value))]

//! A rust parsing library for [beancount](https://beancount.github.io/docs/) files
//!
//! At its core, this library provides is a [`Parser`] type that
//! is an iterator over the directives.
//!
//! ## Example
//! ```
//! use beancount_parser::{Date, Directive, Parser, Error};
//!
//! # fn main() -> Result<(), Error> {
//! let beancount = r#"
//! 2022-09-11 * "Coffee beans"
//!   Expenses:Groceries   10 CHF
//!   Assets:Bank
//! "#;
//!
//! let directives: Vec<Directive<'_>> = Parser::new(beancount).collect::<Result<_, _>>()?;
//! let transaction = directives[0].as_transaction().unwrap();
//! assert_eq!(transaction.narration(), Some("Coffee beans"));
//!
//! let first_posting_amount = transaction.postings()[0].amount().unwrap();
//! assert_eq!(first_posting_amount.currency(), "CHF");
//! assert_eq!(first_posting_amount.value().try_into_f64()?, 10.0);
//! # Ok(()) }
//! ```

mod account;
mod amount;
mod date;
mod directive;
mod error;
mod string;
mod transaction;

use crate::directive::directive;

pub use crate::{
    account::{Account, Type},
    amount::{Amount, ConversionError, Expression, Value},
    date::Date,
    directive::Directive,
    error::Error,
    transaction::{Flag, Posting, PriceType, Transaction},
};

use nom::{
    branch::alt,
    character::complete::{line_ending, not_line_ending},
    combinator::{map, opt, value},
    sequence::tuple,
    IResult,
};

/// Parser of a beancount document
///
/// It is an iterator over the beancount directives.
///
/// See the crate documentation for usage example.
pub struct Parser<'a> {
    rest: &'a str,
}

impl<'a> Parser<'a> {
    /// Create a new parser from the beancount string to parse
    #[must_use]
    pub fn new(content: &'a str) -> Self {
        Self { rest: content }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<Directive<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.rest.is_empty() {
            if let Ok((rest, directive)) = next(self.rest) {
                self.rest = rest;
                if let Some(directive) = directive {
                    return Some(Ok(directive));
                }
            } else {
                self.rest = "";
                return Some(Err(Error));
            }
        }
        None
    }
}

fn next(input: &str) -> IResult<&str, Option<Directive<'_>>> {
    alt((
        map(directive, Some),
        value(None, tuple((not_line_ending, opt(line_ending)))),
    ))(input)
}
