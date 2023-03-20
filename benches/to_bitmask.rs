#![feature(portable_simd)]

use bitmask_benchmark::*;
use criterion::Criterion;
use std::{hint::black_box, simd::Mask};

fn bench_8bit(c: &mut Criterion) {
    use elem_8bit::*;
    let m = Mask::from_array([true, false]);

    c.bench_function("m8x2::to_bitmask_scalar", |b| {
        b.iter(|| len_2::to_bitmask_scalar(black_box(m)))
    });
    c.bench_function("m8x2::to_bitmask_simd", |b| {
        b.iter(|| len_2::to_bitmask_simd(black_box(m)))
    });
    c.bench_function("m8x2::to_bitmask_shift", |b| {
        b.iter(|| len_2::to_bitmask_shift(black_box(m)))
    });

    let m = Mask::from_array([true, false, true, false]);
    c.bench_function("m8x4::to_bitmask_simd", |b| {
        b.iter(|| len_4::to_bitmask_simd(black_box(m)))
    });
    c.bench_function("m8x4::to_bitmask_mul", |b| {
        b.iter(|| len_4::to_bitmask_mul(black_box(m)))
    });

    let m = Mask::from_array([true, false, true, false, true, false, true, false]);
    c.bench_function("m8x8::to_bitmask_simd", |b| {
        b.iter(|| len_8::to_bitmask_simd(black_box(m)))
    });
    c.bench_function("m8x8::to_bitmask_mul", |b| {
        b.iter(|| len_8::to_bitmask_mul(black_box(m)))
    });

    let m = Mask::from_array([
        true, false, true, false, true, false, true, false, true, false, true, false, true, false,
        true, false,
    ]);
    c.bench_function("m8x16::to_bitmask_simd", |b| {
        b.iter(|| len_16::to_bitmask_simd(black_box(m)))
    });
    c.bench_function("m8x16::to_bitmask_mul", |b| {
        b.iter(|| len_16::to_bitmask_mul(black_box(m)))
    });
}

fn bench_16bit(c: &mut Criterion) {
    use elem_16bit::*;
    let m = Mask::from_array([true, false]);

    c.bench_function("m16x2::to_bitmask_scalar", |b| {
        b.iter(|| len_2::to_bitmask_scalar(black_box(m)))
    });
    c.bench_function("m16x2::to_bitmask_simd", |b| {
        b.iter(|| len_2::to_bitmask_simd(black_box(m)))
    });
    c.bench_function("m16x2::to_bitmask_shift", |b| {
        b.iter(|| len_2::to_bitmask_shift(black_box(m)))
    });

    let m = Mask::from_array([true, false, true, false]);
    c.bench_function("m16x4::to_bitmask_simd", |b| {
        b.iter(|| len_4::to_bitmask_simd(black_box(m)))
    });
    c.bench_function("m16x4::to_bitmask_mul", |b| {
        b.iter(|| len_4::to_bitmask_mul(black_box(m)))
    });

    let m = Mask::from_array([true, false, true, false, true, false, true, false]);
    c.bench_function("m16x8::to_bitmask_simd", |b| {
        b.iter(|| len_8::to_bitmask_simd(black_box(m)))
    });
    c.bench_function("m16x8::to_bitmask_mul", |b| {
        b.iter(|| len_8::to_bitmask_mul(black_box(m)))
    });
}

fn bench_32bit(c: &mut Criterion) {
    use elem_32bit::*;
    let m = Mask::from_array([true, false]);

    c.bench_function("m32x2::to_bitmask_scalar", |b| {
        b.iter(|| len_2::to_bitmask_scalar(black_box(m)))
    });
    c.bench_function("m32x2::to_bitmask_simd", |b| {
        b.iter(|| len_2::to_bitmask_simd(black_box(m)))
    });
    c.bench_function("m32x2::to_bitmask_shift", |b| {
        b.iter(|| len_2::to_bitmask_shift(black_box(m)))
    });

    let m = Mask::from_array([true, false, true, false]);
    c.bench_function("m32x4::to_bitmask_simd", |b| {
        b.iter(|| len_4::to_bitmask_simd(black_box(m)))
    });
    c.bench_function("m32x4::to_bitmask_mul", |b| {
        b.iter(|| len_4::to_bitmask_mul(black_box(m)))
    });
}

criterion::criterion_group!(benches, bench_8bit, bench_16bit, bench_32bit);
criterion::criterion_main!(benches);