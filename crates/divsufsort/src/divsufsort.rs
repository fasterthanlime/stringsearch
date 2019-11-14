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
    let mut k: Idx;
    let mut t: Idx;
    let mut m: Idx;

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
                if !(0 <= i) {
                    break;
                }
                c0 = T.get(i);
                if !(c0 <= c1) {
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

    if (0 < m) {
        // Sort the type B* suffixes by their first two characters
        let PAb = SAPtr(n - m);
        let ISAb = SAPtr(m);

        for i in (0..=(m - 2)).rev() {
            t = SA[PAb + i];
            c0 = T.get(t);
            c1 = T.get(t + 1);
            B.bstar()[(c0, c1)] -= 1;
            SA[B.bstar()[(c0, c1)]] = i;
        }
        t = SA[PAb + m - 1];
        c0 = T.get(t);
        c1 = T.get(t + 1);
        B.bstar()[(c0, c1)] -= 1;
        SA[B.bstar()[(c0, c1)]] = m - 1;

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
                    SA_dump(&SA.range(i..j), "sssort(A)");
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
                    SA_dump(&SA.range(i..j), "sssort(B)");
                }

                // iter (inner)
                j = i;
                c1 -= 1;
            }

            // iter (outer)
            c0 -= 1;
        }

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

        // Set the sorted order of type B* suffixes
        {
            // init
            i = n - 1;
            j = m;
            c0 = T.get(n - 1);
            while 0 <= i {
                // init
                i -= 1;
                c1 = c0;

                loop {
                    // cond
                    if !(0 <= i) {
                        break;
                    }
                    c0 = T.get(i);
                    if !(c0 >= c1) {
                        break;
                    }

                    // body (empty)

                    // iter
                    i -= 1;
                    c1 = c0;
                }

                if 0 <= i {
                    t = i;

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

                        // body (empty)

                        // iter
                        i -= 1;
                        c1 = c0;
                    }

                    j -= 1;
                    {
                        let pos = SA[ISAb + j];
                        SA[pos] = if (t == 0) || (1 < (t - i)) { t } else { !t };
                    }
                }
            }
        } // End: Set the sorted order of type B* suffixes

        // Calculate the index of start/end point of each bucket
        {
            B.b()[(ALPHABET_SIZE as Idx - 1, ALPHABET_SIZE as Idx - 1)] = n; // end point

            // init
            c0 = ALPHABET_SIZE as Idx - 2;
            k = m - 1;

            while 0 <= c0 {
                i = A[c0 + 1] - 1;

                // init
                c1 = ALPHABET_SIZE as Idx - 1;
                while c0 < c1 {
                    t = i - B.b()[(c0, c1)];
                    B.b()[(c0, c1)] = i; // end point

                    // Move all type B* suffixes to the correct position
                    {
                        // init
                        i = t;
                        j = B.bstar()[(c0, c1)];

                        while j <= k {
                            SA[i] = SA[k];

                            // iter
                            i -= 1;
                            k -= 1;
                        }
                    } // End: Move all type B* suffixes to the correct position

                    // iter
                    c1 -= 1;
                }
                B.bstar()[(c0, c0 + 1)] = i - B.b()[(c0, c0)] + 1;
                B.b()[(c0, c0)] = i; // end point

                // iter
                c0 -= 1;
            }
        } // End: Calculate the index of start/end point of each bucket
    }

    SortTypeBstarResult { A, B, m }
}

fn construct_SA(T: &Text, SA: &mut SuffixArray, mut A: ABucket, mut B: BMixBucket, m: Idx) {
    let n = T.len() as Idx;
    let mut i: SAPtr;
    let mut j: SAPtr;
    let mut k: Idx;
    let mut s: Idx;
    let mut c0: Idx;
    let mut c1: Idx;
    let mut c2: Idx;

    if 0 < m {
        // Construct the sorted order of type B suffixes by using the
        // sorted order of type B* suffixes
        c1 = ALPHABET_SIZE as Idx - 2;
        while 0 <= c1 {
            // Scan the suffix array from right to left
            i = SAPtr(B.bstar()[(c1, c1 + 1)]);
            j = SAPtr(A[c1 + 1] - 1);
            k = 0;
            c2 = -1;

            while i <= j {
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

    // Construct the suffix array by using the sorted order of type B suffixes
    c2 = T.get(n - 1);
    k = A[c2];
    SA[k] = if T.get(n - 2) < c2 { !(n - 1) } else { n - 1 };
    k += 1;
    // Scan the suffix array from left to right
    {
        // init
        i = SAPtr(0);
        j = SAPtr(n);

        while i < j {
            s = SA[i];
            if 0 < s {
                assert!(T[s - 1] >= T[s]);
                s -= 1;
                c0 = T.get(s);
                if (s == 0) || (T.get(s - 1) < c0) {
                    s = !s;
                }
                if (c0 != c2) {
                    A[c2] = k;
                    c2 = c0;
                    k = A[c2];
                }
                assert!(i < k);
                SA[k] = s;
                k += 1;
            } else {
                assert!(s < 0);
                SA[i] = !s;
            }

            // iter
            i += 1;
        }
    }
}
