Implementation of the primitives used in Zcash and Iron Fish.

This is a fork of the original [`librustzcash`][librustzcash] project from
Zcash. The fork was created by the Iron Fish project to add performance
improvements.

## Delta from upstream

These are the differences between this crate and the upstream
[`librustzcash`][librustzcash]:

* Changed the elliptic curve backend from [`bls12_381`][bls12_381] to
  [`blstrs`][blstrs]

[librustzcash]: https://github.com/zcash/librustzcash
[bls12_381]: https://crates.io/crates/bls12_381
[blstrs]: https://crates.io/crates/blstrs
