#![allow(nonstandard_style)]

pub mod divsufsort;

fn main() {
    let input = "Lorem ipsum dolor this is just garbage text".as_bytes();
    let mut SA = vec![0 as divsufsort::Idx; input.len()];

    divsufsort::divsufsort(&input[..], &mut SA[..]);
}
