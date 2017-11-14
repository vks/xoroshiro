use std::io::Write;

use rand::{Rng, SeedableRng, Rand};
use byteorder::{LittleEndian, ByteOrder};

use super::SplitMix64;

/// A xorshift1024*φ random number generator.
///
/// The xorshift1024*φ algorithm is not suitable for cryptographic purposes, but
/// is very fast and has a huge period.  If you do not know for sure that it fits
/// your requirements, use a more secure one such as `IsaacRng` or `OsRng`.
///
/// The algorithm used here is translated from [the `xoroshiro1024star.c`
/// reference source code](http://xorshift.di.unimi.it/xoroshiro1024star.c) by
/// Sebastiano Vigna.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub struct XorShift1024 {
    s: [u64; 16],
    p: usize,
}

impl XorShift1024 {
    /// Creates a new `XorShift1024` instance which is not seeded.
    ///
    /// The initial values of this RNG are constants, so all generators created
    /// by this function will yield the same stream of random numbers. It is
    /// highly recommended that this is created through `SeedableRng` instead of
    /// this function.
    pub fn new_unseeded() -> XorShift1024 {
        XorShift1024::from_seed(0)
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
    /// use xoroshiro::rng::XorShift1024;
    ///
    /// let rng1 = XorShift1024::from_seed(0);
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

impl Rng for XorShift1024 {
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

    #[inline]
    fn fill_bytes(&mut self, mut dest: &mut [u8]) {
        let mut to_write = dest.len();
        let mut buf = [0; 64 / 8];
        while to_write > 0 {
            LittleEndian::write_u64(&mut buf, self.next_u64());
            match dest.write(&buf) {
                Ok(n) => to_write -= n,
                Err(e) => panic!("XorShift1024::fill_bytes failed: {}", e),
            }
        }
    }
}

impl SeedableRng<[u64; 16]> for XorShift1024 {
    /// Reseed an `XorShift1024`.  This will panic if `seed` is entirely 0.
    fn reseed(&mut self, seed: [u64; 16]) {
        assert!(!seed.iter().all(|&x| x == 0),
            "XoroShiftRng1024.reseed called with an all zero seed.");

        self.s = seed;
        self.p = 0;
    }

    /// Create a new `XorShift1024`.  This will panic if `seed` is entirely 0.
    fn from_seed(seed: [u64; 16]) -> XorShift1024 {
        assert!(!seed.iter().all(|&x| x == 0),
            "XorShift1024::from_seed called with an all zero seed.");

        XorShift1024 {
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

impl SeedableRng<u64> for XorShift1024 {
    /// Reseed an `XorShift1024`.  This will use `SplitMix64` to fill the seed.
    fn reseed(&mut self, seed: u64) {
        let mut rng = SplitMix64::from_seed(seed);
        self.reseed(generate_seed_1024(&mut rng));
    }

    /// Create a new `XorShift1024`.  This will use `SplitMix64` to fill the seed.
    fn from_seed(seed: u64) -> XorShift1024 {
        let mut rng = SplitMix64::from_seed(seed);
        XorShift1024::from_seed(generate_seed_1024(&mut rng))
    }
}

impl Rand for XorShift1024 {
    fn rand<R: Rng>(rng: &mut R) -> XorShift1024 {
        XorShift1024::from_seed(generate_seed_1024(rng))
    }
}
