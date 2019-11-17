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
    fn fuzz_cf86735() {
        sort(include_bytes!("./testdata/crash-cf8673530fdca659e0ddf070b4718b9c0bb504ec"));
    }

    #[test]
    fn fuzz_ce407ad() {
        sort(include_bytes!("./testdata/crash-ce407adf7cf638d3fa89b5637a94355d7d658872"));
    }

    #[test]
    fn fuzz_c792e78() {
        sort(include_bytes!("./testdata/crash-c792e788de61771b6cd65c1aa5670c62e57a33c4"));
    }

    #[test]
    fn fuzz_90b42d1() {
        sort(include_bytes!("./testdata/crash-90b42d1c55ee90a8b004fb9db1853429ceb4c4ba"));
    }

    #[test]
    fn fuzz_8765ef2() {
        sort(include_bytes!("./testdata/crash-8765ef2258178ca027876eab83e01d6d58db9ca0"));
    }

    #[test]
    fn fuzz_4f8c31d() {
        sort(include_bytes!("./testdata/crash-4f8c31dec8c3678a07e0fbacc6bd69e7cc9037fb"));
    }

    #[test]
    fn fuzz_16356e9() {
        sort(include_bytes!("./testdata/crash-16356e91966a827f79e49167170194fc3088a7ab"));
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
