#[cfg(feature = "crosscheck")]
use once_cell::sync::Lazy;

use crate::common::{ABucket, BMixBucket, Idx, SuffixArray, ALPHABET_SIZE};
use std::{
    fs::File,
    io::{BufWriter, Write},
    sync::Mutex,
};

#[cfg(feature = "crosscheck")]
pub static CROSSCHECK_FILE: Lazy<Mutex<BufWriter<File>>> = Lazy::new(|| {
    std::fs::create_dir_all("crosscheck").unwrap();
    Mutex::new(BufWriter::new(File::create("crosscheck/rust").unwrap()))
});

#[macro_export]
macro_rules! crosscheck {
    ($($arg: expr),*) => {
        #[cfg(feature = "crosscheck")]
        {
            use std::io::Write;
            let mut f = crate::crosscheck::CROSSCHECK_FILE.lock().unwrap();
            writeln!(f, $($arg),*).unwrap();
        }
    };
}

pub fn flush() {
    #[cfg(feature = "crosscheck")]
    {
        let mut f = crate::crosscheck::CROSSCHECK_FILE.lock().unwrap();
        f.flush().unwrap();
    }
}

pub fn SA_dump(SA: &SuffixArray, label: &str) {
    #[cfg(feature = "crosscheck")]
    {
        use std::io::Write;
        let mut f = crate::crosscheck::CROSSCHECK_FILE.lock().unwrap();

        writeln!(f, ":: {}", label).unwrap();
        for i in 0..SA.0.len() {
            write!(f, "{} ", SA.0[i]).unwrap();
            if (i + 1) % 25 == 0 {
                writeln!(f).unwrap();
            }
        }
        writeln!(f).unwrap();
    }
}

pub fn A_dump(A: &ABucket, label: &str) {
    #[cfg(feature = "crosscheck")]
    {
        crosscheck!(":: {}", label);
        crosscheck!("A = {:?}", A.0);
    }
}

pub fn BSTAR_dump(B: &mut BMixBucket, label: &str) {
    #[cfg(feature = "crosscheck")]
    {
        use std::io::Write;
        let mut f = crate::crosscheck::CROSSCHECK_FILE.lock().unwrap();

        writeln!(f, "{} B* dump:", label).unwrap();
        for ii in 0..(ALPHABET_SIZE as Idx) {
            for jj in 0..(ALPHABET_SIZE as Idx) {
                writeln!(f, "{} B*[{},{}]={}", label, ii, jj, B.bstar()[(ii, jj)]).unwrap();
            }
        }
    }
}
