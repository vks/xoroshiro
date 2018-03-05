use rand::{Rng, SeedableRng};
use faster::vecs::u64x4;

use super::SplitMix64;

/// A xoroshiro128+ random number generator using SIMD to generate 4 `u64` at a time.
///
/// The xoroshiro128+ algorithm is not suitable for cryptographic purposes, but
/// is very fast and has better statistical properties than `XorShiftRng`.  If
/// you do not know for sure that it fits your requirements, use a more secure
/// one such as `IsaacRng` or `OsRng`.
///
/// The algorithm used here is translated from [the `xoroshiro128plus.c`
/// reference source code](http://xorshift.di.unimi.it/xoroshiro128plus.c) by
/// David Blackman and Sebastiano Vigna. It was adapted to use SIMD.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub struct XoroShiro128x4 {
    s0: u64x4,
    s1: u64x4,
}

/// Shifts the bits to the left by a specified amount, `n`,
/// wrapping the truncated bits to the end of the resulting integer.
///
/// Please note this isn't the same operation as `<<`!
#[inline]
fn rotate_left(x: u64x4, n: u32) -> u64x4 {
    // Protect against undefined behaviour for over-long bit shifts
    const BITS: u32 = 64;
    let n = n % BITS;
    (x << n) | (x >> ((BITS - n) % BITS))
}

impl XoroShiro128x4 {
    /// Return the next random `u64x4`.
    #[inline]
    pub fn next_u64x4(&mut self) -> u64x4 {
        let r = self.s0 + self.s1;
        self.s1 ^= self.s0;
        self.s0 = rotate_left(self.s0, 55) ^ self.s1 ^ (self.s1 << 14);
        self.s1 = rotate_left(self.s1, 36);
        r
    }

    /// Create a new `XoroShiro128x4`.
    #[inline]
    pub fn from_seed(seed: [u64; 8]) -> XoroShiro128x4 {
        for i in 0..4 {
            assert_ne!(&seed[(2*i)..(2*i + 2)], &[0, 0]);
        }
        XoroShiro128x4 {
            s0: u64x4::new(seed[0], seed[2], seed[4], seed[6]),
            s1: u64x4::new(seed[1], seed[3], seed[5], seed[7]),
        }
    }

    /// Create a new `XoroShiro128x4`.  This will use `SplitMix64` to fill the seed.
    #[inline]
    pub fn from_seed_u64(seed: u64) -> XoroShiro128x4 {
        let mut rng = SplitMix64::from_seed(seed);
        XoroShiro128x4::from_seed(generate_seed(&mut rng))
    }
}

/// Use an RNG to generate a valid (non-zero) xoroshiro seed.
fn generate_seed<R: Rng>(rng: &mut R) -> [u64; 8] {
    let mut seed = [0; 8];
    for i in 0..4 {
        let mut s: [u64; 2] = rng.gen();
        while s == [0, 0] {
            s = rng.gen();
        }
        seed[2*i] = s[0];
        seed[2*i + 1] = s[1];
    }
    seed
}

#[test]
fn test_vs_non_simd() {
    use super::XoroShiro128;
    let seed = [0, 1, 2, 3, 4, 5, 6, 7];
    let mut rng_simd = XoroShiro128x4::from_seed(seed);
    let mut rngs = [
        XoroShiro128::from_seed([seed[0], seed[1]]),
        XoroShiro128::from_seed([seed[2], seed[3]]),
        XoroShiro128::from_seed([seed[4], seed[5]]),
        XoroShiro128::from_seed([seed[6], seed[7]]),
    ];
    let r_simd = rng_simd.next_u64x4();
    let rs = [
        rngs[0].next_u64(),
        rngs[1].next_u64(),
        rngs[2].next_u64(),
        rngs[3].next_u64(),
    ];
    assert_eq!(r_simd.extract(0), rs[0]);
    assert_eq!(r_simd.extract(1), rs[1]);
    assert_eq!(r_simd.extract(2), rs[2]);
    assert_eq!(r_simd.extract(3), rs[3]);
}
