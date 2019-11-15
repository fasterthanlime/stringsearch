use num_traits::ToPrimitive;
use std::{cmp::min, fmt};

pub struct LongestCommonSubstring<'a> {
    text: &'a [u8],
    start: usize,
    len: usize,
}

impl<'a> fmt::Debug for LongestCommonSubstring<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "T[{}..{}]", self.start, self.start + self.len)
    }
}

impl<'a> LongestCommonSubstring<'a> {
    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8] {
        &self.text[self.start..self.start + self.len]
    }

    #[inline(always)]
    pub fn start(&self) -> usize {
        self.start
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }
}

/// Returns the number of bytes `a` and `b` have in common.
/// Ex: `common_prefix_len("banana", "banter") = 3`
#[inline(always)]
pub fn common_prefix_len(a: &[u8], b: &[u8]) -> usize {
    // TODO: try to exploit SSE 4.2
    let n = min(a.len(), b.len());
    for i in 0..n {
        if a[i] != b[i] {
            return i;
        }
    }
    n
}

/// Searches for the longest substring match for `needle`
/// in `input`, using its suffix array `sa`.
pub fn longest_substring_match<'a, Index>(
    text: &'a [u8],
    mut sa: &[Index],
    needle: &[u8],
) -> LongestCommonSubstring<'a>
where
    Index: num_traits::ToPrimitive,
{
    macro_rules! sa {
        ($x: expr) => {
            sa[$x].to_usize().unwrap()
        };
    }

    macro_rules! suff {
        ($x: expr) => {
            &text[sa!($x)..]
        };
    }

    macro_rules! len {
        ($x: expr) => {
            common_prefix_len(suff!($x), needle)
        };
    }

    macro_rules! lcs {
        ($start: expr, $len: expr) => {
            LongestCommonSubstring {
                text: text,
                start: $start,
                len: $len,
            }
        };
    }

    loop {
        match sa.len() {
            1 => {
                return lcs!(sa!(0), len!(0));
            }
            2 => {
                let x = len!(0);
                let y = len!(1);
                return if x > y {
                    lcs!(sa!(0), x)
                } else {
                    lcs!(sa!(1), y)
                };
            }
            _ => {
                let mid = sa.len() / 2;
                if needle > suff!(mid) {
                    sa = &sa[mid..];
                } else {
                    sa = &sa[..=mid];
                }
            }
        }
    }
}

/// Error returned by `verify` when a suffix array is not sorted.
pub struct NotSorted {
    i: usize,
    j: usize,
}

impl fmt::Debug for NotSorted {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "invariant doesn't hold: suf(SA({})) < suf(SA({}))",
            self.i, self.j
        )
    }
}

impl fmt::Display for NotSorted {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for NotSorted {}

/// Returns an error if `sa` is not the suffix array of `input`,
/// Ok(()) otherwise.
pub fn verify<Index>(input: &[u8], sa: &[Index]) -> Result<(), NotSorted>
where
    Index: ToPrimitive,
{
    macro_rules! sa {
        ($x: expr) => {
            sa[$x].to_usize().unwrap()
        };
    }

    macro_rules! suff {
        ($x: expr) => {
            &input[sa!($x)..]
        };
    }

    for i in 0..(input.len() - 1) {
        if !(suff!(i) < suff!(i + 1)) {
            return Err(NotSorted { i: i, j: i + 1 });
        }
    }
    Ok(())
}

/// A suffix array
pub struct SuffixArray<'a, Index>
where
    Index: ToPrimitive,
{
    sa: Vec<Index>,
    text: &'a [u8],
}

impl<'a, Index> SuffixArray<'a, Index>
where
    Index: ToPrimitive,
{
    /// Create an instance of SuffixArray, taking ownership of `sa`
    pub fn new(text: &'a [u8], sa: Vec<Index>) -> Self {
        Self { sa, text }
    }

    /// Returns the longest
    pub fn longest_substring_match(&self, needle: &[u8]) -> LongestCommonSubstring<'a> {
        longest_substring_match(self.text, &self.sa[..], needle)
    }

    /// Return (text, sa), giving back ownership of `sa`
    pub fn into_parts(self) -> (&'a [u8], Vec<Index>) {
        (self.text, self.sa)
    }

    pub fn verify(&self) -> Result<(), NotSorted> {
        verify(self.text, &self.sa[..])
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
