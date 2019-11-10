#![allow(nonstandard_style)]
#![allow(unused)]

use crate::common::*;
use crate::sssort;

pub fn divsufsort(T: &[Char], SA: &mut [Idx]) {
    if T.len() != SA.len() {
        panic!("divsufsort: T and SA arguments should have same length");
    }

    let n = T.len();
    if n >= MAX_INPUT_SIZE {
        // This one ought to not be a panic, maybe?
        panic!(
            "divsufsort: input too large (max input size: {})",
            MAX_INPUT_SIZE
        )
    }

    // short T cases
    match n {
        0 => return,
        1 => {
            SA[0] = 0;
            return;
        }
        2 => {
            SA.copy_from_slice(if T[0] < T[1] { &[0, 1] } else { &[1, 0] });
            return;
        }
        _ => { /* continue */ }
    }

    let T = Text(T);
    let mut SA = SuffixArray(SA);

    // Suffixsort.
    let _res = sort_typeBstar(&T, &mut SA);
    eprintln!("done enumerating");
}

struct SortTypeBstarResult {
    A: ABucket,
    B: BMixBucket,
    m: Idx,
}

fn sort_typeBstar(T: &Text, SA: &mut SuffixArray) -> SortTypeBstarResult {
    let n = T.len();

    // Initialize bucket arrays
    let A: Vec<Idx> = vec![0; BUCKET_A_SIZE];
    let mut A = ABucket(A);

    let B: Vec<Idx> = vec![0; BUCKET_B_SIZE];
    let mut B = BMixBucket(B);

    // temps
    let mut c0: Idx;
    let mut c1: Idx;
    let mut i: Idx;
    let mut j: Idx;
    let mut m: Idx;
    let mut t: Idx;

    // Count the number of occurences of the first one or two characters of each
    // type A, B and B* suffix. Moreover, store the beginning position of all
    // type B* suffixes into the array SA.
    i = n - 1;
    m = n;
    c0 = T.get(n - 1);

    while 0 <= i {
        // type A suffix (originally do..while)
        loop {
            c1 = c0;
            A[c1] += 1;

            // original loop condition
            i -= 1;
            if !(0 <= i) {
                break;
            }

            c0 = T.get(i);
            if !(c0 >= c1) {
                break;
            }
        }

        if 0 <= i {
            // type B* suffix
            B.bstar()[(c0, c1)] += 1;

            m -= 1;
            SA[m] = i;

            // type B suffix

            // init
            i -= 1;
            c1 = c0;

            loop {
                // cond
                if i < 0 {
                    break;
                }
                c0 = T.get(i);
                if c0 > c1 {
                    break;
                }

                // body
                B.b()[(c0, c1)] += 1;

                // iter
                i -= 1;
                c1 = c0;
            }
        }
    }
    m = n - m;

    // Note: A type B* suffix is lexicographically smaller than a type B suffix
    // that beings with the same first two characters.

    // Calculate the index of start/end point of each bucket.
    {
        i = 0;
        j = 0;
        for c0 in 0..(ALPHABET_SIZE as Idx) {
            // body
            t = i + A[c0];
            A[c0] = i + j; // start point
            i = t + B.b()[(c0, c0)];

            for c1 in (c0 + 1)..(ALPHABET_SIZE as Idx) {
                j += B.bstar()[(c0, c1)];
                B.bstar()[(c0, c1)] = j; // end point
                i += B.b()[(c0, c1)];
            }
        }
    }

    // TODO: rest of sort..

    if (0 < m) {
        // Sort the type B* suffixes by their first two characters
        let PAb = n - m;
        let ISAb = m;

        for i in ((m - 2)..=0).rev() {
            t = SA[PAb + i];
            c0 = T.get(t);
            c1 = T.get(t + 1);

            B.bstar()[(c0, c1)] -= 1;
            SA[B.bstar()[(c0, c1)]] = i;
        }
        t = SA[PAb + m - 1];
        c0 = T.get(t);
        c1 = T.get(t + 1);
        B.bstar()[(c0, c1)] = m - 1;
        SA[B.bstar()[(c0, c1)]] = m - 1;

        // Sort the type B* substrings using sssort.
        let buf = m;
        let bufsize = n - (2 * m);

        // init (outer)
        c0 = ALPHABET_SIZE as Idx - 2;
        j = m;
        while 0 < j {
            // init (inner)
            c1 = ALPHABET_SIZE as Idx - 1;
            i = B.bstar()[(c0, c1)];
            if (i < (j - i)) {
                sssort::sssort(T, SA, PAb, i, j, buf, bufsize, 2, n, SA[i] == (m - 1));
            }

            // iter (outer)
            c0 -= 1;
        }
    }

    for (i, &v) in A.0.iter().enumerate() {
        if v == 0 {
            continue;
        }
        let c = std::char::from_u32(i as u32).unwrap();
        eprintln!("A[{:?}] = {}", c, v);
    }

    for (i, &v) in B.0.iter().enumerate() {
        if v == 0 {
            continue;
        }
        eprintln!("B[{}] = {}", i, v);
    }

    for (i, &v) in SA.0.iter().enumerate() {
        if v == 0 {
            continue;
        }
        eprintln!("SA[{}] = {}", i, v);
    }

    SortTypeBstarResult { A, B, m }
}
