#![allow(dead_code)]
#![allow(unreadable_literal)]

extern crate byteorder;
extern crate rand;
extern crate xoroshiro;

use self::rand::{Rng, SeedableRng, Rand};
use self::byteorder::{LittleEndian, ByteOrder};

use self::xoroshiro::rng::SplitMix64;

/// A xoroshiro128* random number generator.
///
/// The xoroshiro128* algorithm is not suitable for cryptographic purposes, but
/// is very fast and has better statistical properties than `XorShiftRng`.  If
/// you do not know for sure that it fits your requirements, use a more secure
/// one such as `IsaacRng` or `OsRng`.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub struct XoroShiro128 {
    s0: u64,
    s1: u64,
}

impl XoroShiro128 {
    /// Creates a new `XoroShiro128` instance which is not seeded.
    ///
    /// The initial values of this RNG are constants, so all generators created
    /// by this function will yield the same stream of random numbers. It is
    /// highly recommended that this is created through `SeedableRng` instead of
    /// this function.
    pub fn new_unseeded() -> XoroShiro128 {
        // These constants were taken from the `XorShiftRng` implementation.
        // The only requirement imposed by the algorithm is that these values
        // cannot be zero everywhere.
        XoroShiro128 {
            s0: 0x193a6754a8a7d469,
            s1: 0x97830e05113ba7bb,
        }
    }
}

impl Rng for XoroShiro128 {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let r = self.s0.wrapping_mul(self.s1);
        self.s1 ^= self.s0;
        self.s0 = self.s0.rotate_left(55) ^ self.s1 ^ (self.s1 << 14);
        self.s1 = self.s1.rotate_left(36);
        r
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for mut chunk in dest.chunks_mut(8) {
            if chunk.len() == 8 {
                LittleEndian::write_u64(&mut chunk, self.next_u64());
            } else {
                debug_assert!(chunk.len() < 8);
                let r = self.next_u64();
                let mut i = 0;
                for v in chunk.iter_mut() {
                    *v = (r >> 8*i) as u8;
                    i += 1;
                }
            }
        }
    }
}

impl SeedableRng<[u64; 2]> for XoroShiro128 {
    /// Reseed an `XoroShiro128`.  This will panic if `seed` is entirely 0.
    fn reseed(&mut self, seed: [u64; 2]) {
        assert!(seed != [0, 0],
            "XoroShiro128.reseed called with an all zero seed.");

        self.s0 = seed[0];
        self.s1 = seed[1];
    }

    /// Create a new `XoroShiro128`.  This will panic if `seed` is entirely 0.
    fn from_seed(seed: [u64; 2]) -> XoroShiro128 {
        assert!(seed != [0, 0],
            "XoroShiro128::from_seed called with an all zero seed.");

        XoroShiro128 {
            s0: seed[0],
            s1: seed[1],
        }
    }
}

/// Use a RNG to generate a valid (non-zero) xoroshiro seed.
fn generate_seed_128<R: Rng>(rng: &mut R) -> [u64; 2] {
    let mut s: [u64; 2] = rng.gen();
    while s == [0, 0] {
        s = rng.gen();
    }
    s
}

impl SeedableRng<u64> for XoroShiro128 {
    /// Reseed an `XoroShiro128`.  This will use `SplitMix64` to fill the seed.
    fn reseed(&mut self, seed: u64) {
        let mut rng = SplitMix64::from_seed(seed);
        self.reseed(generate_seed_128(&mut rng));
    }

    /// Create a new `XoroShiro128`.  This will use `SplitMix64` to fill the seed.
    fn from_seed(seed: u64) -> XoroShiro128 {
        let mut rng = SplitMix64::from_seed(seed);
        XoroShiro128::from_seed(generate_seed_128(&mut rng))
    }
}

impl Rand for XoroShiro128 {
    fn rand<R: Rng>(rng: &mut R) -> XoroShiro128 {
        XoroShiro128::from_seed(generate_seed_128(rng))
    }
}
