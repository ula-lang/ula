#![feature(associated_type_bounds, specialization, test, type_alias_impl_trait)]

#[macro_use]
extern crate derivative;

pub mod ast;
pub mod compilation;
pub mod parser;
pub mod debug;