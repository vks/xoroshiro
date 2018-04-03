use rand_core;
use rand_core::{RngCore, SeedableRng};
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
        XorShift1024::from_seed_u64(0)
    }

    pub fn from_seed_u64(seed: u64) -> XorShift1024 {
        let mut rng = SplitMix64::from_seed_u64(seed);
        XorShift1024::from_rng(&mut rng).unwrap()
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
    /// let rng1 = XorShift1024::from_seed_u64(0);
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

impl RngCore for XorShift1024 {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
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

/// Seed for `XorShift1024`.
#[derive(Clone)]
pub struct XorShift1024Seed(pub [u8; 16 * 8]);

impl ::std::convert::From<[u8; 16 * 8]> for XorShift1024Seed {
    fn from(seed: [u8; 16 * 8]) -> XorShift1024Seed {
        XorShift1024Seed(seed)
    }
}

impl ::std::convert::AsMut<[u8]> for XorShift1024Seed {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl ::std::default::Default for XorShift1024Seed {
    fn default() -> XorShift1024Seed {
        XorShift1024Seed([0; 16 * 8])
    }
}

impl SeedableRng for XorShift1024 {
    type Seed = XorShift1024Seed;

    /// Create a new `XorShift1024`.  This will panic if `seed` is entirely 0.
    fn from_seed(seed: XorShift1024Seed) -> XorShift1024 {
        let seed = seed.0;
        assert!(!seed.iter().all(|&x| x == 0),
            "XorShift1024::from_seed called with an all zero seed.");

        XorShift1024 {
            s: [
                LittleEndian::read_u64(&seed[0..8]),
                LittleEndian::read_u64(&seed[8..16]),
                LittleEndian::read_u64(&seed[16..24]),
                LittleEndian::read_u64(&seed[24..32]),
                LittleEndian::read_u64(&seed[32..40]),
                LittleEndian::read_u64(&seed[40..48]),
                LittleEndian::read_u64(&seed[48..56]),
                LittleEndian::read_u64(&seed[56..64]),
                LittleEndian::read_u64(&seed[64..72]),
                LittleEndian::read_u64(&seed[72..80]),
                LittleEndian::read_u64(&seed[80..88]),
                LittleEndian::read_u64(&seed[88..96]),
                LittleEndian::read_u64(&seed[96..104]),
                LittleEndian::read_u64(&seed[104..112]),
                LittleEndian::read_u64(&seed[112..120]),
                LittleEndian::read_u64(&seed[120..128]),
            ],
            p: 0,
        }
    }
}
