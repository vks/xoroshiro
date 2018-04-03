use rand_core;
use rand_core::{RngCore, SeedableRng};
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

    pub fn from_seed_u64(seed: u64) -> SplitMix64 {
        let mut x = [0; 8];
        LittleEndian::write_u64(&mut x, seed);
        SplitMix64::from_seed(x)
    }
}

impl RngCore for SplitMix64 {
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

impl SeedableRng for SplitMix64 {
    type Seed = [u8; 8];

    /// Create a new `SplitMix64`.
    fn from_seed(seed: [u8; 8]) -> SplitMix64 {
        SplitMix64 {
            x: LittleEndian::read_u64(&seed),
        }
    }
}
