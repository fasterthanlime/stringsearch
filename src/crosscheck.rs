use crate::common::{ABucket, SuffixArray};
use once_cell::sync::Lazy;
use std::{fs::File, io::Write, sync::Mutex};

pub static CROSSCHECK_FILE: Lazy<Mutex<File>> =
    Lazy::new(|| Mutex::new(File::create("crosscheck/rust").unwrap()));

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
