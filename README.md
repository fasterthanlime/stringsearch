
# suffixsearch

[![Build Status](https://travis-ci.org/fasterthanlime/suffixsearch.svg?branch=master)](https://travis-ci.org/fasterthanlime/suffixsearch)

A collection of SACAs (suffix array construction algorithms) and other
methods of indexing and searching for substrings in all suffixes of a
given input.

## Crates

  * [divsufsort](crates/divsufsort) is Rust version of Yuta Mori's `libdivsufsort`, ported by hand
  * [cdivsfusort](crates/cdivsufsort) is Yuta Mori's original `libdivsufsort`, built with the `cc` crate
  * [divsuftest](crates/divsuftest) is a test executable that allows comparing against the
    above crates.
  * [dc3](crates/dc3) is a naive work-in-progress implementation of DC3 (Differential Cover, v=3)

See the crates' README files for more information on their status,
expected performance and licensing.
