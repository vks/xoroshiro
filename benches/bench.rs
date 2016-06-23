#![feature(test)]

extern crate test;
extern crate rand;
extern crate xoroshiro;

const RAND_BENCH_N: u64 = 1000;

use std::mem::size_of;
use test::{black_box, Bencher};
use rand::{XorShiftRng, StdRng, IsaacRng, Isaac64Rng, Rng, OsRng};
use xoroshiro::XoroShiroRng;

#[bench]
fn rand_xorshift(b: &mut Bencher) {
    let mut rng: XorShiftRng = OsRng::new().unwrap().gen();
    b.iter(|| {
        for _ in 0..RAND_BENCH_N {
            black_box(rng.gen::<usize>());
        }
    });
    b.bytes = size_of::<usize>() as u64 * RAND_BENCH_N;
}

#[bench]
fn rand_isaac(b: &mut Bencher) {
    let mut rng: IsaacRng = OsRng::new().unwrap().gen();
    b.iter(|| {
        for _ in 0..RAND_BENCH_N {
            black_box(rng.gen::<usize>());
        }
    });
    b.bytes = size_of::<usize>() as u64 * RAND_BENCH_N;
}

#[bench]
fn rand_isaac64(b: &mut Bencher) {
    let mut rng: Isaac64Rng = OsRng::new().unwrap().gen();
    b.iter(|| {
        for _ in 0..RAND_BENCH_N {
            black_box(rng.gen::<usize>());
        }
    });
    b.bytes = size_of::<usize>() as u64 * RAND_BENCH_N;
}

#[bench]
fn rand_std(b: &mut Bencher) {
    let mut rng = StdRng::new().unwrap();
    b.iter(|| {
        for _ in 0..RAND_BENCH_N {
            black_box(rng.gen::<usize>());
        }
    });
    b.bytes = size_of::<usize>() as u64 * RAND_BENCH_N;
}

#[bench]
fn rand_xoroshiro(b: &mut Bencher) {
    let mut rng: XoroShiroRng = OsRng::new().unwrap().gen();
    b.iter(|| {
        for _ in 0..RAND_BENCH_N {
            black_box(rng.gen::<usize>());
        }
    });
    b.bytes = size_of::<usize>() as u64 * RAND_BENCH_N;
}
