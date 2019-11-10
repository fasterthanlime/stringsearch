use std::ops::{Index, IndexMut};

pub type Char = u8;
pub type Idx = i32;

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
