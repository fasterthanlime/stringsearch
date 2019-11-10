#![allow(nonstandard_style)]
#![allow(unused)]

pub type Char = u8;
pub type Idx = i32;

pub const ALPHABET_SIZE: usize = u8::max_value() as usize + 1;
pub const BUCKET_A_SIZE: usize = ALPHABET_SIZE;
pub const BUCKET_B_SIZE: usize = ALPHABET_SIZE * ALPHABET_SIZE;

pub const MAX_INPUT_SIZE: usize = i32::max_value() as usize;

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

struct ABucket(Vec<Idx>);

impl Index<Idx> for ABucket {
    type Output = Idx;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<Idx> for ABucket {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

struct BMixBucket(Vec<Idx>);

impl BMixBucket {
    #[inline(always)]
    fn b<'a>(&'a mut self) -> BBucket<'a> {
        BBucket(&mut self.0)
    }

    #[inline(always)]
    fn bstar<'a>(&'a mut self) -> BStarBucket<'a> {
        BStarBucket(&mut self.0)
    }
}

struct BBucket<'a>(&'a mut [Idx]);

impl<'a> Index<(Idx, Idx)> for BBucket<'a> {
    type Output = Idx;

    fn index(&self, index: (Idx, Idx)) -> &Self::Output {
        let (c0, c1) = index;
        &self.0[((c1 << 8) | c0) as usize]
    }
}

impl<'a> IndexMut<(Idx, Idx)> for BBucket<'a> {
    fn index_mut(&mut self, index: (Idx, Idx)) -> &mut Self::Output {
        let (c0, c1) = index;
        &mut self.0[((c1 << 8) | c0) as usize]
    }
}

struct BStarBucket<'a>(&'a mut [Idx]);

impl<'a> Index<(Idx, Idx)> for BStarBucket<'a> {
    type Output = Idx;

    fn index(&self, index: (Idx, Idx)) -> &Self::Output {
        let (c0, c1) = index;
        &self.0[((c0 << 8) | c1) as usize]
    }
}

impl<'a> IndexMut<(Idx, Idx)> for BStarBucket<'a> {
    fn index_mut(&mut self, index: (Idx, Idx)) -> &mut Self::Output {
        let (c0, c1) = index;
        &mut self.0[((c0 << 8) | c1) as usize]
    }
}

fn sort_typeBstar(T: &Text, SA: &mut SuffixArray) -> SortTypeBstarResult {
    let n = T.len();

    // Initialize bucket arrays
    let A: Vec<Idx> = vec![0; BUCKET_A_SIZE];
    let mut A = ABucket(A);

    let B: Vec<Idx> = vec![0; BUCKET_B_SIZE];
    let mut B = BMixBucket(B);

    // #define BUCKET_A(_c0) bucket_A[(_c0)]
    // #define BUCKET_B(_c0, _c1) (bucket_B[((_c1) << 8) | (_c0)])
    // #define BUCKET_BSTAR(_c0, _c1) (bucket_B[((_c0) << 8) | (_c1)])

    // let A_idx = |c0: usize| -> usize { c0 };
    // let B_idx = |c0: usize, c1: usize| -> usize { ((c1 << 8) | c0) };
    // let BS_idx = |c0: usize, c1: usize| -> usize { ((c0 << 8) | c1) };

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
    c0 = T[n - 1] as Idx;

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

            c0 = T[i] as Idx;
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
                c0 = T[i] as Idx;
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

    // TODO: rest of sort..

    SortTypeBstarResult { A, B, m }
}

use std::ops::{Index, IndexMut};

// Read-only input to suffix-sort
struct Text<'a>(&'a [Char]);

impl<'a> Index<Idx> for Text<'a> {
    type Output = Char;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl<'a> Text<'a> {
    fn len(&self) -> Idx {
        self.0.len() as Idx
    }
}

// Indexes of all suffixes in lexicographical order
struct SuffixArray<'a>(&'a mut [Idx]);

impl<'a> Index<Idx> for SuffixArray<'a> {
    type Output = Idx;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl<'a> IndexMut<Idx> for SuffixArray<'a> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}
