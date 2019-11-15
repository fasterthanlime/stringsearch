
# divsufsort

This crate contains a Rust handmade port of Yuta Mori's `libdivsufsort`, as found on:

  * <https://github.com/y-256/libdivsufsort/tree/5f60d6f026c30fb4ac296f696b3c8b0eb71bd428>

## Changes

The main changes from the C codebase are as follows.

Instead of passing pointers to T (the original text) and SA (the suffix array),
slices and indices are passed instead. This sometimes involves adding more parameters
to functions (like `tr_heapsort`).

Some macros (for stacks, used in `sssort` and `trsort`) have been replaced with
proper Rust types. The `SAPtr` type is used to represent an index into `SA`.
A/B/B* access has also been translated from C macros to Rust (inlined) functions.

Cross-checking is only built when the `crosscheck` feature is enabled. It is
not intended for general use, only for debugging the `divsufsort` crate.

## Authors

The original C code was written by Yuta Mori.

The port was done by hand, by [Amos Wenger](https://github.com/fasterthanlime).

## License

`divsufsort` is released under the MIT license, same as the original.

See the `LICENSE` file for details.
