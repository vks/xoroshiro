extern crate rand;

use rand::{Rng, SeedableRng, Rand};

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
#[derive(Clone)]
pub struct XoroShiroRng {
    s0: u64,
    s1: u64,
}

impl XoroShiroRng {
    /// Creates a new XoroShiroRng instance which is not seeded.
    ///
    /// The initial values of this RNG are constants, so all generators created
    /// by this function will yield the same stream of random numbers. It is
    /// highly recommended that this is created through `SeedableRng` instead of
    /// this function
    pub fn new_unseeded() -> XoroShiroRng {
        // These constants were taken from the XorShiftRng implementation above.
        // The only requirement imposed by the algorithm is that these values
        // cannot be zero everywhere.
        XoroShiroRng {
            s0: 0x193a6754a8a7d469,
            s1: 0x97830e05113ba7bb,
        }
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
    /// Reseed an XoroShiroRng.  This will panic if `seed` is entirely 0.
    fn reseed(&mut self, seed: [u64; 2]) {
        assert!(!seed.iter().all(|&x| x == 0),
                "XoroShiroRng.reseed called with an all zero seed.");

        self.s0 = seed[0];
        self.s1 = seed[1];
    }

    /// Create a new XoroShiroRng.  This will panic if `seed` is entirely 0.
    fn from_seed(seed: [u64; 2]) -> XoroShiroRng {
        assert!(!seed.iter().all(|&x| x == 0),
            "XoroShiroRng::from_seed called with an all zero seed.");

        XoroShiroRng {
            s0: seed[0],
            s1: seed[1],
        }
    }
}

impl Rand for XoroShiroRng {
    fn rand<R: Rng>(rng: &mut R) -> XoroShiroRng {
        let mut seed: [u64; 2] = rng.gen();
        while seed == [0, 0] {
            seed = rng.gen();
        }
        XoroShiroRng {
            s0: seed[0],
            s1: seed[1],
        }
    }
}
