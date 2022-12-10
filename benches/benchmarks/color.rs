#![allow(deprecated)]

use criterion::{criterion_group, Criterion};


pub fn c_color(c: &mut Criterion) {
    c.bench_function("with_step: u8 to rgba", |b| b.iter(with_step));
    c.bench_function("with_chunks: u8 to rgba", |b| b.iter(with_chunks));
}

#[inline]
fn with_step() {}

#[inline]
fn with_chunks() {}

criterion_group!(bench, c_color);
