#![allow(nonstandard_style)]
#![allow(unused)]

pub mod common;
pub mod divsufsort;
pub mod sssort;
pub mod trsort;

use common::*;

fn main() {
    better_panic::install();

    extern "C" {
        fn divsufsort(T: *const u8, SA: *mut i32, n: i32) -> i32;
        fn dss_flush();
    }

    let first_arg = std::env::args()
        .skip(1 /* skip our binary's name.. */)
        .next()
        .unwrap_or(
            std::path::PathBuf::from("testdata")
                .join("input.txt")
                .to_string_lossy()
                .into(),
        );
    let input = std::fs::read(first_arg).unwrap();

    println!();
    println!("================= C =================");
    unsafe {
        let mut SA = vec![0 as Idx; input.len()];
        divsufsort(input.as_ptr(), SA.as_mut_ptr(), input.len() as i32);
        dss_flush();
    }

    println!();
    println!("================ Rust ===============");
    {
        let mut SA = vec![0 as Idx; input.len()];
        divsufsort::divsufsort(&input[..], &mut SA[..]);
    }

    println!();
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
