#![allow(unused)]

use crate::common::*;

// Substring sort
pub fn sssort(
    T: &Text,
    SA: &mut SuffixArray,
    PA: Idx,
    mut first: Idx,
    last: Idx,
    mut buf: Idx,
    mut bufsize: Idx,
    depth: Idx,
    n: Idx,
    lastsuffix: bool,
) {
    let mut a: Idx;
    let mut b: Idx;
    let mut middle: Idx;
    let mut curbuf: Idx;
    let mut j: Idx;
    let mut k: Idx;
    let mut curbufsize: Idx;
    let mut limit: Idx;
    let mut i: Idx;

    if lastsuffix {
        first += 1;
    }

    limit = ss_isqrt(last - first);
    if ((bufsize < SS_BLOCKSIZE) && (bufsize < (last - first)) && (bufsize < limit)) {
        if (SS_BLOCKSIZE < limit) {
            limit = SS_BLOCKSIZE;
        }
        middle = last - limit;
        buf = middle;
        bufsize = limit;
    } else {
        middle = last;
        limit = 0;
    }

    a = first;
    i = 0;
    while SS_BLOCKSIZE < (middle - a) {
        ss_mintrosort(T, SA, PA, a, a + SS_BLOCKSIZE, depth);

        // iter
        a += SS_BLOCKSIZE;
        i += 1;
    }
}

const lg_table: [Idx; 256] = [
    -1, 0, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    6, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
    7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
    7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
    7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
    7,
];

const sqq_table: [Idx; 256] = [
    0, 16, 22, 27, 32, 35, 39, 42, 45, 48, 50, 53, 55, 57, 59, 61, 64, 65, 67, 69, 71, 73, 75, 76,
    78, 80, 81, 83, 84, 86, 87, 89, 90, 91, 93, 94, 96, 97, 98, 99, 101, 102, 103, 104, 106, 107,
    108, 109, 110, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 128,
    128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 144, 145,
    146, 147, 148, 149, 150, 150, 151, 152, 153, 154, 155, 155, 156, 157, 158, 159, 160, 160, 161,
    162, 163, 163, 164, 165, 166, 167, 167, 168, 169, 170, 170, 171, 172, 173, 173, 174, 175, 176,
    176, 177, 178, 178, 179, 180, 181, 181, 182, 183, 183, 184, 185, 185, 186, 187, 187, 188, 189,
    189, 190, 191, 192, 192, 193, 193, 194, 195, 195, 196, 197, 197, 198, 199, 199, 200, 201, 201,
    202, 203, 203, 204, 204, 205, 206, 206, 207, 208, 208, 209, 209, 210, 211, 211, 212, 212, 213,
    214, 214, 215, 215, 216, 217, 217, 218, 218, 219, 219, 220, 221, 221, 222, 222, 223, 224, 224,
    225, 225, 226, 226, 227, 227, 228, 229, 229, 230, 230, 231, 231, 232, 232, 233, 234, 234, 235,
    235, 236, 236, 237, 237, 238, 238, 239, 240, 240, 241, 241, 242, 242, 243, 243, 244, 244, 245,
    245, 246, 246, 247, 247, 248, 248, 249, 249, 250, 250, 251, 251, 252, 252, 253, 253, 254, 254,
    255,
];

/// Fast sqrt, using lookup tables
#[allow(overflowing_literals)] // ☠☠☠
#[inline(always)]
pub fn ss_isqrt(x: Idx) -> Idx {
    let mut y: Idx;
    let e: Idx;

    if x >= (SS_BLOCKSIZE * SS_BLOCKSIZE) {
        return SS_BLOCKSIZE;
    }

    e = if (x & 0xffff_0000) > 0 {
        if (x & 0xff00_0000) > 0 {
            24 + lg_table[((x >> 24) & 0xff) as usize]
        } else {
            16 + lg_table[((x >> 16) & 0xff) as usize]
        }
    } else {
        if (x & 0x0000_ff00) > 0 {
            8 + lg_table[(((x >> 8) & 0xff) as usize)]
        } else {
            0 + lg_table[(((x >> 0) & 0xff) as usize)]
        }
    };

    if e >= 16 {
        y = sqq_table[(x >> ((e - 6) - (e & 1))) as usize] << ((e >> 1) - 7);
        if e >= 24 {
            y = (y + 1 + x / y) >> 1;
        }
        y = (y + 1 + x / y) >> 1;
    } else if e >= 8 {
        y = (sqq_table[(x >> ((e - 6) - (e & 1))) as usize] >> (7 - (e >> 1))) + 1;
    } else {
        return sqq_table[x as usize] >> 4;
    }

    if x < (y * y) {
        y - 1
    } else {
        y
    }
}

/// Fast log2, using lookup tables
pub fn ss_ilg(n: Idx) -> Idx {
    if (n & 0xff00) > 0 {
        8 + lg_table[((n >> 8) & 0xff) as usize]
    } else {
        0 + lg_table[((n >> 0) & 0xff) as usize]
    }
}

pub fn ss_mintrosort(T: &Text, SA: &SuffixArray, PA: Idx, first: Idx, last: Idx, depth: Idx) {
    const STACK_SIZE: usize = 16;
    #[derive(Clone, Copy)]
    struct StackItem {
        a: Idx,
        b: Idx,
        c: Idx,
        d: Idx,
    }
    let stack_item_zero = StackItem {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    };

    let stack: [StackItem; STACK_SIZE] = [
        stack_item_zero,
        stack_item_zero,
        stack_item_zero,
        stack_item_zero,
        stack_item_zero,
        stack_item_zero,
        stack_item_zero,
        stack_item_zero,
        stack_item_zero,
        stack_item_zero,
        stack_item_zero,
        stack_item_zero,
        stack_item_zero,
        stack_item_zero,
        stack_item_zero,
        stack_item_zero,
    ];

    let mut a: Idx;
    let mut b: Idx;
    let mut c: Idx;
    let mut d: Idx;
    let mut e: Idx;
    let mut f: Idx;
    let mut s: Idx;
    let mut t: Idx;
    let mut ssize: Idx;
    let mut limit: Idx;
    let mut v: Idx;
    let mut x: Idx;

    ssize = 0;
    limit = ss_ilg(last - first);
    loop {
        unimplemented!();
    }
}
