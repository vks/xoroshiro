extern crate aesni;
extern crate byteorder;
extern crate rand;

use self::rand::{RngCore, SeedableRng};
use self::aesni::Aes128;
use self::byteorder::{LittleEndian, ByteOrder};

#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub struct AesRng {
    aes: Aes128,
    key: [u8; 16],
}

impl RngCore for AesRng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let AesRng { aes, mut key } = *self;
        aes.encrypt(&mut key);
        self.aes = Aes128::new(&key);
        LittleEndian::read_u64(&key)
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
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), ::rand::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl SeedableRng for AesRng {
    type Seed = [u8; 16];

    /// Create a new `AesRng`.
    fn from_seed(seed: [u8; 16]) -> AesRng {
        AesRng {
            aes: Aes128::new(&seed),
            key: seed,
        }
    }
}

#[test]
fn test_check_aesni() {
    assert!(self::aesni::check_aesni());
}
