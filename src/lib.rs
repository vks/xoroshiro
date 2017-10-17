extern crate rand;

use rand::{Rng, SeedableRng, Rand};

/// A splitmix random number generator.
///
/// The splitmix algorithm is not suitable for cryptographic purposes, but is
/// very fast and has a 64 bit state.  Usually `XoroShiroRng` should be prefered.
/// If you do not know for sure that it fits your requirements, use a more
/// secure one such as `IsaacRng` or `OsRng`.
///
/// The algorithm used here is translated from [the `splitmix64.c`
/// reference source code](http://xorshift.di.unimi.it/splitmix64.c) by
/// Sebastiano Vigna.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub struct SplitMixRng {
    x: u64,
}

impl SplitMixRng {
    /// Creates a new `SplitMixRng` instance which is not seeded.
    ///
    /// The initial values of this RNG are constants, so all generators created
    /// by this function will yield the same stream of random numbers. It is
    /// highly recommended that this is created through `SeedableRng` instead of
    /// this function.
    pub fn new_unseeded() -> SplitMixRng {
        // The state can be seeded with any value.
        SplitMixRng {
            x: 0,
        }
    }
}

impl Rng for SplitMixRng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.x += 0x9e3779b97f4a7c15;
        let mut z = self.x;
        z = (z ^ (z >> 30)) * 0xbf58476d1ce4e5b9;
        z = (z ^ (z >> 27)) * 0x94d049bb133111eb;
        return z ^ (z >> 31);
    }
}

impl SeedableRng<u64> for SplitMixRng {
    /// Reseed a `SplitMixRng`.
    fn reseed(&mut self, seed: u64) {
        self.x = seed;
    }

    /// Create a new `SplitMixRng`.
    fn from_seed(seed: u64) -> SplitMixRng {
        SplitMixRng {
            x: seed,
        }
    }
}

impl Rand for SplitMixRng {
    fn rand<R: Rng>(rng: &mut R) -> SplitMixRng {
        SplitMixRng {
            x: rng.gen(),
        }
    }
}

/// A xoroshiro+ random number generator.
///
/// The xoroshiro+ algorithm is not suitable for cryptographic purposes, but
/// is very fast and has better statistical properties than `XorShiftRng`.  If
/// you do not know for sure that it fits your requirements, use a more secure
/// one such as `IsaacRng` or `OsRng`.
///
/// The algorithm used here is translated from [the `xoroshiro128plus.c`
/// reference source code](http://xorshift.di.unimi.it/xoroshiro128plus.c) by
/// David Blackman and Sebastiano Vigna.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub struct XoroShiroRng {
    s0: u64,
    s1: u64,
}

impl XoroShiroRng {
    /// Creates a new `XoroShiroRng` instance which is not seeded.
    ///
    /// The initial values of this RNG are constants, so all generators created
    /// by this function will yield the same stream of random numbers. It is
    /// highly recommended that this is created through `SeedableRng` instead of
    /// this function.
    pub fn new_unseeded() -> XoroShiroRng {
        // These constants were taken from the `XorShiftRng` implementation.
        // The only requirement imposed by the algorithm is that these values
        // cannot be zero everywhere.
        XoroShiroRng {
            s0: 0x193a6754a8a7d469,
            s1: 0x97830e05113ba7bb,
        }
    }

    /// Jump forward, equivalently to 2^64 calls to `next_u64()`.
    ///
    /// This can be used to generate 2^64 non-overlapping subsequences for
    /// parallel computations.
    pub fn jump(&mut self) {
        const JUMP: [u64; 2] = [0x8a5cd789635d2dff, 0x121fd2155c472f96];
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

impl Rng for XoroShiroRng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let r = self.s0.wrapping_add(self.s1);
        self.s1 ^= self.s0;
        self.s0 = self.s0.rotate_left(55) ^ self.s1 ^ (self.s1 << 14);
        self.s1 = self.s1.rotate_left(36);
        r
    }
}

impl SeedableRng<[u64; 2]> for XoroShiroRng {
    /// Reseed an `XoroShiroRng`.  This will panic if `seed` is entirely 0.
    fn reseed(&mut self, seed: [u64; 2]) {
        assert!(seed != [0, 0],
            "XoroShiroRng.reseed called with an all zero seed.");

        self.s0 = seed[0];
        self.s1 = seed[1];
    }

    /// Create a new `XoroShiroRng`.  This will panic if `seed` is entirely 0.
    fn from_seed(seed: [u64; 2]) -> XoroShiroRng {
        assert!(seed != [0, 0],
            "XoroShiroRng::from_seed called with an all zero seed.");

        XoroShiroRng {
            s0: seed[0],
            s1: seed[1],
        }
    }
}

/// Use a RNG to generate a valid (non-zero) xoroshiro seed.
fn generate_seed<R: Rng>(rng: &mut R) -> [u64; 2] {
    let mut s: [u64; 2] = rng.gen();
    while s == [0, 0] {
        s = rng.gen();
    }
    s
}

impl SeedableRng<u64> for XoroShiroRng {
    /// Reseed an `XoroShiroRng`.  This will use `SplitMixRng` to fill the seed.
    fn reseed(&mut self, seed: u64) {
        let mut rng = SplitMixRng::from_seed(seed);
        self.reseed(generate_seed(&mut rng));
    }

    /// Create a new `XoroShiroRng`.  This will use `SplitMixRng` to fill the seed.
    fn from_seed(seed: u64) -> XoroShiroRng {
        let mut rng = SplitMixRng::from_seed(seed);
        XoroShiroRng::from_seed(generate_seed(&mut rng))
    }
}

impl Rand for XoroShiroRng {
    fn rand<R: Rng>(rng: &mut R) -> XoroShiroRng {
        XoroShiroRng::from_seed(generate_seed(rng))
    }
}
