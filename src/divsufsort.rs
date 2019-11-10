#![allow(nonstandard_style)]

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
    let m = sort_typeBstar(&T, &mut SA);
}

struct SortTypeBstarResult {
    bucket_A: Vec<Idx>,
    bucket_B: Vec<Idx>,
    m: Idx,
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

fn sort_typeBstar(T: &Text, SA: &mut SuffixArray) -> SortTypeBstarResult {
    let n = T.len();

    // Initialize bucket arrays
    let mut A: Vec<Idx> = vec![0; BUCKET_A_SIZE];
    let mut B: Vec<Idx> = vec![0; BUCKET_B_SIZE];

    // #define BUCKET_A(_c0) bucket_A[(_c0)]
    // #define BUCKET_B(_c0, _c1) (bucket_B[((_c1) << 8) | (_c0)])
    // #define BUCKET_BSTAR(_c0, _c1) (bucket_B[((_c0) << 8) | (_c1)])

    let A_idx = |c0: Char| -> usize { c0 as usize };
    let B_idx = |c0: Char, c1: Char| -> usize { (((c1 as usize) << 8) | (c0 as usize)) };
    let BS_idx = |c0: Char, c1: Char| -> usize { (((c0 as usize) << 8) | (c1 as usize)) };

    // Count the number of occurences of the first one or two characters of each
    // type A, B and B* suffix. Moreover, store the beginning position of all
    // type B* suffixes into the array SA.
    {
        let mut i = n - 1;
        let mut m = n;
        let mut c0 = T[(n - 1)];
        let mut c1 = 0;

        while 0 <= i {
            // type A suffix
            loop {
                c1 = c0;
                A[A_idx(c1)] += 1;

                i -= 1;
                if i < 0 {
                    break;
                }

                c0 = T[i];
                if c0 < c1 {
                    break;
                }
            }
        }

        if 0 <= i {
            // type B* suffix
            B[BS_idx(c0, c1)] += 1;
            m -= 1;
            SA[m] = i;
        }
    }

    unimplemented!()
}
