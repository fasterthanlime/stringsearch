#![allow(nonstandard_style)]

pub mod common;
pub mod divsufsort;
pub mod sssort;

use common::*;

fn main() {
    let first_arg = std::env::args()
        .skip(1 /* skip our binary's name.. */)
        .next()
        .unwrap();
    let input = std::fs::read(first_arg).unwrap();

    let mut SA = vec![0 as Idx; input.len()];
    divsufsort::divsufsort(&input[..], &mut SA[..]);
}

#[cfg(test)]
mod tests {
    use crate::sssort::ss_isqrt;
    extern "C" {
        fn exported_ss_isqrt(x: i32) -> i32;
    }

    #[test]
    fn test_isqrt() {
        for i in 0..10000 {
            let ours = ss_isqrt(i) as i32;
            let theirs = unsafe { exported_ss_isqrt(i) } as i32;
            assert_eq!(ours, theirs, "for i = {}", i);
        }
    }
}
