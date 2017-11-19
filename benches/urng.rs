use rand::{Rng, SeedableRng, Rand};
use byteorder::{LittleEndian, ByteOrder};

use xoroshiro::rng::SplitMix64;

#[allow(missing_copy_implementations)]
#[derive(Clone)]
/// 64-bit universal RNG by [Marsaglia and Tsang][1].
///
/// Generates floats directly; generating integers is inefficient.
///
/// [1]: https://doi.org/10.1016/j.spl.2003.11.001
pub struct Urng64 {
    u: [f64; 98],
    c: f64,
    i: usize,
    j: usize,
}

impl Rng for Urng64 {
    #[inline]
    fn next_f64(&mut self) -> f64 {
        const R: f64 = 9007199254740881.0/9007199254740992.;
        const D: f64 = 362436069876.0/9007199254740992.0;
        let mut x = self.u[self.i] - self.u[self.j];
        if x < 0. {
            x += 1.;
        }
        self.u[self.i] = x;
        self.i -= 1;
        if self.i == 0 {
            self.i = 97;
        }
        self.j -= 1;
        if self.j == 0 {
            self.j = 97;
        }
        self.c -= D;
        if self.c < 0. {
            self.c += R;
        }
        x -= self.c;
        if x < 0. {
            x + 1.
        } else {
            x
        }
    }

    #[inline]
    fn next_u32(&mut self) -> u32 {
        (self.next_f64() * 4294967296.) as u32
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

    /*
    pub fn seed(&mut self, seed1: i32, seed2: i32) {
        let mut x = seed1;
        let mut y = seed2;
        for i in 1..98 {
            let mut s = 0.;
            let mut t = 0.5;
            for _ in 1..54 {
                x = 6969i32.wrapping_mul(x) % 65543;
                y = 8888i32.wrapping_mul(y) % 65579;
                if ((x ^ y) & 32) > 0 {
                    s += t;
                }
                t *= 0.5;
            }
            self.u[i] = s;
        }
    }
    */

impl SeedableRng<[f64; 98]> for Urng64 {
    /// Reseed an `Urng64`.
    fn reseed(&mut self, seed: [f64; 98]) {
        let rng = Urng64::from_seed(seed);
        *self = rng;
    }

    /// Create a new `Urng64`.
    fn from_seed(seed: [f64; 98]) -> Urng64 {
        Urng64 {
            u: seed,
            c: 0.,
            i: 97,
            j: 33,
        }
    }
}

fn gen_u(rng: &mut Rng) -> [f64; 98] {
    let mut u = [0.; 98];
    for i in 1..98 {
        u[i] = rng.next_f64();
    }
    u
}

impl SeedableRng<u64> for Urng64 {
    /// Reseed an `Urng64`.  This will use `SplitMix64` to fill the seed.
    fn reseed(&mut self, seed: u64) {
        let mut rng = SplitMix64::from_seed(seed);
        self.reseed(gen_u(&mut rng));
    }

    /// Create a new `Urng64`.  This will use `SplitMix64` to fill the seed.
    fn from_seed(seed: u64) -> Urng64 {
        let mut rng = SplitMix64::from_seed(seed);
        Urng64::from_seed(gen_u(&mut rng))
    }
}

impl Rand for Urng64 {
    fn rand<R: Rng>(rng: &mut R) -> Urng64 {
        Urng64::from_seed(gen_u(rng))
    }
}
