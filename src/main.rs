#![allow(nonstandard_style)]

pub mod common;
pub mod divsufsort;
pub mod sssort;

use common::*;

extern "C" {
    fn exported_ss_isqrt(x: i32) -> i32;
}

fn main() {
    for x in (10000..20000).step_by(111) {
        println!(
            "isqrt({}) = {} <=> {}",
            x,
            sssort::ss_isqrt(x as Idx),
            unsafe { exported_ss_isqrt(x as i32) }
        );
    }

    let first_arg = std::env::args()
        .skip(1 /* skip our binary's name.. */)
        .next()
        .unwrap();
    let input = std::fs::read(first_arg).unwrap();

    let mut SA = vec![0 as Idx; input.len()];
    divsufsort::divsufsort(&input[..], &mut SA[..]);
}
