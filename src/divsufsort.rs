use crate::{common::*, crosscheck, crosscheck::*, sssort, trsort};

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
    let res = sort_typeBstar(&T, &mut SA);
    construct_SA(&T, &mut SA, res.A, res.B, res.m);
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
            crosscheck!("increment A[{}]", c1);
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
            crosscheck!("increment BSTAR[{}, {}]", c0, c1);
            B.bstar()[(c0, c1)] += 1;

            m -= 1;
            SA[m] = i;

            // type B suffix

            // init
            i -= 1;
            c1 = c0;

            loop {
                // cond
                if !(0 <= i) {
                    break;
                }
                c0 = T.get(i);
                if !(c0 <= c1) {
                    break;
                }

                // body
                crosscheck!("increment B[{}, {}]", c0, c1);
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
            crosscheck!("sp={}", A[c0]);
            i = t + B.b()[(c0, c0)];

            for c1 in (c0 + 1)..(ALPHABET_SIZE as Idx) {
                j += B.bstar()[(c0, c1)];
                crosscheck!("j+={}", B.bstar()[(c0, c1)]);
                B.bstar()[(c0, c1)] = j; // end point
                crosscheck!("ep={}", B.bstar()[(c0, c1)]);
                i += B.b()[(c0, c1)];
                crosscheck!("i+={}", B.b()[(c0, c1)]);
            }
        }
    }

    crosscheck!("before-0<m, m = {}", m);

    crosscheck!("before B* suffix sort, m = {}", m);
    if (0 < m) {
        SA_dump(SA, "before B* suffix sort");

        // Sort the type B* suffixes by their first two characters
        let PAb = SAPtr(n - m);
        let ISAb = SAPtr(m);

        for i in (0..=(m - 2)).rev() {
            t = SA[PAb + i];
            c0 = T.get(t);
            c1 = T.get(t + 1);
            crosscheck!("t={} c0={} c1={}", t, c0, c1);
            B.bstar()[(c0, c1)] -= 1;
            SA[B.bstar()[(c0, c1)]] = i;
        }
        t = SA[PAb + m - 1];
        c0 = T.get(t);
        c1 = T.get(t + 1);
        crosscheck!("(*) t={} c0={} c1={}", t, c0, c1);
        B.bstar()[(c0, c1)] -= 1;
        SA[B.bstar()[(c0, c1)]] = m - 1;

        SA_dump(SA, "before all ssort");

        // Sort the type B* substrings using sssort.
        let buf = SAPtr(m);
        let bufsize = n - (2 * m);

        // init (outer)
        c0 = ALPHABET_SIZE as Idx - 2;
        j = m;
        while 0 < j {
            // init (inner)
            c1 = ALPHABET_SIZE as Idx - 1;
            while c0 < c1 {
                // body (inner)
                i = B.bstar()[(c0, c1)];

                if (1 < (j - i)) {
                    crosscheck!("sssort() i={} j={}", i, j);
                    sssort::sssort(
                        T,
                        SA,
                        PAb,
                        SAPtr(i),
                        SAPtr(j),
                        buf,
                        bufsize,
                        2,
                        n,
                        SA[i] == (m - 1),
                    );
                    SA_dump(SA, "");
                }

                // iter (inner)
                j = i;
                c1 -= 1;
            }

            // iter (outer)
            c0 -= 1;
        }

        SA_dump(SA, "after all sssort()");

        // Compute ranks of type B* substrings
        i = m - 1;
        while 0 <= i {
            if (0 <= SA[i]) {
                j = i;
                loop {
                    {
                        let SAi = SA[i];
                        SA[ISAb + SAi] = i;
                    }

                    i -= 1;
                    if !((0 <= i) && (0 <= SA[i])) {
                        break;
                    }
                }

                SA[i + 1] = i - j;
                if (i <= 0) {
                    break;
                }
            }
            j = i;
            loop {
                SA[i] = !SA[i];
                {
                    let idx = ISAb + SA[i];
                    SA[idx] = j;
                }

                i -= 1;
                if !(SA[i] < 0) {
                    break;
                }
            }
            {
                let idx = ISAb + SA[i];
                SA[idx] = j;
            }

            i -= 1;
        }

        // Construct the inverse suffix array of type B* suffixes using trsort.
        trsort::trsort(ISAb, SA, m, 1);
    }

    SortTypeBstarResult { A, B, m }
}

fn construct_SA(T: &Text, SA: &mut SuffixArray, A: ABucket, mut B: BMixBucket, m: Idx) {
    let n = T.len() as Idx;
    let mut i: SAPtr;
    let mut j: SAPtr;
    let mut k: Idx;
    let mut s: Idx;
    let mut c0: Idx;
    let mut c1: Idx;
    let mut c2: Idx;

    crosscheck!("construct_SA start");
    crosscheck!("m = {}", m);

    if 0 < m {
        A_dump(&A, "in if(0 < m)");

        // Construct the sorted order of type B suffixes by using the
        // sorted order of type B* suffixes
        c1 = ALPHABET_SIZE as Idx - 2;
        while 0 <= c1 {
            crosscheck!("(for) c1 = {}", c1);
            crosscheck!("BSTAR(c, c1 + 1) = {}", B.bstar()[(c1, c1 + 1)]);

            // Scan the suffix array from right to left
            i = SAPtr(B.bstar()[(c1, c1 + 1)]);
            j = SAPtr(A[c1 + 1] - 1);
            k = 0;
            c2 = -1;

            while i <= j {
                crosscheck!("c1={} i={} j={}", c1, i, j);
                s = SA[j];
                if (0 < s) {
                    assert_eq!(T.get(s), c1);
                    assert!((s + 1) < n);
                    assert!(T[s] <= T[s + 1]);

                    SA[j] = !s;
                    s -= 1;
                    c0 = T.get(s);
                    if (0 < s) && (T.get(s - 1) > c0) {
                        s = !s;
                    }
                    if (c0 != c2) {
                        if (0 <= c2) {
                            B.b()[(c2, c1)] = k;
                        }
                        c2 = c0;
                        k = B.b()[(c2, c1)];
                    }
                    assert!(k < j);
                    SA[k] = s;
                    k -= 1;
                } else {
                    assert!(((s == 0) && (T.get(s) == c1)) || (s < 0));
                    SA[j] = !s;
                }

                // iter
                j -= 1;
            }

            // iter
            c1 -= 1;
        }
    }

    SA_dump(SA, "after construct");
}
