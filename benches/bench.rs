#![allow(unknown_lints)]

#[macro_use]
extern crate bencher;
extern crate rand;
extern crate xoroshiro;
extern crate byteorder;

const RAND_BENCH_N: u64 = 100_000;
const RAND_BENCH_BYTES: usize = 1 << 20;  // > 1_000_000

use std::mem::size_of;
use bencher::{black_box, Bencher};
use rand::{SeedableRng, Rng, RngCore};
use rand::{XorShiftRng, IsaacRng, Isaac64Rng, OsRng, ChaChaRng};
use xoroshiro::rng::{XoroShiro128, SplitMix64, XorShift1024};

//#[cfg(feature = "unstable")]
//mod aes;
//mod xoroshiro128star;
//mod smallprng;
//mod sfc64;
//mod urng;

//#[cfg(feature = "unstable")]
//use aes::AesRng;
//use xoroshiro128star::XoroShiro128 as XoroShiro128Star;
//use smallprng::SmallPrng128;
//use sfc64::Sfc64;
//use urng::Urng64;

macro_rules! make_bench_u64 {
    ($name:ident, $rng:ident) => {
        fn $name(b: &mut Bencher) {
            let mut rng = $rng::from_rng(OsRng::new().unwrap()).unwrap();
            b.iter(|| {
                for _ in 0..RAND_BENCH_N {
                    black_box(rng.gen::<u64>());
                }
            });
            b.bytes = size_of::<u64>() as u64 * RAND_BENCH_N;
        }
    }
}

macro_rules! make_bench_f64 {
    ($name:ident, $rng:ident) => {
        fn $name(b: &mut Bencher) {
            let mut rng = $rng::from_rng(OsRng::new().unwrap()).unwrap();
            b.iter(|| {
                for _ in 0..RAND_BENCH_N {
                    black_box(rng.gen::<f64>());
                }
            });
            b.bytes = size_of::<f64>() as u64 * RAND_BENCH_N;
        }
    }
}

macro_rules! make_bench_bytes {
    ($name:ident, $rng:ident) => {
        fn $name(b: &mut Bencher) {
            let mut rng = $rng::from_rng(OsRng::new().unwrap()).unwrap();
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
make_bench_u64!(rand_u64_chacha, ChaChaRng);
make_bench_u64!(rand_u64_xoroshiro128, XoroShiro128);
#[cfg(feature = "unstable")]
make_bench_u64!(rand_u64_xoroshiro128x4, XoroShiro128x4);
make_bench_u64!(rand_u64_xorshift1024, XorShift1024);
make_bench_u64!(rand_u64_splitmix, SplitMix64);
//#[cfg(feature = "unstable")]
//make_bench_u64!(rand_u64_aes, AesRng);
//make_bench_u64!(rand_u64_xoroshirostar, XoroShiro128Star);
//make_bench_u64!(rand_u64_smallprng, SmallPrng128);
//make_bench_u64!(rand_u64_sfc64, Sfc64);
//make_bench_u64!(rand_u64_urng64, Urng64);

make_bench_f64!(rand_f64_xorshift, XorShiftRng);
make_bench_f64!(rand_f64_isaac, IsaacRng);
make_bench_f64!(rand_f64_isaac64, Isaac64Rng);
make_bench_f64!(rand_f64_chacha, ChaChaRng);
make_bench_f64!(rand_f64_xoroshiro128, XoroShiro128);
#[cfg(feature = "unstable")]
make_bench_f64!(rand_f64_xoroshiro128x4, XoroShiro128x4);
make_bench_f64!(rand_f64_xorshift1024, XorShift1024);
make_bench_f64!(rand_f64_splitmix, SplitMix64);
//#[cfg(feature = "unstable")]
//make_bench_f64!(rand_f64_aes, AesRng);
//make_bench_f64!(rand_f64_xoroshirostar, XoroShiro128Star);
//make_bench_f64!(rand_f64_smallprng, SmallPrng128);
//make_bench_f64!(rand_f64_sfc64, Sfc64);
//make_bench_f64!(rand_f64_urng64, Urng64);

make_bench_bytes!(rand_bytes_xorshift, XorShiftRng);
make_bench_bytes!(rand_bytes_isaac, IsaacRng);
make_bench_bytes!(rand_bytes_isaac64, Isaac64Rng);
make_bench_bytes!(rand_bytes_chacha, ChaChaRng);
make_bench_bytes!(rand_bytes_xoroshiro128, XoroShiro128);
#[cfg(feature = "unstable")]
make_bench_bytes!(rand_bytes_xoroshiro128x4, XoroShiro128x4);
make_bench_bytes!(rand_bytes_xorshift1024, XorShift1024);
make_bench_bytes!(rand_bytes_splitmix, SplitMix64);
//#[cfg(feature = "unstable")]
//make_bench_bytes!(rand_bytes_aes, AesRng);
//make_bench_bytes!(rand_bytes_xoroshirostar, XoroShiro128Star);
//make_bench_bytes!(rand_bytes_smallprng, SmallPrng128);
//make_bench_bytes!(rand_bytes_sfc64, Sfc64);
//make_bench_bytes!(rand_bytes_urng64, Urng64);

#[cfg(feature = "unstable")]
benchmark_group!(benches,
    rand_u64_xorshift,
    rand_u64_isaac,
    rand_u64_isaac64,
    rand_u64_chacha,
    rand_u64_xoroshiro128,
    rand_u64_xoroshiro128x4,
    rand_u64_xorshift1024,
    rand_u64_splitmix,
    //rand_u64_aes,
    //rand_u64_xoroshirostar,
    //rand_u64_smallprng,
    //rand_u64_sfc64,
    //rand_u64_urng64,

    rand_f64_xorshift,
    rand_f64_isaac,
    rand_f64_isaac64,
    rand_f64_chacha,
    rand_f64_xoroshiro128,
    rand_f64_xoroshiro128x4,
    rand_f64_xorshift1024,
    rand_f64_splitmix,
    //rand_f64_aes,
    //rand_f64_xoroshirostar,
    //rand_f64_smallprng,
    //rand_f64_sfc64,
    //rand_f64_urng64,

    rand_bytes_xorshift,
    rand_bytes_isaac,
    rand_bytes_isaac64,
    rand_bytes_chacha,
    rand_bytes_xoroshiro128,
    rand_bytes_xoroshiro128x4,
    rand_bytes_xorshift1024,
    rand_bytes_splitmix,
    //rand_bytes_aes,
    //rand_bytes_xoroshirostar,
    //rand_bytes_smallprng,
    //rand_bytes_sfc64,
    //rand_bytes_urng64
);
#[cfg(not(feature = "unstable"))]
benchmark_group!(benches,
    rand_u64_xorshift,
    rand_u64_isaac,
    rand_u64_isaac64,
    rand_u64_chacha,
    rand_u64_xoroshiro128,
    rand_u64_xorshift1024,
    rand_u64_splitmix,
    //rand_u64_xoroshirostar,
    //rand_u64_smallprng,
    //rand_u64_sfc64,
    //rand_u64_urng64,

    rand_f64_xorshift,
    rand_f64_isaac,
    rand_f64_isaac64,
    rand_f64_chacha,
    rand_f64_xoroshiro128,
    rand_f64_xorshift1024,
    rand_f64_splitmix,
    //rand_f64_xoroshirostar,
    //rand_f64_smallprng,
    //rand_f64_sfc64,
    //rand_f64_urng64,

    rand_bytes_xorshift,
    rand_bytes_isaac,
    rand_bytes_isaac64,
    rand_bytes_chacha,
    rand_bytes_xoroshiro128,
    rand_bytes_xorshift1024,
    rand_bytes_splitmix,
    //rand_bytes_xoroshirostar,
    //rand_bytes_smallprng,
    //rand_bytes_sfc64,
    //rand_bytes_urng64
);
benchmark_main!(benches);
