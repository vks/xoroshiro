#![feature(test)]

extern crate test;
extern crate rand;
extern crate xoroshiro;

const RAND_BENCH_N: u64 = 10_000;
const RAND_BENCH_BYTES: usize = 1_000_000;

use std::mem::size_of;
use test::{black_box, Bencher};
use rand::{XorShiftRng, IsaacRng, Isaac64Rng, Rng, OsRng};
use xoroshiro::{XoroShiroRng, SplitMixRng, XorShift1024Rng, AesRng};

macro_rules! make_bench_u64 {
    ($name:ident, $rng:ident) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut rng: $rng = OsRng::new().unwrap().gen();
            b.iter(|| {
                for _ in 0..RAND_BENCH_N {
                    black_box(rng.gen::<u64>());
                }
            });
            b.bytes = size_of::<u64>() as u64 * RAND_BENCH_N;
        }
    }
}

macro_rules! make_bench_bytes {
    ($name:ident, $rng:ident) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut rng: $rng = OsRng::new().unwrap().gen();
            let mut buf = vec![0; RAND_BENCH_BYTES];
            b.iter(|| {
                rng.fill_bytes(&mut buf);
            });
            b.bytes = RAND_BENCH_BYTES as u64;
        }
    }
}

make_bench_u64!(rand_u64_xorshift, XorShiftRng);
make_bench_u64!(rand_u64_isaac, IsaacRng);
make_bench_u64!(rand_u64_isaac64, Isaac64Rng);
make_bench_u64!(rand_u64_xoroshiro, XoroShiroRng);
make_bench_u64!(rand_u64_xorshif1024, XorShift1024Rng);
make_bench_u64!(rand_u64_splitmix, SplitMixRng);
make_bench_u64!(rand_u64_aes, AesRng);

make_bench_bytes!(rand_bytes_xorshift, XorShiftRng);
make_bench_bytes!(rand_bytes_isaac, IsaacRng);
make_bench_bytes!(rand_bytes_isaac64, Isaac64Rng);
make_bench_bytes!(rand_bytes_xoroshiro, XoroShiroRng);
make_bench_bytes!(rand_bytes_xorshif1024, XorShift1024Rng);
make_bench_bytes!(rand_bytes_splitmix, SplitMixRng);
make_bench_bytes!(rand_bytes_aes, AesRng);
