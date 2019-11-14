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

#[cfg(test)]
mod tests {
    #[test]
    fn fuzz1() {
        sort(include_bytes!("./testdata/fuzz1"));
    }

    #[test]
    fn shruggy() {
        sort(r#"¯\_(ツ)_/¯"#);
    }

    fn sort<T>(s: T)
    where
        T: AsRef<[u8]>,
    {
        let s = s.as_ref();
        let mut SA = vec![0; s.len()];
        super::divsufsort(s, &mut SA[..]);
    }
}
