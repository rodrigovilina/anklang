#![deny(clippy::complexity)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::empty_structs_with_brackets)]
#![deny(clippy::expect_used)]
#![deny(clippy::min_ident_chars)]
#![deny(clippy::panic)]
// #![warn(clippy::unwrap_used)]
//
// #![deny(clippy::restriction)]
// #![allow(clippy::implicit_return)]
// #![allow(clippy::missing_docs_in_private_items)]

pub mod env;
pub mod lexer;
pub mod node;
pub mod parser;
pub mod token;

use {node::Node, token::Token};
