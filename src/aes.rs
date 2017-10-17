extern crate aesni;
extern crate byteorder;

use std::io::Write;

use rand::{Rng, SeedableRng, Rand};
use self::aesni::{Aes128, check_aesni};
use self::byteorder::{LittleEndian, ReadBytesExt};

#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub struct AesRng {
    aes: Aes128,
    key: [u8; 16],
}

impl Rng for AesRng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let AesRng { aes, mut key } = *self;
        aes.encrypt(&mut key);
        self.aes = Aes128::new(&key);
        key.as_ref().read_u64::<LittleEndian>().unwrap()
    }

    #[inline]
    fn fill_bytes(&mut self, mut dest: &mut [u8]) {
        let mut to_write = dest.len();
        while to_write > 0 {
            let AesRng { aes, mut key } = *self;
            aes.encrypt(&mut key);
            self.aes = Aes128::new(&key);
            match dest.write(&key) {
                Ok(n) => to_write -= n,
                Err(e) => panic!("AesRng::fill_bytes failed: {}", e),
            }
        }
    }
}

impl SeedableRng<[u8; 16]> for AesRng {
    /// Reseed an `AesRng`.
    fn reseed(&mut self, seed: [u8; 16]) {
        self.aes = Aes128::new(&seed);
        self.key = seed;
    }

    /// Create a new `AesRng`.
    fn from_seed(seed: [u8; 16]) -> AesRng {
        AesRng {
            aes: Aes128::new(&seed),
            key: seed,
        }
    }
}

impl Rand for AesRng {
    fn rand<R: Rng>(rng: &mut R) -> AesRng {
        let key = rng.gen();
        AesRng {
            aes: Aes128::new(&key),
            key: key,
        }
    }
}

#[test]
fn test_check_aesni() {
    assert!(check_aesni());
}
