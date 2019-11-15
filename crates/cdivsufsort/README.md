
# cdivsufsort

This crate contains Yuta Mori's C codebase `libdivsufsort`, as found on:

  * <https://github.com/y-256/libdivsufsort/tree/5f60d6f026c30fb4ac296f696b3c8b0eb71bd428>

...and a minimal Rust interface to it.

## Changes

There are no functional changes to the codebase, however:

  * Parts of the code have been formatted with clang-format (LLVM style)
  * Many of the loops (for, do..while) have been given names in comments, for
    ease of translation.
  * The codebase contains "cross-checking" facilities (the macros `crosscheck`,
    `SA_dump`, etc.) so its behavior can be compared with the Rust port.

Cross-checking is only built when the `crosscheck` feature is enabled. It is
not intended for general use, only for debugging the `divsufsort` crate.

## Further reading

The divsufsort algorithm is based on "",

## Authors

The original code was written by Yuta Mori, and its essence is not changed
here.

## License

`cdivsufsort` is released under the MIT license, same as the original.

See the `LICENSE` and `c-sources/LICENSE` files for details.

