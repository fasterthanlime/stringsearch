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

use common::Idx;
use sacabase::SuffixArray;

/// Sort suffixes of `text` and store their lexographic order
/// in the given suffix array `sa`.
/// Will panic if `sa.len()` != `text.len()`
pub fn sort_in_place(text: &[u8], sa: &mut [Idx]) {
    divsufsort::divsufsort(text, sa);
}

//// Sort suffixes
pub fn sort(text: &[u8]) -> sacabase::SuffixArray<Idx> {
    let mut sa = vec![0; text.len()];
    sort_in_place(text, &mut sa);
    sacabase::SuffixArray::new(text, sa)
}

#[cfg(test)]
mod tests {
    #[test]
    fn fuzz1() {
        sort(include_bytes!("./testdata/fuzz1"));
    }

    #[test]
    fn fuzz2() {
        sort(include_bytes!("./testdata/fuzz2"));
    }

    #[test]
    fn fuzz3() {
        sort(include_bytes!("./testdata/fuzz3"));
    }

    #[test]
    fn shruggy() {
        sort(r#"¯\_(ツ)_/¯"#.as_bytes());
    }

    fn sort(s: &[u8]) {
        let sa = super::sort(s);
        sa.verify().unwrap();
    }
}
