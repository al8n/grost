

/// A parser for the Grost DSL
#[derive(pest_derive::Parser)]
#[grammar = "../grost.pest"]
pub struct GrostParser;
