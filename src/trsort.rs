use crate::common::*;
use std::mem::swap;

//--------------------
// Private functions
//--------------------

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
#[allow(overflowing_literals)]
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

//------------------------------------------------------------------------------

use std::default::Default;
const STACK_SIZE: usize = 64;

struct StackItem {
    a: SAPtr,
    b: SAPtr,
    c: SAPtr,
    d: Idx,
    e: Idx,
}

impl Default for StackItem {
    fn default() -> Self {
        Self {
            a: SAPtr(0),
            b: SAPtr(0),
            c: SAPtr(0),
            d: 0,
            e: 0,
        }
    }
}

struct Stack {
    items: [StackItem; STACK_SIZE],
    size: usize,
}

impl Stack {
    fn new() -> Self {
        Self {
            items: [
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
            ],
            size: 0,
        }
    }

    #[inline(always)]
    fn push(&mut self, a: SAPtr, b: SAPtr, c: SAPtr, d: Idx, e: Idx) {
        assert!(self.size < STACK_SIZE);
        self.items[self.size].a = a;
        self.items[self.size].b = b;
        self.items[self.size].c = c;
        self.items[self.size].d = d;
        self.items[self.size].e = e;
        self.size += 1;
    }

    #[inline(always)]
    fn pop(&mut self, a: &mut SAPtr, b: &mut SAPtr, c: &mut SAPtr, d: &mut Idx, e: &mut Idx) {
        if (self.size == 0) {
            return;
        }
        *a = self.items[self.size].a;
        *b = self.items[self.size].b;
        *c = self.items[self.size].c;
        *d = self.items[self.size].d;
        *e = self.items[self.size].e;
        self.size -= 1;
    }
}

//------------------------------------------------------------------------------

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

//------------------------------------------------------------------------------

#[inline(always)]
pub fn tr_fixdown(ISAd: SAPtr, SA: &mut SuffixArray, mut i: Idx, size: Idx) {
    let mut j: Idx;
    let mut k: Idx;
    let mut d: Idx;
    let mut e: Idx;

    let v = SA[i];
    let c = SA[ISAd + v];

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
            SA.swap(m, m / 2);
        }
    }

    for i in (0..(m / 2)).rev() {
        tr_fixdown(ISAd, SA, i, m);
    }
    if (size % 2) == 0 {
        SA.swap(0, m);
        tr_fixdown(ISAd, SA, 0, m);
    }
    for i in (1..m).rev() {
        t = SA[0];
        SA[0] = SA[i];
        tr_fixdown(ISAd, SA, 0, i);
        SA[i] = t;
    }
}

//------------------------------------------------------------------------------

/// Returns the median of three elements
#[inline(always)]
pub fn tr_median3(SA: &SuffixArray, ISAd: SAPtr, mut v1: SAPtr, mut v2: SAPtr, v3: SAPtr) -> SAPtr {
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

/// Returns the median of five elements
#[inline(always)]
pub fn tr_median5(
    SA: &SuffixArray,
    ISAd: SAPtr,
    mut v1: SAPtr,
    mut v2: SAPtr,
    v3: SAPtr,
    v4: SAPtr,
    v5: SAPtr,
) -> SAPtr {
    unimplemented!()
}

/// Returns the pivot element
#[inline(always)]
pub fn tr_pivot(SA: &SuffixArray, ISAd: SAPtr, first: SAPtr, last: SAPtr) {
    unimplemented!()
}

//------------------------------------------------------------------------------

pub struct Budget {
    pub chance: Idx,
    pub remain: Idx,
    pub incval: Idx,
    pub count: Idx,
}

impl Budget {
    pub fn new(chance: Idx, incval: Idx) -> Self {
        Self {
            chance,
            remain: incval,
            incval,
            count: 0,
        }
    }
}

//------------------------------------------------------------------------------

#[inline(always)]
pub fn tr_partition(
    SA: &mut SuffixArray,
    ISAd: SAPtr,
    first: SAPtr,
    middle: SAPtr,
    last: SAPtr,
    pa: &mut SAPtr,
    pb: &mut SAPtr,
    v: Idx,
) {
    unimplemented!()
}

pub fn tr_copy(
    ISA: SAPtr,
    SA: &mut SuffixArray,
    first: SAPtr,
    a: SAPtr,
    b: SAPtr,
    last: SAPtr,
    depth: Idx,
) {
    unimplemented!()
}

pub fn tr_partialcopy(
    ISA: SAPtr,
    SA: &mut SuffixArray,
    first: SAPtr,
    a: SAPtr,
    b: SAPtr,
    last: SAPtr,
    depth: Idx,
) {
    unimplemented!()
}

pub fn tr_introsort(
    ISA: SAPtr,
    ISAd: SAPtr,
    SA: &mut SuffixArray,
    first: SAPtr,
    last: SAPtr,
    budget: &mut Budget,
) {
    unimplemented!()
}

//------------------------------------------------------------------------------

//------------------------------------------------------------------------------

//--------------------
// Function
//--------------------

/// Tandem repeat sort
pub fn trsort(ISA: SAPtr, SA: &mut SuffixArray, n: Idx, depth: Idx) {
    let n = SA.len();

    let mut ISAd: SAPtr;
    let mut first: SAPtr;
    let mut last: SAPtr;
    let mut t: Idx;
    let mut skip: Idx;
    let mut unsorted: Idx;
    let mut budget = Budget::new(tr_ilg(n) * 2 / 3, n);

    unimplemented!()
}
