use rand_core;
use rand_core::{RngCore, SeedableRng};
use byteorder::{LittleEndian, ByteOrder};

use super::SplitMix64;

/// A xoroshiro128+ random number generator.
///
/// The xoroshiro128+ algorithm is not suitable for cryptographic purposes, but
/// is very fast and has better statistical properties than `XorShiftRng`.  If
/// you do not know for sure that it fits your requirements, use a more secure
/// one such as `IsaacRng` or `OsRng`.
///
/// The algorithm used here is translated from [the `xoroshiro128plus.c`
/// reference source code](http://xorshift.di.unimi.it/xoroshiro128plus.c) by
/// David Blackman and Sebastiano Vigna.
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

    pub fn from_seed_u64(seed: u64) -> XoroShiro128 {
        let mut rng = SplitMix64::from_seed_u64(seed);
        XoroShiro128::from_rng(&mut rng).unwrap()
    }

    /// Jump forward, equivalently to 2^64 calls to `next_u64()`.
    ///
    /// This can be used to generate 2^64 non-overlapping subsequences for
    /// parallel computations.
    ///
    /// ```
    /// # extern crate rand;
    /// # extern crate xoroshiro;
    /// # fn main() {
    /// use rand::SeedableRng;
    /// use xoroshiro::rng::XoroShiro128;
    ///
    /// let rng1 = XoroShiro128::from_seed_u64(0);
    /// let mut rng2 = rng1.clone();
    /// rng2.jump();
    /// let mut rng3 = rng2.clone();
    /// rng3.jump();
    /// # }
    /// ```
    pub fn jump(&mut self) {
        const JUMP: [u64; 2] = [0xbeac0467eba5facb, 0xd86b048b86aa9922];
        let mut s0 = 0;
        let mut s1 = 0;
        for j in &JUMP {
            for b in 0..64 {
                if (j & 1 << b) != 0 {
                    s0 ^= self.s0;
                    s1 ^= self.s1;
                }
                self.next_u64();
            }
        }
        self.s0 = s0;
        self.s1 = s1;
    }
}

impl RngCore for XoroShiro128 {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        // The two lowest bits have some linear dependencies, so we use the
        // upper bits instead.
        (self.next_u64() >> 32) as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let r = self.s0.wrapping_add(self.s1);
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

    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl SeedableRng for XoroShiro128 {
    type Seed = [u8; 16];

    /// Create a new `XoroShiro128`.  This will panic if `seed` is entirely 0.
    fn from_seed(seed: [u8; 16]) -> XoroShiro128 {
        assert!(seed != [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            "XoroShiro128::from_seed called with an all zero seed.");

        XoroShiro128 {
            s0: LittleEndian::read_u64(&seed[..8]),
            s1: LittleEndian::read_u64(&seed[8..]),
        }
    }
}
