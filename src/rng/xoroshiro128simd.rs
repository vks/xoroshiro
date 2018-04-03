use rand_core::{BlockRngCore, Error, RngCore};
use rand_core::impls::BlockRng;
use faster::PackedTransmute;
use faster::vecs::u64x4;
use byteorder::{LittleEndian, ByteOrder};

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
pub struct XoroShiro128x4Core {
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

impl XoroShiro128x4Core {
    /// Return the next random `u64x4`.
    #[inline]
    pub fn next_u64x4(&mut self) -> u64x4 {
        let r = self.s0 + self.s1;
        self.s1 ^= self.s0;
        self.s0 = rotate_left(self.s0, 55) ^ self.s1 ^ (self.s1 << 14);
        self.s1 = rotate_left(self.s1, 36);
        r
    }

    /// Create a new `XoroShiro128x4Core`.
    #[inline]
    pub fn from_seed(seed: [u8; 64]) -> XoroShiro128x4Core {
        for i in 0..4 {
            assert_ne!(&seed[16*i..16*(i + 1)], &[0; 16]);
        }
        XoroShiro128x4Core {
            s0: u64x4::new(
                    LittleEndian::read_u64(&seed[0..8]),
                    LittleEndian::read_u64(&seed[16..24]),
                    LittleEndian::read_u64(&seed[32..40]),
                    LittleEndian::read_u64(&seed[48..56]),
                ),
            s1: u64x4::new(
                    LittleEndian::read_u64(&seed[8..16]),
                    LittleEndian::read_u64(&seed[24..32]),
                    LittleEndian::read_u64(&seed[40..48]),
                    LittleEndian::read_u64(&seed[56..64]),
                ),
        }
    }

    /// Create a new `XoroShiro128x4Core`.  This will use `SplitMix64` to fill the seed.
    #[inline]
    pub fn from_seed_u64(seed: u64) -> XoroShiro128x4Core {
        let mut rng = SplitMix64::from_seed_u64(seed);
        XoroShiro128x4Core::from_seed(generate_seed(&mut rng))
    }
}

/// Use an RNG to generate a valid (non-zero) xoroshiro seed.
fn generate_seed<R: RngCore>(rng: &mut R) -> [u8; 64] {
    let mut seed = [0; 64];
    for i in 0..4 {
        let mut s = &mut seed[i..i*16];
        while s == [0; 16] {
            rng.fill_bytes(&mut s);
        }
    }
    seed
}

impl BlockRngCore for XoroShiro128x4Core {
    type Item = u32;
    type Results = [u32; 8];

    #[inline]
    fn generate(&mut self, results: &mut Self::Results) {
        let r = self.next_u64x4().be_u32s();
        r.store(results, 0);
    }
}

#[derive(Clone, Debug)]
pub struct XoroShiro128x4(BlockRng<XoroShiro128x4Core>);

impl XoroShiro128x4 {
    /// Create a new `XoroShiro128x4Core`.
    #[inline]
    pub fn from_seed(seed: [u8; 64]) -> XoroShiro128x4 {
        let results_empty = [0; 8];
        XoroShiro128x4(BlockRng {
            core: XoroShiro128x4Core::from_seed(seed),
            index: results_empty.as_ref().len(),  // generate on first use
            results: results_empty,
        })
    }

    /// Create a new `XoroShiro128x4`.  This will use `SplitMix64` to fill the seed.
    #[inline]
    pub fn from_seed_u64(seed: u64) -> XoroShiro128x4 {
        let results_empty = [0; 8];
        XoroShiro128x4(BlockRng {
            core: XoroShiro128x4Core::from_seed_u64(seed),
            index: results_empty.as_ref().len(),  // generate on first use
            results: results_empty,
        })
    }
}

impl RngCore for XoroShiro128x4 {
    #[inline(always)]
    fn next_u32(&mut self) -> u32 {
        self.0.next_u32()
    }

    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.fill_bytes(dest);
    }

    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.0.try_fill_bytes(dest)
    }
}

#[test]
fn test_vs_non_simd() {
    use ::rand_core::SeedableRng;
    use super::XoroShiro128;

    let mut seed = [0; 64];
    LittleEndian::write_u64(&mut seed[0..8], 0);
    LittleEndian::write_u64(&mut seed[8..16], 1);
    LittleEndian::write_u64(&mut seed[16..24], 2);
    LittleEndian::write_u64(&mut seed[24..32], 3);
    LittleEndian::write_u64(&mut seed[32..40], 4);
    LittleEndian::write_u64(&mut seed[40..48], 5);
    LittleEndian::write_u64(&mut seed[48..56], 6);
    LittleEndian::write_u64(&mut seed[56..64], 7);

    let mut rng_simd = XoroShiro128x4Core::from_seed(seed);

    fn xoroshiro_from_slice(slice: &[u8]) -> XoroShiro128 {
        let mut seed = [0; 16];
        for (x, y) in slice.iter().zip(seed.iter_mut()) {
            *y = *x;
        }
        XoroShiro128::from_seed(seed)
    }

    let mut rngs = [
        xoroshiro_from_slice(&seed[0..16]),
        xoroshiro_from_slice(&seed[16..32]),
        xoroshiro_from_slice(&seed[32..48]),
        xoroshiro_from_slice(&seed[48..64]),
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
