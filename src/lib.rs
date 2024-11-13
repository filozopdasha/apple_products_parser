#![doc = include_str!("../docs.md")]

use pest::error::Error as PestError;
use pest_derive::Parser;
use thiserror::Error;


/// This struct defines the parser and associates it with the grammar file.
#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;



/// Custom error type for JSON parsing errors. This uses the `thiserror` crate for easy error definition.
#[derive(Error, Debug)]
#[error("Error while parsing JSON: \n{0}")]
pub struct JsonParserError(#[from] PestError<Rule>);
