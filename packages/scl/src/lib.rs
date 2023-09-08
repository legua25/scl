// lib.rs
#![feature(box_patterns, inline_const, negative_impls, auto_traits, try_trait_v2, try_blocks)]

mod parser;
mod values;
mod blobs;

pub use crate::values::{ Id, Value };
pub use crate::blobs::{ Blob };
