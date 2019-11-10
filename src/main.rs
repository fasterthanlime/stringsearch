#![allow(nonstandard_style)]

pub mod divsufsort;

fn main() {
    let first_arg = std::env::args()
        .skip(1 /* skip our binary's name.. */)
        .next()
        .unwrap();
    let input = std::fs::read(first_arg).unwrap();

    let mut SA = vec![0 as divsufsort::Idx; input.len()];
    divsufsort::divsufsort(&input[..], &mut SA[..]);
}
