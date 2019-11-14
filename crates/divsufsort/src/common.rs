use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::ops::{self, Add, AddAssign, Div, Index, IndexMut, Sub, SubAssign};

pub type Char = u8;
pub type Idx = i32;

pub const TR_INSERTIONSORT_THRESHOLD: Idx = 8;

pub const SS_INSERTIONSORT_THRESHOLD: Idx = 8;
pub const SS_BLOCKSIZE: Idx = 1024;

pub const ALPHABET_SIZE: usize = u8::max_value() as usize + 1;
pub const BUCKET_A_SIZE: usize = ALPHABET_SIZE;
pub const BUCKET_B_SIZE: usize = ALPHABET_SIZE * ALPHABET_SIZE;

pub const MAX_INPUT_SIZE: usize = i32::max_value() as usize;

// Read-only input to suffix-sort
pub struct Text<'a>(pub &'a [Char]);

impl<'a> Index<Idx> for Text<'a> {
    type Output = Char;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl<'a> Text<'a> {
    #[inline(always)]
    pub fn get(&self, i: Idx) -> Idx {
        assert!(i >= 0, "assert violated: {} >= 0", i);
        self.0[i as usize] as Idx
    }

    #[inline(always)]
    pub fn len(&self) -> Idx {
        self.0.len() as Idx
    }
}

// Indexes of all suffixes in lexicographical order
#[derive(Debug)]
pub struct SuffixArray<'a>(pub &'a mut [Idx]);

impl<'a> SuffixArray<'a> {
    #[inline(always)]
    pub fn swap<A: Into<Idx>, B: Into<Idx>>(&mut self, a: A, b: B) {
        self.0.swap(a.into() as usize, b.into() as usize);
    }

    pub fn range<'b, I: Into<Idx>>(&'b mut self, range: ops::Range<I>) -> SuffixArray<'b> {
        let usize_range = (range.start.into() as usize)..(range.end.into() as usize);
        SuffixArray(&mut self.0[usize_range])
    }

    pub fn range_to<'b, I: Into<Idx>>(&'b mut self, range: ops::RangeTo<I>) -> SuffixArray<'b> {
        let usize_range = ..(range.end.into() as usize);
        SuffixArray(&mut self.0[usize_range])
    }

    pub fn range_from<'b, I: Into<Idx>>(&'b mut self, range: ops::RangeFrom<I>) -> SuffixArray<'b> {
        let usize_range = (range.start.into() as usize)..;
        SuffixArray(&mut self.0[usize_range])
    }
}

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

impl<'a> Index<SAPtr> for SuffixArray<'a> {
    type Output = Idx;

    fn index(&self, index: SAPtr) -> &Self::Output {
        &self.0[index.0 as usize]
    }
}

impl<'a> IndexMut<SAPtr> for SuffixArray<'a> {
    fn index_mut(&mut self, index: SAPtr) -> &mut Self::Output {
        &mut self.0[index.0 as usize]
    }
}

impl<'a> SuffixArray<'a> {
    pub fn len(&self) -> Idx {
        self.0.len() as Idx
    }
}

// ---------- Immutable variant ----------- *shakes fist at borrowck*

// Indexes of all suffixes in lexicographical order
#[derive(Debug)]
pub struct SuffixArrayImm<'a>(pub &'a [Idx]);

impl<'a> Index<Idx> for SuffixArrayImm<'a> {
    type Output = Idx;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl<'a> Index<SAPtr> for SuffixArrayImm<'a> {
    type Output = Idx;

    fn index(&self, index: SAPtr) -> &Self::Output {
        &self.0[index.0 as usize]
    }
}

//-------------------------------------------
// Suffix array pointers
//-------------------------------------------

#[derive(Clone, Copy)]
pub struct SAPtr(pub Idx);

use std::fmt;

impl fmt::Display for SAPtr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for SAPtr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SAPtr({})", self.0)
    }
}

impl Div<Idx> for SAPtr {
    type Output = SAPtr;

    #[inline(always)]
    fn div(self, rhs: Idx) -> Self::Output {
        SAPtr(self.0 / rhs)
    }
}

impl Add<Idx> for SAPtr {
    type Output = SAPtr;

    #[inline(always)]
    fn add(self, rhs: Idx) -> Self::Output {
        SAPtr(self.0 + rhs)
    }
}

impl Add<Self> for SAPtr {
    type Output = SAPtr;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        SAPtr(self.0 + rhs.0)
    }
}

impl AddAssign<Idx> for SAPtr {
    fn add_assign(&mut self, rhs: Idx) {
        self.0 += rhs
    }
}

impl AddAssign<Self> for SAPtr {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Sub<Idx> for SAPtr {
    type Output = SAPtr;

    #[inline(always)]
    fn sub(self, rhs: Idx) -> Self::Output {
        SAPtr(self.0 - rhs)
    }
}

impl Sub<Self> for SAPtr {
    type Output = SAPtr;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        SAPtr(self.0 - rhs.0)
    }
}

impl Into<Idx> for SAPtr {
    #[inline(always)]
    fn into(self) -> Idx {
        self.0
    }
}

impl From<Idx> for SAPtr {
    #[inline(always)]
    fn from(idx: Idx) -> Self {
        SAPtr(idx)
    }
}

impl SubAssign<Idx> for SAPtr {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Idx) {
        self.0 -= rhs
    }
}

impl SubAssign<Self> for SAPtr {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl PartialEq<Idx> for SAPtr {
    #[inline(always)]
    fn eq(&self, other: &Idx) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<Idx> for SAPtr {
    #[inline(always)]
    fn partial_cmp(&self, other: &Idx) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialEq<SAPtr> for Idx {
    #[inline(always)]
    fn eq(&self, other: &SAPtr) -> bool {
        *self == other.0
    }
}

impl PartialOrd<SAPtr> for Idx {
    #[inline(always)]
    fn partial_cmp(&self, other: &SAPtr) -> Option<Ordering> {
        self.partial_cmp(&other.0)
    }
}

impl PartialEq<Self> for SAPtr {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl PartialOrd<Self> for SAPtr {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

//----------------------------------------------
// Bucket types
//----------------------------------------------

pub struct BMixBucket(pub Vec<Idx>);

impl BMixBucket {
    #[inline(always)]
    pub fn b<'a>(&'a mut self) -> BBucket<'a> {
        BBucket(&mut self.0)
    }

    #[inline(always)]
    pub fn bstar<'a>(&'a mut self) -> BStarBucket<'a> {
        BStarBucket(&mut self.0)
    }
}

pub struct ABucket(pub Vec<Idx>);

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

pub struct BBucket<'a>(pub &'a mut [Idx]);

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

pub struct BStarBucket<'a>(&'a mut [Idx]);

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
