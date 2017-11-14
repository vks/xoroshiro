use std::io::Write;

use rand::{Rng, SeedableRng, Rand};
use byteorder::{LittleEndian, ByteOrder};

/// A splitmix random number generator.
///
/// The splitmix algorithm is not suitable for cryptographic purposes, but is
/// very fast and has a 64 bit state.  Usually `XoroShiro128` should be prefered.
/// If you do not know for sure that it fits your requirements, use a more
/// secure one such as `IsaacRng` or `OsRng`.
///
/// The algorithm used here is translated from [the `splitmix64.c`
/// reference source code](http://xorshift.di.unimi.it/splitmix64.c) by
/// Sebastiano Vigna.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub struct SplitMix64 {
    x: u64,
}

impl SplitMix64 {
    /// Creates a new `SplitMix64` instance which is not seeded.
    ///
    /// The initial values of this RNG are constants, so all generators created
    /// by this function will yield the same stream of random numbers. It is
    /// highly recommended that this is created through `SeedableRng` instead of
    /// this function.
    pub fn new_unseeded() -> SplitMix64 {
        // The state can be seeded with any value.
        SplitMix64 {
            x: 0,
        }
    }
}

impl Rng for SplitMix64 {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.x = self.x.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.x;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        return z ^ (z >> 31);
    }

    #[inline]
    fn fill_bytes(&mut self, mut dest: &mut [u8]) {
        let mut to_write = dest.len();
        let mut buf = [0; 64 / 8];
        while to_write > 0 {
            LittleEndian::write_u64(&mut buf, self.next_u64());
            match dest.write(&buf) {
                Ok(n) => to_write -= n,
                Err(e) => panic!("SplitMix64::fill_bytes failed: {}", e),
            }
        }
    }
}

impl SeedableRng<u64> for SplitMix64 {
    /// Reseed a `SplitMix64`.
    fn reseed(&mut self, seed: u64) {
        self.x = seed;
    }

    /// Create a new `SplitMix64`.
    fn from_seed(seed: u64) -> SplitMix64 {
        SplitMix64 {
            x: seed,
        }
    }
}

impl Rand for SplitMix64 {
    fn rand<R: Rng>(rng: &mut R) -> SplitMix64 {
        SplitMix64 {
            x: rng.gen(),
        }
    }
}
