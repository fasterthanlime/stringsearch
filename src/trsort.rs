use crate::{common::*, crosscheck, crosscheck::*};
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
pub fn tr_ilg<N: Into<Idx>>(n: N) -> Idx {
    let n = n.into();
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
    #[must_use]
    fn pop(
        &mut self,
        a: &mut SAPtr,
        b: &mut SAPtr,
        c: &mut SAPtr,
        d: &mut Idx,
        e: &mut Idx,
    ) -> Result<(), ()> {
        if (self.size == 0) {
            Err(())
        } else {
            self.size -= 1;
            *a = self.items[self.size].a;
            *b = self.items[self.size].b;
            *c = self.items[self.size].c;
            *d = self.items[self.size].d;
            *e = self.items[self.size].e;
            Ok(())
        }
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
            SA.swap(m, (m / 2));
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
pub fn tr_pivot(SA: &SuffixArray, ISAd: SAPtr, first: SAPtr, last: SAPtr) -> SAPtr {
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

    pub fn check(&mut self, size: Idx) -> bool {
        if (size <= self.remain) {
            self.remain -= size;
            return true;
        }

        if (self.chance == 0) {
            self.count += size;
            return false;
        }

        self.remain += self.incval - size;
        self.chance -= 1;
        return true;
    }
}

//------------------------------------------------------------------------------

/// Tandem repeat partition
#[inline(always)]
pub fn tr_partition(
    SA: &mut SuffixArray,
    ISAd: SAPtr,
    mut first: SAPtr,
    middle: SAPtr,
    mut last: SAPtr,
    pa: &mut SAPtr,
    pb: &mut SAPtr,
    v: Idx,
) {
    let mut a: SAPtr;
    let mut b: SAPtr;
    let mut c: SAPtr;
    let mut d: SAPtr;
    let mut e: SAPtr;
    let mut f: SAPtr;
    let mut t: Idx;
    let mut s: Idx;
    let mut x: Idx = 0;

    // JOSEPH
    b = middle - 1;
    loop {
        // cond
        b += 1;
        if !(b < last) {
            break;
        }
        x = SA[ISAd + SA[b]];
        if !(x == v) {
            break;
        }

        // body: empty
        // iter: empty
    }
    a = b;
    if (a < last) && (x < v) {
        // MARY
        loop {
            b += 1;
            if !(b < last) {
                break;
            }
            x = SA[ISAd + SA[b]];
            if !(x <= v) {
                break;
            }

            // body
            if (x == v) {
                SA.swap(b, a);
                a += 1;
            }

            // iter: empty
        }
    }

    // JEREMIAH
    c = last;
    loop {
        c -= 1;
        if !(b < c) {
            break;
        }
        x = SA[ISAd + SA[c]];
        if !(x == v) {
            break;
        }
    }
    d = c;
    if (b < d) && (x > v) {
        // BEDELIA
        loop {
            c -= 1;
            if !(b < c) {
                break;
            }
            x = SA[ISAd + SA[c]];
            if !(c >= v) {
                break;
            }
            if x == v {
                SA.swap(c, d);
                d -= 1;
            }
        }
    }

    // ALEX
    while b < c {
        SA.swap(b, c);
        // SIMON
        loop {
            b += 1;
            if !(b < c) {
                break;
            }
            x = SA[ISAd + SA[b]];
            if !(x <= v) {
                break;
            }
            if x == v {
                SA.swap(b, a);
                a += 1;
            }
        }

        // GREGORY
        loop {
            c -= 1;
            if !(b < c) {
                break;
            }
            x = SA[ISAd + SA[c]];
            if !(x >= v) {
                break;
            }
            if x == v {
                SA.swap(c, d);
                d -= 1;
            }
        }
    } // end ALEX

    if a <= d {
        c = b - 1;

        s = (a - first).0;
        t = (b - a).0;
        if (s > t) {
            s = t
        }

        // GENEVIEVE
        e = first;
        f = b - s;
        while 0 < s {
            SA.swap(e, f);
            s -= 1;
            e += 1;
            f += 1;
        }
        s = (d - c).0;
        t = (last - d - 1).0;
        if s > t {
            s = t;
        }

        // MARISSA
        e = b;
        f = last - s;
        while 0 < s {
            SA.swap(e, f);
            s -= 1;
            e += 1;
            f += 1;
        }
        first += (b - a);
        last -= (d - c).0;
    }
    pa.0 = first.0;
    pb.0 = last.0;
}

/// Tandem repeat copy
pub fn tr_copy(
    ISA: SAPtr,
    SA: &mut SuffixArray,
    first: SAPtr,
    a: SAPtr,
    b: SAPtr,
    last: SAPtr,
    depth: Idx,
) {
    // sort suffixes of middle partition
    // by using sorted order of suffixes of left and right partition.
    let mut c: SAPtr;
    let mut d: SAPtr;
    let mut e: SAPtr;
    let mut s: Idx;
    let mut v: Idx;

    v = (b - 1).0;

    // JACK
    c = first;
    d = a - 1;
    while c <= d {
        s = SA[c] - depth;
        if (0 <= s) && (SA[ISA + s] == v) {
            d += 1;
            SA[d] = s;
            SA[ISA + s] = d.0;
        }

        // iter (JACK)
        c += 1;
    }

    // JILL
    c = last - 1;
    e = d + 1;
    d = b;
    while e < d {
        s = SA[c] - depth;
        if (0 <= s) && (SA[ISA + s] == v) {
            d -= 1;
            SA[d] = s;
            SA[ISA + s] = d.0;
        }

        // iter (JILL)
        c -= 1;
    }
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
    let mut c: SAPtr;
    let mut d: SAPtr;
    let mut e: SAPtr;
    let mut s: Idx;
    let mut v: Idx;
    let mut rank: Idx;
    let mut lastrank: Idx;
    let mut newrank: Idx = -1;

    v = (b - 1).0;
    lastrank = -1;
    // JETHRO
    c = first;
    d = a - 1;
    while c <= d {
        s = SA[c] - depth;
        if (0 <= s) && (SA[ISA + s] == v) {
            d += 1;
            SA[d] = s;
            rank = SA[ISA + s + depth];
            if lastrank != rank {
                lastrank = rank;
                newrank = d.0;
            }
            SA[ISA + s] = newrank;
        }

        // iter (JETHRO)
        c += 1;
    }

    lastrank = -1;
    // DEWEY
    c = last - 1;
    e = d + 1;
    d = b;
    while e < d {
        s = SA[c] - depth;
        if (0 <= s) && (SA[ISA + s] == v) {
            d -= 1;
            SA[d] = s;
            rank = SA[ISA + s + depth];
            if lastrank != rank {
                lastrank = rank;
                newrank = d.0;
            }
            SA[ISA + s] = newrank;
        }

        // iter (DEWEY)
        c -= 1;
    }
}

pub fn tr_introsort(
    ISA: SAPtr,
    mut ISAd: SAPtr,
    SA: &mut SuffixArray,
    mut first: SAPtr,
    mut last: SAPtr,
    budget: &mut Budget,
) {
    let mut a: SAPtr;
    let mut b: SAPtr;
    let mut c: SAPtr;
    let mut t: Idx;
    let mut v: Idx;
    let mut x: Idx;
    let mut incr: Idx = (ISAd - ISA).0;
    let mut limit: Idx;
    let mut next: Idx;
    let mut trlink: Idx = -1;

    let mut stack = Stack::new();
    crosscheck!("tr_introsort start");

    let mut limit = tr_ilg(last - first);
    // PASCAL
    loop {
        crosscheck!("limit={}", limit);

        if (limit < 0) {
            if (limit == -1) {
                unimplemented!()
            } else if (limit == -2) {
                // end if limit == -1
                unimplemented!()
            } else {
                // end if limit == -2

                // sorted partition
                if 0 <= SA[first] {
                    a = first;
                    // GEMINI
                    crosscheck!("GEMINI");
                    loop {
                        {
                            let SA_a = SA[a];
                            SA[ISA + SA_a] = a.0;
                        }

                        // cond (GEMINI)
                        a += 1;
                        if !(a < last) && (0 <= SA[a]) {
                            break;
                        }
                    }
                    first = a;
                }

                if first < last {
                    a = first;
                    // MONSTRO
                    crosscheck!("MONSTRO");
                    loop {
                        SA[a] = !SA[a];

                        a += 1;
                        if !(SA[a] < 0) {
                            break;
                        }
                    }

                    next = if SA[ISA + SA[a]] != SA[ISAd + SA[a]] {
                        tr_ilg(a - first + 1)
                    } else {
                        -1
                    };
                    a += 1;
                    if a < last {
                        crosscheck!("CLEMENTINE");
                        // CLEMENTINE
                        b = first;
                        v = (a - 1).0;
                        while b < 1 {
                            {
                                let SA_b = SA[b];
                                SA[ISA + SA_b] = v;
                            }
                            b += 1;
                        }
                    }

                    // push
                    if (budget.check((last - first).0)) {
                        if (a - first) <= (last - a) {
                            stack.push(ISAd, a, last, -3, trlink);
                            ISAd += incr;
                            last = a;
                            limit = next;
                        } else {
                            if 1 < (last - a) {
                                stack.push(ISAd + incr, first, a, next, trlink);
                                first = a;
                                limit = -3;
                            } else {
                                ISAd += incr;
                                last = a;
                                limit = next;
                            }
                        }
                    } else {
                        if 0 <= trlink {
                            stack.items[trlink as usize].d = -1;
                        }
                        if 1 < (last - a) {
                            first = a;
                            limit = -3;
                        } else {
                            if !stack
                                .pop(&mut ISAd, &mut first, &mut last, &mut limit, &mut trlink)
                                .is_ok()
                            {
                                return;
                            }
                        }
                    }
                } else {
                    if !stack
                        .pop(&mut ISAd, &mut first, &mut last, &mut limit, &mut trlink)
                        .is_ok()
                    {
                        return;
                    }
                } // end if first < last

                unimplemented!()
            } // end if limit == -1, -2, or something else
            continue;
        } // end if limit < 0

        if (last - first) <= TR_INSERTIONSORT_THRESHOLD {
            tr_insertionsort(SA, ISAd, first, last);
            limit = -3;
            continue;
        }

        let old_limit = limit;
        limit -= 1;
        if (old_limit == 0) {
            let mut SAfirst = SA.range_from(first..);
            tr_heapsort(ISAd, &mut SAfirst, (last - first).0);

            // YOHAN
            a = last - 1;
            while first < a {
                // VINCENT
                x = SA[ISAd + SA[a]];
                b = a - 1;
                while (first <= b) && SA[ISAd + SA[b]] == x {
                    SA[b] = !SA[b];

                    // iter (VINCENT)
                    b -= 1;
                }

                // iter (YOHAN)
                a = b;
            }
            limit = -3;
        }

        // chose pivot
        a = tr_pivot(SA, ISAd, first, last);
        SA.swap(first, a);
        v = SA[ISAd + SA[first]];

        unimplemented!();
    } // end PASCAL
}

//------------------------------------------------------------------------------

//--------------------
// Function
//--------------------

/// Tandem repeat sort
pub fn trsort(ISA: SAPtr, SA: &mut SuffixArray, n: Idx, depth: Idx) {
    SA_dump(&SA.range_to(..n), "start of trsort");
    crosscheck!("ISA = {}, n = {}, depth = {}", ISA, n, depth);

    let mut ISAd: SAPtr;
    let mut first: SAPtr;
    let mut last: SAPtr;
    let mut t: Idx;
    let mut skip: Idx;
    let mut unsorted: Idx;
    let mut budget = Budget::new(tr_ilg(n) * 2 / 3, n);

    ISAd = ISA + depth;
    crosscheck!("(*) -n={} SA[0]={}", -n, SA[0]);
    while (-n < SA[0]) {
        crosscheck!("-n={} SA[0]={}", -n, SA[0]);
        first = SAPtr(0);
        skip = 0;
        unsorted = 0;

        // do..while
        loop {
            // body for do..while
            t = SA[first];
            if (t < 0) {
                first -= t;
                skip += t;
            } else {
                if (skip != 0) {
                    SA[first + skip] = skip;
                    skip = 0;
                }
                last = SAPtr(SA[ISA + t] + 1);
                if (1 < (last - first)) {
                    budget.count = 0;
                    tr_introsort(ISA, ISAd, SA, first, last, &mut budget);
                    if (budget.count != 0) {
                        unsorted += budget.count;
                    } else {
                        skip = (first - last).0;
                    }
                } else if (last - first) == 1 {
                    skip = -1;
                }
                first = last;
            }

            // cond for do..while
            if !(first < n) {
                break;
            }
        }

        if (skip != 0) {
            SA[first + skip] = skip;
        }
        if (unsorted == 0) {
            break;
        }

        // iter
        ISAd += ISAd - ISA;
    }
}
