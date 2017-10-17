extern crate rand;

use rand::{Rng, SeedableRng, Rand};

/// A splitmix random number generator.
///
/// The splitmix algorithm is not suitable for cryptographic purposes, but is
/// very fast and has a 64 bit state.  Usually `XoroShiroRng128` should be prefered.
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
        self.x = self.x.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.x;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
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
pub struct XoroShiroRng128 {
    s0: u64,
    s1: u64,
}

impl XoroShiroRng128 {
    /// Creates a new `XoroShiroRng128` instance which is not seeded.
    ///
    /// The initial values of this RNG are constants, so all generators created
    /// by this function will yield the same stream of random numbers. It is
    /// highly recommended that this is created through `SeedableRng` instead of
    /// this function.
    pub fn new_unseeded() -> XoroShiroRng128 {
        // These constants were taken from the `XorShiftRng` implementation.
        // The only requirement imposed by the algorithm is that these values
        // cannot be zero everywhere.
        XoroShiroRng128 {
            s0: 0x193a6754a8a7d469,
            s1: 0x97830e05113ba7bb,
        }
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
    /// use xoroshiro::XoroShiroRng128;
    ///
    /// let rng1 = XoroShiroRng128::from_seed(0);
    /// let mut rng2 = rng1.clone();
    /// rng2.jump();
    /// let mut rng3 = rng2.clone();
    /// rng3.jump();
    /// # }
    /// ```
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

impl Rng for XoroShiroRng128 {
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

impl SeedableRng<[u64; 2]> for XoroShiroRng128 {
    /// Reseed an `XoroShiroRng128`.  This will panic if `seed` is entirely 0.
    fn reseed(&mut self, seed: [u64; 2]) {
        assert!(seed != [0, 0],
            "XoroShiroRng128.reseed called with an all zero seed.");

        self.s0 = seed[0];
        self.s1 = seed[1];
    }

    /// Create a new `XoroShiroRng128`.  This will panic if `seed` is entirely 0.
    fn from_seed(seed: [u64; 2]) -> XoroShiroRng128 {
        assert!(seed != [0, 0],
            "XoroShiroRng128::from_seed called with an all zero seed.");

        XoroShiroRng128 {
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

impl SeedableRng<u64> for XoroShiroRng128 {
    /// Reseed an `XoroShiroRng128`.  This will use `SplitMixRng` to fill the seed.
    fn reseed(&mut self, seed: u64) {
        let mut rng = SplitMixRng::from_seed(seed);
        self.reseed(generate_seed_128(&mut rng));
    }

    /// Create a new `XoroShiroRng128`.  This will use `SplitMixRng` to fill the seed.
    fn from_seed(seed: u64) -> XoroShiroRng128 {
        let mut rng = SplitMixRng::from_seed(seed);
        XoroShiroRng128::from_seed(generate_seed_128(&mut rng))
    }
}

impl Rand for XoroShiroRng128 {
    fn rand<R: Rng>(rng: &mut R) -> XoroShiroRng128 {
        XoroShiroRng128::from_seed(generate_seed_128(rng))
    }
}

/// A xorshift1024* random number generator.
///
/// The xorshift1024* algorithm is not suitable for cryptographic purposes, but
/// is very fast and has a huge period.  If you do not know for sure that it fits
/// your requirements, use a more secure one such as `IsaacRng` or `OsRng`.
///
/// The algorithm used here is translated from [the `xoroshiro1024star.c`
/// reference source code](http://xorshift.di.unimi.it/xoroshiro1024star.c) by
/// Sebastiano Vigna.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub struct XorShiftRng1024 {
    s: [u64; 16],
    p: usize,
}

impl XorShiftRng1024 {
    /// Creates a new `XorShiftRng1024` instance which is not seeded.
    ///
    /// The initial values of this RNG are constants, so all generators created
    /// by this function will yield the same stream of random numbers. It is
    /// highly recommended that this is created through `SeedableRng` instead of
    /// this function.
    pub fn new_unseeded() -> XorShiftRng1024 {
        XorShiftRng1024::from_seed(0)
    }

    /// Jump forward, equivalently to 2^512 calls to `next_u64()`.
    ///
    /// This can be used to generate 2^512 non-overlapping subsequences for
    /// parallel computations.
    ///
    /// ```
    /// # extern crate rand;
    /// # extern crate xoroshiro;
    /// # fn main() {
    /// use rand::SeedableRng;
    /// use xoroshiro::XorShiftRng1024;
    ///
    /// let rng1 = XorShiftRng1024::from_seed(0);
    /// let mut rng2 = rng1.clone();
    /// rng2.jump();
    /// let mut rng3 = rng2.clone();
    /// rng3.jump();
    /// # }
    /// ```
    pub fn jump(&mut self) {
        const JUMP: [u64; 16] = [0x84242f96eca9c41d,
            0xa3c65b8776f96855, 0x5b34a39f070b5837, 0x4489affce4f31a1e,
            0x2ffeeb0a48316f40, 0xdc2d9891fe68c022, 0x3659132bb12fea70,
            0xaac17d8efa43cab8, 0xc4cb815590989b13, 0x5ee975283d71c93b,
            0x691548c86c1bd540, 0x7910c41d10a1e6a5, 0x0b5fc64563b3e2a8,
            0x047f7684e9fc949d, 0xb99181f2d8f685ca, 0x284600e3f30e38c3];
        let mut t = [0; 16];
        for j in &JUMP {
            for b in 0..64 {
                if (j & 1 << b) != 0 {
                    for i in 0..16usize {
                        let index = i.wrapping_add(self.p) & 15;
                        t[i] ^= self.s[index];
                    }
                }
                self.next_u64();
            }
        }
        for i in 0..16usize {
            let index = i.wrapping_add(self.p) & 15;
            self.s[index] = t[i];
        }
    }
}

impl Rng for XorShiftRng1024 {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let s0 = self.s[self.p];
        self.p = self.p.wrapping_add(1) & 15;
        let mut s1 = self.s[self.p];
        s1 ^= s1 << 31;
        self.s[self.p] = s1 ^ s0 ^ (s1 >> 11) ^ (s0 >> 30);
        self.s[self.p].wrapping_mul(0x9e3779b97f4a7c13)
    }
}

impl SeedableRng<[u64; 16]> for XorShiftRng1024 {
    /// Reseed an `XorShiftRng1024`.  This will panic if `seed` is entirely 0.
    fn reseed(&mut self, seed: [u64; 16]) {
        assert!(!seed.iter().all(|&x| x == 0),
            "XoroShiftRng1024.reseed called with an all zero seed.");

        self.s = seed;
        self.p = 0;
    }

    /// Create a new `XorShiftRng1024`.  This will panic if `seed` is entirely 0.
    fn from_seed(seed: [u64; 16]) -> XorShiftRng1024 {
        assert!(!seed.iter().all(|&x| x == 0),
            "XorShiftRng1024::from_seed called with an all zero seed.");

        XorShiftRng1024 {
            s: seed,
            p: 0,
        }
    }
}

/// Use a RNG to generate a valid (non-zero) xorshift1024 seed.
fn generate_seed_1024<R: Rng>(rng: &mut R) -> [u64; 16] {
    let mut s: [u64; 16] = rng.gen();
    while s.iter().all(|&x| x == 0) {
        s = rng.gen();
    }
    s
}

impl SeedableRng<u64> for XorShiftRng1024 {
    /// Reseed an `XorShiftRng1024`.  This will use `SplitMixRng` to fill the seed.
    fn reseed(&mut self, seed: u64) {
        let mut rng = SplitMixRng::from_seed(seed);
        self.reseed(generate_seed_1024(&mut rng));
    }

    /// Create a new `XorShiftRng1024`.  This will use `SplitMixRng` to fill the seed.
    fn from_seed(seed: u64) -> XorShiftRng1024 {
        let mut rng = SplitMixRng::from_seed(seed);
        XorShiftRng1024::from_seed(generate_seed_1024(&mut rng))
    }
}

impl Rand for XorShiftRng1024 {
    fn rand<R: Rng>(rng: &mut R) -> XorShiftRng1024 {
        XorShiftRng1024::from_seed(generate_seed_1024(rng))
    }
}
