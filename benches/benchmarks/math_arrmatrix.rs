#![allow(deprecated)]

use criterion::{criterion_group, Criterion};
use rmg::math::arrmatrix::{Affine, ArrMatrix};

pub const SIZE: usize = 2000;

pub fn c_translate_y(c: &mut Criterion) {
    fn run() {
        let matrix = ArrMatrix {
            arr: [0; SIZE * SIZE].as_slice(),
            width: SIZE as u32,
            height: SIZE as u32,
        };

        matrix.translate_y(4, true).unwrap();
    }

    c.bench_function("fib 20", |b| b.iter(|| run()));
}

criterion_group!(bench, c_translate_y,);
