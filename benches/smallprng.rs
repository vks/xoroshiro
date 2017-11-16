extern crate rand;
extern crate byteorder;

use self::rand::{Rng, SeedableRng, Rand};
use self::byteorder::{LittleEndian, ByteOrder};

#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub struct SmallPrng128 {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

#[inline]
fn rot(x: u32, k: u32) -> u32 {
    (x << k) | (x >> (32 - k))
}

impl Rng for SmallPrng128 {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        let e = self.a.wrapping_sub(rot(self.b, 27));
        self.a = self.b ^ rot(self.c, 17);
        self.b = self.c.wrapping_add(self.d);
        self.c = self.d.wrapping_add(e);
        self.d = e.wrapping_add(self.a);
        self.d
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

impl SeedableRng<[u32; 3]> for SmallPrng128 {
    /// Reseed a `SmallPrng128`.
    fn reseed(&mut self, seed: [u32; 3]) {
        *self = SmallPrng128 {
            a: 0xf1ea5eed,
            b: seed[0],
            c: seed[1],
            d: seed[2],
        };
        for _ in 0..2 {
            self.next_u32();
        }
    }

    /// Create a new `SmallPrng128`.
    fn from_seed(seed: [u32; 3]) -> SmallPrng128 {
        let mut rng = SmallPrng128 {
            a: 0xf1ea5eed,
            b: seed[0],
            c: seed[1],
            d: seed[2],
        };
        for _ in 0..2 {
            rng.next_u32();
        }
        rng
    }
}

impl SeedableRng<u32> for SmallPrng128 {
    /// Reseed a `SmallPrng128`.
    fn reseed(&mut self, seed: u32) {
        self.reseed([seed, seed, seed]);
    }

    /// Create a new `SmallPrng128`.
    fn from_seed(seed: u32) -> SmallPrng128 {
        SmallPrng128::from_seed([seed, seed, seed])
    }
}

impl SeedableRng<u64> for SmallPrng128 {
    /// Reseed a `SmallPrng128`.
    fn reseed(&mut self, seed: u64) {
        self.reseed([seed as u32, (seed >> 32) as u32, seed as u32]);
    }

    /// Create a new `SmallPrng128`.
    fn from_seed(seed: u64) -> SmallPrng128 {
        SmallPrng128::from_seed([seed as u32, (seed >> 32) as u32, seed as u32])
    }
}


impl Rand for SmallPrng128 {
    fn rand<R: Rng>(rng: &mut R) -> SmallPrng128 {
        let seed: [u32; 3] = rng.gen();
        SmallPrng128::from_seed(seed)
    }
}
