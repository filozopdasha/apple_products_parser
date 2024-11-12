use pest_derive::Parser;
use thiserror::Error;
use pest::error::Error as PestError; 

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

#[derive(Error, Debug)]
#[error("Error while parsing JSON: \n{0}")]
pub struct JsonParserError(#[from] PestError<Rule>);