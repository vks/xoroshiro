//! This crate provides the [xoroshiro128+, xorshift1024*φ and
//! splitmix64](http://xoroshiro.di.unimi.it) random number generators.
//!
//! It is recommended to use `XoroShiro128` unless you need a period larger
//! than 2^128 - 1, where `XorShift1024` with a period of 2^1024 - 1 is more
//! appropriate. `SplitMix64` is only used to initialize the other generators,
//! it should not be used directly.

extern crate rand;
extern crate byteorder;
extern crate faster;

/// Pseudo-random number generators.
pub mod rng;
