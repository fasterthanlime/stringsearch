
# sacapart

Computing the suffix array (the lexicographic order of all suffixes of a
text) is expensive, especially as the text gets large.

Sometimes, for very large inputs, a compromise is possible. Instead of
computing the suffix array of the *whole text*, we can compute the suffix
array of the first half, and the suffix array of the second half.

Memory usage remains roughly the same (depending on the SACA used), lookup
time gets worse by a constant factor (the number of partitions), and, across
partitions boundaries, worse (shorter) matches are sometimes found.

For some applications, like diffing very large files, this compromise makes
sense. Read the docs and the tests to see if `sacapart` is right for you.

Note: `sacapart` is meant to be used in conjuction with a SACA that supports
`sacabase`, like `divsufsort`.
