#![feature(portable_simd)]

use bitmask_benchmark::*;
use criterion::Criterion;
use std::hint::black_box;

fn bench_8bit(c: &mut Criterion) {
    use elem_8bit::*;
    let m = 0b10;

    c.bench_function("m8x2::from_bitmask_scalar", |b| {
        b.iter(|| len_2::from_bitmask_scalar(black_box(m)))
    });
    c.bench_function("m8x2::from_bitmask_simd", |b| {
        b.iter(|| len_2::from_bitmask_simd(black_box(m)))
    });

    let m = 0b1010;
    c.bench_function("m8x4::from_bitmask_simd", |b| {
        b.iter(|| len_4::from_bitmask_simd(black_box(m)))
    });
    c.bench_function("m8x4::from_bitmask_scalar", |b| {
        b.iter(|| len_4::from_bitmask_scalar(black_box(m)))
    });

    let m = 0b10101010;
    c.bench_function("m8x8::from_bitmask_simd", |b| {
        b.iter(|| len_8::from_bitmask_simd(black_box(m)))
    });

    let m = 0b1010101010101010;
    c.bench_function("m8x16::from_bitmask_simd", |b| {
        b.iter(|| len_16::from_bitmask_simd(black_box(m)))
    });
}

fn bench_16bit(c: &mut Criterion) {
    use elem_16bit::*;
    let m = 0b10;

    c.bench_function("m16x2::from_bitmask_scalar", |b| {
        b.iter(|| len_2::from_bitmask_scalar(black_box(m)))
    });
    c.bench_function("m16x2::from_bitmask_simd", |b| {
        b.iter(|| len_2::from_bitmask_simd(black_box(m)))
    });

    let m = 0b1010;
    c.bench_function("m16x4::from_bitmask_simd", |b| {
        b.iter(|| len_4::from_bitmask_simd(black_box(m)))
    });
    c.bench_function("m16x4::from_bitmask_scalar", |b| {
        b.iter(|| len_4::from_bitmask_scalar(black_box(m)))
    });

    let m = 0b10101010;
    c.bench_function("m16x8::from_bitmask_simd", |b| {
        b.iter(|| len_8::from_bitmask_simd(black_box(m)))
    });
}

fn bench_32bit(c: &mut Criterion) {
    use elem_32bit::*;
    let m = 0b10;

    c.bench_function("m32x2::from_bitmask_scalar", |b| {
        b.iter(|| len_2::from_bitmask_scalar(black_box(m)))
    });
    c.bench_function("m32x2::from_bitmask_simd", |b| {
        b.iter(|| len_2::from_bitmask_simd(black_box(m)))
    });

    let m = 0b1010;
    c.bench_function("m32x4::from_bitmask_simd", |b| {
        b.iter(|| len_4::from_bitmask_simd(black_box(m)))
    });
    c.bench_function("m32x4::from_bitmask_scalar", |b| {
        b.iter(|| len_4::from_bitmask_scalar(black_box(m)))
    });
}


criterion::criterion_group!(benches, bench_8bit, bench_16bit, bench_32bit);
criterion::criterion_main!(benches);