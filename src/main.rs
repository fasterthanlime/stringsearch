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
        for i in 0..(i16::max_value() as i32) {
            let ours = ss_isqrt(i) as i32;
            let theirs = unsafe { exported_ss_isqrt(i) } as i32;
            assert_eq!(ours, theirs, "for i = {}", i);
        }

        for i in (0..i32::max_value()).step_by(111) {
            let ours = ss_isqrt(i) as i32;
            let theirs = unsafe { exported_ss_isqrt(i) } as i32;
            assert_eq!(ours, theirs, "for i = {}", i);
        }

        for i in (0..i32::max_value()).step_by(317) {
            let ours = ss_isqrt(i) as i32;
            let theirs = unsafe { exported_ss_isqrt(i) } as i32;
            assert_eq!(ours, theirs, "for i = {}", i);
        }
    }
}
