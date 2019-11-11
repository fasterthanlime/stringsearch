#![allow(nonstandard_style)]
use crate::common::*;
use std::mem::swap;

#[rustfmt::skip]
const lg_table: [Idx; 256] = [
 -1,0,1,1,2,2,2,2,3,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,
  5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,
  6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,
  6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,
  7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,
  7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,
  7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,
  7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7
];

#[inline(always)]
pub fn tr_ilg(n: Idx) -> Idx {
    if (n & 0xffff_0000) > 0 {
        if (n & 0xff00_0000) > 0 {
            24 + lg_table[((n >> 24) & 0xff) as usize]
        } else {
            16 + lg_table[((n >> 16) & 0xff) as usize]
        }
    } else {
        if (n & 0x0000_ff00) > 0 {
            8 + lg_table[((n >> 8) & 0xff) as usize]
        } else {
            0 + lg_table[((n >> 0) & 0xff) as usize]
        }
    }
}

/// Simple insertionsort for small size groups
pub fn tr_insertionsort(SA: &mut SuffixArray, ISAd: SAPtr, first: SAPtr, last: SAPtr) {
    let mut a: SAPtr;
    let mut b: SAPtr;
    let mut t: Idx;
    let mut r: Idx;

    a = first + 1;
    while a < last {
        t = SA[a];
        b = a - 1;

        loop {
            // cond
            r = SA[ISAd + t] - SA[ISAd + SA[b]];
            if !(0 > r) {
                break;
            }

            loop {
                SA[b + 1] = SA[b];

                // cond
                b -= 1;
                if !((first <= b) && (SA[b] < 0)) {
                    break;
                }
            }

            // body
            if b < first {
                break;
            }
        }

        if r == 0 {
            SA[b] = !SA[b];
        }
        SA[b + 1] = t;

        // iter
        a += 1;
    }
}

#[inline(always)]
pub fn tr_fixdown(ISAd: SAPtr, SA: &mut SuffixArray, i: Idx, size: Idx) {
    let mut j: Idx;
    let mut k: Idx;
    let mut v: Idx;
    let mut c: Idx;
    let mut d: Idx;
    let mut e: Idx;

    v = SA[i];
    c = SA[ISAd + v];

    loop {
        // cond
        j = 2 * i + 1;
        if !(j < size) {
            break;
        }

        // body
        k = j;
        d = SA[ISAd + SA[k]];
        j += 1;
        e = SA[ISAd + SA[j]];
        if d < e {
            k = j;
            d = e;
        }
        if d <= c {
            break;
        }
        SA[i] = v;

        // iter
        SA[i] = SA[k];
        i = k;
    }
}

/// Simple top-down heapsort
pub fn tr_heapsort(ISAd: SAPtr, SA: &mut SuffixArray, size: Idx) {
    let mut i: Idx;
    let mut m: Idx;
    let mut t: Idx;

    m = size;
    if (size % 2) == 0 {
        m -= 1;
        if SA[ISAd + SA[m / 2]] < SA[ISAd + SA[m]] {
            swap(&mut SA[m], &mut SA[m / 2]);
        }
    }

    for i in (0..(m / 2)).rev() {
        tr_fixdown(ISAd, SA, i, m);
    }
    if (size % 2) == 0 {
        swap(&mut SA[0], &mut SA[m]);
        tr_fixdown(ISAd, SA, 0, m);
    }
    for i in (1..m).rev() {
        t = SA[0];
        SA[0] = SA[i];
        tr_fixdown(ISAd, SA, 0, i);
        SA[i] = t;
    }
}

/// Returns the median of three elements
#[inline(always)]
pub fn tr_median3(SA: &SuffixArray, ISAd: SAPtr, v1: SAPtr, v2: SAPtr, v3: SAPtr) -> SAPtr {
    if SA[ISAd + SA[v1]] > SA[ISAd + SA[v2]] {
        swap(&mut v1, &mut v2);
    }
    if SA[ISAd + SA[v2]] > SA[ISAd + SA[v3]] {
        if SA[ISAd + SA[v1]] > SA[ISAd + SA[v3]] {
            v1
        } else {
            v3
        }
    } else {
        v2
    }
}

struct Budget {
    chance: Idx,
    remain: Idx,
    incval: Idx,
    count: Idx,
}

impl Budget {
    fn new(chance: Idx, incval: Idx) -> Self {
        Self {
            chance,
            remain: incval,
            incval,
            count: 0,
        }
    }
}

/// Tandem repeat sort
pub fn trsort(ISA: SAPtr, SA: &mut SuffixArray, depth: Idx) {
    let n = SA.len();

    let mut ISAd: SAPtr;
    let mut first: SAPtr;
    let mut last: SAPtr;
    let mut t: Idx;
    let mut skip: Idx;
    let mut unsorted: Idx;
    let mut budget = Budget::new(tr_ilg(n) * 2 / 3, n);
}
