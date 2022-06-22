#![allow(deprecated)]

use criterion::{criterion_group, Criterion};
use rmg::color::rgba::TransRgba;

pub static mut RES_BUFFER: Vec<u32> = Vec::new();
pub static mut COLOR_BUFFER: Vec<u8> = Vec::new();

pub fn c_color(c: &mut Criterion) {
    c.bench_function("with_step: u8 to rgba", |b| b.iter(with_step));
    c.bench_function("with_chunks: u8 to rgba", |b| b.iter(with_chunks));
}

#[inline]
fn with_step() {
    unsafe {
        COLOR_BUFFER = vec![0; 40];

        for f in (4..COLOR_BUFFER.len()).step_by(4) {
            RES_BUFFER.push(TransRgba::rgba_to_u32(
                &COLOR_BUFFER[f - 4..f].try_into().unwrap(),
            ));
        }
    }
}

#[inline]
fn with_chunks() {
    unsafe {
        COLOR_BUFFER = vec![0; 40];

        for f in COLOR_BUFFER.as_slice().chunks(4) {
            RES_BUFFER.push(TransRgba::rgba_to_u32(f.try_into().unwrap()));
        }
    }
}

criterion_group!(bench, c_color);
