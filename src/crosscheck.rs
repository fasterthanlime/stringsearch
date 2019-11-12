use crate::common::{ABucket, BMixBucket, Idx, SuffixArray, ALPHABET_SIZE};
use once_cell::sync::Lazy;
use std::{
    fs::File,
    io::{BufWriter, Write},
    sync::Mutex,
};

pub static CROSSCHECK_FILE: Lazy<Mutex<BufWriter<File>>> =
    Lazy::new(|| Mutex::new(BufWriter::new(File::create("crosscheck/rust").unwrap())));

#[macro_export]
macro_rules! crosscheck {
    ($($arg: expr),*) => {
        {
            use std::io::Write;
            let mut f = crate::crosscheck::CROSSCHECK_FILE.lock().unwrap();
            writeln!(f, $($arg),*);
        }
    };
}

pub fn SA_dump(SA: &SuffixArray, label: &str) {
    crosscheck!(":: {}", label);
    crosscheck!("SA = {:?}", SA.0);
}

pub fn A_dump(A: &ABucket, label: &str) {
    crosscheck!(":: {}", label);
    crosscheck!("A = {:?}", A.0);
}

pub fn BSTAR_dump(B: &mut BMixBucket, label: &str) {
    crosscheck!("{} B* dump:", label);
    for ii in 0..(ALPHABET_SIZE as Idx) {
        for jj in 0..(ALPHABET_SIZE as Idx) {
            crosscheck!("{} B*[{},{}]={}", label, ii, jj, B.bstar()[(ii, jj)]);
        }
    }
}
