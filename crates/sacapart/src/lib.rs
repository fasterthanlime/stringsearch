use num_traits::ToPrimitive;
use rayon::prelude::*;
use sacabase::{LongestCommonSubstring, StringIndex, SuffixArray};

/// A partitioned suffix array, that is faster to construct but finds
/// slightly worse matches in a slightly longer amount of time.
///
/// Suffix sorting is an expensive operation that is hard to parallelize
/// well. The idea behind a partitioned suffix array is to suffix sort
/// multiple parts of a text (in parallel) rather than the full text.
///
/// Using two partitions will result in *roughly* 2x faster construction
/// (assuming there are two cores available), but search will now take
/// O(2 * log n), and matches across the boundaries may be much worse.
///
/// For example, the text "totor" may be partitioned into "tot" and "or".
/// Looking for matches for "tor" may well only return a substring of "to",
/// at offset 0, because the first partition only has the suffixes "t",
/// "to", and "tot". The second partition only has the suffixes "or" and "r".
/// So, it finds the substring "(to)tor", tries to extend it to the right,
/// and fails. It doesn't try to extend "to(t)or", because in the suffix
/// array of that partition, that substring is a weaker match than "(to)tor".
///
/// For some applications (like bsdiff-like algorithms), this is an acceptable
/// tradeoff (the resulting patch will be slightly larger). For others, it isn't.
pub struct PartitionedSuffixArray<'a, Index>
where
    Index: ToPrimitive + Send,
{
    partition_size: usize,
    text: &'a [u8],
    sas: Vec<SuffixArray<'a, Index>>,
}

impl<'a, Index> PartitionedSuffixArray<'a, Index>
where
    Index: ToPrimitive + Send,
{
    pub fn new<F>(text: &'a [u8], num_partitions: usize, f: F) -> Self
    where
        F: Fn(&'a [u8]) -> SuffixArray<'a, Index> + Sync,
    {
        let partition_size = text.len() / num_partitions + 1;

        let mut sas: Vec<_> = text
            .par_chunks(text.len() / num_partitions + 1)
            .enumerate()
            .map(|(i, chunk)| (i, f(chunk)))
            .collect();
        sas.sort_by(|(i, _), (j, _)| i.cmp(j));
        let sas = sas.into_iter().map(|(_, chunk)| chunk).collect();

        Self {
            partition_size,
            text,
            sas,
        }
    }

    pub fn num_partitions(&self) -> usize {
        self.sas.len()
    }
}

impl<'a, Index> StringIndex<'a> for PartitionedSuffixArray<'a, Index>
where
    Index: ToPrimitive + Send,
{
    fn longest_substring_match(&self, needle: &[u8]) -> LongestCommonSubstring<'a> {
        let mut best_lcs: Option<LongestCommonSubstring> = None;
        for (i, sa) in self.sas.iter().enumerate() {
            let mut lcs = sa.longest_substring_match(needle);
            let offset = i * self.partition_size;

            // if match reaches the end of the partition's text, it may be
            // extended.
            let may_extend = lcs.start + lcs.len == sa.text().len();

            // start was relative to the partition's beginning, make it absolute
            lcs.start += offset;
            lcs.text = self.text;
            if may_extend {
                lcs.len = sacabase::common_prefix_len(&self.text[lcs.start..], needle);
            }

            let replace = match best_lcs {
                None => true,
                Some(ref prev_lcs) => lcs.len > prev_lcs.len,
            };
            if replace {
                best_lcs.replace(lcs);
            }
        }
        best_lcs.expect(
            "partitioned suffix arrays should always find at least one longest common substring",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sacabase::StringIndex;

    #[test]
    fn worse_test() {
        let input = "totor";
        let sa_full = divsufsort::sort(input.as_bytes());
        let sa_part = PartitionedSuffixArray::new(input.as_bytes(), 2, divsufsort::sort);

        let needle = "tor";

        let full_match = sa_full.longest_substring_match(needle.as_bytes());
        assert_eq!(needle.as_bytes(), full_match.as_bytes());

        let part_match = sa_part.longest_substring_match(needle.as_bytes());
        assert_eq!(needle[..2].as_bytes(), part_match.as_bytes());

        let needle = "otor";

        let full_match = sa_full.longest_substring_match(needle.as_bytes());
        assert_eq!(needle.as_bytes(), full_match.as_bytes());

        let part_match = sa_part.longest_substring_match(needle.as_bytes());
        assert_eq!(needle.as_bytes(), part_match.as_bytes());
    }

    #[test]
    fn equivalent_test() {
        let input = "This is a rather long text. We can probably find matches that span two partitions. Oh yes.";
        let sa_full = divsufsort::sort(input.as_bytes());

        for &partitions in &[1, 2, 3] {
            println!("{} partitions", partitions);
            for needle in &[
                "rather long",
                "text. We can",
                "We can probably find matches that span",
            ] {
                println!("needle: {:?}", needle);
                let sa_part =
                    PartitionedSuffixArray::new(input.as_bytes(), partitions, divsufsort::sort);

                let full_match = sa_full.longest_substring_match(needle.as_bytes());
                let part_match = sa_part.longest_substring_match(needle.as_bytes());

                assert_eq!(
                    full_match.as_bytes(),
                    part_match.as_bytes(),
                    "should find same match bytes for {:?}",
                    needle
                );
                assert_eq!(
                    full_match.start, part_match.start,
                    "should find same match start for {:?}",
                    needle
                );
                assert_eq!(
                    full_match.len, part_match.len,
                    "should find same match len for {:?}",
                    needle
                );
            }
        }
    }
}
