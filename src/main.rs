#![allow(nonstandard_style)]
#![allow(unused_variables)]
#![allow(unused_parens)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::time::Instant;

pub mod common;
pub mod crosscheck;
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

    #[cfg(debug_assertions)]
    println!("{:>20} {}", "C", "Running...");

    let mut SA_c = vec![0 as Idx; input.len()];
    let before_c = Instant::now();
    let c_duration;

    unsafe {
        divsufsort(input.as_ptr(), SA_c.as_mut_ptr(), input.len() as i32);
        c_duration = before_c.elapsed();
        dss_flush();
    }

    #[cfg(debug_assertions)]
    println!("{:>20} {}", "Rust", "Running...");

    let rust_duration = {
        let res = std::panic::catch_unwind(|| {
            let mut SA_rust = vec![0 as Idx; input.len()];
            let before_rust = Instant::now();

            std::thread::spawn(|| loop {
                std::thread::sleep(std::time::Duration::from_millis(500));
                crosscheck::flush();
            });

            divsufsort::divsufsort(&input[..], &mut SA_rust[..]);
            let rust_duration = before_rust.elapsed();
            assert!(SA_c == SA_rust, "suffix arrays should be equal");
            println!(
                "{:>20} c {:?} rust {:?}",
                "Durations", c_duration, rust_duration
            );
        });
        crosscheck::flush();
        res.unwrap()
    };
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

    #[test]
    fn test_divsufsort() {
        let inputs = ["Fool", "Love fool", "You are a love fool"];
    }
}
