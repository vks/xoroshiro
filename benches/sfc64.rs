extern crate rand;
extern crate byteorder;

use self::rand::{Rng, SeedableRng, Rand};
use self::byteorder::{LittleEndian, ByteOrder};

#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub struct Sfc64 {
    a: u64,
    b: u64,
    c: u64,
    counter: u64,
}

impl Rng for Sfc64 {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        const RSHIFT: u64 = 11;
        const LSHIFT: u64 = 3;
        const BARREL_SHIFT: u64 = 24;

        let tmp = self.a + self.b + self.counter;
        self.counter += 1;
        self.a = self.b ^ (self.b >> RSHIFT);
        self.b = self.c.wrapping_add(self.c << LSHIFT);
        self.c = ((self.c << BARREL_SHIFT) | (self.c >> (64 - BARREL_SHIFT))) + tmp;
        tmp
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

impl SeedableRng<[u64; 3]> for Sfc64 {
    /// Reseed a `Sfc64`.
    fn reseed(&mut self, seed: [u64; 3]) {
        *self = Sfc64 {
            a: seed[0],
            b: seed[1],
            c: seed[2],
            counter: 1,
        };
        for _ in 0..18 {
            self.next_u64();
        }
    }

    /// Create a new `Sfc64`.
    fn from_seed(seed: [u64; 3]) -> Sfc64 {
        let mut rng = Sfc64 {
            a: seed[0],
            b: seed[1],
            c: seed[2],
            counter: 1,
        };
        for _ in 0..18 {
            rng.next_u64();
        }
        rng
    }
}

impl SeedableRng<u64> for Sfc64 {
    /// Reseed a `Sfc64`.
    fn reseed(&mut self, seed: u64) {
        *self = Sfc64 {
            a: seed,
            b: seed,
            c: seed,
            counter: 1,
        };
        for _ in 0..18 {
            self.next_u64();
        }
    }

    /// Create a new `Sfc64`.
    fn from_seed(seed: u64) -> Sfc64 {
        let mut rng = Sfc64 {
            a: seed,
            b: seed,
            c: seed,
            counter: 1,
        };
        for _ in 0..12 {
            rng.next_u64();
        }
        rng
    }
}


impl Rand for Sfc64 {
    fn rand<R: Rng>(rng: &mut R) -> Sfc64 {
        let seed: [u64; 3] = rng.gen();
        Sfc64::from_seed(seed)
    }
}
