use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign};

pub type Char = u8;
pub type Idx = i32;

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
    pub fn dump(&self, label: &str) {
        println!("=> {}", label);
        println!("SA = {:?}", self.0);
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

#[derive(Clone, Copy, Debug)]
pub struct SAPtr(pub Idx);

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

impl SAPtr {
    /// w is for write
    #[inline(always)]
    pub fn w<'a>(&self, sa: &'a mut SuffixArray<'_>) -> SuffixArray<'a> {
        SuffixArray(&mut sa.0[(self.0 as usize)..])
    }

    /// r is for read
    #[inline(always)]
    pub fn r<'a>(&self, sa: &'a SuffixArray<'_>) -> SuffixArrayImm<'a> {
        SuffixArrayImm(&sa.0[(self.0 as usize)..])
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
