#![allow(nonstandard_style)]
#![allow(unused_variables)]
#![allow(unused_parens)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(dead_code)]

mod common;
pub mod crosscheck;
mod divsufsort;
mod sssort;
mod trsort;

pub use crate::divsufsort::divsufsort;
pub use common::Idx;
