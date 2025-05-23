/// Abstract Syntax Tree (AST) type definitions for the Grost DSL.
/// 
/// This module contains all the AST node types that represent the semantic
/// structure of parsed Grost DSL documents. These types are produced by
/// converting the Concrete Syntax Tree (CST) from the pest parser.
pub mod ast;

/// Parser implementation for the Grost DSL using the [`pest`].
/// 
/// This module provides the parser that converts Grost DSL source text into
/// a Concrete Syntax Tree (CST), which can then be transformed into the AST
/// types defined in the [`ast`] module.
pub mod parser;
